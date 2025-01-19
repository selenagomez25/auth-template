use reqwest;
use serde::{Deserialize, Serialize};
use log::{info, error};
use std::fs;
use std::sync::{Arc, Mutex};
use warp::Filter;
use rand::Rng; 

#[derive(Clone, Serialize, Deserialize)]
struct HwidList {
    valid_hwids: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Config {
    webhook_url: String,
    embed_success_message: String,
    embed_failure_message: String,
    api_key: String,
    port: u16,
}

#[derive(Clone, Serialize, Deserialize)]
struct UserInfo {
    hwid: String,
    username: Option<String>,
    ip: Option<String>,
}

async fn get_avatar_url(uuid: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://crafatar.com/avatars/{}?size=128&overlay", uuid);
    Ok(url)
}

async fn get_minecraft_uuid(username: &str) -> Option<String> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", username);
    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                let response_json: serde_json::Value = response.json().await.unwrap_or_default();
                response_json.get("id").and_then(|id| id.as_str().map(String::from))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

async fn send_webhook_embed(webhook_url: &str, title: &str, description: &str, color: u32, avatar_url: Option<String>) {
    let client = reqwest::Client::new();
    let mut payload = serde_json::json!({
        "embeds": [{
            "title": title,
            "description": description,
            "color": color
        }]
    });

    if let Some(url) = avatar_url {
        payload["embeds"][0]["thumbnail"] = serde_json::json!({
            "url": url
        });
    }

    match client.post(webhook_url).json(&payload).send().await {
        Ok(response) => {
            if response.status().is_success() {
                info!("Webhook sent successfully: {}", title);
            } else {
                error!("Failed to send webhook: {}", response.status());
            }
        }
        Err(e) => error!("Error sending webhook: {}", e),
    }
}

fn load_valid_hwids() -> HwidList {
    let data = fs::read_to_string("hwids.yaml")
        .expect("Unable to read hwids.yaml");
    serde_yaml::from_str(&data)
        .expect("Unable to parse YAML")
}

fn load_config() -> Config {
    let data = fs::read_to_string("config.yaml").expect("Unable to read config.yaml");
    let mut config: Config = serde_yaml::from_str(&data).expect("Unable to parse YAML");

    // all this does is settings the port to a random number if ur port is 0
    if config.port == 0 {
        let mut rng = rand::thread_rng();
        config.port = rng.gen_range(1024..65535)
    }

    config
}

fn is_valid_api_key(provided_key: &str, config: &Config) -> bool {
    provided_key == config.api_key
}

#[tokio::main]
async fn main() {
    let config = Arc::new(load_config());
    let valid_hwids = Arc::new(Mutex::new(load_valid_hwids()));

    let authenticate_hwid = warp::path("auth")
        .and(warp::post())
        .and(warp::header::<String>("X-API-Key"))
        .and(warp::body::json())
        .and(with_valid_hwids(valid_hwids.clone()))
        .and(with_config(config.clone()))
        .map(move |api_key: String, user_info: UserInfo, valid_hwids: Arc<Mutex<HwidList>>, config: Arc<Config>| {
            if !is_valid_api_key(&api_key, &config) {
                return warp::reply::with_status(
                    warp::reply::json(&"invalid api key"), 
                    warp::http::StatusCode::UNAUTHORIZED
                );
            }

            let reloaded_hwids = load_valid_hwids();
            *valid_hwids.lock().unwrap() = reloaded_hwids;

            let hwid_list = valid_hwids.lock().unwrap();
            let success = hwid_list.valid_hwids.contains(&user_info.hwid);
            let message = if success {
                "authentication succeeded"
            } else {
                "authentication failed"
            };

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
                    let title = if success {
                        "Authentication Succeeded"
                    } else {
                        "Authentication Failed"
                    };
                    let color = if success { 0x00FF00 } else { 0xFF0000 };
                    let description = format!(
                        "HWID: {}\nUsername: {}\nIP: {}",
                        hwid, username, ip
                    );

                    let avatar_url = if let Some(handle) = avatar_url {
                        handle.await.unwrap_or_default()
                    } else {
                        String::new()
                    };

                    send_webhook_embed(
                        &config_clone.webhook_url,
                        title,
                        &description,
                        color,
                        Some(avatar_url),
                    )
                    .await;
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