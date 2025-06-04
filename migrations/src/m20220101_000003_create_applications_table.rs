use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create applications table for job applications
        manager
            .create_table(
                Table::create()
                    .table(Application::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Application::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Application::JobId).uuid().not_null())
                    .col(ColumnDef::new(Application::UserId).uuid().not_null())
                    .col(ColumnDef::new(Application::CoverLetter).text())
                    .col(ColumnDef::new(Application::ResumeUrl).string())
                    .col(ColumnDef::new(Application::AvailabilityNote).text())
                    .col(ColumnDef::new(Application::ExperienceYears).integer())
                    .col(ColumnDef::new(Application::RegistrationNumber).string()) // AHPRA number
                    .col(ColumnDef::new(Application::PreferredContactMethod).string())
                    .col(
                        ColumnDef::new(Application::Status)
                            .string()
                            .not_null()
                            .default("Pending")
                            .check(Expr::col(Application::Status).is_in([
                                "Pending", "Reviewing", "Shortlisted", "Interviewed", 
                                "Offered", "Accepted", "Rejected", "Withdrawn"
                            ]))
                    )
                    .col(ColumnDef::new(Application::ReviewerNotes).text())
                    .col(ColumnDef::new(Application::InterviewScheduledAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Application::ReviewedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Application::ReviewedBy).uuid())
                    .col(
                        ColumnDef::new(Application::AppliedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Application::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_application_job")
                            .from(Application::Table, Application::JobId)
                            .to(Job::Table, Job::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_application_user")
                            .from(Application::Table, Application::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_application_reviewer")
                            .from(Application::Table, Application::ReviewedBy)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique constraint to prevent duplicate applications
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_application_unique_user_job")
                    .table(Application::Table)
                    .col(Application::UserId)
                    .col(Application::JobId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create indexes for common queries
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_application_job_status")
                    .table(Application::Table)
                    .col(Application::JobId)
                    .col(Application::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_application_user_status")
                    .table(Application::Table)
                    .col(Application::UserId)
                    .col(Application::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_application_applied_at")
                    .table(Application::Table)
                    .col(Application::AppliedAt)
                    .to_owned(),
            )
            .await?;

        // Create sessions table for user authentication
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Session::UserId).uuid().not_null())
                    .col(ColumnDef::new(Session::Token).string().not_null().unique_key())
                    .col(ColumnDef::new(Session::DeviceInfo).text())
                    .col(ColumnDef::new(Session::IpAddress).string())
                    .col(ColumnDef::new(Session::UserAgent).text())
                    .col(ColumnDef::new(Session::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Session::ExpiresAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Session::LastAccessedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Session::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_session_user")
                            .from(Session::Table, Session::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index for session cleanup
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_session_expires_at")
                    .table(Session::Table)
                    .col(Session::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_session_user_active")
                    .table(Session::Table)
                    .col(Session::UserId)
                    .col(Session::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await?;
        
        manager
            .drop_table(Table::drop().table(Application::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Application {
    Table,
    Id,
    JobId,
    UserId,
    CoverLetter,
    ResumeUrl,
    AvailabilityNote,
    ExperienceYears,
    RegistrationNumber,
    PreferredContactMethod,
    Status,
    ReviewerNotes,
    InterviewScheduledAt,
    ReviewedAt,
    ReviewedBy,
    AppliedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Session {
    Table,
    Id,
    UserId,
    Token,
    DeviceInfo,
    IpAddress,
    UserAgent,
    IsActive,
    ExpiresAt,
    LastAccessedAt,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Job {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}