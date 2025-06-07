-- Setup Super Administrator and core tables for Loco Platform
-- Migration: 20250605000001_setup_super_admin.sql

-- Create enum types
CREATE TYPE job_type AS ENUM ('FullTime', 'PartTime', 'Contract', 'Casual', 'Internship');
CREATE TYPE application_status AS ENUM ('pending', 'reviewed', 'interviewed', 'accepted', 'rejected');
CREATE TYPE user_role AS ENUM ('user', 'admin', 'super_admin');

-- Create jobs table
CREATE TABLE IF NOT EXISTS jobs (
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
CREATE TABLE IF NOT EXISTS applications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id UUID REFERENCES jobs(id) ON DELETE CASCADE,
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    cover_letter TEXT,
    resume_url TEXT,
    status application_status DEFAULT 'pending',
    applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(job_id, user_id)
);

-- Create user profiles table for extended user information
CREATE TABLE IF NOT EXISTS user_profiles (
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

-- Create saved jobs table
CREATE TABLE IF NOT EXISTS saved_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    job_id UUID REFERENCES jobs(id) ON DELETE CASCADE,
    saved_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, job_id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_jobs_created_at ON jobs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_jobs_location ON jobs(location);
CREATE INDEX IF NOT EXISTS idx_jobs_job_type ON jobs(job_type);
CREATE INDEX IF NOT EXISTS idx_jobs_urgent ON jobs(is_urgent);
CREATE INDEX IF NOT EXISTS idx_applications_user_id ON applications(user_id);
CREATE INDEX IF NOT EXISTS idx_applications_job_id ON applications(job_id);
CREATE INDEX IF NOT EXISTS idx_applications_status ON applications(status);
CREATE INDEX IF NOT EXISTS idx_user_profiles_role ON user_profiles(role);

-- Insert sample jobs for development
INSERT INTO jobs (title, company, location, job_type, salary_range_start, salary_range_end, description, is_urgent) VALUES
('Senior Pharmacist', 'Sydney Pharmacy Group', 'Sydney CBD, NSW', 'FullTime', 120000, 140000, 'Leading role in busy city pharmacy. Excellent opportunity for experienced pharmacist looking to advance their career.', false),
('Locum Pharmacist', 'Melbourne Community Pharmacy', 'Melbourne, VIC', 'Contract', 55000, 75000, 'Flexible locum position available. Perfect for pharmacists seeking work-life balance.', true),
('Hospital Pharmacist', 'Brisbane General Hospital', 'Brisbane, QLD', 'FullTime', 90000, 110000, 'Hospital pharmacy role focusing on clinical services and patient care.', false),
('Pharmacist Manager', 'Perth Pharmacy Chain', 'Perth, WA', 'FullTime', 130000, 150000, 'Management opportunity for experienced pharmacist. Lead a team in growing pharmacy business.', false),
('Graduate Pharmacist', 'Adelaide Family Pharmacy', 'Adelaide, SA', 'PartTime', 65000, 80000, 'Excellent opportunity for new graduate. Supportive environment with mentorship program.', false);

-- Enable Row Level Security (RLS)
ALTER TABLE jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE applications ENABLE ROW LEVEL SECURITY;
ALTER TABLE user_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE saved_jobs ENABLE ROW LEVEL SECURITY;

-- Create RLS policies

-- Jobs policies
CREATE POLICY "Jobs are viewable by everyone" ON jobs FOR SELECT USING (true);
CREATE POLICY "Authenticated users can create jobs" ON jobs FOR INSERT WITH CHECK (auth.role() = 'authenticated');
CREATE POLICY "Users can update their own jobs" ON jobs FOR UPDATE USING (created_by = auth.uid());
CREATE POLICY "Admin users can manage all jobs" ON jobs USING (
    EXISTS (
        SELECT 1 FROM user_profiles 
        WHERE id = auth.uid() 
        AND role IN ('admin', 'super_admin')
    )
);

-- Applications policies
CREATE POLICY "Users can view their own applications" ON applications FOR SELECT USING (user_id = auth.uid());
CREATE POLICY "Users can create applications" ON applications FOR INSERT WITH CHECK (user_id = auth.uid());
CREATE POLICY "Users can update their own applications" ON applications FOR UPDATE USING (user_id = auth.uid());
CREATE POLICY "Job creators can view applications for their jobs" ON applications FOR SELECT USING (
    EXISTS (
        SELECT 1 FROM jobs 
        WHERE jobs.id = applications.job_id 
        AND jobs.created_by = auth.uid()
    )
);

-- User profiles policies
CREATE POLICY "Users can view their own profile" ON user_profiles FOR SELECT USING (id = auth.uid());
CREATE POLICY "Users can update their own profile" ON user_profiles FOR UPDATE USING (id = auth.uid());
CREATE POLICY "Users can insert their own profile" ON user_profiles FOR INSERT WITH CHECK (id = auth.uid());
CREATE POLICY "Admin users can view all profiles" ON user_profiles FOR SELECT USING (
    EXISTS (
        SELECT 1 FROM user_profiles up 
        WHERE up.id = auth.uid() 
        AND up.role IN ('admin', 'super_admin')
    )
);

-- Saved jobs policies
CREATE POLICY "Users can manage their own saved jobs" ON saved_jobs USING (user_id = auth.uid());

-- Create functions for automatic profile creation
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

-- Create function to update job application counts
CREATE OR REPLACE FUNCTION update_job_application_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE jobs 
        SET applications_count = applications_count + 1,
            updated_at = NOW()
        WHERE id = NEW.job_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE jobs 
        SET applications_count = GREATEST(applications_count - 1, 0),
            updated_at = NOW()
        WHERE id = OLD.job_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create trigger for application count updates
DROP TRIGGER IF EXISTS trigger_update_job_application_count ON applications;
CREATE TRIGGER trigger_update_job_application_count
    AFTER INSERT OR DELETE ON applications
    FOR EACH ROW EXECUTE FUNCTION update_job_application_count();

-- Grant necessary permissions
GRANT USAGE ON SCHEMA public TO anon, authenticated;
GRANT ALL ON ALL TABLES IN SCHEMA public TO anon, authenticated;
GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO anon, authenticated;