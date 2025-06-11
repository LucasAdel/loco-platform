use leptos::*;
use leptos::prelude::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use crate::components::ui::{Button, ButtonVariant, ButtonSize, LoadingSpinner, SpinnerSize};
use web_sys::{window, Storage, File, FileReader};
use wasm_bindgen::{JsCast, closure::Closure};
use js_sys::Date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub title: String,
    pub location: String,
    pub ahpra_number: Option<String>,
    pub years_experience: u32,
    pub specializations: Vec<String>,
    pub skills: Vec<String>,
    pub linkedin_url: Option<String>,
    pub website_url: Option<String>,
    pub availability: String,
    pub preferred_locations: Vec<String>,
    pub salary_expectations: Option<SalaryRange>,
    pub work_preferences: WorkPreferences,
    pub notifications: NotificationSettings,
    pub privacy_settings: PrivacySettings,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryRange {
    pub min: u32,
    pub max: u32,
    pub currency: String,
    pub period: String, // "annual", "hourly", "daily"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkPreferences {
    pub employment_types: Vec<String>, // Full-time, Part-time, Casual, Contract
    pub work_arrangements: Vec<String>, // On-site, Remote, Hybrid
    pub shift_preferences: Vec<String>, // Day, Evening, Night, Weekend
    pub travel_willingness: String, // None, Local, State, National
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub email_job_alerts: bool,
    pub email_application_updates: bool,
    pub email_marketing: bool,
    pub sms_urgent_updates: bool,
    pub push_notifications: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    pub profile_visibility: String, // Public, Private, Employers Only
    pub show_contact_info: bool,
    pub show_salary_expectations: bool,
    pub show_availability: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobApplication {
    pub id: String,
    pub job_title: String,
    pub company: String,
    pub applied_date: String,
    pub status: String,
    pub last_updated: String,
    pub notes: Option<String>,
    pub interview_date: Option<String>,
    pub salary_offered: Option<u32>,
}

// Sample data for profile
fn get_sample_profile() -> UserProfile {
    UserProfile {
        id: "user_123".to_string(),
        first_name: "Sarah".to_string(),
        last_name: "Johnson".to_string(),
        email: "sarah.johnson@email.com".to_string(),
        phone: Some("+61 412 345 678".to_string()),
        avatar_url: None,
        bio: Some("Experienced clinical pharmacist with a passion for patient care and medication optimization. Specialized in oncology and critical care pharmacy services.".to_string()),
        title: "Senior Clinical Pharmacist".to_string(),
        location: "Sydney, NSW".to_string(),
        ahpra_number: Some("PHA0001234567".to_string()),
        years_experience: 8,
        specializations: vec!["Clinical Pharmacy".to_string(), "Oncology".to_string(), "Critical Care".to_string()],
        skills: vec!["Medication Review".to_string(), "Clinical Trials".to_string(), "Patient Counselling".to_string(), "IV Compounding".to_string()],
        linkedin_url: Some("https://linkedin.com/in/sarahjohnson".to_string()),
        website_url: None,
        availability: "Available immediately".to_string(),
        preferred_locations: vec!["Sydney, NSW".to_string(), "Melbourne, VIC".to_string()],
        salary_expectations: Some(SalaryRange {
            min: 95000,
            max: 125000,
            currency: "AUD".to_string(),
            period: "annual".to_string(),
        }),
        work_preferences: WorkPreferences {
            employment_types: vec!["Full-time".to_string(), "Part-time".to_string()],
            work_arrangements: vec!["On-site".to_string(), "Hybrid".to_string()],
            shift_preferences: vec!["Day".to_string(), "Evening".to_string()],
            travel_willingness: "State".to_string(),
        },
        notifications: NotificationSettings {
            email_job_alerts: true,
            email_application_updates: true,
            email_marketing: false,
            sms_urgent_updates: true,
            push_notifications: true,
        },
        privacy_settings: PrivacySettings {
            profile_visibility: "Employers Only".to_string(),
            show_contact_info: false,
            show_salary_expectations: true,
            show_availability: true,
        },
        created_at: "2023-01-15".to_string(),
        updated_at: "2024-12-06".to_string(),
    }
}

// Sample applications data
fn get_sample_applications() -> Vec<JobApplication> {
    vec![
        JobApplication {
            id: "app_1".to_string(),
            job_title: "Senior Clinical Pharmacist - Oncology".to_string(),
            company: "Royal Prince Alfred Hospital".to_string(),
            applied_date: "2024-12-04".to_string(),
            status: "Interview Scheduled".to_string(),
            last_updated: "2024-12-05".to_string(),
            notes: Some("Interview scheduled for Monday 9th Dec at 2:00 PM".to_string()),
            interview_date: Some("2024-12-09".to_string()),
            salary_offered: Some(115000),
        },
        JobApplication {
            id: "app_2".to_string(),
            job_title: "Pharmacy Manager".to_string(),
            company: "Terry White Chemmart Miranda".to_string(),
            applied_date: "2024-11-28".to_string(),
            status: "Under Review".to_string(),
            last_updated: "2024-12-02".to_string(),
            notes: None,
            interview_date: None,
            salary_offered: None,
        },
        JobApplication {
            id: "app_3".to_string(),
            job_title: "Clinical Consultant - Remote".to_string(),
            company: "Pfizer Australia".to_string(),
            applied_date: "2024-11-20".to_string(),
            status: "Application Viewed".to_string(),
            last_updated: "2024-11-22".to_string(),
            notes: Some("HR confirmed receipt of application".to_string()),
            interview_date: None,
            salary_offered: None,
        },
        JobApplication {
            id: "app_4".to_string(),
            job_title: "Weekend Locum Pharmacist".to_string(),
            company: "Chemist Warehouse Bondi".to_string(),
            applied_date: "2024-11-15".to_string(),
            status: "Offer Extended".to_string(),
            last_updated: "2024-11-25".to_string(),
            notes: Some("Offer extended - $68/hour for weekend shifts".to_string()),
            interview_date: None,
            salary_offered: Some(68),
        },
    ]
}

#[component]
pub fn Profile() -> impl IntoView {
    // Enhanced state management
    let (active_tab, set_active_tab) = create_signal("overview".to_string());
    let (profile_data, set_profile_data) = create_signal(get_sample_profile());
    let (applications_data, set_applications_data) = create_signal(get_sample_applications());
    let (is_editing, set_is_editing) = create_signal(false);
    let (saving, set_saving) = create_signal(false);
    let (avatar_uploading, set_avatar_uploading) = create_signal(false);

    // Calculate profile completion percentage
    let profile_completion = move || {
        let profile = profile_data.get();
        let mut completed = 0;
        let total = 15;
        
        if !profile.first_name.is_empty() { completed += 1; }
        if !profile.last_name.is_empty() { completed += 1; }
        if !profile.email.is_empty() { completed += 1; }
        if profile.phone.is_some() { completed += 1; }
        if profile.bio.is_some() { completed += 1; }
        if !profile.title.is_empty() { completed += 1; }
        if !profile.location.is_empty() { completed += 1; }
        if profile.ahpra_number.is_some() { completed += 1; }
        if profile.years_experience > 0 { completed += 1; }
        if !profile.specializations.is_empty() { completed += 1; }
        if !profile.skills.is_empty() { completed += 1; }
        if !profile.preferred_locations.is_empty() { completed += 1; }
        if profile.salary_expectations.is_some() { completed += 1; }
        if !profile.work_preferences.employment_types.is_empty() { completed += 1; }
        if profile.avatar_url.is_some() { completed += 1; }
        
        (completed * 100) / total
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-tiffany-light/20 via-white to-blue-50 relative overflow-hidden">
            // Animated Background Elements
            <div class="absolute inset-0 overflow-hidden pointer-events-none">
                <div class="absolute -top-40 -right-40 w-80 h-80 bg-gradient-to-br from-tiffany-blue/20 to-purple-400/20 rounded-full blur-3xl animate-pulse"></div>
                <div class="absolute -bottom-40 -left-40 w-80 h-80 bg-gradient-to-tr from-blue-400/20 to-tiffany-blue/20 rounded-full blur-3xl animate-pulse"></div>
            </div>

            <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                // Enhanced Profile Header
                <div class="glass bg-white/90 rounded-2xl p-8 mb-8 shadow-2xl">
                    <ProfileHeader 
                        profile_data=profile_data 
                        set_profile_data=set_profile_data
                        is_editing=is_editing 
                        set_is_editing=set_is_editing
                        avatar_uploading=avatar_uploading
                        set_avatar_uploading=set_avatar_uploading
                        profile_completion=profile_completion
                    />
                </div>

                // Enhanced Navigation Tabs
                <div class="glass bg-white/90 rounded-2xl mb-8 shadow-xl">
                    <ProfileTabs active_tab=active_tab set_active_tab=set_active_tab />
                </div>

                // Content Area
                <div class="glass bg-white/95 rounded-2xl shadow-2xl overflow-hidden">
                    {move || match active_tab.get().as_str() {
                        "overview" => view! {
                            <ProfileOverview 
                                profile_data=profile_data 
                                applications_data=applications_data
                                profile_completion=profile_completion
                            />
                        }.into_view(),
                        "edit" => view! {
                            <ProfileEditor 
                                profile_data=profile_data 
                                set_profile_data=set_profile_data
                                saving=saving
                                set_saving=set_saving
                            />
                        }.into_view(),
                        "applications" => view! {
                            <ApplicationsManager 
                                applications_data=applications_data 
                                set_applications_data=set_applications_data
                            />
                        }.into_view(),
                        "settings" => view! {
                            <ProfileSettings 
                                profile_data=profile_data 
                                set_profile_data=set_profile_data
                            />
                        }.into_view(),
                        "privacy" => view! {
                            <PrivacySettings 
                                profile_data=profile_data 
                                set_profile_data=set_profile_data
                            />
                        }.into_view(),
                        _ => view! {
                            <ProfileOverview 
                                profile_data=profile_data 
                                applications_data=applications_data
                                profile_completion=profile_completion
                            />
                        }.into_view(),
                    }}
                </div>
            </div>

            // Enhanced CSS Framework
            <style>
                {ENHANCED_PROFILE_CSS}
            </style>
        </div>
    }
}

#[component]
fn ProfileField(
    label: &'static str,
    value: &'static str,
) -> impl IntoView {
    view! {
        <div>
            <span class="text-sm text-gray-500">{label}</span>
            <p class="text-gray-900">{value}</p>
        </div>
    }
}

#[component]
fn ApplicationItem(
    title: &'static str,
    company: &'static str,
    date: &'static str,
    status: &'static str,
) -> impl IntoView {
    let status_class = match status {
        "Under Review" => "bg-yellow-100 text-yellow-800",
        "Interview Scheduled" => "bg-green-100 text-green-800",
        "Application Viewed" => "bg-blue-100 text-blue-800",
        _ => "bg-gray-100 text-gray-800",
    };
    
    view! {
        <div class="border-b pb-4 last:border-0">
            <div class="flex justify-between items-start">
                <div>
                    <h3 class="font-semibold text-gray-900">{title}</h3>
                    <p class="text-gray-600">{company}</p>
                    <p class="text-sm text-gray-500 mt-1">{date}</p>
                </div>
                <span class=format!("px-3 py-1 rounded-full text-sm {}", status_class)>
                    {status}
                </span>
            </div>
        </div>
    }
}