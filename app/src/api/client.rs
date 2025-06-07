use leptos::*;
use shared::types::ApiError;

#[derive(Clone)]
pub struct ApiClient {
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        // In production, this would come from environment config
        let base_url = if cfg!(debug_assertions) {
            "http://localhost:3070".to_string()
        } else {
            window().location().origin().unwrap_or_default()
        };
        
        Self { base_url }
    }

    pub async fn get<T>(&self, path: &str) -> Result<T, ApiError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        
        let response = gloo_net::http::Request::get(&url)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| ApiError::Network(e.to_string()))?;
        
        if response.ok() {
            response
                .json()
                .await
                .map_err(|e| ApiError::Parse(e.to_string()))
        } else {
            Err(ApiError::Http(response.status()))
        }
    }

    pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<T, ApiError>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let url = format!("{}{}", self.base_url, path);
        let body_str = serde_json::to_string(body)
            .map_err(|e| ApiError::Parse(e.to_string()))?;
        
        let response = gloo_net::http::Request::post(&url)
            .header("Content-Type", "application/json")
            .body(body_str)
            .map_err(|e| ApiError::Network(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::Network(e.to_string()))?;
        
        if response.ok() {
            response
                .json()
                .await
                .map_err(|e| ApiError::Parse(e.to_string()))
        } else {
            Err(ApiError::Http(response.status()))
        }
    }
}

// Provide API client as context
pub fn provide_api_client() {
    provide_context(ApiClient::new());
}

pub fn use_api_client() -> ApiClient {
    use_context::<ApiClient>()
        .expect("ApiClient not provided. Make sure to call provide_api_client() in your app.")
}