use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{Response, IntoResponse},
    Json,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use validator::Validate;

/// Validation middleware for request bodies
pub async fn validation_middleware<T>(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode>
where
    T: DeserializeOwned + Validate,
{
    // For now, just pass through - validation will be handled in handlers
    // with ValidatedJson extractor (to be implemented)
    Ok(next.run(request).await)
}

/// Validated JSON extractor that automatically validates the request body
pub struct ValidatedJson<T>(pub T);

#[async_trait::async_trait]
impl<T, S> axum::extract::FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ValidationError;

    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Extract JSON from request
        let axum::extract::Json(value) = axum::extract::Json::<T>::from_request(req, state)
            .await
            .map_err(|err| ValidationError::InvalidJson(err.to_string()))?;

        // Validate the extracted value
        value
            .validate()
            .map_err(|err| ValidationError::ValidationFailed(err))?;

        Ok(ValidatedJson(value))
    }
}

/// Validation error types
#[derive(Debug)]
pub enum ValidationError {
    InvalidJson(String),
    ValidationFailed(validator::ValidationErrors),
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ValidationError::InvalidJson(msg) => {
                (StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", msg))
            }
            ValidationError::ValidationFailed(errors) => {
                let error_messages = format_validation_errors(&errors);
                (StatusCode::BAD_REQUEST, error_messages)
            }
        };

        let body = json!({
            "error": error_message,
            "status": "error",
            "code": status.as_u16(),
        });

        (status, Json(body)).into_response()
    }
}

/// Format validation errors into a readable string
fn format_validation_errors(errors: &validator::ValidationErrors) -> String {
    let mut messages = Vec::new();
    
    for (field, field_errors) in errors.field_errors() {
        for error in field_errors {
            let message = match &error.message {
                Some(msg) => format!("{}: {}", field, msg),
                None => format!("{}: validation failed", field),
            };
            messages.push(message);
        }
    }
    
    messages.join(", ")
}

/// Australian-specific validation functions
pub mod validators {
    use validator::ValidationError;
    use regex::Regex;
    use lazy_static::lazy_static;
    
    lazy_static! {
        static ref AU_PHONE_REGEX: Regex = Regex::new(r"^(\+61|0)[2-478]\d{8}$").unwrap();
        static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    }
    
    /// Validate Australian phone number
    pub fn validate_au_phone(phone: &str) -> Result<(), ValidationError> {
        let cleaned = phone.chars().filter(|c| c.is_numeric() || *c == '+').collect::<String>();
        
        if AU_PHONE_REGEX.is_match(&cleaned) {
            Ok(())
        } else {
            Err(ValidationError::new("invalid_australian_phone"))
        }
    }
    
    /// Validate Australian postcode
    pub fn validate_au_postcode(postcode: &str) -> Result<(), ValidationError> {
        if postcode.len() == 4 && postcode.chars().all(|c| c.is_numeric()) {
            let code = postcode.parse::<u16>().unwrap();
            if code >= 200 && code <= 9999 {
                Ok(())
            } else {
                Err(ValidationError::new("invalid_postcode_range"))
            }
        } else {
            Err(ValidationError::new("invalid_postcode_format"))
        }
    }
    
    /// Validate Australian Business Number (ABN)
    pub fn validate_abn(abn: &str) -> Result<(), ValidationError> {
        let cleaned = abn.chars().filter(|c| c.is_numeric()).collect::<String>();
        
        if cleaned.len() != 11 {
            return Err(ValidationError::new("invalid_abn_length"));
        }
        
        // ABN validation algorithm
        let digits: Vec<u32> = cleaned.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
            
        let weights = [10, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let mut sum = 0;
        
        // Subtract 1 from first digit
        let mut modified_digits = digits.clone();
        modified_digits[0] -= 1;
        
        for i in 0..11 {
            sum += modified_digits[i] * weights[i];
        }
        
        if sum % 89 == 0 {
            Ok(())
        } else {
            Err(ValidationError::new("invalid_abn_checksum"))
        }
    }
    
    /// Validate AHPRA number (Australian Health Practitioner Regulation Agency)
    pub fn validate_ahpra(ahpra: &str) -> Result<(), ValidationError> {
        // AHPRA numbers are typically 3 letters followed by 7 digits
        let cleaned = ahpra.to_uppercase();
        if cleaned.len() == 10 {
            let (prefix, number) = cleaned.split_at(3);
            if prefix.chars().all(|c| c.is_alphabetic()) && number.chars().all(|c| c.is_numeric()) {
                Ok(())
            } else {
                Err(ValidationError::new("invalid_ahpra_format"))
            }
        } else {
            Err(ValidationError::new("invalid_ahpra_length"))
        }
    }
    
    /// Validate salary range
    pub fn validate_salary_range(start: Option<i32>, end: Option<i32>) -> Result<(), ValidationError> {
        match (start, end) {
            (Some(s), Some(e)) if s > e => {
                Err(ValidationError::new("salary_start_greater_than_end"))
            }
            (Some(s), _) if s < 0 => {
                Err(ValidationError::new("negative_salary"))
            }
            (_, Some(e)) if e < 0 => {
                Err(ValidationError::new("negative_salary"))
            }
            _ => Ok(())
        }
    }
    
    /// Validate Australian state code
    pub fn validate_au_state(state: &str) -> Result<(), ValidationError> {
        const VALID_STATES: &[&str] = &["NSW", "VIC", "QLD", "WA", "SA", "TAS", "ACT", "NT"];
        
        if VALID_STATES.contains(&state.to_uppercase().as_str()) {
            Ok(())
        } else {
            Err(ValidationError::new("invalid_australian_state"))
        }
    }
    
    /// Validate email format
    pub fn validate_email(email: &str) -> Result<(), ValidationError> {
        if EMAIL_REGEX.is_match(email) {
            Ok(())
        } else {
            Err(ValidationError::new("invalid_email_format"))
        }
    }
}

/// Rate limiting middleware (simplified implementation)
pub async fn rate_limit_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // For now, just pass through
    // In production, you'd implement actual rate limiting with Redis
    Ok(next.run(request).await)
}

/// Request size limit middleware
pub async fn request_size_limit_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Check content-length header
    if let Some(content_length) = request.headers().get("content-length") {
        if let Ok(length_str) = content_length.to_str() {
            if let Ok(length) = length_str.parse::<usize>() {
                // Limit to 10MB
                if length > 10 * 1024 * 1024 {
                    return Err(StatusCode::PAYLOAD_TOO_LARGE);
                }
            }
        }
    }
    
    Ok(next.run(request).await)
}