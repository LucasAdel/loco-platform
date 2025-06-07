use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use chrono::NaiveDate;

use crate::types::{JobType, AustralianState};

/// Validated job creation request with comprehensive Australian-specific validation
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_create_job_request"))]
pub struct ValidatedCreateJobRequest {
    #[validate(length(min = 5, max = 200, message = "Title must be between 5 and 200 characters"))]
    pub title: String,
    
    #[validate(length(min = 2, max = 100, message = "Company name must be between 2 and 100 characters"))]
    pub company: String,
    
    #[validate(length(min = 50, max = 5000, message = "Description must be between 50 and 5000 characters"))]
    pub description: String,
    
    // Location fields
    #[validate(length(min = 5, max = 200, message = "Address must be between 5 and 200 characters"))]
    pub address: String,
    
    #[validate(length(min = 2, max = 50, message = "Suburb must be between 2 and 50 characters"))]
    pub suburb: String,
    
    #[validate(custom(function = "validate_postcode"))]
    pub postcode: String,
    
    #[validate(custom(function = "validate_state"))]
    pub state: String,
    
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    
    // Job details
    pub job_type: JobType,
    
    // Salary information
    #[validate(range(min = 0, max = 500000, message = "Salary must be between 0 and 500,000"))]
    pub salary_min: Option<i32>,
    
    #[validate(range(min = 0, max = 500000, message = "Salary must be between 0 and 500,000"))]
    pub salary_max: Option<i32>,
    
    // Salary validation is handled in struct-level validation
    
    // Date and time
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    
    // Date validation is handled in struct-level validation
    
    // Requirements
    pub requirements: Vec<String>,
    pub benefits: Vec<String>,
    
    // Pharmacy specific
    pub requires_ahpra: Option<bool>,
    
    pub requires_vaccination: Option<bool>,
    pub requires_police_check: Option<bool>,
    
    // Metadata
    pub is_urgent: bool,
    pub remote_possible: bool,
    
    // Contact information
    #[validate(email(message = "Invalid email format"))]
    pub contact_email: Option<String>,
    
    #[validate(custom(function = "validate_phone"))]
    pub contact_phone: Option<String>,
}

/// Validate Australian postcode
fn validate_postcode(postcode: &str) -> Result<(), ValidationError> {
    if postcode.len() != 4 || !postcode.chars().all(|c| c.is_numeric()) {
        return Err(ValidationError::new("invalid_postcode_format"));
    }
    
    let code = postcode.parse::<u16>().map_err(|_| ValidationError::new("invalid_postcode"))?;
    
    if code < 200 || code > 9999 {
        return Err(ValidationError::new("postcode_out_of_range"));
    }
    
    Ok(())
}

/// Validate Australian state
fn validate_state(state: &str) -> Result<(), ValidationError> {
    if AustralianState::from_abbrev(state).is_none() {
        return Err(ValidationError::new("invalid_australian_state"));
    }
    Ok(())
}

/// Struct-level validation for job creation request
fn validate_create_job_request(job: &ValidatedCreateJobRequest) -> Result<(), ValidationError> {
    // Validate salary range
    if let (Some(min), Some(max)) = (job.salary_min, job.salary_max) {
        if min > max {
            return Err(ValidationError::new("salary_min_greater_than_max"));
        }
    }
    
    // Validate date range
    if let (Some(start), Some(end)) = (job.start_date, job.end_date) {
        if start > end {
            return Err(ValidationError::new("start_date_after_end_date"));
        }
    }
    
    // Validate AHPRA requirement (only for pharmacist roles)
    if job.job_type == JobType::Pharmacist && job.requires_ahpra != Some(true) {
        // Warning, not error - pharmacist jobs should require AHPRA
        // In production, this might log a warning
    }
    
    Ok(())
}

/// Validate Australian phone number
fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let cleaned: String = phone.chars().filter(|c| c.is_numeric() || *c == '+').collect();
    
    // Check for Australian mobile (04xx xxx xxx) or landline formats
    if cleaned.starts_with("04") && cleaned.len() == 10 {
        return Ok(());
    }
    
    if cleaned.starts_with("+614") && cleaned.len() == 12 {
        return Ok(());
    }
    
    // Check landline formats (02, 03, 07, 08)
    if (cleaned.starts_with("02") || cleaned.starts_with("03") || 
        cleaned.starts_with("07") || cleaned.starts_with("08")) && cleaned.len() == 10 {
        return Ok(());
    }
    
    Err(ValidationError::new("invalid_australian_phone"))
}

