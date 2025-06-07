-- Simple table creation for Loco Platform
-- Migration: 20250605000002_create_tables_simple.sql

-- First, create enum types if they don't exist
DO $$ BEGIN
    CREATE TYPE job_type AS ENUM ('FullTime', 'PartTime', 'Contract', 'Casual', 'Internship');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE application_status AS ENUM ('pending', 'reviewed', 'interviewed', 'accepted', 'rejected');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE user_role AS ENUM ('user', 'admin', 'super_admin');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Create user profiles table
CREATE TABLE IF NOT EXISTS public.user_profiles (
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
CREATE TABLE IF NOT EXISTS public.jobs (
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
CREATE TABLE IF NOT EXISTS public.applications (
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
CREATE TABLE IF NOT EXISTS public.saved_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    job_id UUID REFERENCES public.jobs(id) ON DELETE CASCADE,
    saved_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, job_id)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_jobs_created_at ON public.jobs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_jobs_location ON public.jobs(location);
CREATE INDEX IF NOT EXISTS idx_jobs_job_type ON public.jobs(job_type);
CREATE INDEX IF NOT EXISTS idx_jobs_urgent ON public.jobs(is_urgent);
CREATE INDEX IF NOT EXISTS idx_applications_user_id ON public.applications(user_id);
CREATE INDEX IF NOT EXISTS idx_applications_job_id ON public.applications(job_id);
CREATE INDEX IF NOT EXISTS idx_applications_status ON public.applications(status);
CREATE INDEX IF NOT EXISTS idx_user_profiles_role ON public.user_profiles(role);

-- Enable Row Level Security
ALTER TABLE public.jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.applications ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.user_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.saved_jobs ENABLE ROW LEVEL SECURITY;

-- Drop existing policies if they exist
DROP POLICY IF EXISTS "Jobs are viewable by everyone" ON public.jobs;
DROP POLICY IF EXISTS "Authenticated users can create jobs" ON public.jobs;
DROP POLICY IF EXISTS "Users can update their own jobs" ON public.jobs;
DROP POLICY IF EXISTS "Admin users can manage all jobs" ON public.jobs;
DROP POLICY IF EXISTS "Users can view their own applications" ON public.applications;
DROP POLICY IF EXISTS "Users can create applications" ON public.applications;
DROP POLICY IF EXISTS "Users can update their own applications" ON public.applications;
DROP POLICY IF EXISTS "Job creators can view applications for their jobs" ON public.applications;
DROP POLICY IF EXISTS "Users can view their own profile" ON public.user_profiles;
DROP POLICY IF EXISTS "Users can update their own profile" ON public.user_profiles;
DROP POLICY IF EXISTS "Users can insert their own profile" ON public.user_profiles;
DROP POLICY IF EXISTS "Admin users can view all profiles" ON public.user_profiles;
DROP POLICY IF EXISTS "Users can manage their own saved jobs" ON public.saved_jobs;

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
INSERT INTO public.jobs (title, company, location, job_type, salary_range_start, salary_range_end, description, is_urgent) VALUES
('Senior Pharmacist', 'Sydney Pharmacy Group', 'Sydney CBD, NSW', 'FullTime', 120000, 140000, 'Leading role in busy city pharmacy. Excellent opportunity for experienced pharmacist looking to advance their career.', false),
('Locum Pharmacist', 'Melbourne Community Pharmacy', 'Melbourne, VIC', 'Contract', 55000, 75000, 'Flexible locum position available. Perfect for pharmacists seeking work-life balance.', true),
('Hospital Pharmacist', 'Brisbane General Hospital', 'Brisbane, QLD', 'FullTime', 90000, 110000, 'Hospital pharmacy role focusing on clinical services and patient care.', false),
('Pharmacist Manager', 'Perth Pharmacy Chain', 'Perth, WA', 'FullTime', 130000, 150000, 'Management opportunity for experienced pharmacist. Lead a team in growing pharmacy business.', false),
('Graduate Pharmacist', 'Adelaide Family Pharmacy', 'Adelaide, SA', 'PartTime', 65000, 80000, 'Excellent opportunity for new graduate. Supportive environment with mentorship program.', false)
ON CONFLICT DO NOTHING;

-- Create function for new user profile creation
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

-- Create trigger for new user profile creation
DROP TRIGGER IF EXISTS on_auth_user_created ON auth.users;
CREATE TRIGGER on_auth_user_created
    AFTER INSERT ON auth.users
    FOR EACH ROW EXECUTE FUNCTION public.handle_new_user();

-- Grant permissions
GRANT USAGE ON SCHEMA public TO anon, authenticated;
GRANT ALL ON ALL TABLES IN SCHEMA public TO anon, authenticated;
GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO anon, authenticated;