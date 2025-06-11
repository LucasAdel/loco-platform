use leptos::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use gloo_net::http::Request;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub description: String,
    pub company: String,
    pub location: String,
    pub salary_range: Option<String>,
    pub job_type: String,
    pub is_urgent: bool,
    pub created_at: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub user_type: Option<String>,
    pub tenant_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub user_type: String,
    pub tenant_slug: Option<String>,
}

// Supabase configuration from environment
const SUPABASE_URL: &str = "https://kpmmsogskffsiubbegvc.supabase.co";
const SUPABASE_ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImtwbW1zb2dza2Zmc2l1YmJlZ3ZjIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDkxMDQ5NDcsImV4cCI6MjA2NDY4MDk0N30.SqVBXS-r8eG0jxo2lCdHiCKEiAHDpTJbKqfr0NGeSqM";

/// Supabase authentication client for Leptos
pub struct SupabaseAuth;

impl SupabaseAuth {
    /// Get the API base URL
    fn api_url() -> String {
        // Use backend API for now, will switch to Supabase direct later
        "http://localhost:3070/api/v1".to_string()
    }
    
    /// Get Supabase URL
    fn supabase_url() -> String {
        SUPABASE_URL.to_string()
    }

    /// Sign in with email and password
    pub async fn sign_in(email: String, password: String) -> Result<AuthResponse, String> {
        let request = LoginRequest { email, password };
        
        Request::post(&format!("{}/auth/login", Self::api_url()))
            .header("Content-Type", "application/json")
            .json(&request)
            .map_err(|e| format!("Failed to create request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?
            .json::<AuthResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Sign up new user
    pub async fn sign_up(request: RegisterRequest) -> Result<AuthResponse, String> {
        Request::post(&format!("{}/auth/register", Self::api_url()))
            .header("Content-Type", "application/json")
            .json(&request)
            .map_err(|e| format!("Failed to create request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?
            .json::<AuthResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Sign out
    pub async fn sign_out(token: &str) -> Result<(), String> {
        Request::post(&format!("{}/auth/logout", Self::api_url()))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;
        
        Ok(())
    }

    /// Get current user profile
    pub async fn get_user(token: &str) -> Result<serde_json::Value, String> {
        Request::get(&format!("{}/profile", Self::api_url()))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Update user profile
    pub async fn update_user(token: &str, data: serde_json::Value) -> Result<serde_json::Value, String> {
        Request::put(&format!("{}/profile", Self::api_url()))
            .header("Authorization", &format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Failed to create request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Request password reset
    pub async fn reset_password(email: String) -> Result<serde_json::Value, String> {
        let request = json!({ "email": email });
        
        Request::post(&format!("{}/auth/forgot-password", Self::api_url()))
            .header("Content-Type", "application/json")
            .json(&request)
            .map_err(|e| format!("Failed to create request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Verify OTP
    pub async fn verify_otp(email: String, token: String, otp_type: String) -> Result<AuthResponse, String> {
        let request = json!({
            "email": email,
            "token": token,
            "type": otp_type
        });
        
        Request::post(&format!("{}/auth/verify-otp", Self::api_url()))
            .header("Content-Type", "application/json")
            .json(&request)
            .map_err(|e| format!("Failed to create request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Get OAuth sign in URL
    pub async fn get_oauth_url(provider: &str) -> Result<serde_json::Value, String> {
        Request::get(&format!("{}/auth/oauth/{}", Self::api_url(), provider))
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Store auth token in local storage
    pub fn store_token(token: &str) -> Result<(), String> {
        let window = web_sys::window().ok_or("No window object")?;
        let storage = window.local_storage()
            .map_err(|_| "Failed to access local storage")?
            .ok_or("No local storage available")?;
        
        storage.set_item("sb-access-token", token)
            .map_err(|_| "Failed to store token")?;
        
        Ok(())
    }

    /// Get auth token from local storage
    pub fn get_token() -> Option<String> {
        let window = web_sys::window()?;
        let storage = window.local_storage().ok()??;
        storage.get_item("sb-access-token").ok()?
    }

    /// Remove auth token from local storage
    pub fn remove_token() -> Result<(), String> {
        let window = web_sys::window().ok_or("No window object")?;
        let storage = window.local_storage()
            .map_err(|_| "Failed to access local storage")?
            .ok_or("No local storage available")?;
        
        storage.remove_item("sb-access-token")
            .map_err(|_| "Failed to remove token")?;
        
        Ok(())
    }
}

/// Auth context provider for Leptos
#[derive(Clone, Debug)]
pub struct AuthContext {
    pub user: RwSignal<Option<UserInfo>>,
    pub token: RwSignal<Option<String>>,
    pub loading: RwSignal<bool>,
}

impl AuthContext {
    pub fn new() -> Self {
        Self {
            user: RwSignal::new(None),
            token: RwSignal::new(None),
            loading: RwSignal::new(false),
        }
    }

    /// Check if user is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.user.get().is_some()
    }

    /// Sign in user
    pub async fn sign_in(&self, email: String, password: String) -> Result<(), String> {
        self.loading.set(true);
        
        match SupabaseAuth::sign_in(email, password).await {
            Ok(response) => {
                self.user.set(Some(response.user));
                self.token.set(Some(response.access_token.clone()));
                SupabaseAuth::store_token(&response.access_token)?;
                self.loading.set(false);
                Ok(())
            }
            Err(e) => {
                self.loading.set(false);
                Err(e)
            }
        }
    }

    /// Sign up user
    pub async fn sign_up(&self, request: RegisterRequest) -> Result<(), String> {
        self.loading.set(true);
        
        match SupabaseAuth::sign_up(request).await {
            Ok(response) => {
                self.user.set(Some(response.user));
                self.token.set(Some(response.access_token.clone()));
                SupabaseAuth::store_token(&response.access_token)?;
                self.loading.set(false);
                Ok(())
            }
            Err(e) => {
                self.loading.set(false);
                Err(e)
            }
        }
    }

    /// Sign out user
    pub async fn sign_out(&self) -> Result<(), String> {
        if let Some(token) = self.token.get() {
            SupabaseAuth::sign_out(&token).await?;
        }
        
        self.user.set(None);
        self.token.set(None);
        SupabaseAuth::remove_token()?;
        Ok(())
    }

    /// Initialize auth from stored token
    pub async fn init_auth(&self) -> Result<(), String> {
        if let Some(token) = SupabaseAuth::get_token() {
            self.loading.set(true);
            
            match SupabaseAuth::get_user(&token).await {
                Ok(user_data) => {
                    // Parse user data and set context
                    if let Ok(user) = serde_json::from_value::<UserInfo>(user_data["user"].clone()) {
                        self.user.set(Some(user));
                        self.token.set(Some(token));
                    }
                    self.loading.set(false);
                    Ok(())
                }
                Err(_) => {
                    // Token might be expired, clear it
                    SupabaseAuth::remove_token()?;
                    self.loading.set(false);
                    Ok(())
                }
            }
        } else {
            Ok(())
        }
    }
}

/// Provide auth context to components
#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let auth_context = AuthContext::new();
    let auth_clone = auth_context.clone();
    
    // Initialize auth on mount
    Effect::new(move |_| {
        let auth = auth_clone.clone();
        spawn_local(async move {
            let _ = auth.init_auth().await;
        });
    });
    
    provide_context(auth_context);
    
    children()
}

/// Hook to use auth context
pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>()
        .expect("AuthContext not found. Make sure to wrap your app with AuthProvider")
}

/// Provide auth context
pub fn provide_auth_context() {
    let auth_context = AuthContext::new();
    provide_context(auth_context);
}

/// Get Supabase client instance
pub fn use_supabase() -> SupabaseAuth {
    SupabaseAuth
}

impl SupabaseAuth {
    /// Get all active jobs
    pub async fn get_jobs(&self) -> Result<Vec<Job>, String> {
        // For now, return mock data
        Ok(vec![
            Job {
                id: "1".to_string(),
                title: "Senior Pharmacist".to_string(),
                description: "Leading pharmacy role in Sydney CBD".to_string(),
                company: "HealthPlus Pharmacy".to_string(),
                location: "Sydney, NSW".to_string(),
                salary_range: Some("$90k - $110k".to_string()),
                job_type: "Full-time".to_string(),
                is_urgent: true,
                created_at: "2025-01-06".to_string(),
                latitude: Some(-33.8688),
                longitude: Some(151.2093),
                status: "active".to_string(),
            },
            Job {
                id: "2".to_string(),
                title: "Locum Pharmacist".to_string(),
                description: "Flexible locum position across Melbourne".to_string(),
                company: "MediStaff Solutions".to_string(),
                location: "Melbourne, VIC".to_string(),
                salary_range: Some("$50/hr - $65/hr".to_string()),
                job_type: "Contract".to_string(),
                is_urgent: false,
                created_at: "2025-01-05".to_string(),
                latitude: Some(-37.8136),
                longitude: Some(144.9631),
                status: "active".to_string(),
            },
            Job {
                id: "3".to_string(),
                title: "Hospital Pharmacist".to_string(),
                description: "Join our hospital pharmacy team in Brisbane".to_string(),
                company: "Brisbane General Hospital".to_string(),
                location: "Brisbane, QLD".to_string(),
                salary_range: Some("$85k - $95k".to_string()),
                job_type: "Full-time".to_string(),
                is_urgent: true,
                created_at: "2025-01-04".to_string(),
                latitude: Some(-27.4698),
                longitude: Some(153.0251),
                status: "active".to_string(),
            },
        ])
    }
}