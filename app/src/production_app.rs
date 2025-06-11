use leptos::*;
use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

// Data structures
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub description: String,
    pub salary_min: Option<f64>,
    pub salary_max: Option<f64>,
    pub job_type: String,
    pub posted_date: String,
    pub remote_option: Option<bool>,
}

// Main App Component
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=path!("/") view=HomePage/>
                <Route path=path!("/login") view=LoginPage/>
                <Route path=path!("/register") view=RegisterPage/>
                <Route path=path!("/dashboard") view=DashboardPage/>
                <Route path=path!("/jobs") view=JobsPage/>
                <Route path=path!("/map") view=MapPage/>
                <Route path=path!("/profile") view=ProfilePage/>
            </Routes>
        </Router>
    }
}

// Home Page
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 via-white to-purple-50">
            <Header/>
            
            // Hero Section
            <section class="relative px-6 py-24 sm:py-32 lg:px-8">
                <div class="mx-auto max-w-2xl text-center">
                    <h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-6xl">
                        "Find Your Dream Pharmacy Job"
                    </h1>
                    <p class="mt-6 text-lg leading-8 text-gray-600">
                        "Connect with top pharmacy employers across Australia. Your next career opportunity is just a click away."
                    </p>
                    <div class="mt-10 flex items-center justify-center gap-x-6">
                        <a
                            href="/jobs"
                            class="rounded-xl bg-gradient-to-r from-blue-600 to-blue-700 px-6 py-3 text-base font-semibold text-white shadow-lg hover:from-blue-700 hover:to-blue-800 transition-all duration-200 transform hover:scale-105"
                        >
                            "Browse Jobs"
                        </a>
                        <a
                            href="/register"
                            class="rounded-xl bg-white/70 backdrop-blur-xl px-6 py-3 text-base font-semibold text-gray-900 shadow-lg ring-1 ring-gray-900/10 hover:bg-white/90 transition-all duration-200"
                        >
                            "Post a Job"
                        </a>
                    </div>
                </div>
            </section>
            
            // Features Section
            <section class="py-24 sm:py-32">
                <div class="mx-auto max-w-7xl px-6 lg:px-8">
                    <div class="mx-auto max-w-2xl text-center">
                        <h2 class="text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
                            "Everything you need to advance your pharmacy career"
                        </h2>
                    </div>
                    <div class="mx-auto mt-16 max-w-2xl sm:mt-20 lg:mt-24 lg:max-w-none">
                        <div class="grid max-w-xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-3">
                            <FeatureCard
                                icon="🔍"
                                title="Smart Job Matching"
                                description="Our AI-powered system matches you with jobs that fit your skills and preferences."
                            />
                            <FeatureCard
                                icon="📍"
                                title="Location-Based Search"
                                description="Find opportunities near you with our interactive map and location filters."
                            />
                            <FeatureCard
                                icon="📊"
                                title="Career Insights"
                                description="Get real-time data on salaries, demand, and career growth in your area."
                            />
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}

// Header Component
#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="absolute inset-x-0 top-0 z-50">
            <nav class="flex items-center justify-between p-6 lg:px-8">
                <div class="flex lg:flex-1">
                    <a href="/" class="-m-1.5 p-1.5 flex items-center gap-2">
                        <div class="w-10 h-10 bg-gradient-to-br from-blue-600 to-purple-600 rounded-xl flex items-center justify-center text-white font-bold text-lg">
                            "L"
                        </div>
                        <span class="text-xl font-bold text-gray-900">"Loco Platform"</span>
                    </a>
                </div>
                <div class="flex gap-x-6">
                    <a href="/login" class="text-sm font-semibold leading-6 text-gray-900 hover:text-blue-600">
                        "Log in"
                    </a>
                    <a href="/register" class="text-sm font-semibold leading-6 text-gray-900 hover:text-blue-600">
                        "Sign up"
                    </a>
                </div>
            </nav>
        </header>
    }
}

// Feature Card Component
#[component]
fn FeatureCard(icon: &'static str, title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center text-center">
            <div class="text-5xl mb-6">{icon}</div>
            <h3 class="text-xl font-semibold text-gray-900 mb-4">{title}</h3>
            <p class="text-gray-600">{description}</p>
        </div>
    }
}

