use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create jobs table with comprehensive Australian pharmacy job fields
        manager
            .create_table(
                Table::create()
                    .table(Job::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Job::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Job::Title).string().not_null())
                    .col(ColumnDef::new(Job::Description).text().not_null())
                    .col(ColumnDef::new(Job::PharmacyName).string().not_null())
                    .col(ColumnDef::new(Job::HourlyRate).decimal().not_null())
                    .col(ColumnDef::new(Job::Address).text().not_null())
                    .col(ColumnDef::new(Job::Suburb).string().not_null())
                    .col(ColumnDef::new(Job::Postcode).string_len(4).not_null())
                    .col(
                        ColumnDef::new(Job::State)
                            .string_len(3)
                            .not_null()
                            .check(Expr::col(Job::State).is_in([
                                "NSW", "VIC", "QLD", "WA", "SA", "TAS", "ACT", "NT"
                            ]))
                    )
                    .col(ColumnDef::new(Job::Latitude).double())
                    .col(ColumnDef::new(Job::Longitude).double())
                    .col(ColumnDef::new(Job::StartDate).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Job::EndDate).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Job::StartTime).string().not_null()) // Format: HH:MM
                    .col(ColumnDef::new(Job::EndTime).string().not_null())   // Format: HH:MM
                    .col(
                        ColumnDef::new(Job::JobType)
                            .string()
                            .not_null()
                            .check(Expr::col(Job::JobType).is_in([
                                "Intern", "Student", "Pharmacist", "PharmacyAssistant", "PharmacyTechnician"
                            ]))
                    )
                    .col(
                        ColumnDef::new(Job::Status)
                            .string()
                            .not_null()
                            .default("Draft")
                            .check(Expr::col(Job::Status).is_in([
                                "Active", "Closed", "Draft", "Filled", "Cancelled", "Expired"
                            ]))
                    )
                    .col(ColumnDef::new(Job::IsUrgent).boolean().not_null().default(false))
                    .col(ColumnDef::new(Job::RequirementsText).text())
                    .col(ColumnDef::new(Job::BenefitsText).text())
                    .col(ColumnDef::new(Job::ContactEmail).string())
                    .col(ColumnDef::new(Job::ContactPhone).string())
                    .col(ColumnDef::new(Job::ApplicationDeadline).timestamp_with_time_zone())
                    .col(ColumnDef::new(Job::ViewCount).integer().not_null().default(0))
                    .col(ColumnDef::new(Job::ApplicationCount).integer().not_null().default(0))
                    .col(ColumnDef::new(Job::CreatedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(Job::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Job::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Job::DeletedAt).timestamp_with_time_zone()) // Soft delete
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_job_created_by")
                            .from(Job::Table, Job::CreatedBy)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for common query patterns
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_job_status_active")
                    .table(Job::Table)
                    .col(Job::Status)
                    .col(Job::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_job_location")
                    .table(Job::Table)
                    .col(Job::State)
                    .col(Job::Suburb)
                    .col(Job::Postcode)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_job_type_rate")
                    .table(Job::Table)
                    .col(Job::JobType)
                    .col(Job::HourlyRate)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_job_coordinates")
                    .table(Job::Table)
                    .col(Job::Latitude)
                    .col(Job::Longitude)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_job_urgent_date")
                    .table(Job::Table)
                    .col(Job::IsUrgent)
                    .col(Job::StartDate)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_job_soft_delete")
                    .table(Job::Table)
                    .col(Job::DeletedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Job::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Job {
    Table,
    Id,
    Title,
    Description,
    PharmacyName,
    HourlyRate,
    Address,
    Suburb,
    Postcode,
    State,
    Latitude,
    Longitude,
    StartDate,
    EndDate,
    StartTime,
    EndTime,
    JobType,
    Status,
    IsUrgent,
    RequirementsText,
    BenefitsText,
    ContactEmail,
    ContactPhone,
    ApplicationDeadline,
    ViewCount,
    ApplicationCount,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}