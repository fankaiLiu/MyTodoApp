use gloo_net::http::Response;
use serde::{de::DeserializeOwned, Serialize};
use crate::api::error::{ApiError, ApiResult};
use crate::store::get_local_storage_item;

const DEFAULT_BASE_URL: &str = "http://[::1]:5800";
const TOKEN_KEY: &str = "todo_token";

#[derive(Clone)]
pub struct ApiClient {
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: Option<String>) -> Self {
        Self {
            base_url: base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn get_token() -> Option<String> {
        get_local_storage_item(TOKEN_KEY)
    }

    async fn do_get(&self, path: &str) -> ApiResult<Response> {
        let url = self.build_url(path);
        let mut req = gloo_net::http::Request::get(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");
        if let Some(token) = Self::get_token() {
            req = req.header("Authorization", &format!("Bearer {}", token));
        }
        req.send().await.map_err(|e| ApiError::network(e.to_string()))
    }

    async fn do_post<B: Serialize>(&self, path: &str, body: &B) -> ApiResult<Response> {
        let url = self.build_url(path);
        let json_str = serde_json::to_string(body).map_err(|e| ApiError::network(e.to_string()))?;
        let mut builder = gloo_net::http::Request::post(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");
        if let Some(token) = Self::get_token() {
            builder = builder.header("Authorization", &format!("Bearer {}", token));
        }
        builder
            .json(&json_str)
            .map_err(|e| ApiError::network(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::network(e.to_string()))
    }

    async fn do_put<B: Serialize>(&self, path: &str, body: &B) -> ApiResult<Response> {
        let url = self.build_url(path);
        let json_str = serde_json::to_string(body).map_err(|e| ApiError::network(e.to_string()))?;
        let mut builder = gloo_net::http::Request::put(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");
        if let Some(token) = Self::get_token() {
            builder = builder.header("Authorization", &format!("Bearer {}", token));
        }
        builder
            .json(&json_str)
            .map_err(|e| ApiError::network(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::network(e.to_string()))
    }

    async fn do_delete(&self, path: &str) -> ApiResult<Response> {
        let url = self.build_url(path);
        let mut req = gloo_net::http::Request::delete(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");
        if let Some(token) = Self::get_token() {
            req = req.header("Authorization", &format!("Bearer {}", token));
        }
        req.send().await.map_err(|e| ApiError::network(e.to_string()))
    }

    async fn handle_response<T: DeserializeOwned>(response: Response) -> ApiResult<T> {
        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(|e| ApiError::network(e.to_string()))?;

        if status >= 400 {
            let api_error: ApiError = serde_json::from_str(&text).unwrap_or_else(|_| {
                ApiError::new(status, "Unknown Error".to_string(), text.clone())
            });
            return Err(api_error);
        }

        serde_json::from_str(&text).map_err(|e| {
            ApiError::new(
                status,
                "Parse Error".to_string(),
                format!("Failed to parse response: {}", e),
            )
        })
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> ApiResult<T> {
        let resp = self.do_get(path).await?;
        Self::handle_response(resp).await
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> ApiResult<T> {
        let resp = self.do_post(path, body).await?;
        Self::handle_response(resp).await
    }

    pub async fn put<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> ApiResult<T> {
        let resp = self.do_put(path, body).await?;
        Self::handle_response(resp).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> ApiResult<T> {
        let resp = self.do_delete(path).await?;
        Self::handle_response(resp).await
    }
}
