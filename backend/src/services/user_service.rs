use uuid::Uuid;
use shared::types::{User, UserType};
use crate::AppError;

pub struct UserService;

impl UserService {
    /// Get a user by ID (Demo mode)
    pub async fn get_user_by_id(
        user_id: Uuid,
    ) -> Result<Option<User>, AppError> {
        // Demo mode: Return sample user
        let users = Self::get_sample_users();
        Ok(users.into_iter().find(|u| u.id == user_id))
    }
    
    /// Get a user by email (Demo mode)
    pub async fn get_user_by_email(
        email: &str,
    ) -> Result<Option<User>, AppError> {
        // Demo mode: Find user by email
        let users = Self::get_sample_users();
        Ok(users.into_iter().find(|u| u.email == email))
    }
    
    /// Create a new user (Demo mode)
    pub async fn create_user(
        email: String,
        _password_hash: String,
        first_name: String,
        last_name: String,
        user_type: String,
    ) -> Result<User, AppError> {
        // Demo mode: Create new user
        let user_type_enum = match user_type.as_str() {
            "Professional" => UserType::Professional,
            "Employer" => UserType::Employer,
            "SuperAdmin" => UserType::SuperAdmin,
            _ => UserType::Professional,
        };
        
        let user = User {
            id: Uuid::new_v4(),
            email,
            first_name,
            last_name,
            phone: None,
            user_type: user_type_enum,
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        Ok(user)
    }
    
    /// Update user profile (Demo mode)
    pub async fn update_user(
        user_id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        _phone: Option<String>,
    ) -> Result<Option<User>, AppError> {
        // Demo mode: Return updated user if exists
        let users = Self::get_sample_users();
        if let Some(mut user) = users.into_iter().find(|u| u.id == user_id) {
            if let Some(fname) = first_name {
                user.first_name = fname;
            }
            if let Some(lname) = last_name {
                user.last_name = lname;
            }
            user.updated_at = chrono::Utc::now();
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
    
    /// Deactivate user account (Demo mode)
    pub async fn deactivate_user(
        user_id: Uuid,
    ) -> Result<bool, AppError> {
        // Demo mode: Check if user exists
        let users = Self::get_sample_users();
        Ok(users.iter().any(|u| u.id == user_id))
    }
    
    /// Update last login (Demo mode)
    pub async fn update_last_login(
        user_id: Uuid,
    ) -> Result<(), AppError> {
        // Demo mode: Do nothing
        let _ = user_id;
        Ok(())
    }
    
    /// Get sample users for demo mode
    fn get_sample_users() -> Vec<User> {
        vec![
            User {
                id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440003").unwrap(),
                email: "john.doe@example.com".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                phone: Some("+61 421 123 456".to_string()),
                user_type: UserType::Professional,
                is_active: true,
                created_at: chrono::Utc::now() - chrono::Duration::days(30),
                updated_at: chrono::Utc::now() - chrono::Duration::days(5),
            },
            User {
                id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440004").unwrap(),
                email: "jane.smith@pharmacy.com.au".to_string(),
                first_name: "Jane".to_string(),
                last_name: "Smith".to_string(),
                phone: Some("+61 2 9876 5432".to_string()),
                user_type: UserType::Employer,
                is_active: true,
                created_at: chrono::Utc::now() - chrono::Duration::days(60),
                updated_at: chrono::Utc::now() - chrono::Duration::days(1),
            },
        ]
    }
}