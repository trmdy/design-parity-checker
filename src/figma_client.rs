use crate::error::{DpcError, Result};
#[cfg(test)]
use reqwest::header::HeaderMap;
use reqwest::{header::RETRY_AFTER, Client, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

const DEFAULT_BASE_URL: &str = "https://api.figma.com";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone)]
pub enum FigmaAuth {
    PersonalAccessToken(String),
    OAuthToken(String),
}

impl FigmaAuth {
    pub fn from_env() -> Option<Self> {
        if let Ok(token) = std::env::var("FIGMA_TOKEN") {
            if !token.is_empty() {
                return Some(Self::PersonalAccessToken(token));
            }
        }

        if let Ok(token) = std::env::var("FIGMA_OAUTH_TOKEN") {
            if !token.is_empty() {
                return Some(Self::OAuthToken(token));
            }
        }

        None
    }

    fn apply(&self, builder: RequestBuilder) -> RequestBuilder {
        self.apply_headers(builder)
    }

    fn apply_headers(&self, builder: RequestBuilder) -> RequestBuilder {
        match self {
            FigmaAuth::PersonalAccessToken(token) => builder.header("X-FIGMA-TOKEN", token),
            FigmaAuth::OAuthToken(token) => builder.bearer_auth(token),
        }
    }

