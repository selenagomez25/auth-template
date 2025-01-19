pub async fn get_avatar_url(uuid: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://crafatar.com/avatars/{}", uuid);
    Ok(url)
}

pub async fn get_minecraft_uuid(username: &str) -> Option<String> {
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