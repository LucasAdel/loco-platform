use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use serde::de::DeserializeOwned;
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
    type Rejection = (StatusCode, String);

    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Extract JSON from request
        let axum::extract::Json(value) = axum::extract::Json::<T>::from_request(req, state)
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", err)))?;

        // Validate the extracted value
        value
            .validate()
            .map_err(|err| (StatusCode::BAD_REQUEST, format!("Validation error: {}", err)))?;

        Ok(ValidatedJson(value))
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