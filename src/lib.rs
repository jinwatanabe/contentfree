use serde::Deserialize;
use anyhow::bail;
use std::env;

#[derive(Debug, Deserialize)]
pub struct ContentfulResponse<T> {
    pub includes: Includes,
    pub items: Vec<Item<T>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Includes {
		#[serde(rename = "Asset")]
    pub asset: Vec<Asset>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Asset {
    pub sys: Sys,
    pub fields: AssetFields,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AssetFields {
    pub file: File,
}

#[derive(Debug, Deserialize, Clone)]
pub struct File {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Sys {
    pub id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Item<T> {
    pub sys: Sys,
    pub fields: T,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
	pub sys: Sys,
}

pub struct ContentfulClient {
    url: String,
}

impl ContentfulClient {
    pub async fn get_content(&self) -> anyhow::Result<reqwest::Response> {
        let client = reqwest::Client::new();
        let response = client
            .get(self.url.as_str())
            .send()
            .await?;

        if !response.status().is_success() {
            let status_code = response.status().to_string();
            let body = response.text().await?;
            tracing::error!(status_code, body, "fetch error");
            bail!("api returned non success status code.")
        }

        return Ok(response);
    }

    pub fn new(space: String, content_type: String) -> Self {
        dotenv::dotenv().ok();
        let base_url = std::env::var("CONTENTFUL_BASE_URL").unwrap_or_else(|_| "https://cdn.contentful.com/space".to_string());
        let access_token = env::var("CONTENTFUL_ACCESS_TOKEN").expect("CONTENTFUL_ACCESS_TOKEN must be set");
        let url = format!("{}/{}/entries?content-type={}&access_token={}", base_url, space, content_type, access_token);

        Self {
            url
        }
    }
}

