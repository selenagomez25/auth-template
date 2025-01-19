mod config;
mod hwid;
mod webhook;
mod minecraft;

use std::sync::{Arc, Mutex};
use warp::Filter;

use config::Config;
use hwid::HwidList;
use webhook::send_webhook_embed;
use minecraft::{get_minecraft_uuid, get_avatar_url};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct UserInfo {
    hwid: String,
    username: Option<String>,
    ip: Option<String>,
}

#[tokio::main]
async fn main() {
    let config = Arc::new(Config::load());
    let valid_hwids = Arc::new(Mutex::new(HwidList::load()));

    let authenticate_hwid = warp::path("auth")
        .and(warp::post())
        .and(warp::header::<String>("X-API-Key"))
        .and(warp::body::json())
        .and(with_valid_hwids(valid_hwids.clone()))
        .and(with_config(config.clone()))
        .map(move |api_key: String, user_info: UserInfo, valid_hwids: Arc<Mutex<HwidList>>, config: Arc<Config>| {
            if api_key != config.api_key {
                return warp::reply::with_status(
                    warp::reply::json(&"invalid api key"), 
                    warp::http::StatusCode::UNAUTHORIZED
                );
            }

            let reloaded_hwids = HwidList::load();
            *valid_hwids.lock().unwrap() = reloaded_hwids;

            let hwid_list = valid_hwids.lock().unwrap();
            let success = hwid_list.is_valid(&user_info.hwid);
            let message = if success { "authentication succeeded" } else { "authentication failed" };

            let avatar_url = if let Some(username) = user_info.username.clone() {
                let handle = tokio::spawn(async move {
                    match get_minecraft_uuid(&username).await {
                        Some(uuid) => match get_avatar_url(&uuid).await {
                            Ok(url) => url,
                            Err(_) => String::new(),
                        },
                        None => String::new(),
                    }
                });
                Some(handle)
            } else {
                None
            };

            tokio::spawn({
                let config_clone = config.clone();
                let hwid = user_info.hwid.clone();
                let username = user_info.username.clone().unwrap_or_default();
                let ip = user_info.ip.clone().unwrap_or_default();
                async move {
                    let avatar_url = if let Some(handle) = avatar_url {
                        handle.await.unwrap_or_default()
                    } else {
                        String::new()
                    };

                    send_webhook_embed(
                        &config_clone,
                        success,
                        &hwid,
                        &username,
                        &ip,
                        Some(avatar_url)
                    ).await;
                }
            });

            let status = if success {
                warp::http::StatusCode::OK
            } else {
                warp::http::StatusCode::UNAUTHORIZED
            };

            warp::reply::with_status(warp::reply::json(&message), status)
        });

    let routes = authenticate_hwid;

    println!("Server is running on http://127.0.0.1:{}", config.port);
    warp::serve(routes).run(([127, 0, 0, 1], config.port)).await;
}

fn with_valid_hwids(
    valid_hwids: Arc<Mutex<HwidList>>,
) -> impl Filter<Extract = (Arc<Mutex<HwidList>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || valid_hwids.clone())
}

fn with_config(
    config: Arc<Config>,
) -> impl Filter<Extract = (Arc<Config>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || config.clone())
}