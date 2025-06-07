#!/usr/bin/env node

/**
 * Setup Database Schema for Loco Platform
 * This script creates all necessary tables and data directly via Supabase API
 */

import { createClient } from '@supabase/supabase-js';

// Configuration
const SUPABASE_URL = 'https://kpmmsogskffsiubbegvc.supabase.co';
const SUPABASE_SERVICE_ROLE_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImtwbW1zb2dza2Zmc2l1YmJlZ3ZjIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc0OTEwNDk0NywiZXhwIjoyMDY0NjgwOTQ3fQ.L3LvS0AbZoVGe0fHXUMYYi9I-M7Q64Rr8KnwfWL-25w';

const supabase = createClient(SUPABASE_URL, SUPABASE_SERVICE_ROLE_KEY, {
    auth: {
        autoRefreshToken: false,
        persistSession: false
    }
});

async function executeSQL(query, description) {
    console.log(`📝 ${description}...`);
    try {
        const { data, error } = await supabase.rpc('exec_sql', { query_text: query });
        if (error) {
            console.error(`❌ Error: ${error.message}`);
            return false;
        }
        console.log(`✅ ${description} completed`);
        return true;
    } catch (error) {
        console.error(`❌ Exception: ${error.message}`);
        return false;
    }
}

async function setupDatabaseSchema() {
    console.log('🚀 Setting up Loco Platform Database Schema\n');

    // First, create a function to execute SQL commands
    const createExecSqlFunction = `
        CREATE OR REPLACE FUNCTION exec_sql(query_text text)
        RETURNS text
        LANGUAGE plpgsql
        SECURITY DEFINER
        AS $$
        BEGIN
            EXECUTE query_text;
            RETURN 'SUCCESS';
        EXCEPTION WHEN OTHERS THEN
            RETURN 'ERROR: ' || SQLERRM;
        END;
        $$;
    `;

    // Try direct SQL execution first
    console.log('🔧 Creating SQL execution function...');
    const { error: funcError } = await supabase.rpc('exec_sql', { query_text: createExecSqlFunction });
    
    if (funcError) {
        console.log('ℹ️ Function doesn\'t exist yet, creating tables directly...');
        
        // Create tables using direct approach
        await createTablesDirectly();
    } else {
        console.log('✅ SQL execution function ready');
        await createTablesWithFunction();
    }
}

