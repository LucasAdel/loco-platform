use async_trait::async_trait;
use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr};
use uuid::Uuid;

use crate::entities::user;
use super::{BaseRepository, PaginationParams, PaginatedResult};

pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BaseRepository<user::ActiveModel, user::Model> for UserRepository {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find_by_id(id).one(db).await
    }

    async fn create(&self, db: &DatabaseConnection, model: user::ActiveModel) -> Result<user::Model, DbErr> {
        model.insert(db).await
    }

    async fn update(&self, db: &DatabaseConnection, model: user::ActiveModel) -> Result<user::Model, DbErr> {
        model.update(db).await
    }

    async fn delete(&self, db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        user::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}

impl UserRepository {
    /// Find user by email
    pub async fn find_by_email(&self, db: &DatabaseConnection, email: &str) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }
}