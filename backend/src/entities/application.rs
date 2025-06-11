use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "application")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub job_id: Uuid,
    pub user_id: Uuid,
    
    // Application content
    pub cover_letter: Option<String>,
    pub resume_url: Option<String>,
    pub availability_note: Option<String>,
    pub experience_years: Option<i32>,
    pub registration_number: Option<String>, // AHPRA registration
    pub preferred_contact_method: Option<String>,
    
    // Application status and review
    pub status: ApplicationStatus,
    pub reviewer_notes: Option<String>,
    pub interview_scheduled_at: Option<DateTimeWithTimeZone>,
    pub reviewed_at: Option<DateTimeWithTimeZone>,
    pub reviewed_by: Option<Uuid>,
    
    // Timestamps
    pub applied_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum ApplicationStatus {
    #[sea_orm(string_value = "Pending")]
    Pending,
    #[sea_orm(string_value = "Reviewing")]
    Reviewing,
    #[sea_orm(string_value = "Shortlisted")]
    Shortlisted,
    #[sea_orm(string_value = "Interviewed")]
    Interviewed,
    #[sea_orm(string_value = "Offered")]
    Offered,
    #[sea_orm(string_value = "Accepted")]
    Accepted,
    #[sea_orm(string_value = "Rejected")]
    Rejected,
    #[sea_orm(string_value = "Withdrawn")]
    Withdrawn,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::job::Entity",
        from = "Column::JobId",
        to = "super::job::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Job,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::ReviewedBy",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Reviewer,
}

impl Related<super::job::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Job.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            status: Set(ApplicationStatus::Pending),
            applied_at: Set(chrono::Utc::now().into()),
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

impl Model {
    /// Check if the application is in a pending state
    pub fn is_pending(&self) -> bool {
        matches!(self.status, ApplicationStatus::Pending)
    }
    
    /// Check if the application has been reviewed
    pub fn is_reviewed(&self) -> bool {
        self.reviewed_at.is_some()
    }
    
    /// Check if the application is successful (offered or accepted)
    pub fn is_successful(&self) -> bool {
        matches!(self.status, ApplicationStatus::Offered | ApplicationStatus::Accepted)
    }
    
    /// Check if the application is closed (rejected, withdrawn, or accepted)
    pub fn is_closed(&self) -> bool {
        matches!(
            self.status, 
            ApplicationStatus::Rejected | 
            ApplicationStatus::Withdrawn | 
            ApplicationStatus::Accepted
        )
    }
    
    /// Check if an interview is scheduled
    pub fn has_interview_scheduled(&self) -> bool {
        self.interview_scheduled_at.is_some()
    }
    
    /// Get the application's age in days
    pub fn age_in_days(&self) -> i64 {
        let now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let applied_at: chrono::DateTime<chrono::Utc> = self.applied_at.into();
        now.signed_duration_since(applied_at).num_days()
    }
    
    /// Check if the application can be withdrawn
    pub fn can_be_withdrawn(&self) -> bool {
        !matches!(
            self.status,
            ApplicationStatus::Withdrawn | 
            ApplicationStatus::Accepted |
            ApplicationStatus::Rejected
        )
    }
    
    /// Get status display text for Australian context
    pub fn status_display(&self) -> &'static str {
        match self.status {
            ApplicationStatus::Pending => "Awaiting Review",
            ApplicationStatus::Reviewing => "Under Review",
            ApplicationStatus::Shortlisted => "Shortlisted",
            ApplicationStatus::Interviewed => "Interviewed",
            ApplicationStatus::Offered => "Offer Extended",
            ApplicationStatus::Accepted => "Offer Accepted",
            ApplicationStatus::Rejected => "Application Unsuccessful",
            ApplicationStatus::Withdrawn => "Application Withdrawn",
        }
    }
    
    /// Get the next possible status transitions
    pub fn possible_next_statuses(&self) -> Vec<ApplicationStatus> {
        match self.status {
            ApplicationStatus::Pending => vec![
                ApplicationStatus::Reviewing,
                ApplicationStatus::Rejected,
                ApplicationStatus::Withdrawn,
            ],
            ApplicationStatus::Reviewing => vec![
                ApplicationStatus::Shortlisted,
                ApplicationStatus::Rejected,
                ApplicationStatus::Withdrawn,
            ],
            ApplicationStatus::Shortlisted => vec![
                ApplicationStatus::Interviewed,
                ApplicationStatus::Rejected,
                ApplicationStatus::Withdrawn,
            ],
            ApplicationStatus::Interviewed => vec![
                ApplicationStatus::Offered,
                ApplicationStatus::Rejected,
                ApplicationStatus::Withdrawn,
            ],
            ApplicationStatus::Offered => vec![
                ApplicationStatus::Accepted,
                ApplicationStatus::Rejected,
                ApplicationStatus::Withdrawn,
            ],
            _ => vec![], // Terminal states
        }
    }
}