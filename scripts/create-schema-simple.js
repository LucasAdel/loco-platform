#!/usr/bin/env node

/**
 * Simple Schema Creation for Loco Platform
 * Creates tables using basic Supabase operations
 */

import { createClient } from '@supabase/supabase-js';

// Configuration
const SUPABASE_URL = 'https://kpmmsogskffsiubbegvc.supabase.co';
const SUPABASE_SERVICE_ROLE_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImtwbW1zb2dza2Zmc2l1YmJlZ3ZjIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc0OTEwNDk0NywiZXhwIjoyMDY0NjgwOTQ3fQ.L3LvS0AbZoVGe0fHXUMYYi9I-M7Q64Rr8KnwfWL-25w';

const supabase = createClient(SUPABASE_URL, SUPABASE_SERVICE_ROLE_KEY);

async function createSampleJobs() {
    console.log('📊 Creating sample jobs...');
    
    // Since we can't create tables via API easily, let's just test what we can do
    // and create sample data using the available public APIs
    
    const sampleJobs = [
        {
            id: 'sample-job-1',
            title: 'Senior Pharmacist',
            company: 'Sydney Pharmacy Group',
            location: 'Sydney CBD, NSW',
            job_type: 'FullTime',
            salary_range_start: 120000,
            salary_range_end: 140000,
            description: 'Leading role in busy city pharmacy. Excellent opportunity for experienced pharmacist looking to advance their career.',
            is_urgent: false,
            views_count: 156,
            applications_count: 12,
            created_at: new Date().toISOString()
        },
        {
            id: 'sample-job-2',
            title: 'Locum Pharmacist',
            company: 'Melbourne Community Pharmacy',
            location: 'Melbourne, VIC',
            job_type: 'Contract',
            salary_range_start: 55000,
            salary_range_end: 75000,
            description: 'Flexible locum position available. Perfect for pharmacists seeking work-life balance.',
            is_urgent: true,
            views_count: 89,
            applications_count: 23,
            created_at: new Date(Date.now() - 86400000).toISOString()
        },
        {
            id: 'sample-job-3',
            title: 'Hospital Pharmacist',
            company: 'Brisbane General Hospital',
            location: 'Brisbane, QLD',
            job_type: 'FullTime',
            salary_range_start: 90000,
            salary_range_end: 110000,
            description: 'Hospital pharmacy role focusing on clinical services and patient care.',
            is_urgent: false,
            views_count: 134,
            applications_count: 8,
            created_at: new Date(Date.now() - 172800000).toISOString()
        },
        {
            id: 'sample-job-4',
            title: 'Pharmacist Manager',
            company: 'Perth Pharmacy Chain',
            location: 'Perth, WA',
            job_type: 'FullTime',
            salary_range_start: 130000,
            salary_range_end: 150000,
            description: 'Management opportunity for experienced pharmacist. Lead a team in growing pharmacy business.',
            is_urgent: false,
            views_count: 78,
            applications_count: 15,
            created_at: new Date(Date.now() - 259200000).toISOString()
        },
        {
            id: 'sample-job-5',
            title: 'Graduate Pharmacist',
            company: 'Adelaide Family Pharmacy',
            location: 'Adelaide, SA',
            job_type: 'PartTime',
            salary_range_start: 65000,
            salary_range_end: 80000,
            description: 'Excellent opportunity for new graduate. Supportive environment with mentorship program.',
            is_urgent: false,
            views_count: 92,
            applications_count: 6,
            created_at: new Date(Date.now() - 345600000).toISOString()
        }
    ];

    return sampleJobs;
}

async function testAuthentication() {
    console.log('🔐 Testing Super Administrator authentication...');
    
    try {
        // Test login with Super Admin credentials
        const { data, error } = await supabase.auth.signInWithPassword({
            email: 'lw@hamiltonbailey.com',
            password: 'password123'
        });

        if (error) {
            console.error('❌ Authentication failed:', error.message);
            return false;
        }

        console.log('✅ Super Administrator authentication successful!');
        console.log('👤 User ID:', data.user.id);
        console.log('📧 Email:', data.user.email);
        console.log('🕐 Created:', data.user.created_at);
        
        // Sign out
        await supabase.auth.signOut();
        console.log('🚪 Signed out successfully');
        
        return true;
    } catch (error) {
        console.error('❌ Authentication test error:', error.message);
        return false;
    }
}

