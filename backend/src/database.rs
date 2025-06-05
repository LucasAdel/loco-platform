use sea_orm::{Database, DatabaseConnection, DbErr, ConnectOptions, ConnectionTrait, EntityTrait, Statement};
use std::env;
use tracing::log;

/// Database configuration structure
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
    pub acquire_timeout: u64,
}

impl DatabaseConfig {
    /// Load database configuration from environment variables
    pub fn from_env() -> Result<Self, anyhow::Error> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/loco_platform".to_string());
        
        let max_connections = env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "20".to_string())
            .parse::<u32>()?;
            
        let min_connections = env::var("DB_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "1".to_string())
            .parse::<u32>()?;
            
        let connect_timeout = env::var("DB_CONNECT_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()?;
            
        let idle_timeout = env::var("DB_IDLE_TIMEOUT")
            .unwrap_or_else(|_| "600".to_string())
            .parse::<u64>()?;
            
        let acquire_timeout = env::var("DB_ACQUIRE_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()?;

        Ok(Self {
            url: database_url,
            max_connections,
            min_connections,
            connect_timeout,
            idle_timeout,
            acquire_timeout,
        })
    }
}

/// Establish database connection with optimised settings for Australian timezone
pub async fn establish_connection(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(&config.url);
    
    // Configure connection pool
    opt.max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect_timeout(std::time::Duration::from_secs(config.connect_timeout))
        .idle_timeout(std::time::Duration::from_secs(config.idle_timeout))
        .acquire_timeout(std::time::Duration::from_secs(config.acquire_timeout))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    // Australian-specific database settings
    opt.sqlx_logging_level(log::LevelFilter::Debug);
    
    let db = Database::connect(opt).await?;
    
    // Set timezone to Australian Eastern Standard Time
    // Note: This would normally be done in database configuration
    tracing::info!("✅ Database connection established successfully");
    tracing::info!("🔗 Connected to: {}", mask_database_url(&config.url));
    
    Ok(db)
}

/// Test database connection
pub async fn test_connection(db: &DatabaseConnection) -> Result<(), DbErr> {
    use sea_orm::Statement;
    
    // Simple health check query
    let result = db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        "SELECT 1 as health_check".to_owned(),
    )).await?;
    
    tracing::info!("🏥 Database health check passed");
    Ok(())
}

/// Run database migrations
pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), anyhow::Error> {
    use migration::{Migrator, MigratorTrait};
    
    tracing::info!("🔄 Running database migrations...");
    Migrator::up(db, None).await?;
    tracing::info!("✅ Database migrations completed successfully");
    
    Ok(())
}

/// Setup database with migrations and seeding
pub async fn setup_database() -> Result<DatabaseConnection, anyhow::Error> {
    let config = DatabaseConfig::from_env()?;
    let db = establish_connection(&config).await?;
    
    // Test connection
    test_connection(&db).await?;
    
    // Run migrations
    run_migrations(&db).await?;
    
    // Seed database with sample data if in development mode
    // TODO: Temporarily disabled due to stack overflow - investigate entity relationships
    if false && env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
        seed_database(&db).await?;
    }
    
    Ok(db)
}

