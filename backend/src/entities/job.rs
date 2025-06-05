use sea_orm::entity::prelude::*;
use sea_orm::{Set, ActiveModelTrait};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use rust_decimal::Decimal;
use shared::types::{JobId, UserId, JobType, JobStatus, AustralianState, Postcode};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "job")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub title: String,
    pub description: String,
    pub pharmacy_name: String,
    pub hourly_rate: Decimal,
    
    // Location fields
    pub address: String,
    pub suburb: String,
    pub postcode: String,
    pub state: String, // Will be validated as AustralianState
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    
    // Schedule fields
    pub start_date: DateTimeWithTimeZone,
    pub end_date: DateTimeWithTimeZone,
    pub start_time: String, // Format: HH:MM
    pub end_time: String,   // Format: HH:MM
    
    // Job properties
    pub job_type: JobTypeDb,
    pub status: JobStatusDb,
    pub is_urgent: bool,
    
    // Optional fields
    pub requirements_text: Option<String>,
    pub benefits_text: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub application_deadline: Option<DateTimeWithTimeZone>,
    
    // Metrics
    pub view_count: i32,
    pub application_count: i32,
    
    // Audit fields
    pub created_by: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub deleted_at: Option<DateTimeWithTimeZone>, // Soft delete
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(30))")]
pub enum JobTypeDb {
    #[sea_orm(string_value = "Intern")]
    Intern,
    #[sea_orm(string_value = "Student")]
    Student,
    #[sea_orm(string_value = "Pharmacist")]
    Pharmacist,
    #[sea_orm(string_value = "PharmacyAssistant")]
    PharmacyAssistant,
    #[sea_orm(string_value = "PharmacyTechnician")]
    PharmacyTechnician,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum JobStatusDb {
    #[sea_orm(string_value = "Active")]
    Active,
    #[sea_orm(string_value = "Closed")]
    Closed,
    #[sea_orm(string_value = "Draft")]
    Draft,
    #[sea_orm(string_value = "Filled")]
    Filled,
    #[sea_orm(string_value = "Cancelled")]
    Cancelled,
    #[sea_orm(string_value = "Expired")]
    Expired,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::CreatedBy",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    User,
    #[sea_orm(has_many = "super::application::Entity")]
    Application,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::application::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Application.def()
    }
}


#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            status: Set(JobStatusDb::Draft),
            is_urgent: Set(false),
            view_count: Set(0),
            application_count: Set(0),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
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

// Conversion implementations for type safety
impl From<JobTypeDb> for JobType {
    fn from(db_type: JobTypeDb) -> Self {
        match db_type {
            JobTypeDb::Intern => JobType::Intern,
            JobTypeDb::Student => JobType::Student,
            JobTypeDb::Pharmacist => JobType::Pharmacist,
            JobTypeDb::PharmacyAssistant => JobType::PharmacyAssistant,
            JobTypeDb::PharmacyTechnician => JobType::PharmacyTechnician,
        }
    }
}

impl From<JobType> for JobTypeDb {
    fn from(job_type: JobType) -> Self {
        match job_type {
            JobType::Intern => JobTypeDb::Intern,
            JobType::Student => JobTypeDb::Student,
            JobType::Pharmacist => JobTypeDb::Pharmacist,
            JobType::PharmacyAssistant => JobTypeDb::PharmacyAssistant,
            JobType::PharmacyTechnician => JobTypeDb::PharmacyTechnician,
        }
    }
}

impl From<JobStatusDb> for JobStatus {
    fn from(db_status: JobStatusDb) -> Self {
        match db_status {
            JobStatusDb::Active => JobStatus::Active,
            JobStatusDb::Closed => JobStatus::Closed,
            JobStatusDb::Draft => JobStatus::Draft,
            JobStatusDb::Filled => JobStatus::Filled,
            JobStatusDb::Cancelled => JobStatus::Cancelled,
            JobStatusDb::Expired => JobStatus::Expired,
        }
    }
}

impl From<JobStatus> for JobStatusDb {
    fn from(status: JobStatus) -> Self {
        match status {
            JobStatus::Active => JobStatusDb::Active,
            JobStatus::Closed => JobStatusDb::Closed,
            JobStatus::Draft => JobStatusDb::Draft,
            JobStatus::Filled => JobStatusDb::Filled,
            JobStatus::Cancelled => JobStatusDb::Cancelled,
            JobStatus::Expired => JobStatusDb::Expired,
        }
    }
}

impl Model {
    /// Check if the job is currently active
    pub fn is_active(&self) -> bool {
        matches!(self.status, JobStatusDb::Active) && self.deleted_at.is_none()
    }
    
    /// Check if the job can accept applications
    pub fn can_accept_applications(&self) -> bool {
        self.is_active() && 
        self.application_deadline.map_or(true, |deadline| deadline > chrono::Utc::now())
    }
    
    /// Get the job's location as an Australian state
    pub fn australian_state(&self) -> Option<AustralianState> {
        AustralianState::from_abbrev(&self.state)
    }
    
    /// Get the job's postcode as a validated type
    pub fn validated_postcode(&self) -> Result<Postcode, shared::errors::AppError> {
        Postcode::new(&self.postcode)
    }
    
    /// Convert to a JobId type-safe wrapper
    pub fn job_id(&self) -> JobId {
        JobId(self.id)
    }
    
    /// Convert to a UserId type-safe wrapper for the creator
    pub fn creator_id(&self) -> UserId {
        UserId(self.created_by)
    }
    
    /// Increment view count
    pub fn increment_view_count(&mut self) {
        self.view_count += 1;
    }
    
    /// Increment application count
    pub fn increment_application_count(&mut self) {
        self.application_count += 1;
    }
    
    /// Check if this is a soft-deleted record
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
    
    /// Get the full location string
    pub fn full_location(&self) -> String {
        format!("{}, {} {}", self.suburb, self.state, self.postcode)
    }
    
    /// Calculate distance to a given point (in kilometres)
    pub fn distance_to(&self, lat: f64, lng: f64) -> Option<f64> {
        if let (Some(job_lat), Some(job_lng)) = (self.latitude, self.longitude) {
            // Haversine formula for calculating distance
            let earth_radius = 6371.0; // Earth's radius in kilometres
            
            let lat1_rad = job_lat.to_radians();
            let lat2_rad = lat.to_radians();
            let delta_lat = (lat - job_lat).to_radians();
            let delta_lng = (lng - job_lng).to_radians();
            
            let a = (delta_lat / 2.0).sin().powi(2) +
                    lat1_rad.cos() * lat2_rad.cos() * (delta_lng / 2.0).sin().powi(2);
            let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
            
            Some(earth_radius * c)
        } else {
            None
        }
    }
}