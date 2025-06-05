use sea_orm::entity::prelude::*;
use sea_orm::{Set, ActiveValue, FromJsonQueryResult, Value};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tenants")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub domain: Option<String>,
    #[sea_orm(column_type = "JsonBinary")]
    pub settings: TenantSettings,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct TenantSettings {
    pub primary_colour: String,
    pub secondary_colour: String,
    pub logo_url: Option<String>,
    pub max_users: i32,
    pub max_jobs: i32,
    pub features: Vec<String>,
}

impl Default for TenantSettings {
    fn default() -> Self {
        Self {
            primary_colour: "#1e40af".to_string(),
            secondary_colour: "#3b82f6".to_string(),
            logo_url: None,
            max_users: 10,
            max_jobs: 50,
            features: vec!["basic".to_string()],
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tenant_users::Entity")]
    TenantUsers,
}

impl Related<super::tenant_users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TenantUsers.def()
    }
}


impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            settings: Set(TenantSettings::default()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        }
    }

}