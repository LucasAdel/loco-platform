use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Supabase configuration
#[derive(Debug, Clone)]
pub struct SupabaseConfig {
    pub url: String,
    pub anon_key: String,
    pub service_role_key: Option<String>,
}

impl SupabaseConfig {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            url: std::env::var("VITE_SUPABASE_URL")
                .or_else(|_| std::env::var("SUPABASE_URL"))
                .map_err(|_| "SUPABASE_URL not found in environment")?,
            anon_key: std::env::var("VITE_SUPABASE_ANON_KEY")
                .or_else(|_| std::env::var("SUPABASE_ANON_KEY"))
                .map_err(|_| "SUPABASE_ANON_KEY not found in environment")?,
            service_role_key: std::env::var("SUPABASE_SERVICE_ROLE_KEY").ok(),
        })
    }
}

/// Multi-tenant database schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub domain: Option<String>,
    pub settings: TenantSettings,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantSettings {
    pub primary_colour: String,
    pub secondary_colour: String,
    pub logo_url: Option<String>,
    pub max_users: i32,
    pub max_jobs: i32,
    pub features: Vec<String>,
}

/// User with tenant association
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantUser {
    pub id: Uuid,
    pub email: String,
    pub tenant_id: Uuid,
    pub role: TenantRole,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TenantRole {
    Owner,
    Admin,
    Manager,
    Member,
}

/// Row-level security policies for multi-tenancy
pub mod rls_policies {
    pub const TENANT_ISOLATION: &str = r#"
        -- Ensure users can only see data from their tenant
        CREATE POLICY tenant_isolation ON public.jobs
        FOR ALL
        USING (tenant_id = auth.jwt() ->> 'tenant_id');
        
        CREATE POLICY tenant_isolation ON public.users
        FOR ALL
        USING (tenant_id = auth.jwt() ->> 'tenant_id');
        
        CREATE POLICY tenant_isolation ON public.applications
        FOR ALL
        USING (tenant_id = auth.jwt() ->> 'tenant_id');
    "#;
    
    pub const ROLE_BASED_ACCESS: &str = r#"
        -- Admin users can manage all tenant data
        CREATE POLICY admin_access ON public.jobs
        FOR ALL
        USING (
            EXISTS (
                SELECT 1 FROM public.tenant_users
                WHERE user_id = auth.uid()
                AND tenant_id = jobs.tenant_id
                AND role IN ('Owner', 'Admin')
            )
        );
    "#;
}

/// Supabase authentication types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupabaseAuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub user: SupabaseUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupabaseUser {
    pub id: Uuid,
    pub email: String,
    pub email_confirmed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub app_metadata: AppMetadata,
    pub user_metadata: UserMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMetadata {
    pub tenant_id: Uuid,
    pub role: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserMetadata {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

/// JWT claims for multi-tenant authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantClaims {
    pub sub: Uuid,           // User ID
    pub email: String,
    pub tenant_id: Uuid,
    pub role: String,
    pub permissions: Vec<String>,
    pub exp: i64,            // Expiration
    pub iat: i64,            // Issued at
}

/// Database migrations for multi-tenancy
pub mod migrations {
    pub const CREATE_TENANTS_TABLE: &str = r#"
        CREATE TABLE IF NOT EXISTS tenants (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL,
            slug VARCHAR(255) UNIQUE NOT NULL,
            domain VARCHAR(255) UNIQUE,
            settings JSONB NOT NULL DEFAULT '{}',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        
        CREATE INDEX idx_tenants_slug ON tenants(slug);
        CREATE INDEX idx_tenants_domain ON tenants(domain);
    "#;
    
    pub const CREATE_TENANT_USERS_TABLE: &str = r#"
        CREATE TABLE IF NOT EXISTS tenant_users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
            tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
            role VARCHAR(50) NOT NULL,
            permissions JSONB NOT NULL DEFAULT '[]',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            UNIQUE(user_id, tenant_id)
        );
        
        CREATE INDEX idx_tenant_users_user ON tenant_users(user_id);
        CREATE INDEX idx_tenant_users_tenant ON tenant_users(tenant_id);
    "#;
    
    pub const ADD_TENANT_ID_TO_TABLES: &str = r#"
        -- Add tenant_id to all business tables
        ALTER TABLE jobs ADD COLUMN tenant_id UUID REFERENCES tenants(id);
        ALTER TABLE applications ADD COLUMN tenant_id UUID REFERENCES tenants(id);
        ALTER TABLE users ADD COLUMN tenant_id UUID REFERENCES tenants(id);
        
        -- Create indexes for performance
        CREATE INDEX idx_jobs_tenant ON jobs(tenant_id);
        CREATE INDEX idx_applications_tenant ON applications(tenant_id);
        CREATE INDEX idx_users_tenant ON users(tenant_id);
    "#;
}

/// Helper functions for tenant context
pub mod tenant_context {
    use super::*;
    
    /// Extract tenant ID from JWT claims
    pub fn extract_tenant_id(_token: &str) -> Result<Uuid, String> {
        // In production, decode JWT and extract tenant_id
        // This is a placeholder implementation
        Err("JWT decoding not implemented".to_string())
    }
    
    /// Validate tenant access
    pub fn validate_tenant_access(
        user_tenant_id: Uuid,
        resource_tenant_id: Uuid,
    ) -> Result<(), String> {
        if user_tenant_id != resource_tenant_id {
            Err("Access denied: resource belongs to different tenant".to_string())
        } else {
            Ok(())
        }
    }
}

/// SQL functions for tenant operations
pub mod sql_functions {
    pub const GET_USER_TENANT: &str = r#"
        CREATE OR REPLACE FUNCTION get_user_tenant(user_id UUID)
        RETURNS UUID AS $$
        BEGIN
            RETURN (
                SELECT tenant_id 
                FROM tenant_users 
                WHERE user_id = $1 
                LIMIT 1
            );
        END;
        $$ LANGUAGE plpgsql;
    "#;
    
    pub const SET_TENANT_CONTEXT: &str = r#"
        CREATE OR REPLACE FUNCTION set_tenant_context(tenant_id UUID)
        RETURNS VOID AS $$
        BEGIN
            PERFORM set_config('app.current_tenant', tenant_id::TEXT, true);
        END;
        $$ LANGUAGE plpgsql;
    "#;
}