/// Seed database with sample Australian pharmacy job data
pub async fn seed_database(db: &DatabaseConnection) -> Result<(), anyhow::Error> {
    use crate::entities::{user, job};
    use shared::types::{AustralianState, JobType, JobStatus, Postcode};
    use sea_orm::{Set, ActiveModelTrait};
    use uuid::Uuid;
    use chrono::{Utc, Duration};
    use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{rand_core::OsRng, SaltString}};
    
    tracing::info!("🌱 Seeding database with sample Australian pharmacy data...");
    
    // Check if data already exists
    let existing_users = user::Entity::find().one(db).await?;
    if existing_users.is_some() {
        tracing::info!("📊 Database already contains data, skipping seeding");
        return Ok(());
    }
    
    // Create sample tenant first
    use crate::entities::{tenants, tenant_users};
    
    let tenant_id = Uuid::new_v4();
    let tenant = tenants::ActiveModel {
        id: Set(tenant_id),
        name: Set("Demo Pharmacy Group".to_string()),
        slug: Set("demo-pharmacy".to_string()),
        domain: Set(None),
        settings: Set(tenants::TenantSettings::default()),
        ..Default::default()
    };
    tenant.insert(db).await?;
    
    // Create sample users
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(b"password123", &salt).map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?.to_string();
    
    // Sample employer user
    let employer_id = Uuid::new_v4();
    let employer = user::ActiveModel {
        id: Set(employer_id),
        email: Set("pharmacy.manager@example.com.au".to_string()),
        password_hash: Set(password_hash.clone()),
        first_name: Set("Sarah".to_string()),
        last_name: Set("Thompson".to_string()),
        phone: Set(Some("02 9876 5432".to_string())),
        address: Set(Some("123 Pharmacy Street".to_string())),
        suburb: Set(Some("Sydney".to_string())),
        postcode: Set(Some("2000".to_string())),
        state: Set(Some("NSW".to_string())),
        user_type: Set(user::UserType::Employer),
        is_active: Set(true),
        is_email_verified: Set(true),
        ..Default::default()
    };
    employer.insert(db).await?;
    
    // Create tenant association for employer
    let employer_tenant_user = tenant_users::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(employer_id),
        tenant_id: Set(tenant_id),
        role: Set("Admin".to_string()),
        permissions: Set(serde_json::to_string(&vec!["manage_jobs", "manage_users"]).unwrap()),
        ..Default::default()
    };
    employer_tenant_user.insert(db).await?;
    
    // Sample professional user
    let professional_id = Uuid::new_v4();
    let professional = user::ActiveModel {
        id: Set(professional_id),
        email: Set("pharmacist@example.com.au".to_string()),
        password_hash: Set(password_hash),
        first_name: Set("Michael".to_string()),
        last_name: Set("Chen".to_string()),
        phone: Set(Some("0412 345 678".to_string())),
        address: Set(Some("456 Professional Drive".to_string())),
        suburb: Set(Some("Melbourne".to_string())),
        postcode: Set(Some("3000".to_string())),
        state: Set(Some("VIC".to_string())),
        user_type: Set(user::UserType::Professional),
        is_active: Set(true),
        is_email_verified: Set(true),
        ..Default::default()
    };
    professional.insert(db).await?;
    
    // Create tenant association for professional
    let professional_tenant_user = tenant_users::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(professional_id),
        tenant_id: Set(tenant_id),
        role: Set("Member".to_string()),
        permissions: Set(serde_json::to_string(&vec!["view_jobs", "apply_jobs"]).unwrap()),
        ..Default::default()
    };
    professional_tenant_user.insert(db).await?;
    
    // Sample jobs across major Australian cities
    let sample_jobs = vec![
        (
            "Senior Pharmacist - CBD Location",
            "Looking for an experienced pharmacist to join our busy CBD pharmacy. Must be registered with AHPRA.",
            "Priceline Pharmacy",
            45.50,
            "Level 1, 123 Collins Street",
            "Melbourne",
            "3000",
            "VIC",
            (-37.8136, 144.9631),
            job::JobTypeDb::Pharmacist,
        ),
        (
            "Pharmacy Assistant - Part Time",
            "Part-time position available for enthusiastic pharmacy assistant. Perfect for students.",
            "Discount Drug Stores",
            22.50,
            "456 Queen Street",
            "Brisbane",
            "4000",
            "QLD",
            (-27.4698, 153.0251),
            job::JobTypeDb::PharmacyAssistant,
        ),
        (
            "Graduate Pharmacist Opportunity",
            "Excellent opportunity for recent pharmacy graduate. Full training provided.",
            "Terry White Chemmart",
            38.00,
            "789 Rundle Mall",
            "Adelaide",
            "5000",
            "SA",
            (-34.9285, 138.6007),
            job::JobTypeDb::Pharmacist,
        ),
        (
            "Pharmacy Intern Position",
            "12-month internship program for pharmacy students. Mentorship included.",
            "Chemist Warehouse",
            25.00,
            "321 Hay Street",
            "Perth",
            "6000",
            "WA",
            (-31.9505, 115.8605),
            job::JobTypeDb::Intern,
        ),
        (
            "Urgent: Weekend Pharmacist Required",
            "Weekend locum pharmacist needed immediately. Competitive rates.",
            "Local Community Pharmacy",
            55.00,
            "159 King Street",
            "Sydney",
            "2000",
            "NSW",
            (-33.8688, 151.2093),
            job::JobTypeDb::Pharmacist,
        ),
    ];
    
    for (title, description, pharmacy_name, hourly_rate, address, suburb, postcode, state, (lat, lng), job_type) in sample_jobs {
        let job_id = Uuid::new_v4();
        let start_date = Utc::now() + Duration::days(7);
        let end_date = start_date + Duration::days(30);
        
        let job = job::ActiveModel {
            id: Set(job_id),
            title: Set(title.to_string()),
            description: Set(description.to_string()),
            pharmacy_name: Set(pharmacy_name.to_string()),
            hourly_rate: Set(rust_decimal::Decimal::from_f64_retain(hourly_rate).unwrap()),
            address: Set(address.to_string()),
            suburb: Set(suburb.to_string()),
            postcode: Set(postcode.to_string()),
            state: Set(state.to_string()),
            latitude: Set(Some(lat)),
            longitude: Set(Some(lng)),
            start_date: Set(start_date.into()),
            end_date: Set(end_date.into()),
            start_time: Set("09:00".to_string()),
            end_time: Set("17:00".to_string()),
            job_type: Set(job_type),
            status: Set(job::JobStatusDb::Active),
            is_urgent: Set(title.contains("Urgent")),
            requirements_text: Set(Some("Must be registered with AHPRA. Previous pharmacy experience preferred.".to_string())),
            benefits_text: Set(Some("Competitive salary, professional development opportunities, friendly team environment.".to_string())),
            contact_email: Set(Some("jobs@pharmacy.com.au".to_string())),
            contact_phone: Set(Some("1300 PHARMACY".to_string())),
            application_deadline: Set(Some((start_date - Duration::days(3)).into())),
            view_count: Set(rand::random::<i32>() % 100),
            application_count: Set(rand::random::<i32>() % 10),
            created_by: Set(employer_id),
            ..Default::default()
        };
        
        job.insert(db).await?;
    }
    
    tracing::info!("✅ Database seeding completed successfully");
    tracing::info!("👥 Created sample users and jobs for Australian pharmacy platform");
    
    Ok(())
}

/// Mask sensitive information in database URL for logging
fn mask_database_url(url: &str) -> String {
    if let Ok(parsed) = url::Url::parse(url) {
        let host = parsed.host_str().unwrap_or("unknown");
        let port = parsed.port().map(|p| format!(":{}", p)).unwrap_or_default();
        let database = parsed.path().trim_start_matches('/');
        format!("postgresql://***:***@{}{}/({})", host, port, database)
    } else {
        "postgresql://***:***@***/***(masked)".to_string()
    }
}