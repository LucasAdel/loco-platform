use async_trait::async_trait;
use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr};
use uuid::Uuid;

use crate::entities::application;
use super::{BaseRepository, PaginationParams, PaginatedResult};

#[derive(Clone)]
pub struct ApplicationRepository;

impl ApplicationRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BaseRepository<application::ActiveModel, application::Model> for ApplicationRepository {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<application::Model>, DbErr> {
        application::Entity::find_by_id(id).one(db).await
    }

    async fn create(&self, db: &DatabaseConnection, model: application::ActiveModel) -> Result<application::Model, DbErr> {
        model.insert(db).await
    }

    async fn update(&self, db: &DatabaseConnection, model: application::ActiveModel) -> Result<application::Model, DbErr> {
        model.update(db).await
    }

    async fn delete(&self, db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        application::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}