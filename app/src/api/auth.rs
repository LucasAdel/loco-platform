use crate::api::supabase::{LoginRequest, RegisterRequest, AuthResponse};
use super::client::use_api_client;

pub async fn login(credentials: LoginRequest) -> Result<AuthResponse, String> {
    let client = use_api_client();
    
    client.post("/api/auth/login", &credentials)
        .await
        .map_err(|e| format!("Login failed: {:?}", e))
}

pub async fn register(data: RegisterRequest) -> Result<AuthResponse, String> {
    let client = use_api_client();
    
    client.post("/api/auth/register", &data)
        .await
        .map_err(|e| format!("Registration failed: {:?}", e))
}

pub async fn logout() -> Result<(), String> {
    let client = use_api_client();
    
    client.post::<(), ()>("/api/auth/logout", &())
        .await
        .map_err(|e| format!("Logout failed: {:?}", e))
        .map(|_| ())
}

pub async fn refresh_token(refresh_token: &str) -> Result<AuthResponse, String> {
    let client = use_api_client();
    
    client.post("/api/auth/refresh", &serde_json::json!({
        "refresh_token": refresh_token
    }))
        .await
        .map_err(|e| format!("Token refresh failed: {:?}", e))
}