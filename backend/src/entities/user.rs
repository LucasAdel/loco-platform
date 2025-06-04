use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use shared::types::{UserId, AustralianState};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    #[sea_orm(unique)]
    pub email: String,
    
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub date_of_birth: Option<Date>,
    pub address: Option<String>,
    pub suburb: Option<String>,
    pub postcode: Option<String>,
    pub state: Option<String>, // Will be validated as AustralianState in business logic
    
    pub user_type: UserType,
    pub is_active: bool,
    pub is_email_verified: bool,
    pub last_login_at: Option<DateTimeWithTimeZone>,
    
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum UserType {
    #[sea_orm(string_value = "Professional")]
    Professional,
    #[sea_orm(string_value = "Employer")]
    Employer,
    #[sea_orm(string_value = "SuperAdmin")]
    SuperAdmin,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::job::Entity")]
    Job,
    #[sea_orm(has_many = "super::application::Entity")]
    Application,
    #[sea_orm(has_many = "super::session::Entity")]
    Session,
}

impl Related<super::job::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Job.def()
    }
}

impl Related<super::application::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Application.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            is_active: Set(true),
            is_email_verified: Set(false),
            user_type: Set(UserType::Professional),
            ..ActiveModelTrait::default()
        }
    }

    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut result = self;
        if !insert {
            result.updated_at = Set(chrono::Utc::now().into());
        }
        Ok(result)
    }
}

impl Model {
    /// Get the user's full name
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    
    /// Check if the user is an admin
    pub fn is_admin(&self) -> bool {
        matches!(self.user_type, UserType::SuperAdmin)
    }
    
    /// Check if the user is an employer
    pub fn is_employer(&self) -> bool {
        matches!(self.user_type, UserType::Employer)
    }
    
    /// Get the user's state as an AustralianState enum
    pub fn australian_state(&self) -> Option<AustralianState> {
        self.state.as_ref()
            .and_then(|s| AustralianState::from_abbrev(s))
    }
    
    /// Convert to a UserId type-safe wrapper
    pub fn user_id(&self) -> UserId {
        UserId(self.id)
    }
}