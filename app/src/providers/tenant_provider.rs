use leptos::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use crate::providers::auth_provider::use_auth;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub settings: TenantSettings,
    pub subscription_tier: SubscriptionTier,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TenantSettings {
    pub primary_colour: String,
    pub logo_url: Option<String>,
    pub custom_domain: Option<String>,
    pub features: Vec<String>,
    pub max_users: i32,
    pub max_job_posts: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SubscriptionTier {
    Free,
    Basic,
    Professional,
    Enterprise,
}

#[component]
pub fn TenantProvider(children: Children) -> impl IntoView {
    let auth = use_auth();
    let (current_tenant, set_current_tenant) = create_signal(None::<Tenant>);
    
    // Load tenant data when user is authenticated
    create_effect(move |_| {
        if auth.is_authenticated.get() {
            if let Some(user) = auth.user.get() {
                spawn_local(async move {
                    // Use a default tenant_id since the User struct doesn't have one
                    match load_tenant_data("default".to_string()).await {
                        Ok(tenant) => set_current_tenant.set(Some(tenant)),
                        Err(e) => {
                            leptos::logging::error!("Failed to load tenant data: {}", e);
                        }
                    }
                });
            }
        } else {
            set_current_tenant.set(None);
        }
    });
    
    provide_context(TenantContext {
        current_tenant: current_tenant.into(),
    });
    
    children()
}

#[derive(Clone)]
pub struct TenantContext {
    pub current_tenant: Signal<Option<Tenant>>,
}

pub fn use_tenant() -> TenantContext {
    use_context::<TenantContext>()
        .expect("TenantContext not found. Make sure to wrap your app with TenantProvider")
}

async fn load_tenant_data(tenant_id: String) -> Result<Tenant, String> {
    use gloo_net::http::Request;
    use crate::providers::auth_provider::{get_supabase_config, get_stored_token};
    
    let token = get_stored_token().map_err(|e| e.to_string())?;
    let (supabase_url, supabase_anon_key) = get_supabase_config();
    
    let response = Request::get(&format!("{}/rest/v1/tenants?id=eq.{}", supabase_url, tenant_id))
        .header("apikey", &supabase_anon_key)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch tenant: {}", e))?;
        
    if response.ok() {
        let tenants: Vec<Tenant> = response.json().await
            .map_err(|e| format!("Failed to parse tenant: {}", e))?;
            
        tenants.into_iter().next()
            .ok_or_else(|| "Tenant not found".to_string())
    } else {
        Err("Failed to load tenant data".to_string())
    }
}

// Helper function to check if a feature is enabled for the current tenant
pub fn use_tenant_feature(feature: &'static str) -> Signal<bool> {
    let tenant = use_tenant();
    
    create_memo(move |_| {
        tenant.current_tenant.get()
            .map(|t| t.settings.features.contains(&feature.to_string()))
            .unwrap_or(false)
    }).into()
}

// Helper to get tenant-specific API endpoints
pub fn use_tenant_api_url() -> Signal<String> {
    let tenant = use_tenant();
    
    create_memo(move |_| {
        if let Some(tenant) = tenant.current_tenant.get() {
            if let Some(custom_domain) = &tenant.settings.custom_domain {
                format!("https://api.{}", custom_domain)
            } else {
                format!("https://api.locoplatform.com/t/{}", tenant.slug)
            }
        } else {
            "https://api.locoplatform.com".to_string()
        }
    }).into()
}

pub fn provide_tenant_context() {
    // This is already handled in the TenantProvider component
    // This function exists for compatibility with the AppProviders component
}