use log::{info, error};
use reqwest;
use crate::config::{Config};

#[derive(Debug)]
pub struct WebhookEmbedBuilder {
    title: String,
    description: String,
    color: u32
}

impl WebhookEmbedBuilder {
    pub fn new(
        title: String, 
        description: String, 
        color: u32, 
    ) -> Self {
        Self {
            title,
            description,
            color
        }
    }

    pub async fn send(&self, webhook_url: &str, avatar_url: Option<String>, username: Option<&str>) {
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "username": username.unwrap_or("Authentication Bot"),
            "avatar_url": avatar_url.unwrap_or_default(),
            "embeds": [{
                "title": self.title,
                "description": self.description,
                "color": self.color
            }]
        });
    
        match client.post(webhook_url).json(&payload).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Webhook sent successfully: {}", self.title);
                } else {
                    error!("Failed to send webhook: {}", response.status());
                }
            }
            Err(e) => error!("Error sending webhook: {}", e),
        }
    }
}

pub async fn send_webhook_embed(
    config: &Config,
    success: bool,
    hwid: &str,
    username: &str,
    ip: &str,
    avatar_url: Option<String>
) {
    let embed_config = &config.webhook_embed;
    let (title, color) = if success {
        (
            embed_config.title_template.replace("{status}", "Authentication Succeeded"),
            embed_config.color_success
        )
    } else {
        (
            embed_config.title_template.replace("{status}", "Authentication Failed"),
            embed_config.color_failure
        )
    };

    let description = embed_config.description_template
        .replace("{hwid}", hwid)
        .replace("{username}", username)
        .replace("{ip}", ip);

    let embed = WebhookEmbedBuilder::new(
        title, 
        description, 
        color
    );

    embed.send(&config.webhook_url, avatar_url, Some(username)).await;
}