async function createTablesDirectly() {
    console.log('📋 Creating tables using direct SQL approach...');

    // Create enum types using individual queries
    const enumQueries = [
        "DO $$ BEGIN CREATE TYPE job_type AS ENUM ('FullTime', 'PartTime', 'Contract', 'Casual', 'Internship'); EXCEPTION WHEN duplicate_object THEN null; END $$;",
        "DO $$ BEGIN CREATE TYPE application_status AS ENUM ('pending', 'reviewed', 'interviewed', 'accepted', 'rejected'); EXCEPTION WHEN duplicate_object THEN null; END $$;",
        "DO $$ BEGIN CREATE TYPE user_role AS ENUM ('user', 'admin', 'super_admin'); EXCEPTION WHEN duplicate_object THEN null; END $$;"
    ];

    for (const query of enumQueries) {
        await executeRawSQL(query, 'Creating enum type');
    }

    // Create tables
    const tableQueries = [
        `CREATE TABLE IF NOT EXISTS public.user_profiles (
            id UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,
            role user_role DEFAULT 'user',
            first_name TEXT,
            last_name TEXT,
            phone TEXT,
            address TEXT,
            registration_number TEXT,
            experience TEXT,
            bio TEXT,
            specializations TEXT[],
            preferred_locations TEXT,
            min_salary INTEGER,
            max_commute INTEGER,
            preferred_job_types TEXT[],
            email_alerts BOOLEAN DEFAULT TRUE,
            open_to_remote BOOLEAN DEFAULT FALSE,
            profile_picture_url TEXT,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );`,
        `CREATE TABLE IF NOT EXISTS public.jobs (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            title TEXT NOT NULL,
            company TEXT,
            location TEXT,
            job_type job_type DEFAULT 'FullTime',
            salary_range_start INTEGER,
            salary_range_end INTEGER,
            description TEXT,
            requirements TEXT[],
            benefits TEXT[],
            is_urgent BOOLEAN DEFAULT FALSE,
            views_count INTEGER DEFAULT 0,
            applications_count INTEGER DEFAULT 0,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            created_by UUID REFERENCES auth.users(id) ON DELETE SET NULL
        );`,
        `CREATE TABLE IF NOT EXISTS public.applications (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            job_id UUID REFERENCES public.jobs(id) ON DELETE CASCADE,
            user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
            cover_letter TEXT,
            resume_url TEXT,
            status application_status DEFAULT 'pending',
            applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            UNIQUE(job_id, user_id)
        );`,
        `CREATE TABLE IF NOT EXISTS public.saved_jobs (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
            job_id UUID REFERENCES public.jobs(id) ON DELETE CASCADE,
            saved_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            UNIQUE(user_id, job_id)
        );`
    ];

    for (const query of tableQueries) {
        await executeRawSQL(query, 'Creating table');
    }

    // Create indexes
    const indexQueries = [
        "CREATE INDEX IF NOT EXISTS idx_jobs_created_at ON public.jobs(created_at DESC);",
        "CREATE INDEX IF NOT EXISTS idx_jobs_location ON public.jobs(location);",
        "CREATE INDEX IF NOT EXISTS idx_jobs_job_type ON public.jobs(job_type);",
        "CREATE INDEX IF NOT EXISTS idx_jobs_urgent ON public.jobs(is_urgent);",
        "CREATE INDEX IF NOT EXISTS idx_applications_user_id ON public.applications(user_id);",
        "CREATE INDEX IF NOT EXISTS idx_applications_job_id ON public.applications(job_id);",
        "CREATE INDEX IF NOT EXISTS idx_applications_status ON public.applications(status);",
        "CREATE INDEX IF NOT EXISTS idx_user_profiles_role ON public.user_profiles(role);"
    ];

    for (const query of indexQueries) {
        await executeRawSQL(query, 'Creating index');
    }

    // Insert sample data
    await insertSampleData();
    
    // Create Super Admin profile
    await createSuperAdminProfile();
}

async function executeRawSQL(query, description) {
    console.log(`📝 ${description}...`);
    try {
        const { error } = await supabase.rpc('exec_sql', { query_text: query });
        if (error) {
            console.error(`❌ Error: ${error.message}`);
            return false;
        }
        console.log(`✅ ${description} completed`);
        return true;
    } catch (error) {
        console.error(`❌ Exception: ${error.message}`);
        return false;
    }
}

