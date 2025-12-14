//! Figma API client for fetching file data and exporting images.

use crate::figma_client::FigmaAuth;
use crate::DpcError;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::Deserialize;
use thiserror::Error;

use super::api_types::{FigmaImageExport, FigmaNodesResponse, ImageFormat};

#[derive(Debug, Error)]
pub enum FigmaError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Figma API error ({status}): {message}")]
    Api { status: u16, message: String },
    #[error("Missing access token")]
    MissingToken,
    #[error("Invalid file key: {0}")]
    InvalidFileKey(String),
    #[error("Node not found: {0}")]
    NodeNotFound(String),
    #[error("Rate limited, retry after {0} seconds")]
    RateLimited(u64),
}

pub fn map_figma_error(e: FigmaError) -> DpcError {
    match e {
        FigmaError::Request(req_err) => DpcError::Network(req_err),
        FigmaError::Api { status, message } => DpcError::FigmaApi {
            status: Some(
                reqwest::StatusCode::from_u16(status)
                    .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
            ),
            message,
        },
        FigmaError::MissingToken => DpcError::Config(
            "Missing Figma token; set FIGMA_TOKEN or FIGMA_OAUTH_TOKEN".to_string(),
        ),
        FigmaError::InvalidFileKey(key) => DpcError::FigmaApi {
            status: None,
            message: format!("Invalid Figma file key: {}", key),
        },
        FigmaError::NodeNotFound(id) => DpcError::FigmaApi {
            status: None,
            message: format!("Node not found: {}", id),
        },
        FigmaError::RateLimited(secs) => DpcError::FigmaApi {
            status: Some(reqwest::StatusCode::TOO_MANY_REQUESTS),
            message: format!("Rate limited, retry after {} seconds", secs),
        },
    }
}

#[derive(Debug)]
pub struct FigmaClient {
    client: reqwest::Client,
    access_token: String,
    base_url: String,
}

impl FigmaClient {
    pub fn new(access_token: impl Into<String>) -> std::result::Result<Self, FigmaError> {
        Self::from_auth(FigmaAuth::PersonalAccessToken(access_token.into()))
    }

    pub fn from_auth(auth: FigmaAuth) -> std::result::Result<Self, FigmaError> {
        Self::with_base_url(auth, "https://api.figma.com/v1")
    }

    pub fn with_base_url(
        auth: FigmaAuth,
        base_url: impl Into<String>,
    ) -> std::result::Result<Self, FigmaError> {
        let token = match &auth {
            FigmaAuth::PersonalAccessToken(token) | FigmaAuth::OAuthToken(token) => token.clone(),
        };

        if token.is_empty() {
            return Err(FigmaError::MissingToken);
        }

        let mut headers = HeaderMap::new();
        match auth {
            FigmaAuth::PersonalAccessToken(token) => {
                headers.insert(
                    reqwest::header::HeaderName::from_static("x-figma-token"),
                    HeaderValue::from_str(&token).map_err(|_| FigmaError::MissingToken)?,
                );
            }
            FigmaAuth::OAuthToken(token) => {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token))
                        .map_err(|_| FigmaError::MissingToken)?,
                );
            }
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .no_proxy()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            access_token: token,
            base_url: base_url.into(),
        })
    }

    pub async fn get_file(
        &self,
        file_key: &str,
    ) -> std::result::Result<super::api_types::FigmaFile, FigmaError> {
        let url = format!("{}/files/{}", self.base_url, file_key);
        let response = self.client.get(&url).send().await?;

        self.handle_response(response).await
    }

    pub async fn get_file_nodes(
        &self,
        file_key: &str,
        node_ids: &[&str],
    ) -> std::result::Result<FigmaNodesResponse, FigmaError> {
        let ids = node_ids.join(",");
        let url = format!("{}/files/{}/nodes?ids={}", self.base_url, file_key, ids);
        let response = self.client.get(&url).send().await?;

        self.handle_response(response).await
    }

    pub async fn export_image(
        &self,
        file_key: &str,
        node_id: &str,
        format: ImageFormat,
        scale: f32,
    ) -> std::result::Result<String, FigmaError> {
        let url = format!(
            "{}/images/{}?ids={}&format={}&scale={}",
            self.base_url,
            file_key,
            node_id,
            format.as_str(),
            scale
        );

        let response = self.client.get(&url).send().await?;
        let export: FigmaImageExport = self.handle_response(response).await?;

        export
            .images
            .get(node_id)
            .cloned()
            .ok_or_else(|| FigmaError::NodeNotFound(node_id.to_string()))
    }

    pub async fn download_image(&self, url: &str) -> std::result::Result<Vec<u8>, FigmaError> {
        let response = self.client.get(url).send().await?;
        if !response.status().is_success() {
            return Err(FigmaError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }
        Ok(response.bytes().await?.to_vec())
    }

    async fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        response: reqwest::Response,
    ) -> std::result::Result<T, FigmaError> {
        let status = response.status();

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);
            return Err(FigmaError::RateLimited(retry_after));
        }

        if !status.is_success() {
            let message = response.text().await.unwrap_or_default();
            return Err(FigmaError::Api {
                status: status.as_u16(),
                message,
            });
        }

        Ok(response.json().await?)
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }
}
