-- Enable necessary extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "postgis";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- Create custom types
CREATE TYPE user_type AS ENUM ('Professional', 'Employer', 'Admin');
CREATE TYPE job_type AS ENUM ('FullTime', 'PartTime', 'Casual', 'Contract', 'Locum');
CREATE TYPE job_status AS ENUM ('Draft', 'Active', 'Expired', 'Filled', 'Cancelled');
CREATE TYPE application_status AS ENUM ('Pending', 'Reviewing', 'Shortlisted', 'Interview', 'Offered', 'Accepted', 'Rejected', 'Withdrawn');
CREATE TYPE work_right_status AS ENUM ('Citizen', 'PermanentResident', 'TemporaryVisa', 'WorkingHoliday', 'Student', 'Other');

-- Create users profile table (extends Supabase auth.users)
CREATE TABLE IF NOT EXISTS public.user_profiles (
    id UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,
    email TEXT UNIQUE NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    phone TEXT,
    user_type user_type NOT NULL DEFAULT 'Professional',
    
    -- Professional fields
    ahpra_number TEXT,
    registration_state TEXT,
    registration_expiry DATE,
    years_experience INTEGER,
    work_rights work_right_status,
    
    -- Employer fields
    company_name TEXT,
    company_abn TEXT,
    position_title TEXT,
    
    -- Common fields
    bio TEXT,
    skills TEXT[],
    certifications TEXT[],
    profile_photo_url TEXT,
    resume_url TEXT,
    
    -- Location
    address_line1 TEXT,
    address_line2 TEXT,
    suburb TEXT,
    state TEXT,
    postcode TEXT,
    latitude NUMERIC(10, 8),
    longitude NUMERIC(11, 8),
    
    -- Preferences
    preferred_job_types job_type[],
    preferred_locations TEXT[],
    min_salary INTEGER,
    available_from DATE,
    
    -- Metadata
    is_active BOOLEAN DEFAULT true,
    email_verified BOOLEAN DEFAULT false,
    profile_completed BOOLEAN DEFAULT false,
    last_login TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create jobs table
CREATE TABLE IF NOT EXISTS public.jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employer_id UUID NOT NULL REFERENCES public.user_profiles(id) ON DELETE CASCADE,
    
    -- Basic information
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    company_name TEXT NOT NULL,
    
    -- Job details
    job_type job_type NOT NULL,
    status job_status NOT NULL DEFAULT 'Draft',
    
    -- Location
    address_line1 TEXT,
    address_line2 TEXT,
    suburb TEXT NOT NULL,
    state TEXT NOT NULL,
    postcode TEXT NOT NULL,
    latitude NUMERIC(10, 8),
    longitude NUMERIC(11, 8),
    
    -- Compensation
    salary_min INTEGER,
    salary_max INTEGER,
    salary_period TEXT DEFAULT 'annual', -- annual, hourly, daily
    benefits TEXT[],
    
    -- Requirements
    required_skills TEXT[],
    preferred_skills TEXT[],
    certifications_required TEXT[],
    years_experience_min INTEGER DEFAULT 0,
    years_experience_max INTEGER,
    
    -- Schedule
    start_date DATE,
    end_date DATE, -- For contracts/locums
    hours_per_week NUMERIC(4, 1),
    schedule_details TEXT,
    
    -- Application settings
    application_deadline DATE,
    applications_open BOOLEAN DEFAULT true,
    external_apply_url TEXT,
    contact_email TEXT,
    contact_phone TEXT,
    
    -- Visibility
    is_featured BOOLEAN DEFAULT false,
    is_urgent BOOLEAN DEFAULT false,
    
    -- Analytics
    view_count INTEGER DEFAULT 0,
    application_count INTEGER DEFAULT 0,
    
    -- Metadata
    published_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Full text search
    search_vector tsvector GENERATED ALWAYS AS (
        setweight(to_tsvector('english', coalesce(title, '')), 'A') ||
        setweight(to_tsvector('english', coalesce(description, '')), 'B') ||
        setweight(to_tsvector('english', coalesce(company_name, '')), 'C') ||
        setweight(to_tsvector('english', coalesce(suburb || ' ' || state, '')), 'D')
    ) STORED
);