    #[cfg(test)]
    fn apply_to_header_map(&self, headers: &mut HeaderMap) {
        match self {
            FigmaAuth::PersonalAccessToken(token) => {
                headers.insert("X-FIGMA-TOKEN", token.parse().unwrap());
            }
            FigmaAuth::OAuthToken(token) => {
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {token}").parse().unwrap(),
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FigmaClient {
    http: Client,
    auth: FigmaAuth,
    base_url: Url,
}

/// Public-facing alias to avoid clashing with the figma.rs client.
pub type FigmaApiClient = FigmaClient;

impl FigmaClient {
    pub fn new(auth: FigmaAuth) -> Result<Self> {
        Self::with_base_url_and_timeout(auth, DEFAULT_BASE_URL, DEFAULT_TIMEOUT)
    }

    pub fn with_base_url(auth: FigmaAuth, base_url: impl AsRef<str>) -> Result<Self> {
        Self::with_base_url_and_timeout(auth, base_url, DEFAULT_TIMEOUT)
    }

    pub fn with_base_url_and_timeout(
        auth: FigmaAuth,
        base_url: impl AsRef<str>,
        timeout: Duration,
    ) -> Result<Self> {
        let base_url = Url::parse(base_url.as_ref())?;

        let http = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(DpcError::Network)?;

        Ok(Self {
            http,
            auth,
            base_url,
        })
    }

    pub async fn fetch_file(&self, file_key: &str) -> Result<FigmaFileResponse> {
        let url = self.endpoint(&format!("/v1/files/{file_key}"))?;
        let req = self.authed(self.http.get(url));
        self.send_json(req).await
    }

    pub async fn fetch_nodes(
        &self,
        file_key: &str,
        node_ids: &[String],
    ) -> Result<FigmaNodesResponse> {
        if node_ids.is_empty() {
            return Err(DpcError::Config(
                "node_ids cannot be empty when fetching nodes from Figma".into(),
            ));
        }

        let ids = node_ids.join(",");
        let url = self.endpoint(&format!("/v1/files/{file_key}/nodes?ids={ids}"))?;
        let req = self.authed(self.http.get(url));
        self.send_json(req).await
    }

    pub async fn export_images(
        &self,
        file_key: &str,
        node_ids: &[String],
        options: ImageExportOptions,
    ) -> Result<FigmaImageResponse> {
        validate_node_ids(node_ids, "exporting Figma images")?;
        validate_scale(options.scale)?;

        let ids = node_ids.join(",");
        let url = self.endpoint(&format!(
            "/v1/images/{file_key}?ids={ids}&scale={scale}&format={format}",
            scale = options.scale,
            format = options.format.as_str(),
        ))?;

        let req = self.authed(self.http.get(url));
        self.send_json(req).await
    }

    pub async fn export_image(
        &self,
        file_key: &str,
        node_id: &str,
        options: ImageExportOptions,
    ) -> Result<String> {
        let resp = self
            .export_images(file_key, &[node_id.to_string()], options)
            .await?;
        resp.images
            .get(node_id)
            .cloned()
            .ok_or_else(|| DpcError::FigmaApi {
                status: None,
                message: format!("image URL missing for node {node_id}"),
            })
    }

    pub async fn download_image(&self, url: &str) -> Result<Vec<u8>> {
        let response = self.http.get(url).send().await.map_err(DpcError::Network)?;

        let status = response.status();

        if status.is_success() {
            return response
                .bytes()
                .await
                .map(|b| b.to_vec())
                .map_err(DpcError::Network);
        }

        let body = response.text().await.unwrap_or_default();
        Err(DpcError::figma_api(
            Some(status),
            format!(
                "failed to download image (status {}): {}",
                status.as_u16(),
                body
            ),
        ))
    }

    fn authed(&self, builder: RequestBuilder) -> RequestBuilder {
        self.auth.apply(builder)
    }

    fn endpoint(&self, path: &str) -> Result<Url> {
        self.base_url.join(path).map_err(DpcError::InvalidUrl)
    }

    async fn send_json<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T> {
        let response = request.send().await.map_err(DpcError::Network)?;
        let status = response.status();
        let retry_after = response
            .headers()
            .get(RETRY_AFTER)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);

        let body = response.text().await.unwrap_or_default();

        if status.is_success() {
            return serde_json::from_str(&body).map_err(DpcError::Serialization);
        }

        Err(DpcError::figma_api(
            Some(status),
            error_message(status, &body, retry_after.as_deref()),
        ))
    }
}

fn validate_node_ids(node_ids: &[String], context: &str) -> Result<()> {
    if node_ids.is_empty() {
        return Err(DpcError::Config(format!(
            "node_ids cannot be empty when {context}"
        )));
    }
    Ok(())
}

fn validate_scale(scale: f32) -> Result<()> {
    if scale <= 0.0 {
        return Err(DpcError::Config(
            "scale must be greater than zero for Figma exports".into(),
        ));
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct FigmaFileResponse {
    pub name: Option<String>,
    #[serde(default)]
    pub last_modified: Option<String>,
    #[serde(default)]
    pub document: Value,
    #[serde(default)]
    pub components: HashMap<String, Value>,
    #[serde(default)]
    pub styles: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct FigmaNodesResponse {
    #[serde(default)]
    pub nodes: HashMap<String, FigmaNodeContainer>,
}

#[derive(Debug, Deserialize)]
pub struct FigmaNodeContainer {
    pub document: Value,
    #[serde(default)]
    pub components: Option<Value>,
    #[serde(default)]
    pub styles: Option<Value>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FigmaImageResponse {
    #[serde(default)]
    pub images: HashMap<String, String>,
    #[serde(default)]
    pub err: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct ImageExportOptions {
    pub scale: f32,
    pub format: FigmaImageFormat,
}

impl Default for ImageExportOptions {
    fn default() -> Self {
        Self {
            scale: 1.0,
            format: FigmaImageFormat::Png,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FigmaImageFormat {
    Png,
    Jpg,
    Svg,
}

impl FigmaImageFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            FigmaImageFormat::Png => "png",
            FigmaImageFormat::Jpg => "jpg",
            FigmaImageFormat::Svg => "svg",
        }
    }
}

fn error_message(status: StatusCode, body: &str, retry_after: Option<&str>) -> String {
    let fallback = format!("Figma API returned status {}", status.as_u16());
    let parsed = serde_json::from_str::<Value>(body).ok();
    let from_body = parsed
        .as_ref()
        .and_then(|value| value.get("err").or_else(|| value.get("error")))
        .and_then(Value::as_str)
        .map(str::to_owned);

    match (status, retry_after, from_body) {
        (StatusCode::TOO_MANY_REQUESTS, Some(retry), Some(msg)) => {
            format!("{msg} (rate limited, retry after {retry}s)")
        }
        (StatusCode::TOO_MANY_REQUESTS, Some(retry), None) => {
            format!("rate limited by Figma API, retry after {retry}s")
        }
        (_, _, Some(msg)) => msg,
        _ => fallback,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderMap;
    use std::env;

    #[test]
    fn personal_access_token_sets_expected_header() {
        let auth = FigmaAuth::PersonalAccessToken("secret".into());
        let mut headers = HeaderMap::new();
        auth.apply_to_header_map(&mut headers);

        assert_eq!(headers.get("X-FIGMA-TOKEN").unwrap(), "secret");
    }

    #[test]
    fn oauth_token_sets_bearer_auth_header() {
        let auth = FigmaAuth::OAuthToken("oauth_secret".into());
        let mut headers = HeaderMap::new();
        auth.apply_to_header_map(&mut headers);

        let header = headers
            .get(reqwest::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .unwrap();

        assert_eq!(header, "Bearer oauth_secret");
    }

    #[tokio::test]
    async fn export_images_rejects_empty_nodes() {
        let result = validate_node_ids(&[], "exporting Figma images");

        assert!(matches!(result, Err(DpcError::Config(_))));
    }

    #[test]
    fn auth_from_env_prefers_figma_token() {
        let _guard = EnvGuard::new();
        env::set_var("FIGMA_TOKEN", "pat_token");
        env::set_var("FIGMA_OAUTH_TOKEN", "oauth_token");

        let auth = FigmaAuth::from_env().expect("auth from env");
        match auth {
            FigmaAuth::PersonalAccessToken(token) => assert_eq!(token, "pat_token"),
            _ => panic!("expected personal access token"),
        }
    }

    struct EnvGuard;

    impl EnvGuard {
        fn new() -> Self {
            EnvGuard
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            env::remove_var("FIGMA_TOKEN");
            env::remove_var("FIGMA_OAUTH_TOKEN");
        }
    }
}
