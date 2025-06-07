use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub supabase_url: String,
    pub supabase_anon_key: String,
    pub api_url: String,
    pub app_env: String,
}

impl Config {
    pub fn from_env() -> Self {
        // In production, these would come from actual environment variables
        // For development, you can use dotenv or load from .env file
        Self {
            supabase_url: std::env::var("SUPABASE_URL")
                .unwrap_or_else(|_| "https://your-project.supabase.co".to_string()),
            supabase_anon_key: std::env::var("SUPABASE_ANON_KEY")
                .unwrap_or_else(|_| "your-anon-key".to_string()),
            api_url: std::env::var("API_URL")
                .unwrap_or_else(|_| "http://localhost:3070".to_string()),
            app_env: std::env::var("APP_ENV")
                .unwrap_or_else(|_| "development".to_string()),
        }
    }
    
    pub fn is_development(&self) -> bool {
        self.app_env == "development"
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::from_env);

// For WASM targets, we need to handle this differently
#[cfg(target_arch = "wasm32")]
pub fn init_config() {
    // In WASM, environment variables need to be injected at build time
    // or loaded from a config endpoint
}

#[cfg(not(target_arch = "wasm32"))]
pub fn init_config() {
    // On the server, we can use dotenv
    dotenv::dotenv().ok();
}