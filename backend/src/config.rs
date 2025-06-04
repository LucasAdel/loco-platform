use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub supabase_url: String,
    pub supabase_anon_key: String,
    pub mapbox_token: String,
    pub jwt_secret: String,
    pub superadmin_email: String,
    pub superadmin_password: String,
    pub app_env: String,
    pub log_level: String,
    pub api_url: String,
    pub frontend_url: String,
    pub app_version: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        
        Ok(Config {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://:memory:".to_string()), // Use SQLite in memory for demo
            supabase_url: std::env::var("SUPABASE_URL")
                .unwrap_or_else(|_| "https://demo.supabase.co".to_string()),
            supabase_anon_key: std::env::var("SUPABASE_ANON_KEY")
                .unwrap_or_else(|_| "demo-key".to_string()),
            mapbox_token: std::env::var("MAPBOX_TOKEN")
                .unwrap_or_else(|_| "demo-token".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
            superadmin_email: std::env::var("SUPERADMIN_EMAIL")
                .unwrap_or_else(|_| "admin@loco-platform.com".to_string()),
            superadmin_password: std::env::var("SUPERADMIN_PASSWORD")
                .unwrap_or_else(|_| "demo-password".to_string()),
            app_env: std::env::var("APP_ENV")
                .unwrap_or_else(|_| "development".to_string()),
            log_level: std::env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "debug".to_string()),
            api_url: std::env::var("API_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            frontend_url: std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            app_version: std::env::var("APP_VERSION")
                .unwrap_or_else(|_| "0.1.0".to_string()),
            host: std::env::var("HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
        })
    }
    
    pub fn is_development(&self) -> bool {
        self.app_env == "development"
    }
    
    pub fn is_production(&self) -> bool {
        self.app_env == "production"
    }
}