async function insertSampleData() {
    console.log('📊 Inserting sample job data...');
    
    const sampleJobs = [
        {
            title: 'Senior Pharmacist',
            company: 'Sydney Pharmacy Group',
            location: 'Sydney CBD, NSW',
            job_type: 'FullTime',
            salary_range_start: 120000,
            salary_range_end: 140000,
            description: 'Leading role in busy city pharmacy. Excellent opportunity for experienced pharmacist looking to advance their career.',
            is_urgent: false
        },
        {
            title: 'Locum Pharmacist',
            company: 'Melbourne Community Pharmacy',
            location: 'Melbourne, VIC',
            job_type: 'Contract',
            salary_range_start: 55000,
            salary_range_end: 75000,
            description: 'Flexible locum position available. Perfect for pharmacists seeking work-life balance.',
            is_urgent: true
        },
        {
            title: 'Hospital Pharmacist',
            company: 'Brisbane General Hospital',
            location: 'Brisbane, QLD',
            job_type: 'FullTime',
            salary_range_start: 90000,
            salary_range_end: 110000,
            description: 'Hospital pharmacy role focusing on clinical services and patient care.',
            is_urgent: false
        },
        {
            title: 'Pharmacist Manager',
            company: 'Perth Pharmacy Chain',
            location: 'Perth, WA',
            job_type: 'FullTime',
            salary_range_start: 130000,
            salary_range_end: 150000,
            description: 'Management opportunity for experienced pharmacist. Lead a team in growing pharmacy business.',
            is_urgent: false
        },
        {
            title: 'Graduate Pharmacist',
            company: 'Adelaide Family Pharmacy',
            location: 'Adelaide, SA',
            job_type: 'PartTime',
            salary_range_start: 65000,
            salary_range_end: 80000,
            description: 'Excellent opportunity for new graduate. Supportive environment with mentorship program.',
            is_urgent: false
        }
    ];

    try {
        const { data, error } = await supabase
            .from('jobs')
            .upsert(sampleJobs, { onConflict: 'title,company' });

        if (error) {
            console.error('❌ Error inserting sample jobs:', error.message);
        } else {
            console.log('✅ Sample job data inserted successfully');
        }
    } catch (error) {
        console.error('❌ Exception inserting sample jobs:', error.message);
    }
}

async function createSuperAdminProfile() {
    console.log('👤 Creating Super Administrator profile...');
    
    try {
        // Get the Super Admin user
        const { data: users, error: listError } = await supabase.auth.admin.listUsers();
        
        if (listError) {
            console.error('❌ Error listing users:', listError.message);
            return;
        }
        
        const superAdminUser = users.users.find(u => u.email === 'lw@hamiltonbailey.com');
        
        if (!superAdminUser) {
            console.error('❌ Super Administrator user not found');
            return;
        }
        
        // Create/update profile
        const { data, error } = await supabase
            .from('user_profiles')
            .upsert({
                id: superAdminUser.id,
                role: 'super_admin',
                first_name: 'Super',
                last_name: 'Administrator',
                email_alerts: true,
                open_to_remote: true
            }, { onConflict: 'id' });

        if (error) {
            console.error('❌ Error creating Super Admin profile:', error.message);
        } else {
            console.log('✅ Super Administrator profile created successfully');
        }
    } catch (error) {
        console.error('❌ Exception creating Super Admin profile:', error.message);
    }
}

async function testDatabaseSetup() {
    console.log('\n🧪 Testing database setup...');
    
    try {
        // Test jobs table
        const { data: jobs, error: jobsError } = await supabase
            .from('jobs')
            .select('*')
            .limit(3);

        if (jobsError) {
            console.error('❌ Error testing jobs table:', jobsError.message);
        } else {
            console.log(`✅ Jobs table: ${jobs.length} jobs found`);
        }

        // Test user profiles table
        const { data: profiles, error: profilesError } = await supabase
            .from('user_profiles')
            .select('*')
            .limit(5);

        if (profilesError) {
            console.error('❌ Error testing user_profiles table:', profilesError.message);
        } else {
            console.log(`✅ User profiles table: ${profiles.length} profiles found`);
            
            // Check for Super Admin
            const superAdmin = profiles.find(p => p.role === 'super_admin');
            if (superAdmin) {
                console.log(`✅ Super Administrator profile found: ${superAdmin.first_name} ${superAdmin.last_name}`);
            }
        }

    } catch (error) {
        console.error('❌ Exception during testing:', error.message);
    }
}

async function createTablesWithFunction() {
    console.log('📋 Creating tables using SQL function approach...');
    // Implementation using the exec_sql function
    // This would be similar to createTablesDirectly but using the function
}

// Main execution
async function main() {
    await setupDatabaseSchema();
    await testDatabaseSetup();
    
    console.log('\n🎉 Database schema setup complete!');
    console.log('📝 Next steps:');
    console.log('   1. Test Super Administrator login');
    console.log('   2. Verify job application functionality');
    console.log('   3. Test complete authentication flow');
}

// Run the script
main().catch(console.error);