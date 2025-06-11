use leptos::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
    pub avatar_url: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub user: RwSignal<Option<User>>,
    pub is_authenticated: ReadSignal<bool>,
    pub loading: RwSignal<bool>,
}

impl AuthContext {
    pub async fn login(&self, email: String, password: String) -> Result<(), String> {
        self.loading.set(true);
        
        let response = Request::post("/api/v1/auth/login")
            .json(&serde_json::json!({
                "email": email,
                "password": password
            }))
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;
        
        self.loading.set(false);
        
        if response.ok() {
            let data: LoginResponse = response.json().await.map_err(|e| e.to_string())?;
            
            // Store token in localStorage
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("auth_token", &data.token);
                }
            }
            
            self.user.set(Some(data.user));
            Ok(())
        } else {
            Err("Invalid credentials".to_string())
        }
    }
    
    pub async fn logout(&self) {
        self.user.set(None);
        
        // Clear token from localStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item("auth_token");
            }
        }
        
        // Call logout endpoint
        let _ = Request::post("/api/v1/auth/logout").send().await;
    }
    
    pub async fn check_auth(&self) -> Result<(), String> {
        // Get token from localStorage
        let token = if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                storage.get_item("auth_token").ok().flatten()
            } else {
                None
            }
        } else {
            None
        };
        
        if let Some(token) = token {
            let response = Request::get("/api/v1/profile")
                .header("Authorization", &format!("Bearer {}", token))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            
            if response.ok() {
                let user: User = response.json().await.map_err(|e| e.to_string())?;
                self.user.set(Some(user));
                Ok(())
            } else {
                Err("Invalid token".to_string())
            }
        } else {
            Err("No token found".to_string())
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LoginResponse {
    token: String,
    user: User,
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let user = RwSignal::new(None::<User>);
    let (is_authenticated, _) = signal_from(move || user.get().is_some());
    let loading = RwSignal::new(false);
    
    let auth_context = AuthContext {
        user,
        is_authenticated,
        loading,
    };
    
    // Check auth on mount
    {
        let auth = auth_context.clone();
        spawn_local(async move {
            let _ = auth.check_auth().await;
        });
    }
    
    provide_context(auth_context);
    
    children()
}

pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>().expect("AuthContext not found")
}