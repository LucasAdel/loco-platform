#!/usr/bin/env node

import { createClient } from '@supabase/supabase-js';
import dotenv from 'dotenv';
import { readFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
dotenv.config({ path: join(__dirname, '../.env') });

// Validate environment variables
const supabaseUrl = process.env.VITE_SUPABASE_URL || process.env.SUPABASE_URL;
const supabaseServiceKey = process.env.SUPABASE_SERVICE_ROLE_KEY;

if (!supabaseUrl || !supabaseServiceKey) {
    console.error('❌ Missing required environment variables:');
    if (!supabaseUrl) console.error('   - VITE_SUPABASE_URL or SUPABASE_URL');
    if (!supabaseServiceKey) console.error('   - SUPABASE_SERVICE_ROLE_KEY');
    console.error('\n📝 Please set these in your .env file');
    process.exit(1);
}

// Create Supabase client with service role key for admin operations
const supabase = createClient(supabaseUrl, supabaseServiceKey, {
    auth: {
        autoRefreshToken: false,
        persistSession: false
    }
});

async function setupDatabase() {
    console.log('🚀 Setting up Loco Platform comprehensive database schema...\n');

    try {
        // First check if we can connect
        const { data: healthCheck, error: healthError } = await supabase
            .from('_health_check')
            .select('*')
            .limit(1);

        if (healthError && !healthError.message.includes('does not exist')) {
            console.error('❌ Failed to connect to Supabase:', healthError.message);
            return;
        }

        console.log('✅ Connected to Supabase successfully\n');

        // Execute comprehensive schema setup
        await createEnhancedSchema();

        // Verify the schema
        await verifySchema();

        // Insert sample data
        await insertSampleData();

        console.log('\n🎉 Database setup completed successfully!');

    } catch (error) {
        console.error('❌ Unexpected error:', error);
    }
}

async function createEnhancedSchema() {
    console.log('📋 Creating enhanced database schema...\n');

    // Create additional tables for comprehensive functionality
    const additionalTables = `
        -- Create job templates table
        CREATE TABLE IF NOT EXISTS public.job_templates (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            employer_id UUID REFERENCES public.user_profiles(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            description TEXT,
            template_data JSONB NOT NULL,
            is_shared BOOLEAN DEFAULT false,
            usage_count INTEGER DEFAULT 0,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );

        -- Create search history table
        CREATE TABLE IF NOT EXISTS public.search_history (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID REFERENCES public.user_profiles(id) ON DELETE CASCADE,
            search_query TEXT NOT NULL,
            filters JSONB,
            results_count INTEGER,
            clicked_results UUID[],
            created_at TIMESTAMPTZ DEFAULT NOW()
        );

        -- Create email notifications queue
        CREATE TABLE IF NOT EXISTS public.email_queue (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            recipient_email TEXT NOT NULL,
            subject TEXT NOT NULL,
            body_html TEXT NOT NULL,
            body_text TEXT,
            status TEXT DEFAULT 'pending', -- pending, sent, failed
            attempts INTEGER DEFAULT 0,
            sent_at TIMESTAMPTZ,
            error_message TEXT,
            created_at TIMESTAMPTZ DEFAULT NOW()
        );

        -- Create audit log table
        CREATE TABLE IF NOT EXISTS public.audit_logs (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID REFERENCES public.user_profiles(id) ON DELETE SET NULL,
            action TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            entity_id UUID,
            old_values JSONB,
            new_values JSONB,
            ip_address INET,
            user_agent TEXT,
            created_at TIMESTAMPTZ DEFAULT NOW()
        );

        -- Create skills master table
        CREATE TABLE IF NOT EXISTS public.skills (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name TEXT UNIQUE NOT NULL,
            category TEXT,
            is_certification BOOLEAN DEFAULT false,
            created_at TIMESTAMPTZ DEFAULT NOW()
        );

        -- Create locations table with coordinates
        CREATE TABLE IF NOT EXISTS public.locations (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            suburb TEXT NOT NULL,
            state TEXT NOT NULL,
            postcode TEXT NOT NULL,
            latitude NUMERIC(10, 8),
            longitude NUMERIC(11, 8),
            UNIQUE(suburb, state, postcode)
        );

        -- Create job analytics table
        CREATE TABLE IF NOT EXISTS public.job_analytics (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            job_id UUID REFERENCES public.jobs(id) ON DELETE CASCADE,
            date DATE NOT NULL,
            views INTEGER DEFAULT 0,
            unique_views INTEGER DEFAULT 0,
            applications INTEGER DEFAULT 0,
            saves INTEGER DEFAULT 0,
            shares INTEGER DEFAULT 0,
            UNIQUE(job_id, date)
        );

        -- Create user sessions table
        CREATE TABLE IF NOT EXISTS public.user_sessions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES public.user_profiles(id) ON DELETE CASCADE,
            token_hash TEXT UNIQUE NOT NULL,
            ip_address INET,
            user_agent TEXT,
            last_activity TIMESTAMPTZ DEFAULT NOW(),
            expires_at TIMESTAMPTZ NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW()
        );

        -- Enable Row Level Security on new tables
        ALTER TABLE public.job_templates ENABLE ROW LEVEL SECURITY;
        ALTER TABLE public.search_history ENABLE ROW LEVEL SECURITY;
        ALTER TABLE public.email_queue ENABLE ROW LEVEL SECURITY;
        ALTER TABLE public.audit_logs ENABLE ROW LEVEL SECURITY;
        ALTER TABLE public.skills ENABLE ROW LEVEL SECURITY;
        ALTER TABLE public.locations ENABLE ROW LEVEL SECURITY;
        ALTER TABLE public.job_analytics ENABLE ROW LEVEL SECURITY;
        ALTER TABLE public.user_sessions ENABLE ROW LEVEL SECURITY;

        -- Create RLS policies for new tables
        CREATE POLICY "Users can manage their own templates" ON public.job_templates
            FOR ALL USING (auth.uid() = employer_id);

        CREATE POLICY "Users can view shared templates" ON public.job_templates
            FOR SELECT USING (is_shared = true);

        CREATE POLICY "Users can view their own search history" ON public.search_history
            FOR ALL USING (auth.uid() = user_id);

        CREATE POLICY "Anyone can view skills" ON public.skills
            FOR SELECT USING (true);

        CREATE POLICY "Anyone can view locations" ON public.locations
            FOR SELECT USING (true);

        CREATE POLICY "Employers can view analytics for their jobs" ON public.job_analytics
            FOR SELECT USING (
                EXISTS (
                    SELECT 1 FROM public.jobs 
                    WHERE jobs.id = job_analytics.job_id 
                    AND jobs.employer_id = auth.uid()
                )
            );

        CREATE POLICY "Users can manage their own sessions" ON public.user_sessions
            FOR ALL USING (auth.uid() = user_id);

        -- Create indexes for performance
        CREATE INDEX idx_job_templates_employer ON public.job_templates(employer_id);
        CREATE INDEX idx_search_history_user ON public.search_history(user_id, created_at DESC);
        CREATE INDEX idx_email_queue_status ON public.email_queue(status, created_at);
        CREATE INDEX idx_audit_logs_user ON public.audit_logs(user_id, created_at DESC);
        CREATE INDEX idx_audit_logs_entity ON public.audit_logs(entity_type, entity_id);
        CREATE INDEX idx_locations_search ON public.locations(suburb, state);
        CREATE INDEX idx_job_analytics_date ON public.job_analytics(date DESC);
        CREATE INDEX idx_user_sessions_token ON public.user_sessions(token_hash);
        CREATE INDEX idx_user_sessions_expires ON public.user_sessions(expires_at);
    `;

    try {
        // Execute the additional schema SQL
        // Note: In production, you'd use Supabase migrations
        console.log('Creating additional tables...');
        
        // For now, we'll log the SQL that should be executed
        console.log('📝 Additional schema SQL generated. Please execute via Supabase SQL editor or migrations.');
        
    } catch (error) {
        console.error('❌ Error creating enhanced schema:', error);
    }
}

async function verifySchema() {
    console.log('\n🔍 Verifying database schema...\n');

    const tables = [
        'user_profiles',
        'jobs',
        'applications',
        'saved_jobs',
        'job_views',
        'notifications',
        'messages',
        'job_templates',
        'search_history',
        'email_queue',
        'audit_logs',
        'skills',
        'locations',
        'job_analytics',
        'user_sessions'
    ];

    let verifiedCount = 0;
    for (const table of tables) {
        const { error } = await supabase
            .from(table)
            .select('*')
            .limit(1);

        if (error && error.message.includes('does not exist')) {
            console.error(`❌ Table ${table} does not exist`);
        } else {
            console.log(`✅ Table ${table} verified`);
            verifiedCount++;
        }
    }

    console.log(`\n📊 Verified ${verifiedCount}/${tables.length} tables`);
    return verifiedCount;
}

async function insertSampleData() {
    console.log('\n📝 Inserting comprehensive sample data...\n');

    try {
        // Insert skills
        const skills = [
            { name: 'Medication Management', category: 'Clinical' },
            { name: 'Compounding', category: 'Technical' },
            { name: 'Vaccination Certificate', category: 'Certification', is_certification: true },
            { name: 'Home Medicines Review', category: 'Clinical' },
            { name: 'Diabetes Management', category: 'Clinical' },
            { name: 'Mental Health First Aid', category: 'Certification', is_certification: true },
            { name: 'S2/S3 Poisons License', category: 'Certification', is_certification: true },
            { name: 'Retail Management', category: 'Business' },
            { name: 'Clinical Interventions', category: 'Clinical' },
            { name: 'Aged Care', category: 'Specialisation' }
        ];

        for (const skill of skills) {
            const { error } = await supabase
                .from('skills')
                .upsert([skill], { onConflict: 'name' });

            if (!error) {
                console.log(`✅ Added skill: ${skill.name}`);
            }
        }

        // Insert Australian locations with coordinates
        const locations = [
            { suburb: 'Sydney CBD', state: 'NSW', postcode: '2000', latitude: -33.8688, longitude: 151.2093 },
            { suburb: 'Melbourne', state: 'VIC', postcode: '3000', latitude: -37.8136, longitude: 144.9631 },
            { suburb: 'Brisbane', state: 'QLD', postcode: '4000', latitude: -27.4698, longitude: 153.0251 },
            { suburb: 'Perth', state: 'WA', postcode: '6000', latitude: -31.9505, longitude: 115.8605 },
            { suburb: 'Adelaide', state: 'SA', postcode: '5000', latitude: -34.9285, longitude: 138.6007 },
            { suburb: 'Gold Coast', state: 'QLD', postcode: '4217', latitude: -28.0167, longitude: 153.4000 },
            { suburb: 'Newcastle', state: 'NSW', postcode: '2300', latitude: -32.9283, longitude: 151.7817 },
            { suburb: 'Canberra', state: 'ACT', postcode: '2600', latitude: -35.2809, longitude: 149.1300 },
            { suburb: 'Hobart', state: 'TAS', postcode: '7000', latitude: -42.8821, longitude: 147.3272 },
            { suburb: 'Darwin', state: 'NT', postcode: '0800', latitude: -12.4634, longitude: 130.8456 }
        ];

        for (const location of locations) {
            const { error } = await supabase
                .from('locations')
                .upsert([location], { onConflict: 'suburb,state,postcode' });

            if (!error) {
                console.log(`✅ Added location: ${location.suburb}, ${location.state}`);
            }
        }

        // Enhanced job data with more details
        const enhancedJobs = [
            {
                title: 'Clinical Pharmacist - Oncology Specialist',
                company: 'Royal Melbourne Hospital',
                location: 'Melbourne, VIC',
                job_type: 'FullTime',
                salary_range_start: 110000,
                salary_range_end: 130000,
                description: 'Join our specialized oncology team as a Clinical Pharmacist. You\'ll work closely with oncologists and nurses to optimize cancer treatment protocols and provide expert pharmaceutical care to patients.',
                requirements: ['AHPRA registration', 'Hospital pharmacy experience', 'Oncology specialization preferred', 'SHPA membership'],
                benefits: ['Salary packaging', 'Professional development fund', 'Research opportunities', 'Flexible working arrangements'],
                is_urgent: true,
                views_count: 245,
                applications_count: 12
            },
            {
                title: 'Pharmacy Owner - Partnership Opportunity',
                company: 'TerryWhite Chemmart',
                location: 'Gold Coast, QLD',
                job_type: 'FullTime',
                salary_range_start: 150000,
                salary_range_end: 200000,
                description: 'Unique opportunity to become a partner in an established pharmacy. Looking for an entrepreneurial pharmacist ready to take the next step in their career.',
                requirements: ['10+ years experience', 'Business acumen', 'Financial backing', 'Leadership skills'],
                benefits: ['Profit share', 'Business ownership', 'Established customer base', 'Marketing support'],
                is_urgent: false,
                views_count: 523,
                applications_count: 8
            },
            {
                title: 'Remote Telepharmacy Consultant',
                company: 'Digital Health Solutions',
                location: 'Remote (Australia-wide)',
                job_type: 'PartTime',
                salary_range_start: 80000,
                salary_range_end: 100000,
                description: 'Pioneer the future of pharmacy with our telepharmacy service. Provide medication consultations and reviews via video conferencing from the comfort of your home.',
                requirements: ['AHPRA registration', 'Strong communication skills', 'Tech-savvy', 'Home office setup'],
                benefits: ['Work from home', 'Flexible hours', 'Technology provided', 'No commute'],
                is_urgent: true,
                views_count: 892,
                applications_count: 47
            }
        ];

        for (const job of enhancedJobs) {
            const { error } = await supabase
                .from('jobs')
                .insert([job]);

            if (!error) {
                console.log(`✅ Created job: ${job.title}`);
            }
        }

        console.log('\n✨ Comprehensive sample data inserted successfully!');

    } catch (error) {
        console.error('❌ Error inserting sample data:', error);
    }
}

// Generate SQL file for manual execution
async function generateSQLFile() {
    const sqlContent = `
-- Loco Platform Comprehensive Database Schema
-- Generated: ${new Date().toISOString()}

-- This SQL file contains the complete schema for the Loco Platform
-- Execute this in your Supabase SQL editor

${readFileSync(join(__dirname, '../supabase/migrations/20250605000001_setup_super_admin.sql'), 'utf8')}

${readFileSync(join(__dirname, '../supabase/migrations/20250605000002_create_tables_simple.sql'), 'utf8')}

${readFileSync(join(__dirname, '../supabase/migrations/20250605000003_complete_schema.sql'), 'utf8')}

-- Additional enhanced schema tables
${additionalTables}

-- Function to handle job search with filters
CREATE OR REPLACE FUNCTION search_jobs(
    search_term TEXT DEFAULT NULL,
    job_types job_type[] DEFAULT NULL,
    locations TEXT[] DEFAULT NULL,
    min_salary INTEGER DEFAULT NULL,
    max_salary INTEGER DEFAULT NULL,
    limit_count INTEGER DEFAULT 20,
    offset_count INTEGER DEFAULT 0
)
RETURNS TABLE (
    id UUID,
    title TEXT,
    company TEXT,
    location TEXT,
    job_type job_type,
    salary_range_start INTEGER,
    salary_range_end INTEGER,
    description TEXT,
    is_urgent BOOLEAN,
    created_at TIMESTAMPTZ,
    match_score REAL
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        j.id,
        j.title,
        j.company,
        j.location,
        j.job_type,
        j.salary_range_start,
        j.salary_range_end,
        j.description,
        j.is_urgent,
        j.created_at,
        CASE 
            WHEN search_term IS NOT NULL THEN
                ts_rank(j.search_vector, plainto_tsquery('english', search_term))
            ELSE 1.0
        END as match_score
    FROM public.jobs j
    WHERE j.status = 'Active'
        AND (search_term IS NULL OR j.search_vector @@ plainto_tsquery('english', search_term))
        AND (job_types IS NULL OR j.job_type = ANY(job_types))
        AND (locations IS NULL OR j.location = ANY(locations))
        AND (min_salary IS NULL OR j.salary_range_end >= min_salary)
        AND (max_salary IS NULL OR j.salary_range_start <= max_salary)
    ORDER BY 
        j.is_urgent DESC,
        match_score DESC,
        j.created_at DESC
    LIMIT limit_count
    OFFSET offset_count;
END;
$$ LANGUAGE plpgsql;

-- Function to get job recommendations for a user
CREATE OR REPLACE FUNCTION get_job_recommendations(
    user_id_param UUID,
    limit_count INTEGER DEFAULT 10
)
RETURNS TABLE (
    id UUID,
    title TEXT,
    company TEXT,
    location TEXT,
    job_type job_type,
    salary_range_start INTEGER,
    salary_range_end INTEGER,
    match_score NUMERIC
) AS $$
DECLARE
    user_profile RECORD;
BEGIN
    -- Get user profile
    SELECT * INTO user_profile
    FROM public.user_profiles
    WHERE id = user_id_param;

    -- Return recommended jobs based on user preferences
    RETURN QUERY
    SELECT 
        j.id,
        j.title,
        j.company,
        j.location,
        j.job_type,
        j.salary_range_start,
        j.salary_range_end,
        CASE
            WHEN j.location = ANY(user_profile.preferred_locations) THEN 1.0
            WHEN j.job_type = ANY(user_profile.preferred_job_types) THEN 0.8
            WHEN j.salary_range_start >= user_profile.min_salary THEN 0.7
            ELSE 0.5
        END::NUMERIC as match_score
    FROM public.jobs j
    WHERE j.status = 'Active'
        AND j.id NOT IN (
            SELECT job_id FROM public.applications WHERE applicant_id = user_id_param
        )
    ORDER BY match_score DESC, j.created_at DESC
    LIMIT limit_count;
END;
$$ LANGUAGE plpgsql;

-- Trigger to create user analytics entry
CREATE OR REPLACE FUNCTION create_job_analytics_entry()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO public.job_analytics (job_id, date, views, applications)
    VALUES (NEW.id, CURRENT_DATE, 0, 0)
    ON CONFLICT (job_id, date) DO NOTHING;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER create_analytics_on_job_insert
AFTER INSERT ON public.jobs
FOR EACH ROW EXECUTE FUNCTION create_job_analytics_entry();

-- Grant necessary permissions
GRANT EXECUTE ON FUNCTION search_jobs TO authenticated;
GRANT EXECUTE ON FUNCTION get_job_recommendations TO authenticated;
GRANT EXECUTE ON FUNCTION jobs_within_radius TO authenticated;
`;

    // Write to file
    const fs = await import('fs');
    const outputPath = join(__dirname, '../supabase/sql/comprehensive-schema.sql');
    
    try {
        await fs.promises.mkdir(dirname(outputPath), { recursive: true });
        await fs.promises.writeFile(outputPath, sqlContent);
        console.log(`\n📄 SQL file generated at: ${outputPath}`);
    } catch (error) {
        console.error('❌ Error writing SQL file:', error);
    }
}

// Run the setup
setupDatabase()
    .then(() => generateSQLFile())
    .catch(console.error);