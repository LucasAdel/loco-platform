pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20220101_000002_create_jobs_table;
mod m20220101_000003_create_applications_table;
mod m20220101_000004_create_tenants_table;
mod m20220101_000005_create_tenant_users_table;
mod m20220101_000006_add_tenant_to_tables;
mod m20220101_000007_create_rls_policies;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20220101_000002_create_jobs_table::Migration),
            Box::new(m20220101_000003_create_applications_table::Migration),
            Box::new(m20220101_000004_create_tenants_table::Migration),
            Box::new(m20220101_000005_create_tenant_users_table::Migration),
            Box::new(m20220101_000006_add_tenant_to_tables::Migration),
            Box::new(m20220101_000007_create_rls_policies::Migration),
        ]
    }
}