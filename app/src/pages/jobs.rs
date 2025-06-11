use leptos::*;
use wasm_bindgen_futures::spawn_local;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use shared::types::SimpleJobType;
use crate::components::{JobCard, SearchBar, AdvancedFilters};
use crate::components::ui::{LoadingSpinner, SpinnerSize, Alert, AlertVariant};
use crate::api::jobs::fetch_jobs;
use web_sys::window;
use js_sys::Date;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobFilters {
    pub search_query: String,
    pub job_types: Vec<String>,
    pub locations: Vec<String>,
    pub salary_min: Option<u32>,
    pub salary_max: Option<u32>,
    pub employment_types: Vec<String>,
    pub experience_levels: Vec<String>,
    pub show_urgent_only: bool,
    pub show_featured_only: bool,
    pub show_remote_only: bool,
    pub posted_within_days: Option<u32>,
    pub sort_by: String,
    pub sort_order: String,
}

impl Default for JobFilters {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            job_types: Vec::new(),
            locations: Vec::new(),
            salary_min: None,
            salary_max: None,
            employment_types: Vec::new(),
            experience_levels: Vec::new(),
            show_urgent_only: false,
            show_featured_only: false,
            show_remote_only: false,
            posted_within_days: None,
            sort_by: "relevance".to_string(),
            sort_order: "desc".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedJob {
    pub id: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub job_type: String,
    pub employment_type: String,
    pub description: String,
    pub salary_min: Option<u32>,
    pub salary_max: Option<u32>,
    pub hourly_rate: Option<f64>,
    pub is_urgent: bool,
    pub is_featured: bool,
    pub is_remote: bool,
    pub posted_at: String,
    pub expires_at: Option<String>,
    pub applications_count: u32,
    pub required_experience: String,
    pub benefits: Vec<String>,
    pub tags: Vec<String>,
    pub employer_rating: f64,
    pub employer_logo: Option<String>,
    pub contact_email: String,
    pub contact_person: String,
}

// Generate comprehensive sample data
fn generate_sample_jobs() -> Vec<ExtendedJob> {
    vec![
        ExtendedJob {
            id: "1".to_string(),
            title: "Senior Clinical Pharmacist - Oncology".to_string(),
            company: "Royal Prince Alfred Hospital".to_string(),
            location: "Camperdown, NSW".to_string(),
            job_type: "Clinical Pharmacist".to_string(),
            employment_type: "Full-time".to_string(),
            description: "Lead clinical pharmacy services in our state-of-the-art oncology department. Work with cutting-edge treatments and participate in groundbreaking clinical trials.".to_string(),
            salary_min: Some(110000),
            salary_max: Some(130000),
            hourly_rate: None,
            is_urgent: true,
            is_featured: true,
            is_remote: false,
            posted_at: "2 hours ago".to_string(),
            expires_at: Some("30 days".to_string()),
            applications_count: 5,
            required_experience: "5+ years clinical".to_string(),
            benefits: vec!["Salary packaging".to_string(), "Professional development".to_string(), "Research opportunities".to_string()],
            tags: vec!["Clinical".to_string(), "Hospital".to_string(), "Oncology".to_string()],
            employer_rating: 4.8,
            employer_logo: None,
            contact_email: "careers@rpa.health.nsw.gov.au".to_string(),
            contact_person: "Dr. Sarah Chen".to_string(),
        },
        ExtendedJob {
            id: "2".to_string(),
            title: "Weekend Locum Pharmacist - Flexible Hours".to_string(),
            company: "Chemist Warehouse Bondi".to_string(),
            location: "Bondi Beach, NSW".to_string(),
            job_type: "Pharmacist".to_string(),
            employment_type: "Casual".to_string(),
            description: "Perfect work-life balance opportunity at our busy beachside location. Choose your own hours and enjoy the coastal lifestyle.".to_string(),
            salary_min: None,
            salary_max: None,
            hourly_rate: Some(65.0),
            is_urgent: false,
            is_featured: false,
            is_remote: false,
            posted_at: "1 day ago".to_string(),
            expires_at: Some("14 days".to_string()),
            applications_count: 12,
            required_experience: "2+ years retail".to_string(),
            benefits: vec!["Flexible hours".to_string(), "Beach location".to_string(), "Staff discounts".to_string()],
            tags: vec!["Locum".to_string(), "Retail".to_string(), "Flexible".to_string()],
            employer_rating: 4.2,
            employer_logo: None,
            contact_email: "bondi@chemistwarehouse.com.au".to_string(),
            contact_person: "Mark Johnson".to_string(),
        },
        ExtendedJob {
            id: "3".to_string(),
            title: "Pharmacy Manager - Leadership Opportunity".to_string(),
            company: "Terry White Chemmart".to_string(),
            location: "Miranda, NSW".to_string(),
            job_type: "Pharmacy Manager".to_string(),
            employment_type: "Full-time".to_string(),
            description: "Lead our award-winning team in this management role. Drive business growth while maintaining exceptional patient care standards.".to_string(),
            salary_min: Some(95000),
            salary_max: Some(115000),
            hourly_rate: None,
            is_urgent: false,
            is_featured: true,
            is_remote: false,
            posted_at: "2 days ago".to_string(),
            expires_at: Some("21 days".to_string()),
            applications_count: 8,
            required_experience: "3+ years management".to_string(),
            benefits: vec!["Management training".to_string(), "Performance bonuses".to_string(), "Health services".to_string()],
            tags: vec!["Management".to_string(), "Leadership".to_string(), "Retail".to_string()],
            employer_rating: 4.5,
            employer_logo: None,
            contact_email: "careers@terrywhite.com.au".to_string(),
            contact_person: "Lisa Wong".to_string(),
        },
        ExtendedJob {
            id: "4".to_string(),
            title: "Remote Clinical Consultant - Pharmaceutical Industry".to_string(),
            company: "Pfizer Australia".to_string(),
            location: "Sydney, NSW (Remote)".to_string(),
            job_type: "Clinical Pharmacist".to_string(),
            employment_type: "Full-time".to_string(),
            description: "Join our regulatory affairs team working on drug registrations and clinical protocols. Fully remote position with global impact.".to_string(),
            salary_min: Some(120000),
            salary_max: Some(145000),
            hourly_rate: None,
            is_urgent: false,
            is_featured: true,
            is_remote: true,
            posted_at: "3 days ago".to_string(),
            expires_at: Some("45 days".to_string()),
            applications_count: 15,
            required_experience: "3+ years clinical + industry".to_string(),
            benefits: vec!["Remote work".to_string(), "Global company".to_string(), "Professional development".to_string(), "Health insurance".to_string()],
            tags: vec!["Remote".to_string(), "Industry".to_string(), "Clinical".to_string(), "Regulatory".to_string()],
            employer_rating: 4.9,
            employer_logo: None,
            contact_email: "careers.au@pfizer.com".to_string(),
            contact_person: "Dr. Michael Brown".to_string(),
        },
        ExtendedJob {
            id: "5".to_string(),
            title: "Intern Pharmacist - Graduate Program".to_string(),
            company: "Westmead Hospital".to_string(),
            location: "Westmead, NSW".to_string(),
            job_type: "Intern Pharmacist".to_string(),
            employment_type: "Full-time".to_string(),
            description: "Excellent graduate opportunity in one of Australia's leading teaching hospitals. Comprehensive training program with experienced mentors.".to_string(),
            salary_min: Some(70000),
            salary_max: Some(80000),
            hourly_rate: None,
            is_urgent: true,
            is_featured: false,
            is_remote: false,
            posted_at: "4 hours ago".to_string(),
            expires_at: Some("10 days".to_string()),
            applications_count: 25,
            required_experience: "New graduate".to_string(),
            benefits: vec!["Mentorship program".to_string(), "Training".to_string(), "Career development".to_string()],
            tags: vec!["Graduate".to_string(), "Training".to_string(), "Hospital".to_string()],
            employer_rating: 4.6,
            employer_logo: None,
            contact_email: "pharmacy.intern@health.nsw.gov.au".to_string(),
            contact_person: "Emma Davis".to_string(),
        },
        ExtendedJob {
            id: "6".to_string(),
            title: "Compounding Specialist - High-End Practice".to_string(),
            company: "Specialist Compounding Pharmacy".to_string(),
            location: "Double Bay, NSW".to_string(),
            job_type: "Pharmacist".to_string(),
            employment_type: "Full-time".to_string(),
            description: "Rare opportunity for experienced compounding pharmacist. Work with veterinary, paediatric, and dermatological formulations in premium location.".to_string(),
            salary_min: Some(95000),
            salary_max: Some(115000),
            hourly_rate: None,
            is_urgent: false,
            is_featured: true,
            is_remote: false,
            posted_at: "1 week ago".to_string(),
            expires_at: Some("28 days".to_string()),
            applications_count: 7,
            required_experience: "3+ years compounding".to_string(),
            benefits: vec!["Specialized training".to_string(), "Premium location".to_string(), "Professional development".to_string()],
            tags: vec!["Compounding".to_string(), "Specialist".to_string(), "Premium".to_string()],
            employer_rating: 4.7,
            employer_logo: None,
            contact_email: "careers@compoundingpharmacy.com.au".to_string(),
            contact_person: "Dr. James Mitchell".to_string(),
        },
    ]
}

#[component]
pub fn Jobs() -> impl IntoView {
    // Enhanced state management
    let (filters, set_filters) = create_signal(JobFilters::default());
    let (view_mode, set_view_mode) = create_signal("grid".to_string()); // grid, list, map
    let (show_filters, set_show_filters) = create_signal(false);
    let (saved_searches, set_saved_searches) = create_signal(Vec::<JobFilters>::new());
    let (jobs_data, set_jobs_data) = create_signal(generate_sample_jobs());
    let (loading, set_loading) = create_signal(false);
    let (total_jobs, set_total_jobs) = create_signal(0);
    let (current_page, set_current_page) = create_signal(1);
    let (jobs_per_page, set_jobs_per_page) = create_signal(12);

    // Real-time search functionality
    Effect::new({
        let filters = filters.clone();
        let set_jobs_data = set_jobs_data.clone();
        let set_loading = set_loading.clone();
        let set_total_jobs = set_total_jobs.clone();
        
        move |_| {
            let current_filters = filters.get();
            set_loading.set(true);
            
            // Simulate API call delay
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(300).await;
                
                let mut all_jobs = generate_sample_jobs();
                
                // Apply filters
                let filtered_jobs: Vec<ExtendedJob> = all_jobs.into_iter()
                    .filter(|job| {
                        // Search query filter
                        if !current_filters.search_query.is_empty() {
                            let query = current_filters.search_query.to_lowercase();
                            if !job.title.to_lowercase().contains(&query) &&
                               !job.company.to_lowercase().contains(&query) &&
                               !job.location.to_lowercase().contains(&query) &&
                               !job.description.to_lowercase().contains(&query) {
                                return false;
                            }
                        }
                        
                        // Job type filter
                        if !current_filters.job_types.is_empty() &&
                           !current_filters.job_types.contains(&job.job_type) {
                            return false;
                        }
                        
                        // Employment type filter
                        if !current_filters.employment_types.is_empty() &&
                           !current_filters.employment_types.contains(&job.employment_type) {
                            return false;
                        }
                        
                        // Salary filter
                        if let Some(min_salary) = current_filters.salary_min {
                            if let Some(job_min) = job.salary_min {
                                if job_min < min_salary {
                                    return false;
                                }
                            } else if let Some(hourly) = job.hourly_rate {
                                if ((hourly * 40.0 * 52.0) as u32) < min_salary {
                                    return false;
                                }
                            }
                        }
                        
                        if let Some(max_salary) = current_filters.salary_max {
                            if let Some(job_max) = job.salary_max {
                                if job_max > max_salary {
                                    return false;
                                }
                            }
                        }
                        
                        // Urgent filter
                        if current_filters.show_urgent_only && !job.is_urgent {
                            return false;
                        }
                        
                        // Featured filter
                        if current_filters.show_featured_only && !job.is_featured {
                            return false;
                        }
                        
                        // Remote filter
                        if current_filters.show_remote_only && !job.is_remote {
                            return false;
                        }
                        
                        true
                    })
                    .collect();
                
                set_total_jobs.set(filtered_jobs.len());
                set_jobs_data.set(filtered_jobs);
                set_loading.set(false);
            });
        }
    });

    // Paginated jobs
    let paginated_jobs = move || {
        let jobs = jobs_data.get();
        let page = current_page.get();
        let per_page = jobs_per_page.get();
        let start = ((page - 1) * per_page) as usize;
        let end = (start + per_page as usize).min(jobs.len());
        
        if start < jobs.len() {
            jobs[start..end].to_vec()
        } else {
            Vec::new()
        }
    };

    // Total pages
    let total_pages = move || {
        let total = total_jobs.get();
        let per_page = jobs_per_page.get();
        ((total as f64) / (per_page as f64)).ceil() as usize
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-tiffany-light/20 via-white to-blue-50 relative overflow-hidden">
            // Animated Background Elements
            <div class="absolute inset-0 overflow-hidden pointer-events-none">
                <div class="absolute -top-40 -right-40 w-80 h-80 bg-gradient-to-br from-tiffany-blue/20 to-purple-400/20 rounded-full blur-3xl animate-pulse"></div>
                <div class="absolute -bottom-40 -left-40 w-80 h-80 bg-gradient-to-tr from-blue-400/20 to-tiffany-blue/20 rounded-full blur-3xl animate-pulse"></div>
                <div class="absolute top-1/2 left-1/2 w-60 h-60 bg-gradient-to-br from-tiffany-blue/10 to-transparent rounded-full blur-2xl animate-bounce"></div>
            </div>

            <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                // Ultra-Enhanced Header
                <div class="glass bg-white/90 rounded-2xl p-8 mb-8 shadow-2xl">
                    <div class="flex items-center justify-between mb-6">
                        <div>
                            <h1 class="text-4xl font-bold text-gradient mb-3">
                                "🔍 Pharmacy Career Hub"
                            </h1>
                            <p class="text-xl text-gray-700">
                                "Discover your next career opportunity in pharmacy"
                            </p>
                            <div class="flex items-center gap-6 mt-4 text-sm text-gray-600">
                                <div class="flex items-center gap-2">
                                    <span class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></span>
                                    <span>{move || total_jobs.get()} " active positions"</span>
                                </div>
                                <div class="flex items-center gap-2">
                                    <span class="w-2 h-2 bg-yellow-500 rounded-full animate-pulse"></span>
                                    <span>{move || jobs_data.get().iter().filter(|j| j.is_urgent).count()} " urgent opportunities"</span>
                                </div>
                                <div class="flex items-center gap-2">
                                    <span class="w-2 h-2 bg-blue-500 rounded-full animate-pulse"></span>
                                    <span>{move || jobs_data.get().iter().filter(|j| j.is_remote).count()} " remote positions"</span>
                                </div>
                            </div>
                        </div>
                        
                        // View Mode Toggle
                        <div class="flex items-center gap-4">
                            <div class="flex bg-gray-100 rounded-xl p-1">
                                <button
                                    class=move || format!(
                                        "px-4 py-2 rounded-lg transition-all duration-300 {}",
                                        if view_mode.get() == "grid" {
                                            "bg-white shadow-md text-tiffany-dark"
                                        } else {
                                            "text-gray-600 hover:text-gray-900"
                                        }
                                    )
                                    on:click=move |_| set_view_mode.set("grid".to_string())
                                >
                                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                                        <path d="M5 3a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2V5a2 2 0 00-2-2H5zM5 11a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2v-2a2 2 0 00-2-2H5zM11 5a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V5zM11 13a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"/>
                                    </svg>
                                </button>
                                <button
                                    class=move || format!(
                                        "px-4 py-2 rounded-lg transition-all duration-300 {}",
                                        if view_mode.get() == "list" {
                                            "bg-white shadow-md text-tiffany-dark"
                                        } else {
                                            "text-gray-600 hover:text-gray-900"
                                        }
                                    )
                                    on:click=move |_| set_view_mode.set("list".to_string())
                                >
                                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"/>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </div>

                    // Enhanced Search Bar
                    <div class="relative">
                        <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                            <svg class="h-6 w-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                            </svg>
                        </div>
                        <input
                            type="text"
                            class="enhanced-search-input w-full pl-12 pr-16 py-4 text-lg"
                            placeholder="Search for pharmacy jobs, companies, or locations..."
                            value=move || filters.get().search_query
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_filters.update(|f| f.search_query = value);
                            }
                        />
                        <div class="absolute inset-y-0 right-0 flex items-center gap-2 pr-4">
                            <button
                                class="filter-toggle-button"
                                on:click=move |_| set_show_filters.update(|v| *v = !*v)
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"/>
                                </svg>
                                "Filters"
                            </button>
                        </div>
                    </div>
                </div>

                // Enhanced Filters Panel
                <Show when=move || show_filters.get()>
                    <div class="glass bg-white/95 rounded-2xl p-6 mb-8 shadow-xl">
                        <EnhancedFiltersPanel filters=filters set_filters=set_filters />
                    </div>
                </Show>

                // Jobs Content Area
                <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
                    // Quick Filters Sidebar
                    <div class="lg:col-span-1">
                        <QuickFiltersSidebar filters=filters set_filters=set_filters jobs_data=jobs_data />
                    </div>

                    // Main Jobs Display
                    <div class="lg:col-span-3">
                        // Results Header
                        <div class="flex justify-between items-center mb-6">
                            <div class="flex items-center gap-4">
                                <h2 class="text-2xl font-bold text-gray-900">
                                    {move || format!("{} positions found", total_jobs.get())}
                                </h2>
                                {move || if loading.get() {
                                    view! {
                                        <div class="flex items-center gap-2 text-gray-500">
                                            <LoadingSpinner size=SpinnerSize::Small />
                                            <span>"Searching..."</span>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }}
                            </div>
                            
                            // Sort Options
                            <select
                                class="enhanced-select-small"
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_filters.update(|f| f.sort_by = value);
                                }
                            >
                                <option value="relevance">"Most Relevant"</option>
                                <option value="date">"Newest First"</option>
                                <option value="salary_high">"Highest Salary"</option>
                                <option value="salary_low">"Lowest Salary"</option>
                                <option value="company">"Company A-Z"</option>
                            </select>
                        </div>

                        // Jobs Display
                        <Suspense
                            fallback=move || view! { 
                                <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                                    {(0..6).map(|_| view! { <JobCardSkeleton /> }).collect_view()}
                                </div>
                            }
                        >
                            {move || {
                                let jobs = paginated_jobs();
                                
                                if jobs.is_empty() && !loading.get() {
                                    view! {
                                        <EmptyJobsState filters=filters set_filters=set_filters />
                                    }.into_view()
                                } else {
                                    match view_mode.get().as_str() {
                                        "list" => view! {
                                            <div class="space-y-4">
                                                {jobs.into_iter().map(|job| view! { 
                                                    <EnhancedJobCard job=job view_mode="list".to_string() />
                                                }).collect_view()}
                                            </div>
                                        }.into_view(),
                                        _ => view! {
                                            <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                                                {jobs.into_iter().map(|job| view! { 
                                                    <EnhancedJobCard job=job view_mode="grid".to_string() />
                                                }).collect_view()}
                                            </div>
                                        }.into_view(),
                                    }
                                }
                            }}
                        </Suspense>

                        // Enhanced Pagination
                        <Show when=move || total_pages() > 1>
                            <EnhancedPagination 
                                current_page=current_page 
                                set_current_page=set_current_page 
                                total_pages=total_pages 
                                total_jobs=total_jobs
                            />
                        </Show>
                    </div>
                </div>
            </div>

            // Enhanced CSS Framework
            <style>
                {ENHANCED_JOBS_CSS}
            </style>
        </div>
    }
}

