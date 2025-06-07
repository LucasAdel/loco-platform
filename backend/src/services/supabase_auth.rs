use anyhow::Result;
use axum::http::StatusCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use shared::supabase::{SupabaseAuthResponse, SupabaseConfig, SupabaseUser};
use uuid::Uuid;

/// Supabase authentication service
pub struct SupabaseAuthService {
    client: Client,
    config: SupabaseConfig,
}

impl SupabaseAuthService {
    pub fn new(config: SupabaseConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    /// Sign up a new user
    pub async fn sign_up(&self, input: SignUpInput) -> Result<SupabaseAuthResponse> {
        let response = self
            .client
            .post(format!("{}/auth/v1/signup", self.config.url))
            .header("apikey", &self.config.anon_key)
            .header("Content-Type", "application/json")
            .json(&input)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: SupabaseError = response.json().await?;
            return Err(anyhow::anyhow!("Sign up failed: {}", error.msg));
        }

        Ok(response.json().await?)
    }

    /// Sign in with email and password
    pub async fn sign_in(&self, input: SignInInput) -> Result<SupabaseAuthResponse> {
        let response = self
            .client
            .post(format!("{}/auth/v1/token?grant_type=password", self.config.url))
            .header("apikey", &self.config.anon_key)
            .header("Content-Type", "application/json")
            .json(&input)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: SupabaseError = response.json().await?;
            return Err(anyhow::anyhow!("Sign in failed: {}", error.msg));
        }

        Ok(response.json().await?)
    }

    /// Sign out (revoke refresh token)
    pub async fn sign_out(&self, access_token: &str) -> Result<()> {
        let response = self
            .client
            .post(format!("{}/auth/v1/logout", self.config.url))
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            let error: SupabaseError = response.json().await?;
            return Err(anyhow::anyhow!("Sign out failed: {}", error.msg));
        }

        Ok(())
    }

    /// Get user details
    pub async fn get_user(&self, access_token: &str) -> Result<SupabaseUser> {
        let response = self
            .client
            .get(format!("{}/auth/v1/user", self.config.url))
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            let error: SupabaseError = response.json().await?;
            return Err(anyhow::anyhow!("Get user failed: {}", error.msg));
        }

        Ok(response.json().await?)
    }

    /// Update user metadata
    pub async fn update_user(&self, access_token: &str, input: UpdateUserInput) -> Result<SupabaseUser> {
        let response = self
            .client
            .put(format!("{}/auth/v1/user", self.config.url))
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&input)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: SupabaseError = response.json().await?;
            return Err(anyhow::anyhow!("Update user failed: {}", error.msg));
        }

        Ok(response.json().await?)
    }

    /// Send password reset email
    pub async fn reset_password(&self, email: &str) -> Result<()> {
        let input = serde_json::json!({
            "email": email,
            "type": "recovery"
        });

        let response = self
            .client
            .post(format!("{}/auth/v1/recover", self.config.url))
            .header("apikey", &self.config.anon_key)
            .header("Content-Type", "application/json")
            .json(&input)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: SupabaseError = response.json().await?;
            return Err(anyhow::anyhow!("Password reset failed: {}", error.msg));
        }

        Ok(())
    }

    /// Refresh access token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<SupabaseAuthResponse> {
        let input = serde_json::json!({
            "refresh_token": refresh_token
        });

        let response = self
            .client
            .post(format!("{}/auth/v1/token?grant_type=refresh_token", self.config.url))
            .header("apikey", &self.config.anon_key)
            .header("Content-Type", "application/json")
            .json(&input)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: SupabaseError = response.json().await?;
            return Err(anyhow::anyhow!("Token refresh failed: {}", error.msg));
        }

        Ok(response.json().await?)
    }

    /// Verify user's email with OTP
    pub async fn verify_otp(&self, input: VerifyOtpInput) -> Result<SupabaseAuthResponse> {
        let response = self
            .client
            .post(format!("{}/auth/v1/verify", self.config.url))
            .header("apikey", &self.config.anon_key)
            .header("Content-Type", "application/json")
            .json(&input)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: SupabaseError = response.json().await?;
            return Err(anyhow::anyhow!("OTP verification failed: {}", error.msg));
        }

        Ok(response.json().await?)
    }

    /// Sign in with OAuth provider
    pub async fn sign_in_with_oauth(&self, provider: &str) -> Result<OAuthResponse> {
        let url = format!(
            "{}/auth/v1/authorize?provider={}&redirect_to=http://localhost:3080/auth/callback",
            self.config.url, provider
        );

        Ok(OAuthResponse { url })
    }

    /// Admin: Create user (requires service role key)
    pub async fn admin_create_user(&self, input: AdminCreateUserInput) -> Result<SupabaseUser> {
        let service_role_key = self.config.service_role_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Service role key not configured"))?;

        let response = self
            .client
            .post(format!("{}/auth/v1/admin/users", self.config.url))
            .header("apikey", service_role_key)
            .header("Authorization", format!("Bearer {}", service_role_key))
            .header("Content-Type", "application/json")
            .json(&input)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: SupabaseError = response.json().await?;
            return Err(anyhow::anyhow!("Admin create user failed: {}", error.msg));
        }

        Ok(response.json().await?)
    }

    /// Admin: Delete user (requires service role key)
    pub async fn admin_delete_user(&self, user_id: Uuid) -> Result<()> {
        let service_role_key = self.config.service_role_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Service role key not configured"))?;

        let response = self
            .client
            .delete(format!("{}/auth/v1/admin/users/{}", self.config.url, user_id))
            .header("apikey", service_role_key)
            .header("Authorization", format!("Bearer {}", service_role_key))
            .send()
            .await?;

        if !response.status().is_success() {
            let error: SupabaseError = response.json().await?;
            return Err(anyhow::anyhow!("Admin delete user failed: {}", error.msg));
        }

        Ok(())
    }
}

// Request/Response types

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpInput {
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyOtpInput {
    pub email: String,
    pub token: String,
    pub r#type: String, // "signup" or "recovery"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthResponse {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminCreateUserInput {
    pub email: String,
    pub password: String,
    pub email_confirm: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SupabaseError {
    msg: String,
    code: Option<String>,
}

// Extension methods for Axum integration
impl SupabaseAuthService {
    /// Create auth cookie from response
    pub fn create_auth_cookie(auth_response: &SupabaseAuthResponse) -> String {
        format!(
            "sb-access-token={}; HttpOnly; Secure; SameSite=Lax; Max-Age={}",
            auth_response.access_token,
            auth_response.expires_in
        )
    }

    /// Extract token from cookie or header
    pub fn extract_token(headers: &axum::http::HeaderMap) -> Option<String> {
        // Try Authorization header first
        if let Some(auth_header) = headers.get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    return Some(token.to_string());
                }
            }
        }

        // Try cookie
        if let Some(cookie_header) = headers.get("Cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                for cookie in cookie_str.split(';') {
                    let cookie = cookie.trim();
                    if let Some(token) = cookie.strip_prefix("sb-access-token=") {
                        return Some(token.to_string());
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_token_from_header() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            "Authorization",
            "Bearer test-token-123".parse().unwrap(),
        );

        let token = SupabaseAuthService::extract_token(&headers);
        assert_eq!(token, Some("test-token-123".to_string()));
    }

    #[test]
    fn test_extract_token_from_cookie() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            "Cookie",
            "sb-access-token=cookie-token-456; other=value".parse().unwrap(),
        );

        let token = SupabaseAuthService::extract_token(&headers);
        assert_eq!(token, Some("cookie-token-456".to_string()));
    }
}