// Login Page
#[component]
fn LoginPage() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);
    let loading = RwSignal::new(false);
    
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        loading.set(true);
        error.set(None);
        
        let email_val = email.get();
        let password_val = password.get();
        
        spawn_local(async move {
            match login_api(email_val, password_val).await {
                Ok(_) => {
                    // Navigate to dashboard
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().set_href("/dashboard");
                    }
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
            loading.set(false);
        });
    };
    
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 via-white to-purple-50">
            <div class="w-full max-w-md">
                <div class="bg-white/80 backdrop-blur-xl rounded-3xl shadow-2xl p-8">
                    <div class="text-center mb-8">
                        <div class="w-20 h-20 bg-gradient-to-br from-blue-600 to-purple-600 rounded-2xl flex items-center justify-center text-white font-bold text-3xl mx-auto mb-4">
                            "L"
                        </div>
                        <h2 class="text-3xl font-bold text-gray-900">"Welcome Back"</h2>
                        <p class="text-gray-600 mt-2">"Sign in to your account"</p>
                    </div>
                    
                    <form on:submit=on_submit class="space-y-6">
                        {move || error.get().map(|e| view! {
                            <div class="p-4 bg-red-50 border border-red-200 rounded-xl">
                                <p class="text-sm text-red-600">{e}</p>
                            </div>
                        })}
                        
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Email"
                            </label>
                            <input
                                type="email"
                                class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                placeholder="you@example.com"
                                required
                                on:input=move |ev| email.set(event_target_value(&ev))
                            />
                        </div>
                        
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Password"
                            </label>
                            <input
                                type="password"
                                class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                placeholder="••••••••"
                                required
                                on:input=move |ev| password.set(event_target_value(&ev))
                            />
                        </div>
                        
                        <button
                            type="submit"
                            class="w-full py-3 px-4 bg-gradient-to-r from-blue-600 to-blue-700 text-white font-semibold rounded-xl hover:from-blue-700 hover:to-blue-800 transition-all duration-200 transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed"
                            disabled=loading.get()
                        >
                            {move || if loading.get() { "Signing in..." } else { "Sign In" }}
                        </button>
                    </form>
                    
                    <p class="mt-6 text-center text-sm text-gray-600">
                        "Don't have an account? "
                        <a href="/register" class="font-medium text-blue-600 hover:text-blue-700">
                            "Sign up"
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}

