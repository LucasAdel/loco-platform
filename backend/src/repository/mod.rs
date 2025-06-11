pub mod user_repository;
pub mod job_repository;
pub mod application_repository;
pub mod session_repository;

pub use user_repository::UserRepository;
pub use job_repository::JobRepository;
pub use application_repository::ApplicationRepository;
pub use session_repository::SessionRepository;

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbErr};
use uuid::Uuid;

/// Base repository trait for common operations
#[async_trait]
pub trait BaseRepository<Entity, Model> {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<Model>, DbErr>;
    async fn create(&self, db: &DatabaseConnection, model: Entity) -> Result<Model, DbErr>;
    async fn update(&self, db: &DatabaseConnection, model: Entity) -> Result<Model, DbErr>;
    async fn delete(&self, db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr>;
}

/// Pagination helpers
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

impl PaginationParams {
    pub fn new(page: Option<u64>, page_size: Option<u64>) -> Self {
        Self {
            page,
            page_size,
        }
    }
    
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }
    
    pub fn page_size(&self) -> u64 {
        self.page_size.unwrap_or(20).min(100).max(1)
    }
    
    pub fn offset(&self) -> u64 {
        (self.page() - 1) * self.page_size()
    }
}

#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total_count: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResult<T> {
    pub fn new(items: Vec<T>, total_count: u64, pagination: PaginationParams) -> Self {
        let page = pagination.page();
        let page_size = pagination.page_size();
        let total_pages = (total_count + page_size - 1) / page_size;
        let has_next = page < total_pages;
        let has_prev = page > 1;
        
        Self {
            items,
            total_count,
            page,
            page_size,
            total_pages,
            has_next,
            has_prev,
        }
    }
}