-- Create applications table
CREATE TABLE IF NOT EXISTS public.applications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id UUID NOT NULL REFERENCES public.jobs(id) ON DELETE CASCADE,
    applicant_id UUID NOT NULL REFERENCES public.user_profiles(id) ON DELETE CASCADE,
    
    -- Application content
    cover_letter TEXT,
    resume_url TEXT,
    portfolio_url TEXT,
    
    -- Availability
    available_from DATE,
    salary_expectation INTEGER,
    
    -- Status tracking
    status application_status NOT NULL DEFAULT 'Pending',
    status_updated_at TIMESTAMPTZ DEFAULT NOW(),
    status_history JSONB DEFAULT '[]',
    
    -- Employer actions
    viewed_at TIMESTAMPTZ,
    shortlisted_at TIMESTAMPTZ,
    interview_scheduled_at TIMESTAMPTZ,
    offer_made_at TIMESTAMPTZ,
    
    -- Scoring (for ranking)
    match_score NUMERIC(3, 2), -- 0.00 to 1.00
    employer_rating INTEGER CHECK (employer_rating >= 1 AND employer_rating <= 5),
    employer_notes TEXT,
    
    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Ensure one application per job per user
    UNIQUE(job_id, applicant_id)
);

-- Create saved jobs table
CREATE TABLE IF NOT EXISTS public.saved_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES public.user_profiles(id) ON DELETE CASCADE,
    job_id UUID NOT NULL REFERENCES public.jobs(id) ON DELETE CASCADE,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, job_id)
);

