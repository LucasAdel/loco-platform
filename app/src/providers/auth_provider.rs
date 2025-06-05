use leptos::*;
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};
use shared::types::UserRole;
use uuid::Uuid;
use gloo_net::http::Request;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub tenant_id: Uuid,
    pub tenant_name: String,
    pub permissions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AuthState {
    Loading,
    Authenticated(AuthUser),
    Unauthenticated,
}

impl AuthState {
    pub fn is_authenticated(&self) -> bool {
        matches!(self, AuthState::Authenticated(_))
    }
    
    pub fn user(&self) -> Option<&AuthUser> {
        match self {
            AuthState::Authenticated(user) => Some(user),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub company_name: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: AuthUser,
}

// Supabase configuration from environment
pub fn get_supabase_config() -> (String, String) {
    #[cfg(target_arch = "wasm32")]
    {
        // In WASM, we can't access env vars directly, use hardcoded values or pass from build
        (
            "https://piziiyfwbljvwwqicvlc.supabase.co".to_string(),
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InBpemlpeWZ3Ymxqdnd3cWljdmxjIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDYxNzk3NzcsImV4cCI6MjA2MTc1NTc3N30.Akue6yeM-g4ugLxxKU6TcUSl4kxU06mUYNWUJ2IaTNc".to_string()
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // On server, read from environment
        (
            std::env::var("VITE_SUPABASE_URL").unwrap_or_else(|_| "https://piziiyfwbljvwwqicvlc.supabase.co".to_string()),
            std::env::var("VITE_SUPABASE_ANON_KEY").unwrap_or_else(|_| "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InBpemlpeWZ3Ymxqdnd3cWljdmxjIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDYxNzk3NzcsImV4cCI6MjA2MTc1NTc3N30.Akue6yeM-g4ugLxxKU6TcUSl4kxU06mUYNWUJ2IaTNc".to_string())
        )
    }
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let (auth_state, set_auth_state) = create_signal(AuthState::Loading);
    
    // Check for existing session on mount
    create_effect(move |_| {
        spawn_local(async move {
            if let Some(token) = get_stored_token() {
                match validate_session(&token).await {
                    Ok(user) => set_auth_state.set(AuthState::Authenticated(user)),
                    Err(_) => {
                        clear_stored_token();
                        set_auth_state.set(AuthState::Unauthenticated);
                    }
                }
            } else {
                set_auth_state.set(AuthState::Unauthenticated);
            }
        });
    });
    
    let login = create_action(move |input: &LoginRequest| {
        let input = input.clone();
        async move {
            let (supabase_url, supabase_anon_key) = get_supabase_config();
            let response = Request::post(&format!("{}/auth/v1/token?grant_type=password", supabase_url))
                .header("apikey", &supabase_anon_key)
                .header("Content-Type", "application/json")
                .json(&input)
                .unwrap()
                .send()
                .await
                .map_err(|e| format!("Login failed: {}", e))?;
                
            if response.ok() {
                let auth_response: AuthResponse = response.json().await
                    .map_err(|e| format!("Failed to parse response: {}", e))?;
                    
                store_token(&auth_response.access_token);
                set_auth_state.set(AuthState::Authenticated(auth_response.user));
                Ok(())
            } else {
                Err("Invalid credentials".to_string())
            }
        }
    });
    
    let logout = create_action(move |_: &()| async move {
        // Call Supabase logout endpoint
        let (supabase_url, supabase_anon_key) = get_supabase_config();
        let _ = Request::post(&format!("{}/auth/v1/logout", supabase_url))
            .header("apikey", &supabase_anon_key)
            .header("Authorization", &format!("Bearer {}", get_stored_token().unwrap_or_default()))
            .send()
            .await;
            
        clear_stored_token();
        set_auth_state.set(AuthState::Unauthenticated);
        
        // Redirect to login
        let navigate = use_navigate();
        navigate("/login", Default::default());
    });
    
    let register = create_action(move |input: &RegisterRequest| {
        let input = input.clone();
        async move {
            // First, create the user in Supabase Auth
            let (supabase_url, supabase_anon_key) = get_supabase_config();
            let auth_response = Request::post(&format!("{}/auth/v1/signup", supabase_url))
                .header("apikey", &supabase_anon_key)
                .header("Content-Type", "application/json")
                .json(&serde_json::json!({
                    "email": input.email,
                    "password": input.password,
                }))
                .unwrap()
                .send()
                .await
                .map_err(|e| format!("Registration failed: {}", e))?;
                
            if auth_response.ok() {
                // Create tenant and user profile
                let tenant_response = Request::post(&format!("{}/rest/v1/tenants", supabase_url))
                    .header("apikey", &supabase_anon_key)
                    .header("Content-Type", "application/json")
                    .json(&serde_json::json!({
                        "name": input.company_name,
                        "owner_email": input.email,
                    }))
                    .unwrap()
                    .send()
                    .await
                    .map_err(|e| format!("Failed to create tenant: {}", e))?;
                    
                Ok(())
            } else {
                Err("Registration failed".to_string())
            }
        }
    });
    
    provide_context(AuthContext {
        auth_state: auth_state.into(),
        login,
        logout,
        register,
    });
    
    children()
}

#[derive(Clone)]
pub struct AuthContext {
    pub auth_state: Signal<AuthState>,
    pub login: Action<LoginRequest, Result<(), String>>,
    pub logout: Action<(), ()>,
    pub register: Action<RegisterRequest, Result<(), String>>,
}

pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>()
        .expect("AuthContext not found. Make sure to wrap your app with AuthProvider")
}

// Helper functions
pub fn get_stored_token() -> Option<String> {
    #[cfg(feature = "hydrate")]
    {
        use web_sys::window;
        window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item("auth_token").ok().flatten())
    }
    
    #[cfg(not(feature = "hydrate"))]
    {
        None
    }
}

fn store_token(token: &str) {
    #[cfg(feature = "hydrate")]
    {
        use web_sys::window;
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item("auth_token", token);
        }
    }
}

fn clear_stored_token() {
    #[cfg(feature = "hydrate")]
    {
        use web_sys::window;
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.remove_item("auth_token");
        }
    }
}

async fn validate_session(token: &str) -> Result<AuthUser, String> {
    let (supabase_url, supabase_anon_key) = get_supabase_config();
    let response = Request::get(&format!("{}/auth/v1/user", supabase_url))
        .header("apikey", &supabase_anon_key)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Failed to validate session: {}", e))?;
        
    if response.ok() {
        response.json::<AuthUser>().await
            .map_err(|e| format!("Failed to parse user: {}", e))
    } else {
        Err("Invalid session".to_string())
    }
}

pub fn provide_auth_context() {
    // This is already handled in the AuthProvider component
    // This function exists for compatibility with the AppProviders component
}