use sea_orm_migration::prelude::*;
use sea_orm::sea_query::Expr;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create tenant_users table
        manager
            .create_table(
                Table::create()
                    .table(TenantUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TenantUsers::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(ColumnDef::new(TenantUsers::UserId).uuid().not_null())
                    .col(ColumnDef::new(TenantUsers::TenantId).uuid().not_null())
                    .col(ColumnDef::new(TenantUsers::Role).string().not_null())
                    .col(
                        ColumnDef::new(TenantUsers::Permissions)
                            .json()
                            .not_null()
                            .default(Expr::cust("'[]'::jsonb")),
                    )
                    .col(
                        ColumnDef::new(TenantUsers::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TenantUsers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tenant_users_user")
                            .from(TenantUsers::Table, TenantUsers::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tenant_users_tenant")
                            .from(TenantUsers::Table, TenantUsers::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique constraint
        manager
            .create_index(
                Index::create()
                    .name("idx_tenant_users_unique")
                    .table(TenantUsers::Table)
                    .col(TenantUsers::UserId)
                    .col(TenantUsers::TenantId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_tenant_users_user")
                    .table(TenantUsers::Table)
                    .col(TenantUsers::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tenant_users_tenant")
                    .table(TenantUsers::Table)
                    .col(TenantUsers::TenantId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TenantUsers::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum TenantUsers {
    Table,
    Id,
    UserId,
    TenantId,
    Role,
    Permissions,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}

#[derive(Iden)]
enum Tenants {
    Table,
    Id,
}