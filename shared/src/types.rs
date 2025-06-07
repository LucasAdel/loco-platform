use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;
use std::fmt::{self, Display};
use crate::errors::AppError;

// ============================================================================
// NewType Wrappers for Type Safety (Phase 0 Requirement)
// ============================================================================

/// Job ID with type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JobId(pub Uuid);

impl JobId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_str(s: &str) -> Result<Self, AppError> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl Display for JobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for JobId {
    fn default() -> Self {
        Self::new()
    }
}

/// User ID with type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_str(s: &str) -> Result<Self, AppError> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

/// Australian postcode with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Postcode(pub String);

impl Postcode {
    pub fn new(postcode: impl Into<String>) -> Result<Self, AppError> {
        let postcode = postcode.into();
        
        // Validate Australian postcode format
        if postcode.len() != 4 || !postcode.chars().all(|c| c.is_ascii_digit()) {
            return Err(AppError::validation(
                "postcode",
                "Australian postcodes must be exactly 4 digits"
            ));
        }
        
        // Check if it's in valid Australian range
        let code: u16 = postcode.parse().map_err(|_| {
            AppError::validation("postcode", "Postcode must be numeric")
        })?;
        
        if !(1000..=9999).contains(&code) {
            return Err(AppError::validation(
                "postcode", 
                "Postcode must be between 1000 and 9999"
            ));
        }
        
        Ok(Self(postcode))
    }
}

impl Display for Postcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Australian phone number with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumber(pub String);

impl PhoneNumber {
    pub fn new(phone: impl Into<String>) -> Result<Self, AppError> {
        let phone = phone.into();
        let cleaned = phone.replace([' ', '-', '(', ')', '+'], "");
        
        // Check Australian phone number patterns
        if cleaned.starts_with("61") && cleaned.len() == 11 {
            // International format: +61xxxxxxxxx
            Ok(Self(format!("+{}", cleaned)))
        } else if cleaned.starts_with('0') && cleaned.len() == 10 {
            // National format: 0xxxxxxxxx
            Ok(Self(cleaned))
        } else if cleaned.len() == 9 && !cleaned.starts_with('0') {
            // Without leading 0: xxxxxxxxx
            Ok(Self(format!("0{}", cleaned)))
        } else {
            Err(AppError::validation(
                "phone_number",
                "Please enter a valid Australian phone number"
            ))
        }
    }
}

impl Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Email address with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailAddress(pub String);

impl EmailAddress {
    pub fn new(email: impl Into<String>) -> Result<Self, AppError> {
        let email = email.into().to_lowercase();
        
        // Basic email validation
        if email.contains('@') && email.contains('.') && email.len() > 5 {
            Ok(Self(email))
        } else {
            Err(AppError::validation(
                "email",
                "Please enter a valid email address"
            ))
        }
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
// Core Domain Models with Enhanced Type Safety
// ============================================================================

/// Job listing with comprehensive type safety and Australian localisations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate)]
pub struct Job {
    pub id: JobId,
    
    #[validate(length(min = 5, max = 200, message = "Job title must be between 5 and 200 characters"))]
    pub title: String,
    
    #[validate(length(min = 20, max = 5000, message = "Job description must be between 20 and 5000 characters"))]
    pub description: String,
    
    #[validate(length(min = 2, max = 100, message = "Pharmacy name must be between 2 and 100 characters"))]
    pub pharmacy_name: String,
    
    #[validate(range(min = 15.0, max = 200.0, message = "Hourly rate must be between $15 and $200"))]
    pub hourly_rate: f64,
    
    pub address: String,
    pub suburb: String,
    pub postcode: Postcode,
    pub state: AustralianState,
    
    // Geographic coordinates for mapping
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub start_time: String,  // Format: "HH:MM" in 24-hour format
    pub end_time: String,    // Format: "HH:MM" in 24-hour format
    
    pub job_type: JobType,
    pub status: JobStatus,
    pub is_urgent: bool,
    
    // Calculated field - distance from user's location
    pub distance_km: Option<f64>,
    
    // Audit fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: UserId,
}

/// Builder pattern for Job creation (Phase 0 Requirement)
#[derive(Debug, Default)]
pub struct JobBuilder {
    title: Option<String>,
    description: Option<String>,
    pharmacy_name: Option<String>,
    hourly_rate: Option<f64>,
    address: Option<String>,
    suburb: Option<String>,
    postcode: Option<Postcode>,
    state: Option<AustralianState>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    start_time: Option<String>,
    end_time: Option<String>,
    job_type: Option<JobType>,
    is_urgent: Option<bool>,
    created_by: Option<UserId>,
}

impl JobBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    pub fn pharmacy_name(mut self, name: impl Into<String>) -> Self {
        self.pharmacy_name = Some(name.into());
        self
    }
    
    pub fn hourly_rate(mut self, rate: f64) -> Self {
        self.hourly_rate = Some(rate);
        self
    }
    
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }
    
    pub fn location(mut self, suburb: impl Into<String>, postcode: Postcode, state: AustralianState) -> Self {
        self.suburb = Some(suburb.into());
        self.postcode = Some(postcode);
        self.state = Some(state);
        self
    }
    
