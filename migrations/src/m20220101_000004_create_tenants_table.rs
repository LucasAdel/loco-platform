use sea_orm_migration::prelude::*;
use sea_orm::sea_query::Expr;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create tenants table
        manager
            .create_table(
                Table::create()
                    .table(Tenants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tenants::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(ColumnDef::new(Tenants::Name).string().not_null())
                    .col(
                        ColumnDef::new(Tenants::Slug)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Tenants::Domain).string().unique_key())
                    .col(
                        ColumnDef::new(Tenants::Settings)
                            .json()
                            .not_null()
                            .default(Expr::cust("'{}'::jsonb")),
                    )
                    .col(
                        ColumnDef::new(Tenants::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Tenants::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_tenants_slug")
                    .table(Tenants::Table)
                    .col(Tenants::Slug)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tenants_domain")
                    .table(Tenants::Table)
                    .col(Tenants::Domain)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tenants::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Tenants {
    Table,
    Id,
    Name,
    Slug,
    Domain,
    Settings,
    CreatedAt,
    UpdatedAt,
}