async function provideDashboardInstructions() {
    console.log('\n📋 Manual Database Setup Instructions:');
    console.log('=============================================');
    console.log('');
    console.log('Since direct SQL execution is restricted, please follow these steps:');
    console.log('');
    console.log('1. 🌐 Open Supabase Dashboard:');
    console.log('   https://supabase.com/dashboard/project/kpmmsogskffsiubbegvc');
    console.log('');
    console.log('2. 📝 Go to SQL Editor and run this script:');
    console.log('');
    
    const sqlScript = `
-- Loco Platform Database Schema
-- Execute this in the Supabase SQL Editor

-- Create enum types
CREATE TYPE job_type AS ENUM ('FullTime', 'PartTime', 'Contract', 'Casual', 'Internship');
CREATE TYPE application_status AS ENUM ('pending', 'reviewed', 'interviewed', 'accepted', 'rejected');
CREATE TYPE user_role AS ENUM ('user', 'admin', 'super_admin');

-- Create user profiles table
CREATE TABLE public.user_profiles (
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
);

-- Create jobs table
CREATE TABLE public.jobs (
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
);

-- Create applications table
CREATE TABLE public.applications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id UUID REFERENCES public.jobs(id) ON DELETE CASCADE,
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    cover_letter TEXT,
    resume_url TEXT,
    status application_status DEFAULT 'pending',
    applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(job_id, user_id)
);

-- Create saved jobs table
CREATE TABLE public.saved_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    job_id UUID REFERENCES public.jobs(id) ON DELETE CASCADE,
    saved_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, job_id)
);

-- Create indexes
CREATE INDEX idx_jobs_created_at ON public.jobs(created_at DESC);
CREATE INDEX idx_jobs_location ON public.jobs(location);
CREATE INDEX idx_jobs_job_type ON public.jobs(job_type);
CREATE INDEX idx_jobs_urgent ON public.jobs(is_urgent);
CREATE INDEX idx_applications_user_id ON public.applications(user_id);
CREATE INDEX idx_applications_job_id ON public.applications(job_id);
CREATE INDEX idx_applications_status ON public.applications(status);
CREATE INDEX idx_user_profiles_role ON public.user_profiles(role);

-- Enable Row Level Security
ALTER TABLE public.jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.applications ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.user_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.saved_jobs ENABLE ROW LEVEL SECURITY;

-- Create RLS policies
CREATE POLICY "Jobs are viewable by everyone" ON public.jobs FOR SELECT USING (true);
CREATE POLICY "Authenticated users can create jobs" ON public.jobs FOR INSERT WITH CHECK (auth.role() = 'authenticated');
CREATE POLICY "Users can update their own jobs" ON public.jobs FOR UPDATE USING (created_by = auth.uid());

CREATE POLICY "Users can view their own applications" ON public.applications FOR SELECT USING (user_id = auth.uid());
CREATE POLICY "Users can create applications" ON public.applications FOR INSERT WITH CHECK (user_id = auth.uid());
CREATE POLICY "Users can update their own applications" ON public.applications FOR UPDATE USING (user_id = auth.uid());

CREATE POLICY "Users can view their own profile" ON public.user_profiles FOR SELECT USING (id = auth.uid());
CREATE POLICY "Users can update their own profile" ON public.user_profiles FOR UPDATE USING (id = auth.uid());
CREATE POLICY "Users can insert their own profile" ON public.user_profiles FOR INSERT WITH CHECK (id = auth.uid());

CREATE POLICY "Users can manage their own saved jobs" ON public.saved_jobs USING (user_id = auth.uid());

-- Insert sample jobs
INSERT INTO public.jobs (title, company, location, job_type, salary_range_start, salary_range_end, description, is_urgent, views_count, applications_count) VALUES
('Senior Pharmacist', 'Sydney Pharmacy Group', 'Sydney CBD, NSW', 'FullTime', 120000, 140000, 'Leading role in busy city pharmacy. Excellent opportunity for experienced pharmacist looking to advance their career.', false, 156, 12),
('Locum Pharmacist', 'Melbourne Community Pharmacy', 'Melbourne, VIC', 'Contract', 55000, 75000, 'Flexible locum position available. Perfect for pharmacists seeking work-life balance.', true, 89, 23),
('Hospital Pharmacist', 'Brisbane General Hospital', 'Brisbane, QLD', 'FullTime', 90000, 110000, 'Hospital pharmacy role focusing on clinical services and patient care.', false, 134, 8),
('Pharmacist Manager', 'Perth Pharmacy Chain', 'Perth, WA', 'FullTime', 130000, 150000, 'Management opportunity for experienced pharmacist. Lead a team in growing pharmacy business.', false, 78, 15),
('Graduate Pharmacist', 'Adelaide Family Pharmacy', 'Adelaide, SA', 'PartTime', 65000, 80000, 'Excellent opportunity for new graduate. Supportive environment with mentorship program.', false, 92, 6);

-- Create trigger function for new users
CREATE OR REPLACE FUNCTION public.handle_new_user()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO public.user_profiles (id, first_name, last_name, role)
    VALUES (
        NEW.id,
        COALESCE(NEW.raw_user_meta_data->>'firstName', ''),
        COALESCE(NEW.raw_user_meta_data->>'lastName', ''),
        CASE 
            WHEN NEW.email = 'lw@hamiltonbailey.com' THEN 'super_admin'::user_role
            ELSE 'user'::user_role
        END
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Create trigger
CREATE TRIGGER on_auth_user_created
    AFTER INSERT ON auth.users
    FOR EACH ROW EXECUTE FUNCTION public.handle_new_user();

-- Grant permissions
GRANT USAGE ON SCHEMA public TO anon, authenticated;
GRANT ALL ON ALL TABLES IN SCHEMA public TO anon, authenticated;
GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO anon, authenticated;
`;

    console.log(sqlScript);
    console.log('');
    console.log('3. ✅ After running the SQL, return here to test the setup');
    console.log('');
}

async function main() {
    console.log('🚀 Loco Platform Database Setup');
    console.log('=================================\n');
    
    // Test authentication first
    const authWorking = await testAuthentication();
    
    if (authWorking) {
        console.log('✅ Authentication is working correctly');
    } else {
        console.log('❌ Authentication needs to be fixed first');
        return;
    }
    
    // Provide manual setup instructions
    await provideDashboardInstructions();
    
    console.log('📝 Sample job data ready for testing:');
    const jobs = await createSampleJobs();
    jobs.forEach((job, index) => {
        console.log(`   ${index + 1}. ${job.title} - ${job.company} (${job.location})`);
    });
    
    console.log('\n🎯 After completing the SQL setup:');
    console.log('   1. Test login with: lw@hamiltonbailey.com / password123');
    console.log('   2. Verify job listings load correctly');
    console.log('   3. Test job application functionality');
    console.log('   4. Confirm Super Administrator role access');
}

main().catch(console.error);