// Jobs Page
#[component]
fn JobsPage() -> impl IntoView {
    let jobs = RwSignal::new(Vec::<Job>::new());
    let loading = RwSignal::new(true);
    
    // Fetch jobs on mount
    spawn_local(async move {
        match fetch_jobs().await {
            Ok(job_list) => jobs.set(job_list),
            Err(_) => {
                // Use mock data as fallback
                jobs.set(get_mock_jobs());
            }
        }
        loading.set(false);
    });
    
    view! {
        <div class="min-h-screen bg-gray-50">
            <DashboardHeader/>
            
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                <div class="mb-8">
                    <h1 class="text-3xl font-bold text-gray-900">"Job Listings"</h1>
                    <p class="text-gray-600 mt-2">"Find your perfect pharmacy position"</p>
                </div>
                
                // Search and Filters
                <div class="bg-white rounded-xl shadow-sm p-6 mb-8">
                    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                        <input
                            type="text"
                            placeholder="Search jobs..."
                            class="px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        />
                        <select class="px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent">
                            <option>"All Locations"</option>
                            <option>"Sydney"</option>
                            <option>"Melbourne"</option>
                            <option>"Brisbane"</option>
                        </select>
                        <select class="px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent">
                            <option>"All Types"</option>
                            <option>"Full-time"</option>
                            <option>"Part-time"</option>
                            <option>"Contract"</option>
                        </select>
                        <button class="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200">
                            "More Filters"
                        </button>
                    </div>
                </div>
                
                // Job Listings
                <div class="space-y-4">
                    {move || {
                        if loading.get() {
                            vec![view! {
                                <div class="text-center py-12">
                                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
                                    <p class="text-gray-600 mt-4">"Loading jobs..."</p>
                                </div>
                            }]
                        } else {
                            jobs.get().into_iter().map(|job| view! {
                                <JobCard job=job/>
                            }).collect::<Vec<_>>()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

// Dashboard Header Component
#[component]
fn DashboardHeader() -> impl IntoView {
    view! {
        <header class="bg-white shadow-sm">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    <div class="flex items-center">
                        <a href="/dashboard" class="flex items-center gap-2">
                            <div class="w-10 h-10 bg-gradient-to-br from-blue-600 to-purple-600 rounded-xl flex items-center justify-center text-white font-bold text-lg">
                                "L"
                            </div>
                            <span class="text-xl font-bold text-gray-900">"Loco Platform"</span>
                        </a>
                        <nav class="ml-10 flex space-x-8">
                            <a href="/dashboard" class="text-gray-500 hover:text-gray-900">
                                "Dashboard"
                            </a>
                            <a href="/jobs" class="text-gray-900 font-medium">
                                "Jobs"
                            </a>
                            <a href="/map" class="text-gray-500 hover:text-gray-900">
                                "Map"
                            </a>
                            <a href="/profile" class="text-gray-500 hover:text-gray-900">
                                "Profile"
                            </a>
                        </nav>
                    </div>
                    <div class="flex items-center gap-4">
                        <button class="p-2 text-gray-400 hover:text-gray-500">
                            <span class="sr-only">"Notifications"</span>
                            <span class="text-xl">"🔔"</span>
                        </button>
                        <div class="w-8 h-8 bg-gradient-to-br from-green-400 to-blue-500 rounded-full"></div>
                    </div>
                </div>
            </div>
        </header>
    }
}

// Job Card Component
#[component]
fn JobCard(job: Job) -> impl IntoView {
    let job_id = job.id.clone();
    view! {
        <div class="bg-white rounded-xl shadow-sm hover:shadow-lg transition-shadow p-6">
            <div class="flex justify-between items-start">
                <div class="flex-1">
                    <h3 class="text-xl font-semibold text-gray-900 mb-2">{job.title.clone()}</h3>
                    <p class="text-gray-600 mb-4">{job.company.clone()}</p>
                    
                    <div class="flex flex-wrap gap-2 mb-4">
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800">
                            "📍 " {job.location.clone()}
                        </span>
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-green-100 text-green-800">
                            {job.job_type.clone()}
                        </span>
                    </div>
                    
                    <p class="text-gray-700 line-clamp-2">{job.description.clone()}</p>
                </div>
                
                <div class="ml-6 text-right">
                    <p class="text-2xl font-bold text-gray-900">
                        {format_salary(&job.salary_min, &job.salary_max)}
                    </p>
                    <p class="text-sm text-gray-500 mt-1">{job.posted_date.clone()}</p>
                    <a
                        href=format!("/jobs/{}", job_id)
                        class="inline-block mt-4 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
                    >
                        "View Details"
                    </a>
                </div>
            </div>
        </div>
    }
}

// Other Pages (simplified)
#[component]
fn RegisterPage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 via-white to-purple-50">
            <div class="text-center">
                <h1 class="text-3xl font-bold mb-4">"Register Page"</h1>
                <a href="/" class="text-blue-600 hover:underline">"Back to Home"</a>
            </div>
        </div>
    }
}

#[component]
fn DashboardPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <DashboardHeader/>
            <div class="max-w-7xl mx-auto px-4 py-8">
                <h1 class="text-3xl font-bold mb-8">"Dashboard"</h1>
                <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
                    <StatCard title="Active Jobs" value="24" icon="💼" color="blue"/>
                    <StatCard title="Applications" value="156" icon="📋" color="green"/>
                    <StatCard title="Profile Views" value="892" icon="👁️" color="purple"/>
                    <StatCard title="Messages" value="12" icon="💬" color="yellow"/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn StatCard(title: &'static str, value: &'static str, icon: &'static str, color: &'static str) -> impl IntoView {
    // Use predefined classes to avoid dynamic class generation issues
    let icon_class = match color {
        "blue" => "text-3xl text-blue-800",
        "green" => "text-3xl text-green-800",
        "purple" => "text-3xl text-purple-800",
        "yellow" => "text-3xl text-yellow-800",
        _ => "text-3xl text-gray-800",
    };
    
    view! {
        <div class="bg-white rounded-xl shadow-sm p-6">
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-sm text-gray-600">{title}</p>
                    <p class="text-2xl font-bold text-gray-900 mt-1">{value}</p>
                </div>
                <div class=icon_class>
                    {icon}
                </div>
            </div>
        </div>
    }
}

#[component]
fn MapPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <DashboardHeader/>
            <div class="h-[calc(100vh-4rem)]">
                <div class="h-full flex items-center justify-center bg-gray-100">
                    <div class="text-center">
                        <span class="text-6xl">"🗺️"</span>
                        <p class="text-2xl text-gray-600 mt-4">"Interactive Map Coming Soon"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProfilePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <DashboardHeader/>
            <div class="max-w-7xl mx-auto px-4 py-8">
                <h1 class="text-3xl font-bold mb-8">"Profile"</h1>
                <div class="bg-white rounded-xl shadow-sm p-6">
                    <p class="text-gray-600">"Profile page content coming soon..."</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="text-center">
                <h1 class="text-6xl font-bold text-gray-900">"404"</h1>
                <p class="mt-4 text-xl text-gray-600">"Page not found"</p>
                <a href="/" class="mt-6 inline-block px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                    "Go Home"
                </a>
            </div>
        </div>
    }
}

// API Functions
async fn login_api(email: String, password: String) -> Result<(), String> {
    let response = Request::post("/api/v1/auth/login")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if response.ok() {
        Ok(())
    } else {
        Err("Invalid credentials".to_string())
    }
}

async fn fetch_jobs() -> Result<Vec<Job>, String> {
    let response = Request::get("/api/v1/jobs")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if response.ok() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err("Failed to fetch jobs".to_string())
    }
}

