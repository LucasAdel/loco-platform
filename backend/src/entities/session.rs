use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub user_id: Uuid,
    
    #[sea_orm(unique)]
    pub token: String,
    
    // Session metadata
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    
    // Session state
    pub is_active: bool,
    pub expires_at: DateTimeWithTimeZone,
    pub last_accessed_at: Option<DateTimeWithTimeZone>,
    
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            is_active: Set(true),
            created_at: Set(chrono::Utc::now().into()),
            // Default 30-day expiry
            expires_at: Set((chrono::Utc::now() + chrono::Duration::days(30)).into()),
            ..ActiveModelTrait::default()
        }
    }
}

impl Model {
    /// Check if the session is currently valid
    pub fn is_valid(&self) -> bool {
        self.is_active && !self.is_expired()
    }
    
    /// Check if the session has expired
    pub fn is_expired(&self) -> bool {
        let now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let expires_at: chrono::DateTime<chrono::Utc> = self.expires_at.into();
        now > expires_at
    }
    
    /// Get the remaining time until expiry
    pub fn time_until_expiry(&self) -> Option<chrono::Duration> {
        if self.is_expired() {
            None
        } else {
            let now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
            let expires_at: chrono::DateTime<chrono::Utc> = self.expires_at.into();
            Some(expires_at.signed_duration_since(now))
        }
    }
    
    /// Check if the session was accessed recently (within last hour)
    pub fn is_recently_accessed(&self) -> bool {
        if let Some(last_accessed) = self.last_accessed_at {
            let last_accessed: chrono::DateTime<chrono::Utc> = last_accessed.into();
            let now = chrono::Utc::now();
            now.signed_duration_since(last_accessed) < chrono::Duration::hours(1)
        } else {
            false
        }
    }
    
    /// Get a user-friendly device description
    pub fn device_description(&self) -> String {
        if let Some(device_info) = &self.device_info {
            device_info.clone()
        } else if let Some(user_agent) = &self.user_agent {
            // Parse user agent to extract browser/device info
            if user_agent.contains("Chrome") {
                "Chrome Browser".to_string()
            } else if user_agent.contains("Firefox") {
                "Firefox Browser".to_string()
            } else if user_agent.contains("Safari") {
                "Safari Browser".to_string()
            } else if user_agent.contains("Mobile") {
                "Mobile Device".to_string()
            } else {
                "Unknown Device".to_string()
            }
        } else {
            "Unknown Device".to_string()
        }
    }
    
    /// Get the session age
    pub fn age(&self) -> chrono::Duration {
        let now = chrono::Utc::now();
        let created_at: chrono::DateTime<chrono::Utc> = self.created_at.into();
        now.signed_duration_since(created_at)
    }
    
    /// Create a new session token
    pub fn generate_token() -> String {
        use uuid::Uuid;
        format!("session_{}", Uuid::new_v4().to_string().replace('-', ""))
    }
    
    /// Extract location from IP address (placeholder for geolocation service)
    pub fn estimated_location(&self) -> Option<String> {
        // In a real implementation, this would use a GeoIP service
        // For now, just return the IP if available
        self.ip_address.clone()
    }
}