// Enhanced component implementations
#[component]
fn EnhancedFiltersPanel(
    filters: ReadSignal<JobFilters>,
    set_filters: WriteSignal<JobFilters>,
) -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h3 class="text-lg font-semibold text-gray-900 mb-4">Advanced Filters</h3>
            
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                // Salary Range
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">Salary Range</label>
                    <div class="flex gap-2">
                        <input
                            type="number"
                            placeholder="Min"
                            class="enhanced-input-small"
                            value=move || filters.get().salary_min.map(|x| x.to_string()).unwrap_or_default()
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                let salary = value.parse::<u32>().ok();
                                set_filters.update(|f| f.salary_min = salary);
                            }
                        />
                        <input
                            type="number"
                            placeholder="Max"
                            class="enhanced-input-small"
                            value=move || filters.get().salary_max.map(|x| x.to_string()).unwrap_or_default()
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                let salary = value.parse::<u32>().ok();
                                set_filters.update(|f| f.salary_max = salary);
                            }
                        />
                    </div>
                </div>

                // Job Type
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">Job Type</label>
                    <select class="enhanced-select-small w-full">
                        <option>All Types</option>
                        <option>Pharmacist</option>
                        <option>Clinical Pharmacist</option>
                        <option>Pharmacy Manager</option>
                        <option>Intern Pharmacist</option>
                    </select>
                </div>

                // Employment Type
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">Employment</label>
                    <select class="enhanced-select-small w-full">
                        <option>All Employment Types</option>
                        <option>Full-time</option>
                        <option>Part-time</option>
                        <option>Casual</option>
                        <option>Contract</option>
                    </select>
                </div>
            </div>

            // Quick toggles
            <div class="flex flex-wrap gap-4">
                <label class="flex items-center gap-2 cursor-pointer">
                    <input 
                        type="checkbox" 
                        class="enhanced-checkbox"
                        checked=move || filters.get().show_urgent_only
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_filters.update(|f| f.show_urgent_only = checked);
                        }
                    />
                    <span>Urgent only</span>
                </label>
                <label class="flex items-center gap-2 cursor-pointer">
                    <input 
                        type="checkbox" 
                        class="enhanced-checkbox"
                        checked=move || filters.get().show_featured_only
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_filters.update(|f| f.show_featured_only = checked);
                        }
                    />
                    <span>Featured only</span>
                </label>
                <label class="flex items-center gap-2 cursor-pointer">
                    <input 
                        type="checkbox" 
                        class="enhanced-checkbox"
                        checked=move || filters.get().show_remote_only
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_filters.update(|f| f.show_remote_only = checked);
                        }
                    />
                    <span>Remote only</span>
                </label>
            </div>

            // Clear filters
            <button
                class="btn-secondary"
                on:click=move |_| set_filters.set(JobFilters::default())
            >
                Clear All Filters
            </button>
        </div>
    }
}