    pub fn schedule(
        mut self, 
        start_date: DateTime<Utc>, 
        end_date: DateTime<Utc>,
        start_time: impl Into<String>,
        end_time: impl Into<String>
    ) -> Self {
        self.start_date = Some(start_date);
        self.end_date = Some(end_date);
        self.start_time = Some(start_time.into());
        self.end_time = Some(end_time.into());
        self
    }
    
    pub fn job_type(mut self, job_type: JobType) -> Self {
        self.job_type = Some(job_type);
        self
    }
    
    pub fn urgent(mut self, urgent: bool) -> Self {
        self.is_urgent = Some(urgent);
        self
    }
    
    pub fn created_by(mut self, user_id: UserId) -> Self {
        self.created_by = Some(user_id);
        self
    }
    
    /// Build the Job, validating all required fields
    pub fn build(self) -> Result<Job, AppError> {
        let now = Utc::now();
        
        let job = Job {
            id: JobId::new(),
            title: self.title.ok_or_else(|| AppError::validation("title", "Job title is required"))?,
            description: self.description.ok_or_else(|| AppError::validation("description", "Job description is required"))?,
            pharmacy_name: self.pharmacy_name.ok_or_else(|| AppError::validation("pharmacy_name", "Pharmacy name is required"))?,
            hourly_rate: self.hourly_rate.ok_or_else(|| AppError::validation("hourly_rate", "Hourly rate is required"))?,
            address: self.address.ok_or_else(|| AppError::validation("address", "Address is required"))?,
            suburb: self.suburb.ok_or_else(|| AppError::validation("suburb", "Suburb is required"))?,
            postcode: self.postcode.ok_or_else(|| AppError::validation("postcode", "Postcode is required"))?,
            state: self.state.ok_or_else(|| AppError::validation("state", "State is required"))?,
            latitude: None,
            longitude: None,
            start_date: self.start_date.ok_or_else(|| AppError::validation("start_date", "Start date is required"))?,
            end_date: self.end_date.ok_or_else(|| AppError::validation("end_date", "End date is required"))?,
            start_time: self.start_time.ok_or_else(|| AppError::validation("start_time", "Start time is required"))?,
            end_time: self.end_time.ok_or_else(|| AppError::validation("end_time", "End time is required"))?,
            job_type: self.job_type.ok_or_else(|| AppError::validation("job_type", "Job type is required"))?,
            status: JobStatus::Draft,
            is_urgent: self.is_urgent.unwrap_or(false),
            distance_km: None,
            created_at: now,
            updated_at: now,
            created_by: self.created_by.ok_or_else(|| AppError::validation("created_by", "Creator user ID is required"))?,
        };
        
        // Validate the built job
        job.validate().map_err(|e| AppError::validation("job", format!("Validation failed: {}", e)))?;
        
        Ok(job)
    }
}

/// Australian states and territories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AustralianState {
    #[serde(rename = "NSW")]
    NewSouthWales,
    #[serde(rename = "VIC")]
    Victoria,
    #[serde(rename = "QLD")]
    Queensland,
    #[serde(rename = "WA")]
    WesternAustralia,
    #[serde(rename = "SA")]
    SouthAustralia,
    #[serde(rename = "TAS")]
    Tasmania,
    #[serde(rename = "ACT")]
    AustralianCapitalTerritory,
    #[serde(rename = "NT")]
    NorthernTerritory,
}

impl Display for AustralianState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abbrev = match self {
            AustralianState::NewSouthWales => "NSW",
            AustralianState::Victoria => "VIC",
            AustralianState::Queensland => "QLD",
            AustralianState::WesternAustralia => "WA",
            AustralianState::SouthAustralia => "SA",
            AustralianState::Tasmania => "TAS",
            AustralianState::AustralianCapitalTerritory => "ACT",
            AustralianState::NorthernTerritory => "NT",
        };
        write!(f, "{}", abbrev)
    }
}

impl AustralianState {
    pub fn full_name(&self) -> &'static str {
        match self {
            AustralianState::NewSouthWales => "New South Wales",
            AustralianState::Victoria => "Victoria",
            AustralianState::Queensland => "Queensland",
            AustralianState::WesternAustralia => "Western Australia",
            AustralianState::SouthAustralia => "South Australia",
            AustralianState::Tasmania => "Tasmania",
            AustralianState::AustralianCapitalTerritory => "Australian Capital Territory",
            AustralianState::NorthernTerritory => "Northern Territory",
        }
    }
    
    pub fn from_abbrev(abbrev: &str) -> Option<Self> {
        match abbrev.to_uppercase().as_str() {
            "NSW" => Some(AustralianState::NewSouthWales),
            "VIC" => Some(AustralianState::Victoria),
            "QLD" => Some(AustralianState::Queensland),
            "WA" => Some(AustralianState::WesternAustralia),
            "SA" => Some(AustralianState::SouthAustralia),
            "TAS" => Some(AustralianState::Tasmania),
            "ACT" => Some(AustralianState::AustralianCapitalTerritory),
            "NT" => Some(AustralianState::NorthernTerritory),
            _ => None,
        }
    }
}

