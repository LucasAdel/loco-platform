use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add tenant_id to jobs table
        manager
            .alter_table(
                Table::alter()
                    .table(Job::Table)
                    .add_column(ColumnDef::new(Job::TenantId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_jobs_tenant")
                            .from_tbl(Job::Table)
                            .from_col(Job::TenantId)
                            .to_tbl(Tenants::Table)
                            .to_col(Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Add tenant_id to applications table
        manager
            .alter_table(
                Table::alter()
                    .table(Application::Table)
                    .add_column(ColumnDef::new(Application::TenantId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_applications_tenant")
                            .from_tbl(Application::Table)
                            .from_col(Application::TenantId)
                            .to_tbl(Tenants::Table)
                            .to_col(Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Add tenant_id to users table
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(User::TenantId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_users_tenant")
                            .from_tbl(User::Table)
                            .from_col(User::TenantId)
                            .to_tbl(Tenants::Table)
                            .to_col(Tenants::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for performance
        manager
            .create_index(
                Index::create()
                    .name("idx_jobs_tenant")
                    .table(Job::Table)
                    .col(Job::TenantId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_applications_tenant")
                    .table(Application::Table)
                    .col(Application::TenantId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_tenant")
                    .table(User::Table)
                    .col(User::TenantId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove foreign keys and columns
        manager
            .alter_table(
                Table::alter()
                    .table(Job::Table)
                    .drop_foreign_key(Alias::new("fk_jobs_tenant"))
                    .drop_column(Job::TenantId)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Application::Table)
                    .drop_foreign_key(Alias::new("fk_applications_tenant"))
                    .drop_column(Application::TenantId)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_foreign_key(Alias::new("fk_users_tenant"))
                    .drop_column(User::TenantId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Job {
    Table,
    TenantId,
}

#[derive(Iden)]
enum Application {
    Table,
    TenantId,
}

#[derive(Iden)]
enum User {
    Table,
    TenantId,
}

#[derive(Iden)]
enum Tenants {
    Table,
    Id,
}