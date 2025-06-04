use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create users table with Australian localisation
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(ColumnDef::new(User::FirstName).string().not_null())
                    .col(ColumnDef::new(User::LastName).string().not_null())
                    .col(ColumnDef::new(User::Phone).string())
                    .col(ColumnDef::new(User::DateOfBirth).date())
                    .col(ColumnDef::new(User::Address).text())
                    .col(ColumnDef::new(User::Suburb).string())
                    .col(ColumnDef::new(User::Postcode).string_len(4))
                    .col(
                        ColumnDef::new(User::State)
                            .string_len(3)
                            .check(Expr::col(User::State).is_in([
                                "NSW", "VIC", "QLD", "WA", "SA", "TAS", "ACT", "NT"
                            ]))
                    )
                    .col(
                        ColumnDef::new(User::UserType)
                            .string()
                            .not_null()
                            .default("Professional")
                            .check(Expr::col(User::UserType).is_in([
                                "Professional", "Employer", "SuperAdmin"
                            ]))
                    )
                    .col(ColumnDef::new(User::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(User::IsEmailVerified).boolean().not_null().default(false))
                    .col(ColumnDef::new(User::LastLoginAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on email for fast lookups
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_user_email")
                    .table(User::Table)
                    .col(User::Email)
                    .to_owned(),
            )
            .await?;

        // Create index on state for location filtering
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_user_state")
                    .table(User::Table)
                    .col(User::State)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    PasswordHash,
    FirstName,
    LastName,
    Phone,
    DateOfBirth,
    Address,
    Suburb,
    Postcode,
    State,
    UserType,
    IsActive,
    IsEmailVerified,
    LastLoginAt,
    CreatedAt,
    UpdatedAt,
}