#[component]
fn QuickFiltersSidebar(
    filters: ReadSignal<JobFilters>,
    set_filters: WriteSignal<JobFilters>,
    jobs_data: ReadSignal<Vec<ExtendedJob>>,
) -> impl IntoView {
    view! {
        <div class="glass bg-white/90 rounded-2xl p-6 sticky top-8">
            <h3 class="text-lg font-semibold text-gray-900 mb-4">Quick Filters</h3>
            
            <div class="space-y-4">
                <div>
                    <h4 class="font-medium text-gray-700 mb-2">Job Type</h4>
                    <div class="space-y-2">
                        {["Pharmacist", "Clinical Pharmacist", "Pharmacy Manager", "Intern Pharmacist"].into_iter().map(|job_type| {
                            let count = move || jobs_data.get().iter().filter(|j| j.job_type == job_type).count();
                            view! {
                                <label class="flex items-center justify-between cursor-pointer hover:bg-gray-50 p-2 rounded">
                                    <span class="text-sm">{job_type}</span>
                                    <span class="text-xs text-gray-500">{count}</span>
                                </label>
                            }
                        }).collect_view()}
                    </div>
                </div>

                <div>
                    <h4 class="font-medium text-gray-700 mb-2">Location</h4>
                    <div class="space-y-2">
                        {["NSW", "VIC", "QLD", "SA"].into_iter().map(|state| {
                            let count = move || jobs_data.get().iter().filter(|j| j.location.contains(state)).count();
                            view! {
                                <label class="flex items-center justify-between cursor-pointer hover:bg-gray-50 p-2 rounded">
                                    <span class="text-sm">{state}</span>
                                    <span class="text-xs text-gray-500">{count}</span>
                                </label>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn EnhancedJobCard(job: ExtendedJob, view_mode: String) -> impl IntoView {
    let card_class = if view_mode == "list" {
        "enhanced-job-card-list"
    } else {
        "enhanced-job-card-grid"
    };

    view! {
        <div class={format!("{} group cursor-pointer", card_class)}>
            // Job header
            <div class="flex justify-between items-start mb-3">
                <div class="flex-1">
                    <h3 class="font-bold text-lg text-gray-900 group-hover:text-tiffany-dark transition-colors">
                        {job.title.clone()}
                    </h3>
                    <div class="flex items-center gap-2 mt-1">
                        <span class="text-gray-600">{job.company.clone()}</span>
                        <div class="flex items-center gap-1">
                            {(0..5).map(|i| {
                                let filled = i < (job.employer_rating as usize);
                                view! {
                                    <svg class=format!("w-3 h-3 {}", if filled { "text-yellow-400" } else { "text-gray-300" }) fill="currentColor" viewBox="0 0 20 20">
                                        <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                                    </svg>
                                }
                            }).collect_view()}
                            <span class="text-xs text-gray-500 ml-1">{format!("{:.1}", job.employer_rating)}</span>
                        </div>
                    </div>
                </div>
                <div class="flex gap-2">
                    {if job.is_urgent {
                        view! { <span class="badge-urgent">Urgent</span> }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }}
                    {if job.is_featured {
                        view! { <span class="badge-featured">Featured</span> }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }}
                    {if job.is_remote {
                        view! { <span class="badge-remote">Remote</span> }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }}
                </div>
            </div>

            // Location and salary
            <div class="flex items-center justify-between mb-3">
                <div class="flex items-center gap-1 text-gray-600">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/>
                    </svg>
                    <span class="text-sm">{job.location.clone()}</span>
                </div>
                <div class="text-right">
                    {if let (Some(min), Some(max)) = (job.salary_min, job.salary_max) {
                        view! {
                            <div class="font-semibold text-green-600">
                                {format!("${}-${}k", min / 1000, max / 1000)}
                            </div>
                        }.into_view()
                    } else if let Some(hourly) = job.hourly_rate {
                        view! {
                            <div class="font-semibold text-green-600">
                                {format!("${:.0}/hour", hourly)}
                            </div>
                        }.into_view()
                    } else {
                        view! { <div class="text-gray-500">Salary TBD</div> }.into_view()
                    }}
                    <div class="text-xs text-gray-500">{job.employment_type.clone()}</div>
                </div>
            </div>

            // Description
            <p class="text-gray-700 text-sm mb-4 line-clamp-2">{job.description.clone()}</p>

            // Tags
            <div class="flex flex-wrap gap-2 mb-4">
                {job.tags.into_iter().take(3).map(|tag| view! {
                    <span class="job-tag">{tag}</span>
                }).collect_view()}
            </div>

            // Footer
            <div class="flex items-center justify-between pt-4 border-t border-gray-200">
                <div class="flex items-center gap-4 text-xs text-gray-500">
                    <span>{job.posted_at.clone()}</span>
                    <span>&bull;</span>
                    <span>{job.applications_count} applicants</span>
                </div>
                <button class="btn-primary-small">
                    Apply Now
                </button>
            </div>
        </div>
    }
}

#[component]
fn JobCardSkeleton() -> impl IntoView {
    view! {
        <div class="enhanced-job-card-grid animate-pulse">
            <div class="flex justify-between items-start mb-3">
                <div class="flex-1">
                    <div class="h-5 bg-gray-300 rounded w-3/4 mb-2"></div>
                    <div class="h-4 bg-gray-300 rounded w-1/2"></div>
                </div>
                <div class="h-6 bg-gray-300 rounded w-16"></div>
            </div>
            <div class="h-4 bg-gray-300 rounded w-2/3 mb-3"></div>
            <div class="h-12 bg-gray-300 rounded mb-4"></div>
            <div class="flex gap-2 mb-4">
                <div class="h-6 bg-gray-300 rounded w-16"></div>
                <div class="h-6 bg-gray-300 rounded w-20"></div>
            </div>
            <div class="flex justify-between items-center pt-4 border-t border-gray-200">
                <div class="h-4 bg-gray-300 rounded w-1/3"></div>
                <div class="h-8 bg-gray-300 rounded w-20"></div>
            </div>
        </div>
    }
}

#[component]
fn EmptyJobsState(
    filters: ReadSignal<JobFilters>,
    set_filters: WriteSignal<JobFilters>,
) -> impl IntoView {
    view! {
        <div class="text-center py-16">
            <div class="text-6xl mb-4">"🔍"</div>
            <h3 class="text-xl font-semibold text-gray-900 mb-2">No jobs found</h3>
            <p class="text-gray-600 mb-6">Try adjusting your search criteria or browse all positions</p>
            <div class="flex justify-center gap-4">
                <button 
                    class="btn-primary"
                    on:click=move |_| set_filters.set(JobFilters::default())
                >
                    Clear All Filters
                </button>
                <button class="btn-secondary">Browse All Jobs</button>
            </div>
        </div>
    }
}

#[component]
fn EnhancedPagination(
    current_page: ReadSignal<usize>,
    set_current_page: WriteSignal<usize>,
    total_pages: Signal<usize>,
    total_jobs: ReadSignal<usize>,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between mt-8 px-4 py-3 bg-white/80 rounded-xl">
            <div class="text-sm text-gray-700">
                Showing {(current_page.get() - 1) * 12 + 1} to {(current_page.get() * 12).min(total_jobs.get())} of {total_jobs.get()} results
            </div>
            <div class="flex items-center gap-2">
                <button
                    class="pagination-btn"
                    disabled=move || current_page.get() <= 1
                    on:click=move |_| {
                        if current_page.get() > 1 {
                            set_current_page.set(current_page.get() - 1);
                        }
                    }
                >
                    Previous
                </button>
                {move || {
                    let current = current_page.get();
                    let total = total_pages.get();
                    let start = if current <= 3 { 1 } else { current - 2 };
                    let end = (start + 4).min(total);
                    
                    (start..=end).map(|page| {
                        let is_current = page == current;
                        view! {
                            <button
                                class=format!("pagination-number {}", if is_current { "active" } else { "" })
                                on:click=move |_| set_current_page.set(page)
                            >
                                {page}
                            </button>
                        }
                    }).collect_view()
                }}
                <button
                    class="pagination-btn"
                    disabled=move || current_page.get() >= total_pages.get()
                    on:click=move |_| {
                        if current_page.get() < total_pages.get() {
                            set_current_page.set(current_page.get() + 1);
                        }
                    }
                >
                    Next
                </button>
            </div>
        </div>
    }
}

const ENHANCED_JOBS_CSS: &str = r#"
/* Ultra-Enhanced Jobs Page CSS Framework */

/* Glass Morphism Base */
.glass {
    background: rgba(255, 255, 255, 0.8);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid rgba(255, 255, 255, 0.3);
}

/* Text Gradients */
.text-gradient {
    background: linear-gradient(135deg, #17ddb8, #3b82f6);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
}

/* Enhanced Search Input */
.enhanced-search-input {
    background: rgba(255, 255, 255, 0.95);
    border: 2px solid transparent;
    border-radius: 16px;
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
    font-weight: 500;
}

.enhanced-search-input:focus {
    border-color: #17ddb8;
    box-shadow: 0 0 0 4px rgba(23, 221, 184, 0.1);
    outline: none;
    background: rgba(255, 255, 255, 1);
}

/* Filter Toggle Button */
.filter-toggle-button {
    background: linear-gradient(135deg, #17ddb8, #3b82f6);
    color: white;
    padding: 8px 16px;
    border-radius: 12px;
    border: none;
    cursor: pointer;
    font-weight: 600;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    gap: 8px;
}

.filter-toggle-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(23, 221, 184, 0.3);
}

/* Enhanced Form Controls */
.enhanced-input-small {
    background: rgba(255, 255, 255, 0.9);
    border: 2px solid transparent;
    border-radius: 8px;
    padding: 8px 12px;
    transition: all 0.3s ease;
    font-size: 14px;
}

.enhanced-input-small:focus {
    border-color: #17ddb8;
    box-shadow: 0 0 0 3px rgba(23, 221, 184, 0.1);
    outline: none;
}

.enhanced-select-small {
    background: rgba(255, 255, 255, 0.9);
    border: 2px solid transparent;
    border-radius: 8px;
    padding: 8px 12px;
    transition: all 0.3s ease;
    font-size: 14px;
    cursor: pointer;
}

.enhanced-select-small:focus {
    border-color: #17ddb8;
    box-shadow: 0 0 0 3px rgba(23, 221, 184, 0.1);
    outline: none;
}

/* Enhanced Checkbox */
.enhanced-checkbox {
    width: 18px;
    height: 18px;
    border: 2px solid #d1d5db;
    border-radius: 4px;
    transition: all 0.3s ease;
    cursor: pointer;
    position: relative;
}

.enhanced-checkbox:checked {
    background: linear-gradient(135deg, #17ddb8, #3b82f6);
    border-color: #17ddb8;
}

.enhanced-checkbox:checked::after {
    content: '✓';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: white;
    font-size: 12px;
    font-weight: bold;
}

/* Enhanced Job Cards */
.enhanced-job-card-grid {
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(255, 255, 255, 0.3);
    border-radius: 16px;
    padding: 24px;
    transition: all 0.3s ease;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
}

.enhanced-job-card-grid:hover {
    transform: translateY(-8px);
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
    border-color: rgba(23, 221, 184, 0.3);
}

.enhanced-job-card-list {
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(255, 255, 255, 0.3);
    border-radius: 12px;
    padding: 20px;
    transition: all 0.3s ease;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
}

.enhanced-job-card-list:hover {
    transform: translateX(8px);
    box-shadow: 0 8px 25px rgba(0, 0, 0, 0.1);
    border-color: rgba(23, 221, 184, 0.3);
}

/* Job Tags */
.job-tag {
    background: linear-gradient(135deg, rgba(23, 221, 184, 0.1), rgba(59, 130, 246, 0.1));
    color: #0d9488;
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    border: 1px solid rgba(23, 221, 184, 0.2);
}

/* Badges */
.badge-urgent {
    background: linear-gradient(135deg, #ef4444, #dc2626);
    color: white;
    padding: 4px 8px;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    animation: pulse 2s infinite;
}

.badge-featured {
    background: linear-gradient(135deg, #8b5cf6, #7c3aed);
    color: white;
    padding: 4px 8px;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

.badge-remote {
    background: linear-gradient(135deg, #10b981, #059669);
    color: white;
    padding: 4px 8px;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

/* Buttons */
.btn-primary {
    background: linear-gradient(135deg, #17ddb8, #3b82f6);
    color: white;
    padding: 12px 24px;
    border-radius: 12px;
    border: none;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    box-shadow: 0 4px 15px rgba(23, 221, 184, 0.3);
}

.btn-primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(23, 221, 184, 0.4);
}

.btn-primary-small {
    background: linear-gradient(135deg, #17ddb8, #3b82f6);
    color: white;
    padding: 8px 16px;
    border-radius: 8px;
    border: none;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
}

.btn-primary-small:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 15px rgba(23, 221, 184, 0.3);
}

.btn-secondary {
    background: rgba(255, 255, 255, 0.8);
    color: #374151;
    padding: 12px 24px;
    border-radius: 12px;
    border: 1px solid rgba(0, 0, 0, 0.1);
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    backdrop-filter: blur(10px);
}

.btn-secondary:hover {
    background: rgba(255, 255, 255, 0.9);
    transform: translateY(-1px);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
}

/* Pagination */
.pagination-btn {
    background: rgba(255, 255, 255, 0.8);
    color: #374151;
    padding: 8px 16px;
    border-radius: 8px;
    border: 1px solid rgba(0, 0, 0, 0.1);
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
}

.pagination-btn:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.9);
    transform: translateY(-1px);
}

.pagination-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.pagination-number {
    background: rgba(255, 255, 255, 0.8);
    color: #374151;
    padding: 8px 12px;
    border-radius: 8px;
    border: 1px solid rgba(0, 0, 0, 0.1);
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
    min-width: 40px;
}

.pagination-number:hover {
    background: rgba(255, 255, 255, 0.9);
    transform: translateY(-1px);
}

.pagination-number.active {
    background: linear-gradient(135deg, #17ddb8, #3b82f6);
    color: white;
    border-color: transparent;
}

/* Line clamp utility */
.line-clamp-2 {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
}

/* Color Variables */
:root {
    --tiffany-blue: #17ddb8;
    --tiffany-dark: #0d9488;
    --tiffany-light: #a7f3d0;
}

/* Animations */
@keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.7; }
}

/* Responsive Design */
@media (max-width: 768px) {
    .enhanced-job-card-grid,
    .enhanced-job-card-list {
        padding: 16px;
    }
    
    .enhanced-search-input {
        font-size: 16px; /* Prevent zoom on iOS */
    }
}

/* Accessibility */
@media (prefers-reduced-motion: reduce) {
    * {
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.01ms !important;
    }
}
"#;