/// Job types specific to Australian pharmacy sector
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum JobType {
    /// Pharmacy intern (pre-registration)
    Intern,
    /// Pharmacy student (placement/part-time)
    Student,
    /// Registered pharmacist
    Pharmacist,
    /// Pharmacy assistant (retail/dispensary)
    PharmacyAssistant,
    /// Pharmacy technician (compounding/clinical)
    PharmacyTechnician,
}

impl Display for JobType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            JobType::Intern => "Pharmacy Intern",
            JobType::Student => "Pharmacy Student",
            JobType::Pharmacist => "Pharmacist",
            JobType::PharmacyAssistant => "Pharmacy Assistant",
            JobType::PharmacyTechnician => "Pharmacy Technician",
        };
        write!(f, "{}", name)
    }
}

/// Job status with Australian workflow considerations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobStatus {
    /// Job is active and accepting applications
    Active,
    /// Job is closed (no longer accepting applications)
    Closed,
    /// Job is in draft mode (not published)
    Draft,
    /// Position has been filled
    Filled,
    /// Job posting has been cancelled
    Cancelled,
    /// Job posting has expired
    Expired,
}

impl Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            JobStatus::Active => "Active",
            JobStatus::Closed => "Closed",
            JobStatus::Draft => "Draft",
            JobStatus::Filled => "Filled",
            JobStatus::Cancelled => "Cancelled",
            JobStatus::Expired => "Expired",
        };
        write!(f, "{}", status)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateJobRequest {
    #[validate(length(min = 3, max = 100))]
    pub title: String,
    #[validate(length(min = 10, max = 2000))]
    pub description: String,
    #[validate(length(min = 2, max = 100))]
    pub pharmacy_name: String,
    #[validate(range(min = 15.0, max = 200.0))]
    pub hourly_rate: f64,
    #[validate(length(min = 5, max = 200))]
    pub address: String,
    #[validate(length(min = 2, max = 50))]
    pub suburb: String,
    #[validate(length(min = 4, max = 4))]
    pub postcode: String,
    #[validate(length(min = 2, max = 3))]
    pub state: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub start_time: String,
    pub end_time: String,
    pub job_type: JobType,
    pub is_urgent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub user_type: UserType,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserType {
    Professional,
    Employer,
    SuperAdmin,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,
    Manager,
    User,
    Guest,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Manager => "manager",
            UserRole::User => "user",
            UserRole::Guest => "guest",
        }
    }
}

impl Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for UserRole {
    fn from(s: &str) -> Self {
        match s {
            "admin" => UserRole::Admin,
            "manager" => UserRole::Manager,
            "user" => UserRole::User,
            "guest" => UserRole::Guest,
            _ => UserRole::Guest,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub id: Uuid,
    pub job_id: Uuid,
    pub user_id: Uuid,
    pub cover_letter: Option<String>,
    pub status: ApplicationStatus,
    pub applied_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicationStatus {
    Pending,
    Reviewing,
    Shortlisted,
    Interviewed,
    Offered,
    Accepted,
    Rejected,
    Withdrawn,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JobFilters {
    pub job_type: Option<JobType>,
    pub min_rate: Option<f64>,
    pub max_rate: Option<f64>,
    pub suburb: Option<String>,
    pub state: Option<String>,
    pub is_urgent: Option<bool>,
    pub max_distance: Option<f64>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationPin {
    pub lat: f64,
    pub lng: f64,
    pub job: Job,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: Option<String>,
    pub filters: JobFilters,
    pub user_location: Option<(f64, f64)>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub jobs: Vec<Job>,
    pub total_count: u64,
    pub page: u32,
    pub limit: u32,
    pub has_more: bool,
}

// ============================================================================
// Simplified Types for Frontend Use
// ============================================================================

/// Simplified Job struct for frontend components
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimpleJob {
    pub id: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub description: String,
    pub salary_range: String,
    pub job_type: SimpleJobType,
    pub posted_date: String,
    pub urgent: bool,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

/// Simple job type enum for frontend use
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SimpleJobType {
    #[serde(rename = "Full-time")]
    FullTime,
    #[serde(rename = "Part-time")]
    PartTime,
    #[serde(rename = "Contract")]
    Contract,
    #[serde(rename = "Casual")]
    Casual,
}

impl From<String> for SimpleJobType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Full-time" => SimpleJobType::FullTime,
            "Part-time" => SimpleJobType::PartTime,
            "Contract" => SimpleJobType::Contract,
            "Casual" => SimpleJobType::Casual,
            _ => SimpleJobType::FullTime,
        }
    }
}

impl Display for SimpleJobType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SimpleJobType::FullTime => "Full-time",
            SimpleJobType::PartTime => "Part-time",
            SimpleJobType::Contract => "Contract",
            SimpleJobType::Casual => "Casual",
        };
        write!(f, "{}", s)
    }
}

// ============================================================================
// API Error Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiError {
    Network(String),
    Parse(String),
    Http(u16),
    Validation(String),
    NotFound,
    Unauthorised,
    Internal(String),
}