// Helper Functions
fn format_salary(min: &Option<f64>, max: &Option<f64>) -> String {
    match (min, max) {
        (Some(min), Some(max)) => format!("${:.0}k - ${:.0}k", min / 1000.0, max / 1000.0),
        (Some(min), None) => format!("${:.0}k+", min / 1000.0),
        (None, Some(max)) => format!("Up to ${:.0}k", max / 1000.0),
        (None, None) => "Competitive".to_string(),
    }
}

fn get_mock_jobs() -> Vec<Job> {
    vec![
        Job {
            id: "1".to_string(),
            title: "Senior Pharmacist".to_string(),
            company: "Sydney Hospital".to_string(),
            location: "Sydney, NSW".to_string(),
            description: "Leading pharmacy role in a major teaching hospital with opportunities for clinical specialisation.".to_string(),
            salary_min: Some(95000.0),
            salary_max: Some(115000.0),
            job_type: "Full-time".to_string(),
            posted_date: "2 days ago".to_string(),
            remote_option: Some(false),
        },
        Job {
            id: "2".to_string(),
            title: "Clinical Pharmacist".to_string(),
            company: "Melbourne Medical Centre".to_string(),
            location: "Melbourne, VIC".to_string(),
            description: "Join our multidisciplinary team providing patient-centered care in a modern medical facility.".to_string(),
            salary_min: Some(85000.0),
            salary_max: Some(105000.0),
            job_type: "Full-time".to_string(),
            posted_date: "3 days ago".to_string(),
            remote_option: Some(false),
        },
        Job {
            id: "3".to_string(),
            title: "Community Pharmacist".to_string(),
            company: "Brisbane Pharmacy Group".to_string(),
            location: "Brisbane, QLD".to_string(),
            description: "Rewarding community pharmacy position with focus on patient counselling and health services.".to_string(),
            salary_min: Some(80000.0),
            salary_max: Some(95000.0),
            job_type: "Part-time".to_string(),
            posted_date: "1 week ago".to_string(),
            remote_option: Some(false),
        },
    ]
}