-- Create job views tracking table
CREATE TABLE IF NOT EXISTS public.job_views (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id UUID NOT NULL REFERENCES public.jobs(id) ON DELETE CASCADE,
    user_id UUID REFERENCES public.user_profiles(id) ON DELETE CASCADE,
    ip_address INET,
    user_agent TEXT,
    referrer TEXT,
    viewed_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create notifications table
CREATE TABLE IF NOT EXISTS public.notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES public.user_profiles(id) ON DELETE CASCADE,
    
    type TEXT NOT NULL, -- 'application_received', 'status_changed', 'job_expired', etc.
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    data JSONB,
    
    is_read BOOLEAN DEFAULT false,
    read_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create messages table for direct messaging
CREATE TABLE IF NOT EXISTS public.messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id UUID NOT NULL,
    sender_id UUID NOT NULL REFERENCES public.user_profiles(id) ON DELETE CASCADE,
    recipient_id UUID NOT NULL REFERENCES public.user_profiles(id) ON DELETE CASCADE,
    
    content TEXT NOT NULL,
    is_read BOOLEAN DEFAULT false,
    read_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX idx_jobs_employer_id ON public.jobs(employer_id);
CREATE INDEX idx_jobs_status ON public.jobs(status) WHERE status = 'Active';
CREATE INDEX idx_jobs_location ON public.jobs(state, suburb);
CREATE INDEX idx_jobs_postcode ON public.jobs(postcode);
CREATE INDEX idx_jobs_job_type ON public.jobs(job_type);
CREATE INDEX idx_jobs_created_at ON public.jobs(created_at DESC);
CREATE INDEX idx_jobs_search ON public.jobs USING GIN(search_vector);
CREATE INDEX idx_jobs_geo ON public.jobs USING GIST(ll_to_earth(latitude, longitude));

CREATE INDEX idx_applications_job_id ON public.applications(job_id);
CREATE INDEX idx_applications_applicant_id ON public.applications(applicant_id);
CREATE INDEX idx_applications_status ON public.applications(status);
CREATE INDEX idx_applications_created_at ON public.applications(created_at DESC);

CREATE INDEX idx_saved_jobs_user_id ON public.saved_jobs(user_id);
CREATE INDEX idx_saved_jobs_job_id ON public.saved_jobs(job_id);

CREATE INDEX idx_notifications_user_id ON public.notifications(user_id);
CREATE INDEX idx_notifications_created_at ON public.notifications(created_at DESC);
CREATE INDEX idx_notifications_unread ON public.notifications(user_id, is_read) WHERE is_read = false;

CREATE INDEX idx_messages_conversation ON public.messages(conversation_id);
CREATE INDEX idx_messages_recipient ON public.messages(recipient_id, is_read) WHERE is_read = false;

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply updated_at triggers
CREATE TRIGGER update_user_profiles_updated_at BEFORE UPDATE ON public.user_profiles
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_jobs_updated_at BEFORE UPDATE ON public.jobs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_applications_updated_at BEFORE UPDATE ON public.applications
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Create view count increment function
CREATE OR REPLACE FUNCTION increment_job_view_count()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE public.jobs 
    SET view_count = view_count + 1 
    WHERE id = NEW.job_id;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER increment_view_count_trigger AFTER INSERT ON public.job_views
    FOR EACH ROW EXECUTE FUNCTION increment_job_view_count();

-- Create application count update function
CREATE OR REPLACE FUNCTION update_application_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE public.jobs 
        SET application_count = application_count + 1 
        WHERE id = NEW.job_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE public.jobs 
        SET application_count = application_count - 1 
        WHERE id = OLD.job_id;
    END IF;
    RETURN NULL;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_application_count_trigger 
AFTER INSERT OR DELETE ON public.applications
    FOR EACH ROW EXECUTE FUNCTION update_application_count();

-- Row Level Security (RLS) Policies
ALTER TABLE public.user_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.applications ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.saved_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.job_views ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.notifications ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.messages ENABLE ROW LEVEL SECURITY;

-- User profiles policies
CREATE POLICY "Users can view their own profile" ON public.user_profiles
    FOR SELECT USING (auth.uid() = id);

CREATE POLICY "Users can update their own profile" ON public.user_profiles
    FOR UPDATE USING (auth.uid() = id);

CREATE POLICY "Public profiles are viewable by all" ON public.user_profiles
    FOR SELECT USING (is_active = true);

-- Jobs policies
CREATE POLICY "Anyone can view active jobs" ON public.jobs
    FOR SELECT USING (status = 'Active' AND applications_open = true);

CREATE POLICY "Employers can manage their own jobs" ON public.jobs
    FOR ALL USING (auth.uid() = employer_id);

-- Applications policies
CREATE POLICY "Applicants can view their own applications" ON public.applications
    FOR SELECT USING (auth.uid() = applicant_id);

CREATE POLICY "Employers can view applications for their jobs" ON public.applications
    FOR SELECT USING (
        EXISTS (
            SELECT 1 FROM public.jobs 
            WHERE jobs.id = applications.job_id 
            AND jobs.employer_id = auth.uid()
        )
    );

CREATE POLICY "Applicants can create applications" ON public.applications
    FOR INSERT WITH CHECK (auth.uid() = applicant_id);

CREATE POLICY "Applicants can update their own applications" ON public.applications
    FOR UPDATE USING (auth.uid() = applicant_id);

-- Saved jobs policies
CREATE POLICY "Users can manage their saved jobs" ON public.saved_jobs
    FOR ALL USING (auth.uid() = user_id);

-- Notifications policies
CREATE POLICY "Users can view their own notifications" ON public.notifications
    FOR ALL USING (auth.uid() = user_id);

-- Messages policies
CREATE POLICY "Users can view their own messages" ON public.messages
    FOR SELECT USING (auth.uid() = sender_id OR auth.uid() = recipient_id);

CREATE POLICY "Users can send messages" ON public.messages
    FOR INSERT WITH CHECK (auth.uid() = sender_id);

-- Helper functions for location-based queries
CREATE OR REPLACE FUNCTION jobs_within_radius(
    lat NUMERIC,
    lng NUMERIC,
    radius_km INTEGER
)
RETURNS SETOF public.jobs AS $$
BEGIN
    RETURN QUERY
    SELECT j.*
    FROM public.jobs j
    WHERE j.status = 'Active'
    AND earth_distance(
        ll_to_earth(j.latitude, j.longitude),
        ll_to_earth(lat, lng)
    ) / 1000 <= radius_km
    ORDER BY earth_distance(
        ll_to_earth(j.latitude, j.longitude),
        ll_to_earth(lat, lng)
    );
END;
$$ LANGUAGE plpgsql;

-- Grant permissions
GRANT USAGE ON SCHEMA public TO anon, authenticated;
GRANT ALL ON ALL TABLES IN SCHEMA public TO authenticated;
GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO authenticated;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO authenticated;

-- Insert some initial data for testing
INSERT INTO public.user_profiles (id, email, first_name, last_name, user_type, company_name)
VALUES 
    ('00000000-0000-0000-0000-000000000001', 'demo@employer.com', 'Demo', 'Employer', 'Employer', 'Demo Pharmacy Group')
ON CONFLICT (id) DO NOTHING;