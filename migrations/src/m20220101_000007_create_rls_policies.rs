use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Enable RLS on tables
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE job ENABLE ROW LEVEL SECURITY")
            .await?;
        
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE application ENABLE ROW LEVEL SECURITY")
            .await?;
        
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE \"user\" ENABLE ROW LEVEL SECURITY")
            .await?;

        // Create helper functions
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE OR REPLACE FUNCTION get_current_tenant() 
                RETURNS UUID AS $$
                BEGIN
                    RETURN current_setting('app.current_tenant', true)::UUID;
                EXCEPTION
                    WHEN OTHERS THEN
                        RETURN NULL;
                END;
                $$ LANGUAGE plpgsql SECURITY DEFINER;
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE OR REPLACE FUNCTION set_current_tenant(tenant_id UUID) 
                RETURNS VOID AS $$
                BEGIN
                    PERFORM set_config('app.current_tenant', tenant_id::TEXT, false);
                END;
                $$ LANGUAGE plpgsql SECURITY DEFINER;
                "#,
            )
            .await?;

        // Create RLS policies for jobs
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE POLICY tenant_isolation_jobs ON job
                FOR ALL
                USING (tenant_id = get_current_tenant() OR get_current_tenant() IS NULL);
                "#,
            )
            .await?;

        // Create RLS policies for applications
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE POLICY tenant_isolation_applications ON application
                FOR ALL
                USING (tenant_id = get_current_tenant() OR get_current_tenant() IS NULL);
                "#,
            )
            .await?;

        // Create RLS policies for users
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE POLICY tenant_isolation_users ON "user"
                FOR ALL
                USING (
                    tenant_id = get_current_tenant() 
                    OR get_current_tenant() IS NULL
                    OR EXISTS (
                        SELECT 1 FROM tenant_users 
                        WHERE tenant_users.user_id = "user".id 
                        AND tenant_users.tenant_id = get_current_tenant()
                    )
                );
                "#,
            )
            .await?;

        // Create audit trigger function
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE OR REPLACE FUNCTION update_updated_at_column()
                RETURNS TRIGGER AS $$
                BEGIN
                    NEW.updated_at = CURRENT_TIMESTAMP;
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql;
                "#,
            )
            .await?;

        // Add update triggers to all tables
        let tables = vec![
            ("user", "\"user\""),
            ("job", "job"),
            ("application", "application"),
            ("tenants", "tenants"),
            ("tenant_users", "tenant_users"),
        ];
        for (trigger_name, table_name) in tables {
            manager
                .get_connection()
                .execute_unprepared(&format!(
                    r#"
                    CREATE TRIGGER update_{}_updated_at 
                    BEFORE UPDATE ON {} 
                    FOR EACH ROW 
                    EXECUTE FUNCTION update_updated_at_column();
                    "#,
                    trigger_name, table_name
                ))
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop triggers
        let tables = vec![
            ("user", "\"user\""),
            ("job", "job"),
            ("application", "application"),
            ("tenants", "tenants"),
            ("tenant_users", "tenant_users"),
        ];
        for (trigger_name, table_name) in tables {
            manager
                .get_connection()
                .execute_unprepared(&format!(
                    "DROP TRIGGER IF EXISTS update_{}_updated_at ON {}",
                    trigger_name, table_name
                ))
                .await?;
        }

        // Drop policies
        manager
            .get_connection()
            .execute_unprepared("DROP POLICY IF EXISTS tenant_isolation_jobs ON job")
            .await?;

        manager
            .get_connection()
            .execute_unprepared("DROP POLICY IF EXISTS tenant_isolation_applications ON application")
            .await?;

        manager
            .get_connection()
            .execute_unprepared("DROP POLICY IF EXISTS tenant_isolation_users ON \"user\"")
            .await?;

        // Disable RLS
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE job DISABLE ROW LEVEL SECURITY")
            .await?;

        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE application DISABLE ROW LEVEL SECURITY")
            .await?;

        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE \"user\" DISABLE ROW LEVEL SECURITY")
            .await?;

        // Drop functions
        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS get_current_tenant()")
            .await?;

        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS set_current_tenant(UUID)")
            .await?;

        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS update_updated_at_column()")
            .await?;

        Ok(())
    }
}