/// User registration validation
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ValidatedRegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 8, max = 128, message = "Password must be between 8 and 128 characters"))]
    pub password: String,
    
    #[validate(length(min = 2, max = 50, message = "First name must be between 2 and 50 characters"))]
    pub first_name: String,
    
    #[validate(length(min = 2, max = 50, message = "Last name must be between 2 and 50 characters"))]
    pub last_name: String,
    
    pub phone: Option<String>,
    
    pub user_type: String,
    
    // Professional-specific fields
    pub ahpra_number: Option<String>,
    
    pub registration_state: Option<String>,
    pub years_experience: Option<i32>,
    
    // Employer-specific fields
    pub company_name: Option<String>,
    
    pub company_abn: Option<String>,
}

/// Validate AHPRA number format
fn validate_ahpra_number(ahpra: &str) -> Result<(), ValidationError> {
    let cleaned = ahpra.to_uppercase();
    
    // AHPRA numbers are typically 3 letters followed by 7 digits
    if cleaned.len() != 10 {
        return Err(ValidationError::new("invalid_ahpra_length"));
    }
    
    let (prefix, number) = cleaned.split_at(3);
    
    if !prefix.chars().all(|c| c.is_alphabetic()) {
        return Err(ValidationError::new("invalid_ahpra_prefix"));
    }
    
    if !number.chars().all(|c| c.is_numeric()) {
        return Err(ValidationError::new("invalid_ahpra_number"));
    }
    
    Ok(())
}

/// Validate Australian Business Number (ABN)
fn validate_abn(abn: &str) -> Result<(), ValidationError> {
    let cleaned: String = abn.chars().filter(|c| c.is_numeric()).collect();
    
    if cleaned.len() != 11 {
        return Err(ValidationError::new("invalid_abn_length"));
    }
    
    // ABN validation algorithm
    let digits: Vec<u32> = cleaned.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
        
    let weights = [10, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    
    // Subtract 1 from first digit
    let mut sum = (digits[0] - 1) * weights[0];
    
    for i in 1..11 {
        sum += digits[i] * weights[i];
    }
    
    if sum % 89 != 0 {
        return Err(ValidationError::new("invalid_abn_checksum"));
    }
    
    Ok(())
}

/// Job search/filter validation
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ValidatedJobSearchRequest {
    #[validate(length(max = 200, message = "Search query too long"))]
    pub query: Option<String>,
    
    pub job_types: Option<Vec<JobType>>,
    
    pub locations: Option<Vec<String>>,
    
    #[validate(range(min = 0, message = "Minimum salary cannot be negative"))]
    pub min_salary: Option<i32>,
    
    #[validate(range(min = 0, message = "Maximum salary cannot be negative"))]
    pub max_salary: Option<i32>,
    
    #[validate(range(min = 0, max = 500, message = "Radius must be between 0 and 500 km"))]
    pub radius_km: Option<i32>,
    
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    
    pub is_urgent: Option<bool>,
    pub remote_possible: Option<bool>,
    
    #[validate(range(min = 1, max = 100, message = "Page must be between 1 and 100"))]
    pub page: Option<i32>,
    
    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
    pub limit: Option<i32>,
}

fn validate_location_list(locations: &Vec<String>) -> Result<(), ValidationError> {
    for location in locations {
        // Basic validation - could be enhanced with actual location checking
        if location.is_empty() || location.len() > 100 {
            return Err(ValidationError::new("invalid_location"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_postcode_validation() {
        assert!(validate_postcode("2000").is_ok());
        assert!(validate_postcode("3141").is_ok());
        assert!(validate_postcode("0800").is_ok());
        assert!(validate_postcode("199").is_err());
        assert!(validate_postcode("10000").is_err());
        assert!(validate_postcode("ABCD").is_err());
    }
    
    #[test]
    fn test_phone_validation() {
        assert!(validate_phone("0412345678").is_ok());
        assert!(validate_phone("0298765432").is_ok());
        assert!(validate_phone("+61412345678").is_ok());
        assert!(validate_phone("1234567890").is_err());
    }
    
    #[test]
    fn test_ahpra_validation() {
        assert!(validate_ahpra_number("PHR0000001").is_ok());
        assert!(validate_ahpra_number("MED1234567").is_ok());
        assert!(validate_ahpra_number("ABC123456").is_err()); // Too short
        assert!(validate_ahpra_number("1234567890").is_err()); // No prefix
    }
    
    #[test]
    fn test_abn_validation() {
        assert!(validate_abn("51 824 753 556").is_ok()); // Valid ABN
        assert!(validate_abn("51824753556").is_ok());
        assert!(validate_abn("12345678901").is_err()); // Invalid checksum
        assert!(validate_abn("123456789").is_err()); // Too short
    }
}