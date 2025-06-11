use leptos::*;
use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

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
}

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
                <Route path=path!("/applications") view=ApplicationsPage/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 via-white to-purple-50">
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
                        <a href="/dashboard" class="text-sm font-semibold leading-6 text-white bg-blue-600 px-4 py-2 rounded-lg hover:bg-blue-700">
                            "Get Started"
                        </a>
                    </div>
                </nav>
            </header>
            
            <div class="relative px-6 pt-14 lg:px-8">
                <div class="mx-auto max-w-2xl py-32 sm:py-48 lg:py-56">
                    <div class="text-center">
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
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn LoginPage() -> impl IntoView {
    // Pre-fill with super admin credentials for development
    let email = RwSignal::new("lw@hamiltonbailey.com".to_string());
    let password = RwSignal::new("password123".to_string());
    let loading = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let navigate = leptos_router::hooks::use_navigate();
    
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        loading.set(true);
        error.set(None);
        
        let email_val = email.get();
        let password_val = password.get();
        let navigate = navigate.clone();
        
        spawn_local(async move {
            // Check for super admin credentials
            if email_val == "lw@hamiltonbailey.com" && password_val == "password123" {
                loading.set(false);
                navigate("/dashboard", Default::default());
            } else {
                loading.set(false);
                error.set(Some("Invalid email or password".to_string()));
            }
        });
    };
    
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-tiffany-light/20 via-white to-lavender/30">
            <div class="w-full max-w-md">
                <div class="bg-white/80 backdrop-blur-xl rounded-3xl shadow-2xl p-8 border border-white/20">
                    <div class="text-center mb-8">
                        <div class="w-20 h-20 bg-gradient-to-br from-tiffany to-tiffany-dark rounded-2xl flex items-center justify-center mx-auto mb-4">
                            <span class="text-3xl text-white font-bold">"L"</span>
                        </div>
                        <h2 class="text-3xl font-bold text-gray-900">"Welcome Back"</h2>
                        <p class="text-gray-600 mt-2">"Sign in to your Loco Platform account"</p>
                    </div>
                    
                    {move || error.get().map(|e| view! {
                        <div class="mb-4 p-3 bg-red-50 border border-red-200 rounded-xl text-red-700 text-sm">
                            {e}
                        </div>
                    })}
                    
                    <form on:submit=on_submit class="space-y-6">
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Email"
                            </label>
                            <input
                                type="email"
                                required
                                class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-tiffany focus:border-transparent transition-all duration-200"
                                placeholder="you@example.com"
                                value=move || email.get()
                                on:input=move |ev| email.set(event_target_value(&ev))
                            />
                        </div>
                        
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Password"
                            </label>
                            <input
                                type="password"
                                required
                                class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-tiffany focus:border-transparent transition-all duration-200"
                                placeholder="••••••••"
                                value=move || password.get()
                                on:input=move |ev| password.set(event_target_value(&ev))
                            />
                        </div>
                        
                        <div class="flex items-center justify-between text-sm">
                            <label class="flex items-center">
                                <input type="checkbox" class="mr-2 rounded text-tiffany focus:ring-tiffany"/>
                                <span class="text-gray-700">"Remember me"</span>
                            </label>
                            <a href="/forgot-password" class="text-tiffany hover:text-tiffany-dark">
                                "Forgot password?"
                            </a>
                        </div>
                        
                        <button
                            type="submit"
                            disabled=move || loading.get()
                            class="w-full py-3 px-4 bg-gradient-to-r from-tiffany to-tiffany-dark text-white font-semibold rounded-xl hover:shadow-lg transform hover:scale-[1.02] transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {move || if loading.get() {
                                "Signing in..."
                            } else {
                                "Sign In"
                            }}
                        </button>
                    </form>
                    
                    <div class="mt-6">
                        <div class="relative">
                            <div class="absolute inset-0 flex items-center">
                                <div class="w-full border-t border-gray-300"></div>
                            </div>
                            <div class="relative flex justify-center text-sm">
                                <span class="bg-white px-4 text-gray-500">"Or continue with"</span>
                            </div>
                        </div>
                        
                        <div class="mt-6 grid grid-cols-2 gap-3">
                            <button class="w-full inline-flex justify-center py-2 px-4 border border-gray-300 rounded-xl shadow-sm bg-white text-sm font-medium text-gray-500 hover:bg-gray-50">
                                "Google"
                            </button>
                            <button class="w-full inline-flex justify-center py-2 px-4 border border-gray-300 rounded-xl shadow-sm bg-white text-sm font-medium text-gray-500 hover:bg-gray-50">
                                "LinkedIn"
                            </button>
                        </div>
                    </div>
                    
                    <p class="mt-8 text-center text-sm text-gray-600">
                        "Don't have an account? "
                        <a href="/register" class="font-medium text-tiffany hover:text-tiffany-dark">
                            "Sign up"
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn DashboardPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <AppHeader/>
            <div class="max-w-7xl mx-auto px-4 py-8">
                <h1 class="text-3xl font-bold mb-8">"Dashboard"</h1>
                <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
                    <StatCard title="Active Jobs" value="24" icon="💼"/>
                    <StatCard title="Applications" value="156" icon="📋"/>
                    <StatCard title="Profile Views" value="892" icon="👁️"/>
                    <StatCard title="Messages" value="12" icon="💬"/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn JobsPage() -> impl IntoView {
    let all_jobs = get_mock_jobs();
    let jobs = RwSignal::new(all_jobs.clone());
    let search_term = RwSignal::new(String::new());
    let selected_type = RwSignal::new("All".to_string());
    let selected_location = RwSignal::new("All".to_string());
    
    // Filter jobs based on search and filters
    let filtered_jobs = move || {
        let search = search_term.get().to_lowercase();
        let job_type = selected_type.get();
        let location = selected_location.get();
        
        all_jobs.iter()
            .filter(|job| {
                let matches_search = search.is_empty() || 
                    job.title.to_lowercase().contains(&search) ||
                    job.company.to_lowercase().contains(&search) ||
                    job.description.to_lowercase().contains(&search);
                    
                let matches_type = job_type == "All" || job.job_type == job_type;
                let matches_location = location == "All" || job.location.contains(&location);
                
                matches_search && matches_type && matches_location
            })
            .cloned()
            .collect::<Vec<_>>()
    };
    
    view! {
        <div class="min-h-screen bg-gray-50">
            <AppHeader/>
            
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                <div class="mb-8">
                    <h1 class="text-3xl font-bold text-gray-900">"Job Listings"</h1>
                    <p class="text-gray-600 mt-2">"Find your perfect pharmacy position"</p>
                </div>
                
                // Search and Filter Bar
                <div class="bg-white rounded-xl shadow-sm p-6 mb-6">
                    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                        <div class="md:col-span-2">
                            <input
                                type="text"
                                placeholder="Search jobs, companies, or keywords..."
                                class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-tiffany focus:border-transparent"
                                on:input=move |ev| search_term.set(event_target_value(&ev))
                            />
                        </div>
                        
                        <select
                            class="px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-tiffany focus:border-transparent"
                            on:change=move |ev| selected_type.set(event_target_value(&ev))
                        >
                            <option value="All">"All Types"</option>
                            <option value="Full-time">"Full-time"</option>
                            <option value="Part-time">"Part-time"</option>
                            <option value="Contract">"Contract"</option>
                        </select>
                        
                        <select
                            class="px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-tiffany focus:border-transparent"
                            on:change=move |ev| selected_location.set(event_target_value(&ev))
                        >
                            <option value="All">"All Locations"</option>
                            <option value="Sydney">"Sydney"</option>
                            <option value="Melbourne">"Melbourne"</option>
                            <option value="Brisbane">"Brisbane"</option>
                            <option value="Perth">"Perth"</option>
                            <option value="Adelaide">"Adelaide"</option>
                        </select>
                    </div>
                    
                    <div class="mt-4 flex items-center justify-between">
                        <p class="text-sm text-gray-600">
                            "Found " <span class="font-semibold">{move || filtered_jobs().len()}</span> " jobs"
                        </p>
                        <button class="text-sm text-tiffany hover:text-tiffany-dark font-medium">
                            "Clear filters"
                        </button>
                    </div>
                </div>
                
                <div class="space-y-4">
                    {move || filtered_jobs().into_iter().map(|job| view! {
                        <JobCard job=job/>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

#[component]
fn AppHeader() -> impl IntoView {
    view! {
        <header class="bg-white/95 backdrop-blur-md shadow-sm sticky top-0 z-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    <div class="flex items-center">
                        <a href="/dashboard" class="flex items-center gap-2">
                            <div class="w-10 h-10 bg-gradient-to-br from-tiffany to-tiffany-dark rounded-xl flex items-center justify-center text-white font-bold text-lg">
                                "L"
                            </div>
                            <span class="text-xl font-bold text-gray-900">"Loco Platform"</span>
                        </a>
                        <nav class="ml-10 flex space-x-6">
                            <a href="/dashboard" class="text-gray-600 hover:text-tiffany transition-colors">"Dashboard"</a>
                            <a href="/jobs" class="text-gray-600 hover:text-tiffany transition-colors">"Jobs"</a>
                            <a href="/map" class="text-gray-600 hover:text-tiffany transition-colors">"Map"</a>
                            <a href="/applications" class="text-gray-600 hover:text-tiffany transition-colors">"Applications"</a>
                        </nav>
                    </div>
                    <div class="flex items-center gap-4">
                        <a href="/profile" class="text-gray-600 hover:text-tiffany transition-colors">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                            </svg>
                        </a>
                        <a href="/" class="text-gray-600 hover:text-red-600 transition-colors">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path>
                            </svg>
                        </a>
                    </div>
                </div>
            </div>
        </header>
    }
}

#[component]
fn JobCard(job: Job) -> impl IntoView {
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
                    
                    <p class="text-gray-700">{job.description.clone()}</p>
                </div>
                
                <div class="ml-6 text-right">
                    <p class="text-2xl font-bold text-gray-900">
                        {format_salary(&job.salary_min, &job.salary_max)}
                    </p>
                    <p class="text-sm text-gray-500 mt-1">{job.posted_date.clone()}</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn StatCard(title: &'static str, value: &'static str, icon: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white rounded-xl shadow-sm p-6">
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-sm text-gray-600">{title}</p>
                    <p class="text-2xl font-bold text-gray-900 mt-1">{value}</p>
                </div>
                <div class="text-3xl">
                    {icon}
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

fn format_salary(min: &Option<f64>, max: &Option<f64>) -> String {
    match (min, max) {
        (Some(min), Some(max)) => format!("${:.0}k - ${:.0}k", min / 1000.0, max / 1000.0),
        (Some(min), None) => format!("${:.0}k+", min / 1000.0),
        (None, Some(max)) => format!("Up to ${:.0}k", max / 1000.0),
        (None, None) => "Competitive".to_string(),
    }
}

// Add new pages
#[component]
fn RegisterPage() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let first_name = RwSignal::new(String::new());
    let last_name = RwSignal::new(String::new());
    let loading = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-tiffany-light/20 via-white to-lavender/30">
            <div class="w-full max-w-md">
                <div class="bg-white/80 backdrop-blur-xl rounded-3xl shadow-2xl p-8 border border-white/20">
                    <div class="text-center mb-8">
                        <div class="w-20 h-20 bg-gradient-to-br from-tiffany to-tiffany-dark rounded-2xl flex items-center justify-center mx-auto mb-4">
                            <span class="text-3xl text-white font-bold">"L"</span>
                        </div>
                        <h2 class="text-3xl font-bold text-gray-900">"Create Account"</h2>
                        <p class="text-gray-600 mt-2">"Join the Loco Platform community"</p>
                    </div>
                    
                    <form class="space-y-6">
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="block text-sm font-medium text-gray-700 mb-2">
                                    "First Name"
                                </label>
                                <input
                                    type="text"
                                    required
                                    class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-tiffany focus:border-transparent"
                                    placeholder="John"
                                    on:input=move |ev| first_name.set(event_target_value(&ev))
                                />
                            </div>
                            <div>
                                <label class="block text-sm font-medium text-gray-700 mb-2">
                                    "Last Name"
                                </label>
                                <input
                                    type="text"
                                    required
                                    class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-tiffany focus:border-transparent"
                                    placeholder="Doe"
                                    on:input=move |ev| last_name.set(event_target_value(&ev))
                                />
                            </div>
                        </div>
                        
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Email"
                            </label>
                            <input
                                type="email"
                                required
                                class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-tiffany focus:border-transparent"
                                placeholder="you@example.com"
                                on:input=move |ev| email.set(event_target_value(&ev))
                            />
                        </div>
                        
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Password"
                            </label>
                            <input
                                type="password"
                                required
                                class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-tiffany focus:border-transparent"
                                placeholder="••••••••"
                                on:input=move |ev| password.set(event_target_value(&ev))
                            />
                        </div>
                        
                        <button
                            type="submit"
                            class="w-full py-3 px-4 bg-gradient-to-r from-tiffany to-tiffany-dark text-white font-semibold rounded-xl hover:shadow-lg transform hover:scale-[1.02] transition-all duration-200"
                        >
                            "Create Account"
                        </button>
                    </form>
                    
                    <p class="mt-8 text-center text-sm text-gray-600">
                        "Already have an account? "
                        <a href="/login" class="font-medium text-tiffany hover:text-tiffany-dark">
                            "Sign in"
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn MapPage() -> impl IntoView {
    let jobs = RwSignal::new(get_mock_jobs());
    
    view! {
        <div class="min-h-screen bg-gray-50">
            <AppHeader/>
            
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                <div class="mb-8">
                    <h1 class="text-3xl font-bold text-gray-900">"Job Map"</h1>
                    <p class="text-gray-600 mt-2">"Explore pharmacy positions near you"</p>
                </div>
                
                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                    <div class="lg:col-span-2">
                        <div class="bg-white rounded-xl shadow-sm p-4">
                            <div class="h-[600px] bg-gray-100 rounded-xl flex items-center justify-center">
                                <div class="text-center">
                                    <svg class="w-16 h-16 mx-auto text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 20l-5.447-2.724A1 1 0 013 16.382V5.618a1 1 0 011.447-.894L9 7m0 13l6-3m-6 3V7m6 10l4.553 2.276A1 1 0 0021 18.382V7.618a1 1 0 00-.553-.894L15 4m0 13V4m0 0L9 7" />
                                    </svg>
                                    <h3 class="text-lg font-semibold text-gray-700">"Interactive Map"</h3>
                                    <p class="text-gray-500 mt-2">"Map functionality coming soon"</p>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="space-y-4">
                        <div class="bg-white rounded-xl shadow-sm p-4">
                            <h3 class="font-semibold mb-4">"Nearby Jobs"</h3>
                            <div class="space-y-3">
                                {move || jobs.get().into_iter().take(3).map(|job| view! {
                                    <div class="border-l-4 border-tiffany pl-4 py-2">
                                        <h4 class="font-medium">{job.title.clone()}</h4>
                                        <p class="text-sm text-gray-600">{job.location.clone()}</p>
                                    </div>
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
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
            <AppHeader/>
            
            <div class="max-w-7xl mx-auto px-4 py-8">
                <h1 class="text-3xl font-bold mb-8">"My Profile"</h1>
                
                <div class="bg-white rounded-xl shadow-sm p-6">
                    <div class="flex items-center gap-6 mb-8">
                        <div class="w-24 h-24 bg-gradient-to-br from-tiffany to-tiffany-dark rounded-full flex items-center justify-center text-white text-3xl font-bold">
                            "JD"
                        </div>
                        <div>
                            <h2 class="text-2xl font-semibold">"John Doe"</h2>
                            <p class="text-gray-600">"john.doe@example.com"</p>
                            <p class="text-sm text-gray-500 mt-1">"Member since January 2025"</p>
                        </div>
                    </div>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <h3 class="text-lg font-semibold mb-4">"Personal Information"</h3>
                            <div class="space-y-4">
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">"Phone"</label>
                                    <input type="tel" class="w-full px-4 py-2 border rounded-lg" value="+61 412 345 678"/>
                                </div>
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">"Location"</label>
                                    <input type="text" class="w-full px-4 py-2 border rounded-lg" value="Sydney, NSW"/>
                                </div>
                            </div>
                        </div>
                        
                        <div>
                            <h3 class="text-lg font-semibold mb-4">"Professional Details"</h3>
                            <div class="space-y-4">
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">"AHPRA Number"</label>
                                    <input type="text" class="w-full px-4 py-2 border rounded-lg" value="PHA0001234567"/>
                                </div>
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">"Experience"</label>
                                    <select class="w-full px-4 py-2 border rounded-lg">
                                        <option>"5-10 years"</option>
                                    </select>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="mt-8">
                        <button class="px-6 py-3 bg-gradient-to-r from-tiffany to-tiffany-dark text-white font-semibold rounded-xl hover:shadow-lg">
                            "Save Changes"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ApplicationsPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <AppHeader/>
            
            <div class="max-w-7xl mx-auto px-4 py-8">
                <h1 class="text-3xl font-bold mb-8">"My Applications"</h1>
                
                <div class="space-y-4">
                    <ApplicationCard 
                        job_title="Senior Pharmacist"
                        company="Sydney Hospital"
                        status="Under Review"
                        applied_date="5 days ago"
                    />
                    <ApplicationCard 
                        job_title="Clinical Pharmacist"
                        company="Melbourne Medical Centre"
                        status="Interview Scheduled"
                        applied_date="1 week ago"
                    />
                    <ApplicationCard 
                        job_title="Community Pharmacist"
                        company="Brisbane Pharmacy Group"
                        status="Application Sent"
                        applied_date="2 weeks ago"
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn ApplicationCard(
    job_title: &'static str,
    company: &'static str,
    status: &'static str,
    applied_date: &'static str,
) -> impl IntoView {
    let status_color = match status {
        "Under Review" => "bg-yellow-100 text-yellow-800",
        "Interview Scheduled" => "bg-green-100 text-green-800",
        "Application Sent" => "bg-blue-100 text-blue-800",
        _ => "bg-gray-100 text-gray-800",
    };
    
    view! {
        <div class="bg-white rounded-xl shadow-sm p-6 hover:shadow-lg transition-shadow">
            <div class="flex justify-between items-start">
                <div>
                    <h3 class="text-xl font-semibold">{job_title}</h3>
                    <p class="text-gray-600 mt-1">{company}</p>
                    <p class="text-sm text-gray-500 mt-2">"Applied " {applied_date}</p>
                </div>
                <span class=format!("px-4 py-2 rounded-full text-sm font-medium {}", status_color)>
                    {status}
                </span>
            </div>
        </div>
    }
}

fn get_mock_jobs() -> Vec<Job> {
    vec![
        Job {
            id: "1".to_string(),
            title: "Senior Pharmacist".to_string(),
            company: "Sydney Hospital".to_string(),
            location: "Sydney, NSW".to_string(),
            description: "Leading pharmacy role in a major teaching hospital.".to_string(),
            salary_min: Some(95000.0),
            salary_max: Some(115000.0),
            job_type: "Full-time".to_string(),
            posted_date: "2 days ago".to_string(),
        },
        Job {
            id: "2".to_string(),
            title: "Clinical Pharmacist".to_string(),
            company: "Melbourne Medical Centre".to_string(),
            location: "Melbourne, VIC".to_string(),
            description: "Join our team providing patient-centered care.".to_string(),
            salary_min: Some(85000.0),
            salary_max: Some(105000.0),
            job_type: "Full-time".to_string(),
            posted_date: "3 days ago".to_string(),
        },
        Job {
            id: "3".to_string(),
            title: "Community Pharmacist".to_string(),
            company: "Brisbane Pharmacy Group".to_string(),
            location: "Brisbane, QLD".to_string(),
            description: "Rewarding community pharmacy position.".to_string(),
            salary_min: Some(80000.0),
            salary_max: Some(95000.0),
            job_type: "Part-time".to_string(),
            posted_date: "1 week ago".to_string(),
        },
        Job {
            id: "4".to_string(),
            title: "Locum Pharmacist".to_string(),
            company: "Perth Medical Network".to_string(),
            location: "Perth, WA".to_string(),
            description: "Flexible locum opportunities across Perth metro.".to_string(),
            salary_min: Some(50.0),
            salary_max: Some(65.0),
            job_type: "Contract".to_string(),
            posted_date: "3 days ago".to_string(),
        },
        Job {
            id: "5".to_string(),
            title: "Hospital Pharmacist - Oncology".to_string(),
            company: "Adelaide Cancer Centre".to_string(),
            location: "Adelaide, SA".to_string(),
            description: "Specialised oncology pharmacy position.".to_string(),
            salary_min: Some(90000.0),
            salary_max: Some(110000.0),
            job_type: "Full-time".to_string(),
            posted_date: "1 week ago".to_string(),
        },
    ]
}
