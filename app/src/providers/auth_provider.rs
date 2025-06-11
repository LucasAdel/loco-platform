use leptos::*;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthProvider {
    pub user: RwSignal<Option<User>>,
    pub is_authenticated: RwSignal<bool>,
    pub loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
}

impl AuthProvider {
    pub fn new() -> Self {
        // Check for existing session
        let stored_user = load_user_from_storage();
        let is_authenticated = stored_user.is_some();
        
        Self {
            user: create_rw_signal(stored_user),
            is_authenticated: create_rw_signal(is_authenticated),
            loading: create_rw_signal(false),
            error: create_rw_signal(None),
        }
    }
    
    pub fn login(&self, email: String, password: String) -> impl Future<Output = Result<(), String>> {
        let user_signal = self.user;
        let is_authenticated_signal = self.is_authenticated;
        let loading_signal = self.loading;
        let error_signal = self.error;
        
        async move {
            loading_signal.set(true);
            error_signal.set(None);
            
            // Simulate API call
            gloo_timers::future::TimeoutFuture::new(1000).await;
            
            // For demo, accept any login
            if email.is_empty() || password.is_empty() {
                loading_signal.set(false);
                error_signal.set(Some("Email and password are required".to_string()));
                return Err("Invalid credentials".to_string());
            }
            
            let user = User {
                id: "1".to_string(),
                email: email.clone(),
                name: Some("Demo User".to_string()),
                role: if email.contains("admin") { "admin".to_string() } else { "user".to_string() },
                avatar: None,
            };
            
            user_signal.set(Some(user.clone()));
            is_authenticated_signal.set(true);
            save_user_to_storage(&user);
            
            loading_signal.set(false);
            Ok(())
        }
    }
    
    pub fn register(&self, email: String, password: String, name: String) -> impl Future<Output = Result<(), String>> {
        let user_signal = self.user;
        let is_authenticated_signal = self.is_authenticated;
        let loading_signal = self.loading;
        let error_signal = self.error;
        
        async move {
            loading_signal.set(true);
            error_signal.set(None);
            
            // Simulate API call
            gloo_timers::future::TimeoutFuture::new(1000).await;
            
            // Validation
            if email.is_empty() || password.is_empty() || name.is_empty() {
                loading_signal.set(false);
                error_signal.set(Some("All fields are required".to_string()));
                return Err("All fields are required".to_string());
            }
            
            if password.len() < 6 {
                loading_signal.set(false);
                error_signal.set(Some("Password must be at least 6 characters".to_string()));
                return Err("Password too short".to_string());
            }
            
            let user = User {
                id: "1".to_string(),
                email: email.clone(),
                name: Some(name),
                role: "user".to_string(),
                avatar: None,
            };
            
            user_signal.set(Some(user.clone()));
            is_authenticated_signal.set(true);
            save_user_to_storage(&user);
            
            loading_signal.set(false);
            Ok(())
        }
    }
    
    pub fn logout(&self) {
        self.user.set(None);
        self.is_authenticated.set(false);
        self.error.set(None);
        clear_user_from_storage();
    }
    
    pub fn forgot_password(&self, email: String) -> impl Future<Output = Result<(), String>> {
        let loading_signal = self.loading;
        let error_signal = self.error;
        
        async move {
            loading_signal.set(true);
            error_signal.set(None);
            
            // Simulate API call
            gloo_timers::future::TimeoutFuture::new(1000).await;
            
            if email.is_empty() {
                loading_signal.set(false);
                error_signal.set(Some("Email is required".to_string()));
                return Err("Email is required".to_string());
            }
            
            loading_signal.set(false);
            Ok(())
        }
    }
    
    pub fn update_profile(&self, name: String) -> impl Future<Output = Result<(), String>> {
        let user_signal = self.user;
        let loading_signal = self.loading;
        let error_signal = self.error;
        
        async move {
            loading_signal.set(true);
            error_signal.set(None);
            
            // Simulate API call
            gloo_timers::future::TimeoutFuture::new(500).await;
            
            if let Some(mut user) = user_signal.get_untracked() {
                user.name = Some(name);
                user_signal.set(Some(user.clone()));
                save_user_to_storage(&user);
            }
            
            loading_signal.set(false);
            Ok(())
        }
    }
}

// Local storage helpers
fn load_user_from_storage() -> Option<User> {
    if let Ok(Some(storage)) = web_sys::window()
        .unwrap()
        .local_storage()
    {
        if let Ok(Some(user_str)) = storage.get_item("loco_user") {
            serde_json::from_str(&user_str).ok()
        } else {
            None
        }
    } else {
        None
    }
}

fn save_user_to_storage(user: &User) {
    if let Ok(Some(storage)) = web_sys::window()
        .unwrap()
        .local_storage()
    {
        if let Ok(user_str) = serde_json::to_string(user) {
            let _ = storage.set_item("loco_user", &user_str);
        }
    }
}

fn clear_user_from_storage() {
    if let Ok(Some(storage)) = web_sys::window()
        .unwrap()
        .local_storage()
    {
        let _ = storage.remove_item("loco_user");
    }
}

// Helper to use auth context
pub fn use_auth() -> AuthProvider {
    use_context::<AuthProvider>().expect("AuthProvider not found in context")
}

// Supabase configuration helpers
pub fn get_supabase_config() -> (String, String) {
    // These would normally come from environment variables
    let supabase_url = "https://your-project.supabase.co".to_string();
    let supabase_anon_key = "your-anon-key".to_string();
    (supabase_url, supabase_anon_key)
}

pub fn get_stored_token() -> Result<String, &'static str> {
    if let Ok(Some(storage)) = web_sys::window()
        .unwrap()
        .local_storage()
    {
        if let Ok(Some(token)) = storage.get_item("loco_auth_token") {
            Ok(token)
        } else {
            Err("No token in storage")
        }
    } else {
        Err("Storage not available")
    }
}