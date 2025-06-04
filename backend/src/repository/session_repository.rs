use async_trait::async_trait;
use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr};
use uuid::Uuid;

use crate::entities::session;
use super::{BaseRepository, PaginationParams, PaginatedResult};

pub struct SessionRepository;

impl SessionRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BaseRepository<session::ActiveModel, session::Model> for SessionRepository {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<session::Model>, DbErr> {
        session::Entity::find_by_id(id).one(db).await
    }

    async fn create(&self, db: &DatabaseConnection, model: session::ActiveModel) -> Result<session::Model, DbErr> {
        model.insert(db).await
    }

    async fn update(&self, db: &DatabaseConnection, model: session::ActiveModel) -> Result<session::Model, DbErr> {
        model.update(db).await
    }

    async fn delete(&self, db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        session::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}

impl SessionRepository {
    /// Find session by token
    pub async fn find_by_token(&self, db: &DatabaseConnection, token: &str) -> Result<Option<session::Model>, DbErr> {
        session::Entity::find()
            .filter(session::Column::Token.eq(token))
            .filter(session::Column::IsActive.eq(true))
            .one(db)
            .await
    }
}