use leptos::*;
use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::A;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, Storage};
use gloo_console as console;
use gloo_timers::future::TimeoutFuture;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Local, Duration, Timelike};

// Advanced dashboard data structures
#[derive(Clone, Debug, Serialize, Deserialize)]
struct DashboardMetrics {
    total_applications: u32,
    applications_this_week: u32,
    pending_interviews: u32,
    job_alerts: u32,
    profile_completion: f64,
    avg_response_time: f64,
    success_rate: f64,
    saved_jobs: u32,
    viewed_jobs: u32,
    application_conversion_rate: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ApplicationStatus {
    id: String,
    job_title: String,
    company: String,
    applied_date: String,
    status: String,
    status_type: String,
    next_action: Option<String>,
    salary_range: String,
    location: String,
    urgency: String,
    progress_percentage: u8,
    estimated_response_time: String,
    contact_person: Option<String>,
    interview_date: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JobRecommendation {
    id: String,
    title: String,
    company: String,
    location: String,
    match_score: f64,
    salary_min: u32,
    salary_max: u32,
    job_type: String,
    posted_date: String,
    applications_count: u32,
    urgency: String,
    remote_option: bool,
    benefits_count: u32,
    employer_rating: f64,
    growth_potential: String,
    skills_match: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MarketInsight {
    label: String,
    value: String,
    change: f64,
    trend: String,
    period: String,
    context: String,
    icon: String,
    recommendation: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ActivityFeedItem {
    id: String,
    title: String,
    description: String,
    timestamp: String,
    activity_type: String,
    priority: String,
    action_required: bool,
    related_job_id: Option<String>,
    metadata: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CareerGoal {
    id: String,
    title: String,
    description: String,
    target_date: String,
    progress: f64,
    milestones: Vec<String>,
    next_steps: Vec<String>,
    priority: String,
}

#[component]
pub fn Dashboard() -> impl IntoView {
    // Advanced state management
    let dashboard_metrics = RwSignal::new(generate_dashboard_metrics());
    let applications = RwSignal::new(generate_sample_applications());
    let recommendations = RwSignal::new(generate_job_recommendations());
    let market_insights = RwSignal::new(generate_market_insights());
    let activity_feed = RwSignal::new(generate_activity_feed());
    let career_goals = RwSignal::new(generate_career_goals());
    let loading_states = RwSignal::new(HashMap::<String, bool>::new());
    let view_mode = RwSignal::new("overview".to_string());
    let time_period = RwSignal::new("week".to_string());
    let real_time_updates = RwSignal::new(true);
    let notifications_enabled = RwSignal::new(true);
    let dashboard_theme = RwSignal::new("default".to_string());
    let user_preferences = RwSignal::new(HashMap::<String, String>::new());
    let last_update = RwSignal::new(chrono::Utc::now().format("%H:%M:%S").to_string());
    
    // Get user name from localStorage or default
    let user_name = move || {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                storage.get_item("user_name").ok().flatten()
                    .unwrap_or_else(|| "Professional".to_string())
            } else {
                "Professional".to_string()
            }
        } else {
            "Professional".to_string()
        }
    };

    // Real-time data updates
    Effect::new({
        let dashboard_metrics = dashboard_metrics.clone();
        let applications = applications.clone();
        let last_update = last_update.clone();
        move |_| {
            if real_time_updates.get() {
                spawn_local(async move {
                    loop {
                        TimeoutFuture::new(30_000).await; // Update every 30 seconds
                        
                        // Simulate real-time metric updates
                        dashboard_metrics.update(|metrics| {
                            metrics.applications_this_week += if js_sys::Math::random() > 0.7 { 1 } else { 0 };
                            metrics.job_alerts += if js_sys::Math::random() > 0.8 { 1 } else { 0 };
                            metrics.viewed_jobs += if js_sys::Math::random() > 0.6 { 1 } else { 0 };
                        });
                        
                        // Update applications status
                        applications.update(|apps| {
                            for app in apps.iter_mut() {
                                if js_sys::Math::random() > 0.9 {
                                    match app.status.as_str() {
                                        "Application Sent" => {
                                            app.status = "Under Review".to_string();
                                            app.status_type = "pending".to_string();
                                            app.progress_percentage = 30;
                                        },
                                        "Under Review" => {
                                            app.status = "Interview Scheduled".to_string();
                                            app.status_type = "success".to_string();
                                            app.progress_percentage = 60;
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        });
                        
                        last_update.set(chrono::Utc::now().format("%H:%M:%S").to_string());
                        console::log!("📊 Dashboard data updated in real-time");
                    }
                });
            }
        }
    });

    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50/30 to-teal-50/50 relative overflow-hidden">
            // Animated Background Elements
            <div class="absolute inset-0 overflow-hidden pointer-events-none">
                <div class="absolute -top-40 -right-40 w-80 h-80 bg-gradient-to-br from-tiffany-blue/20 to-purple-400/20 rounded-full blur-3xl animate-pulse"></div>
                <div class="absolute -bottom-40 -left-40 w-96 h-96 bg-gradient-to-tr from-pink-400/20 to-orange-400/20 rounded-full blur-3xl animate-pulse delay-1000"></div>
                <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-64 h-64 bg-gradient-to-r from-blue-400/10 to-green-400/10 rounded-full blur-2xl animate-pulse delay-500"></div>
            </div>

            // Advanced Header with Real-time Status and Floating Design
            <div class="relative z-50 mx-4 mt-4 glass bg-white/95 backdrop-blur-2xl border border-white/20 rounded-3xl shadow-2xl shadow-black/5">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex items-center justify-between h-20">
                        <div class="flex items-center gap-8">
                            <div class="flex items-center gap-4">
                                // Animated Logo/Icon
                                <div class="relative">
                                    <div class="w-12 h-12 bg-gradient-to-br from-tiffany-blue to-blue-600 rounded-2xl flex items-center justify-center shadow-lg rotate-3 hover:rotate-0 transition-transform duration-300">
                                        <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
                                        </svg>
                                    </div>
                                    <div class="absolute -top-1 -right-1 w-4 h-4 bg-green-500 rounded-full animate-ping"></div>
                                    <div class="absolute -top-1 -right-1 w-4 h-4 bg-green-500 rounded-full"></div>
                                </div>
                                
                                <div>
                                    <h1 class="text-3xl font-bold tracking-tight">
                                        <span class="bg-gradient-to-r from-tiffany-blue via-blue-600 to-purple-600 bg-clip-text text-transparent animate-gradient-x">
                                            Pharmacy Career Hub
                                        </span>
                                    </h1>
                                    <p class="text-sm text-gray-600 mt-1 flex items-center gap-2">
                                        <span class="inline-flex items-center gap-1">
                                            <span class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></span>
                                            "Welcome back, "
                                        </span>
                                        <span class="font-semibold bg-gradient-to-r from-tiffany-dark to-blue-700 bg-clip-text text-transparent">
                                            {user_name}
                                        </span>
                                        <span class="text-xl animate-bounce">"👋"</span>
                                    </p>
                                </div>
                            </div>
                            
                            // Enhanced Live Status with Animation
                            <div class="relative">
                                <div class="flex items-center gap-3 px-6 py-3 bg-gradient-to-r from-green-50 to-emerald-50 rounded-full border border-green-200/50 shadow-sm hover:shadow-md transition-all duration-300">
                                    <div class="relative">
                                        <div class="w-3 h-3 bg-green-500 rounded-full animate-pulse"></div>
                                        <div class="absolute inset-0 w-3 h-3 bg-green-400 rounded-full animate-ping"></div>
                                    </div>
                                    <div class="flex flex-col">
                                        <span class="text-sm font-medium text-green-700">"Live Dashboard"</span>
                                        <span class="text-xs text-gray-500">
                                            "Last update: " {move || last_update.get()}
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>
                        
                        // Advanced Controls with Beautiful Design
                        <div class="flex items-center gap-4">
                            // Enhanced View Mode Selector with Icons
                            <div class="flex bg-gradient-to-r from-gray-100/80 to-gray-50/80 backdrop-blur-sm rounded-2xl p-1.5 shadow-inner">
                                <button
                                    class=move || format!(
                                        "flex items-center gap-2 px-5 py-2.5 text-sm font-medium rounded-xl transition-all duration-300 {}",
                                        if view_mode.get() == "overview" { 
                                            "bg-white text-tiffany-dark shadow-lg shadow-tiffany-blue/20 scale-105" 
                                        } else { 
                                            "text-gray-600 hover:text-gray-900 hover:bg-white/50" 
                                        }
                                    )
                                    on:click=move |_| view_mode.set("overview".to_string())
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM4 13a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H5a1 1 0 01-1-1v-6zM16 13a1 1 0 011-1h2a1 1 0 011 1v6a1 1 0 01-1 1h-2a1 1 0 01-1-1v-6z"></path>
                                    </svg>
                                    "Overview"
                                </button>
                                <button
                                    class=move || format!(
                                        "flex items-center gap-2 px-5 py-2.5 text-sm font-medium rounded-xl transition-all duration-300 {}",
                                        if view_mode.get() == "analytics" { 
                                            "bg-white text-tiffany-dark shadow-lg shadow-tiffany-blue/20 scale-105" 
                                        } else { 
                                            "text-gray-600 hover:text-gray-900 hover:bg-white/50" 
                                        }
                                    )
                                    on:click=move |_| view_mode.set("analytics".to_string())
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
                                    </svg>
                                    "Analytics"
                                </button>
                                <button
                                    class=move || format!(
                                        "flex items-center gap-2 px-5 py-2.5 text-sm font-medium rounded-xl transition-all duration-300 {}",
                                        if view_mode.get() == "goals" { 
                                            "bg-white text-tiffany-dark shadow-lg shadow-tiffany-blue/20 scale-105" 
                                        } else { 
                                            "text-gray-600 hover:text-gray-900 hover:bg-white/50" 
                                        }
                                    )
                                    on:click=move |_| view_mode.set("goals".to_string())
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                    </svg>
                                    "Goals"
                                </button>
                            </div>
                            
                            // Enhanced Time Period Selector
                            <div class="relative">
                                <select
                                    class="appearance-none bg-white/90 backdrop-blur-sm border border-gray-200/50 rounded-xl px-4 py-2.5 pr-10 text-sm font-medium text-gray-700 shadow-sm hover:shadow-md transition-all duration-300 focus:ring-2 focus:ring-tiffany-blue/20 focus:border-tiffany-blue"
                                    on:change=move |ev| {
                                        time_period.set(event_target_value(&ev));
                                    }
                                >
                                    <option value="day">"📅 Today"</option>
                                    <option value="week" selected>"📊 This Week"</option>
                                    <option value="month">"📈 This Month"</option>
                                    <option value="quarter">"🎯 This Quarter"</option>
                                </select>
                                <svg class="absolute right-3 top-3 w-4 h-4 text-gray-400 pointer-events-none" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                                </svg>
                            </div>
                            
                            // Enhanced Notifications Toggle
                            <button
                                class=move || format!(
                                    "relative p-3 rounded-xl transition-all duration-300 shadow-sm hover:shadow-md {}",
                                    if notifications_enabled.get() { 
                                        "bg-gradient-to-r from-tiffany-blue to-blue-600 text-white scale-105 shadow-lg shadow-tiffany-blue/25" 
                                    } else { 
                                        "bg-white/90 text-gray-500 hover:bg-gray-50" 
                                    }
                                )
                                on:click=move |_| notifications_enabled.update(|n| *n = !*n)
                            >
                                <svg class="w-5 h-5 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-5 5-5-5h5zM7 7h.01M7 11h.01M7 15h.01"></path>
                                </svg>
                                <Show when=move || notifications_enabled.get()>
                                    <div class="absolute -top-1 -right-1">
                                        <span class="flex h-3 w-3">
                                            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-red-400 opacity-75"></span>
                                            <span class="relative inline-flex rounded-full h-3 w-3 bg-red-500"></span>
                                        </span>
                                    </div>
                                </Show>
                            </button>

                            // Quick Action Menu
                            <div class="relative">
                                <button class="p-3 bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-105">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"></path>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="max-w-7xl mx-auto px-6 py-8 relative z-10">
                // Dynamic Content Based on View Mode
                <Show
                    when=move || view_mode.get() == "overview"
                    fallback=move || {
                        if view_mode.get() == "analytics" {
                            view! { <AnalyticsView 
                                metrics=dashboard_metrics.get()
                                time_period=time_period.get()
                                market_insights=market_insights.get()
                            /> }.into_any()
                        } else if view_mode.get() == "goals" {
                            view! { <GoalsView career_goals=career_goals.get() /> }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }
                >
                    // Overview Dashboard
                    <OverviewDashboard 
                        metrics=dashboard_metrics.get()
                        applications=applications.get()
                        recommendations=recommendations.get()
                        market_insights=market_insights.get()
                        activity_feed=activity_feed.get()
                        user_name=user_name()
                    />
                </Show>
            </div>
            
            // Inject Enhanced CSS
            <style>{ENHANCED_DASHBOARD_CSS}</style>
            
            // Floating Action Button for Quick Actions
            <div class="fixed bottom-8 right-8 z-50">
                <div class="relative group">
                    <button class="w-16 h-16 bg-gradient-to-r from-tiffany-blue to-blue-600 rounded-full shadow-2xl hover:shadow-3xl transition-all duration-300 flex items-center justify-center text-white hover:scale-110 animate-float">
                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
                        </svg>
                    </button>
                    
                    // Quick Action Menu
                    <div class="absolute bottom-20 right-0 opacity-0 group-hover:opacity-100 transition-all duration-300 transform scale-95 group-hover:scale-100 space-y-3">
                        <button class="w-12 h-12 bg-gradient-to-r from-green-500 to-emerald-600 rounded-full shadow-lg hover:shadow-xl transition-all flex items-center justify-center text-white">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                            </svg>
                        </button>
                        <button class="w-12 h-12 bg-gradient-to-r from-purple-500 to-pink-600 rounded-full shadow-lg hover:shadow-xl transition-all flex items-center justify-center text-white">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                            </svg>
                        </button>
                        <button class="w-12 h-12 bg-gradient-to-r from-blue-500 to-cyan-600 rounded-full shadow-lg hover:shadow-xl transition-all flex items-center justify-center text-white">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
            
            // Achievement Toast Notifications
            <div class="fixed top-24 right-8 z-40 space-y-4 max-w-sm">
                <div class="glass-ultra rounded-2xl p-4 shadow-2xl border-l-4 border-green-500 animate-slide-up">
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 bg-gradient-to-br from-green-500 to-emerald-600 rounded-full flex items-center justify-center">
                            <svg class="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                            </svg>
                        </div>
                        <div class="flex-1">
                            <p class="font-semibold text-gray-900">"Profile Achievement!"</p>
                            <p class="text-sm text-gray-600">"You've reached 85% completion"</p>
                        </div>
                        <button class="text-gray-400 hover:text-gray-600">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

// Overview Dashboard Component
#[component]
fn OverviewDashboard(
    #[prop(into)] metrics: MaybeSignal<DashboardMetrics>,
    #[prop(into)] applications: MaybeSignal<Vec<ApplicationStatus>>,
    #[prop(into)] recommendations: MaybeSignal<Vec<JobRecommendation>>,
    #[prop(into)] market_insights: MaybeSignal<Vec<MarketInsight>>,
    #[prop(into)] activity_feed: MaybeSignal<Vec<ActivityFeedItem>>,
    #[prop(into)] user_name: MaybeSignal<String>,
) -> impl IntoView {
    let metrics = metrics.get();
    let applications = applications.get();
    // Removed redundant get() since we use it in the For loop
    let market_insights = market_insights.get();
    let activity_feed = activity_feed.get();
    let user_name = user_name.get();
    view! {
        <div class="space-y-8">
            // Enhanced Welcome Section with Floating Design and Animations
            <div class="relative">
                <div class="absolute inset-0 bg-gradient-to-r from-tiffany-blue/5 via-purple-500/5 to-pink-500/5 rounded-3xl blur-xl"></div>
                <div class="relative glass bg-gradient-to-br from-white/95 to-white/80 backdrop-blur-2xl rounded-3xl p-8 border border-white/20 shadow-2xl shadow-black/5 hover:shadow-3xl transition-all duration-700">
                    <div class="absolute inset-0 bg-gradient-to-r from-tiffany-blue/10 via-transparent to-purple-500/10 rounded-3xl"></div>
                    <div class="relative z-10">
                        <div class="flex items-start justify-between">
                            <div class="space-y-6 flex-1">
                                <div>
                                    <h1 class="text-5xl font-bold mb-3 leading-tight">
                                        <span class="block text-gray-900">
                                            "Good " {get_time_of_day()} ", "
                                        </span>
                                        <span class="bg-gradient-to-r from-tiffany-blue via-blue-600 to-purple-600 bg-clip-text text-transparent animate-gradient-x">
                                            {user_name} "!"
                                        </span>
                                        <span class="inline-block ml-2 text-4xl animate-wave">"👋"</span>
                                    </h1>
                                    <p class="text-xl text-gray-600/80 leading-relaxed max-w-2xl">
                                        "Your pharmacy career is advancing beautifully. Here's your personalised overview with the latest insights and opportunities."
                                    </p>
                                </div>
                                
                                <div class="flex items-center gap-6 text-sm">
                                    <div class="flex items-center gap-3 px-4 py-3 bg-gradient-to-r from-green-50 to-emerald-50 rounded-2xl border border-green-200/50">
                                        <div class="relative">
                                            <svg class="w-5 h-5 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                                            </svg>
                                            <div class="absolute -inset-1 bg-green-400 rounded-full animate-ping opacity-30"></div>
                                        </div>
                                        <div>
                                            <span class="font-semibold text-green-700">
                                                "Profile " {format!("{:.0}%", metrics.profile_completion)} " complete"
                                            </span>
                                            <div class="w-24 h-1.5 bg-green-200 rounded-full mt-1">
                                                <div 
                                                    class="h-full bg-gradient-to-r from-green-500 to-emerald-500 rounded-full transition-all duration-1000"
                                                    style=format!("width: {}%", metrics.profile_completion)
                                                ></div>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="flex items-center gap-3 px-4 py-3 bg-gradient-to-r from-blue-50 to-cyan-50 rounded-2xl border border-blue-200/50">
                                        <div class="relative">
                                            <span class="text-2xl animate-pulse">"🔔"</span>
                                            <div class="absolute -top-1 -right-1 w-3 h-3 bg-red-500 rounded-full animate-bounce"></div>
                                        </div>
                                        <div>
                                            <span class="font-semibold text-blue-700">
                                                {metrics.job_alerts} " new opportunities"
                                            </span>
                                            <p class="text-xs text-blue-600 mt-0.5">"Perfect matches found"</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            
                            <div class="text-right space-y-4">
                                <div class="relative">
                                    <div class="absolute inset-0 bg-gradient-to-br from-green-400/20 to-emerald-500/20 rounded-3xl blur-lg"></div>
                                    <div class="relative bg-gradient-to-br from-green-50 to-emerald-50 rounded-3xl p-6 border border-green-200/50 backdrop-blur-sm">
                                        <div class="flex items-center gap-3 mb-3">
                                            <div class="w-12 h-12 bg-gradient-to-br from-green-500 to-emerald-600 rounded-2xl flex items-center justify-center shadow-lg">
                                                <span class="text-2xl">"🎯"</span>
                                            </div>
                                            <div class="text-left">
                                                <p class="text-sm font-medium text-green-700">"Success Rate"</p>
                                                <p class="text-3xl font-bold bg-gradient-to-r from-green-600 to-emerald-600 bg-clip-text text-transparent">
                                                    {format!("{:.0}%", metrics.success_rate)}
                                                </p>
                                            </div>
                                        </div>
                                        <p class="text-xs text-green-600 font-medium flex items-center gap-1">
                                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"></path>
                                            </svg>
                                            "Above industry average"
                                        </p>
                                    </div>
                                </div>
                                
                                <div class="relative">
                                    <div class="absolute inset-0 bg-gradient-to-br from-blue-400/20 to-purple-500/20 rounded-3xl blur-lg"></div>
                                    <div class="relative bg-gradient-to-br from-blue-50 to-purple-50 rounded-3xl p-6 border border-blue-200/50 backdrop-blur-sm">
                                        <div class="flex items-center gap-3">
                                            <div class="w-12 h-12 bg-gradient-to-br from-blue-500 to-purple-600 rounded-2xl flex items-center justify-center shadow-lg">
                                                <span class="text-2xl">"⚡"</span>
                                            </div>
                                            <div class="text-left">
                                                <p class="text-sm font-medium text-blue-700">"Response Time"</p>
                                                <p class="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                                                    {format!("{:.1}h", metrics.avg_response_time)}
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Advanced Metrics Grid
            <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
                <MetricCard
                    icon="📋"
                    title="Applications"
                    value=metrics.total_applications.to_string()
                    subtitle=format!("{} this week", metrics.applications_this_week)
                    trend="+15%"
                    trend_positive=true
                />
                <MetricCard
                    icon="📅"
                    title="Interviews"
                    value=metrics.pending_interviews.to_string()
                    subtitle="Scheduled"
                    trend="+2"
                    trend_positive=true
                />
                <MetricCard
                    icon="💾"
                    title="Saved Jobs"
                    value=metrics.saved_jobs.to_string()
                    subtitle="Ready to apply"
                    trend=format!("{} viewed", metrics.viewed_jobs)
                    trend_positive=true
                />
                <MetricCard
                    icon="\"⚡\""
                    title="Response Time"
                    value=format!("{:.1}h", metrics.avg_response_time)
                    subtitle="Average"
                    trend="-2.5h"
                    trend_positive=true
                />
            </div>

            <div class="grid lg:grid-cols-3 gap-8">
                // Advanced Applications Panel
                <div class="lg:col-span-2 glass bg-white/80 backdrop-blur-xl rounded-3xl shadow-xl p-8">
                    <div class="flex justify-between items-center mb-6">
                        <div>
                            <h2 class="text-2xl font-bold">Recent Applications</h2>
                            <p class="text-gray-600 mt-1">Track your application progress</p>
                        </div>
                        <A href="/applications" class="btn-primary rounded-xl px-6 py-3 flex items-center gap-2">
                            "View All"
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                            </svg>
                        </A>
                    </div>
                    <div class="space-y-4">
                        <For
                            each=move || applications.clone()
                            key=|app| app.id.clone()
                            children=move |app| {
                                view! { <EnhancedApplicationCard application=app /> }
                            }
                        />
                    </div>
                </div>

                // Smart Recommendations Panel
                <div class="glass bg-white/80 backdrop-blur-xl rounded-3xl shadow-xl p-8">
                    <div class="flex items-center justify-between mb-6">
                        <div>
                            <h2 class="text-xl font-bold">AI Recommendations</h2>
                            <p class="text-sm text-gray-600 mt-1">Powered by machine learning</p>
                        </div>
                        <span class="bg-gradient-to-r from-purple-400 to-pink-400 text-white text-xs px-3 py-1 rounded-full font-medium">
                            "SMART"
                        </span>
                    </div>
                    <div class="space-y-4">
                        {move || {
                            recommendations.get()
                                .iter()
                                .take(3)
                                .cloned()
                                .map(|rec| view! { <SmartRecommendationCard recommendation=rec /> })
                                .collect::<Vec<_>>()
                        }}
                    </div>
                    <A href="/jobs" class="btn btn-secondary w-full mt-6 rounded-xl py-3 flex items-center justify-center gap-2">
                        "Explore All Opportunities"
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"></path>
                        </svg>
                    </A>
                </div>
            </div>

            <div class="grid lg:grid-cols-2 gap-8">
                // Market Intelligence Panel
                <div class="glass bg-white/80 backdrop-blur-xl rounded-3xl shadow-xl p-8">
                    <div class="flex items-center justify-between mb-6">
                        <div>
                            <h2 class="text-xl font-bold">Market Intelligence</h2>
                            <p class="text-sm text-gray-600 mt-1">Real-time industry insights</p>
                        </div>
                        <button class="btn btn-ghost text-sm rounded-lg px-3 py-2">
                            "View Report"
                        </button>
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <For
                            each=move || market_insights.clone()
                            key=|insight| insight.label.clone()
                            children=move |insight| {
                                view! { <MarketInsightCard insight=insight /> }
                            }
                        />
                    </div>
                </div>

                // Activity Feed Panel
                <div class="glass bg-white/80 backdrop-blur-xl rounded-3xl shadow-xl p-8">
                    <div class="flex items-center justify-between mb-6">
                        <div>
                            <h2 class="text-xl font-bold">Recent Activity</h2>
                            <p class="text-sm text-gray-600 mt-1">Your career timeline</p>
                        </div>
                        <div class="flex items-center gap-2">
                            <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                            <span class="text-xs text-gray-500">Live</span>
                        </div>
                    </div>
                    <div class="space-y-4 max-h-96 overflow-y-auto">
                        <For
                            each=move || activity_feed.clone()
                            key=|item| item.id.clone()
                            children=move |item| {
                                view! { <ActivityFeedCard activity=item /> }
                            }
                        />
                    </div>
                </div>
            </div>

            // Quick Action Cards
            <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
                <QuickActionCard
                    icon="🔍"
                    title="Search Jobs"
                    description="Find perfect matches"
                    href="/jobs"
                    color="from-blue-400 to-blue-600"
                />
                <QuickActionCard
                    icon="📝"
                    title="Create Application"
                    description="Apply to new positions"
                    href="/applications/new"
                    color="from-green-400 to-green-600"
                />
                <QuickActionCard
                    icon="📊"
                    title="View Analytics"
                    description="Track your progress"
                    href="/analytics"
                    color="from-purple-400 to-purple-600"
                />
                <QuickActionCard
                    icon="\"🎯\""
                    title="Set Goals"
                    description="Plan your career"
                    href="/goals"
                    color="from-pink-400 to-pink-600"
                />
            </div>
        </div>
    }
}

// Analytics View Component
#[component]
fn AnalyticsView(
    metrics: DashboardMetrics,
    time_period: String,
    market_insights: Vec<MarketInsight>,
) -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div class="glass-tiffany rounded-3xl p-8">
                <h1 class="text-3xl font-bold mb-4">Career Analytics</h1>
                <p class="text-gray-600">
                    "Deep insights into your job search performance for " {time_period.clone()}
                </p>
            </div>

            // Performance Overview
            <div class="grid md:grid-cols-3 gap-6">
                <div class="glass bg-white/80 backdrop-blur-xl rounded-2xl p-6">
                    <h3 class="font-semibold mb-4">Application Funnel</h3>
                    <div class="space-y-3">
                        <div class="flex justify-between items-center">
                            <span class="text-sm text-gray-600">Applications Sent</span>
                            <span class="font-semibold">{metrics.total_applications}</span>
                        </div>
                        <div class="w-full bg-gray-200 rounded-full h-2">
                            <div class="bg-blue-500 h-2 rounded-full" style="width: 100%"></div>
                        </div>
                        <div class="flex justify-between items-center">
                            <span class="text-sm text-gray-600">Responses Received</span>
                            <span class="font-semibold">{(metrics.total_applications as f64 * 0.3) as u32}</span>
                        </div>
                        <div class="w-full bg-gray-200 rounded-full h-2">
                            <div class="bg-yellow-500 h-2 rounded-full" style="width: 30%"></div>
                        </div>
                        <div class="flex justify-between items-center">
                            <span class="text-sm text-gray-600">Interviews</span>
                            <span class="font-semibold">{metrics.pending_interviews}</span>
                        </div>
                        <div class="w-full bg-gray-200 rounded-full h-2">
                            <div class="bg-green-500 h-2 rounded-full" style="width: 15%"></div>
                        </div>
                    </div>
                </div>

                <div class="glass bg-white/80 backdrop-blur-xl rounded-2xl p-6">
                    <h3 class="font-semibold mb-4">Success Metrics</h3>
                    <div class="space-y-4">
                        <div>
                            <div class="flex justify-between items-center mb-1">
                                <span class="text-sm text-gray-600">Response Rate</span>
                                <span class="text-sm font-medium">{format!("{:.1}%", metrics.success_rate)}</span>
                            </div>
                            <div class="w-full bg-gray-200 rounded-full h-2">
                                <div 
                                    class="bg-gradient-to-r from-tiffany-blue to-tiffany-dark h-2 rounded-full" 
                                    style=format!("width: {}%", metrics.success_rate)
                                ></div>
                            </div>
                        </div>
                        <div>
                            <div class="flex justify-between items-center mb-1">
                                <span class="text-sm text-gray-600">Profile Views</span>
                                <span class="text-sm font-medium">{metrics.viewed_jobs}</span>
                            </div>
                            <div class="w-full bg-gray-200 rounded-full h-2">
                                <div class="bg-blue-500 h-2 rounded-full" style="width: 85%"></div>
                            </div>
                        </div>
                        <div>
                            <div class="flex justify-between items-center mb-1">
                                <span class="text-sm text-gray-600">Conversion Rate</span>
                                <span class="text-sm font-medium">{format!("{:.1}%", metrics.application_conversion_rate)}</span>
                            </div>
                            <div class="w-full bg-gray-200 rounded-full h-2">
                                <div class="bg-green-500 h-2 rounded-full" style=format!("width: {}%", metrics.application_conversion_rate)></div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="glass bg-white/80 backdrop-blur-xl rounded-2xl p-6">
                    <h3 class="font-semibold mb-4">Time Analysis</h3>
                    <div class="space-y-4">
                        <div class="text-center">
                            <p class="text-2xl font-bold text-tiffany-dark">{format!("{:.1}h", metrics.avg_response_time)}</p>
                            <p class="text-sm text-gray-600">Avg Response Time</p>
                        </div>
                        <div class="text-center">
                            <p class="text-2xl font-bold text-green-600">{metrics.applications_this_week}</p>
                            <p class="text-sm text-gray-600">This Week</p>
                        </div>
                        <div class="text-center">
                            <p class="text-2xl font-bold text-blue-600">{metrics.job_alerts}</p>
                            <p class="text-sm text-gray-600">New Alerts</p>
                        </div>
                    </div>
                </div>
            </div>

            // Market Analysis
            <div class="glass bg-white/80 backdrop-blur-xl rounded-3xl p-8">
                <h2 class="text-2xl font-bold mb-6">Market Analysis</h2>
                <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <For
                        each=move || market_insights.clone()
                        key=|insight| insight.label.clone()
                        children=move |insight| {
                            view! { <DetailedMarketInsightCard insight=insight /> }
                        }
                    />
                </div>
            </div>
        </div>
    }
}

// Goals View Component
#[component]
fn GoalsView(career_goals: Vec<CareerGoal>) -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div class="glass-tiffany rounded-3xl p-8">
                <h1 class="text-3xl font-bold mb-4">Career Goals</h1>
                <p class="text-gray-600">
                    "Track your professional development and achieve your pharmacy career aspirations."
                </p>
            </div>

            <div class="grid lg:grid-cols-2 gap-8">
                <For
                    each=move || career_goals.clone()
                    key=|goal| goal.id.clone()
                    children=move |goal| {
                        view! { <CareerGoalCard goal=goal /> }
                    }
                />
            </div>

            // Add New Goal Button
            <div class="text-center">
                <button class="btn-primary rounded-xl px-8 py-4 text-lg">
                    "Set New Career Goal"
                </button>
            </div>
        </div>
    }
}

// Individual Component Implementations

#[component]
fn MetricCard(
    icon: &'static str,
    title: &'static str,
    value: String,
    subtitle: String,
    trend: String,
    trend_positive: bool,
) -> impl IntoView {
    let gradient_class = if trend_positive {
        "from-green-400/20 to-emerald-500/20"
    } else {
        "from-red-400/20 to-pink-500/20"
    };
    
    let trend_class = if trend_positive {
        "text-green-600 bg-gradient-to-r from-green-100 to-emerald-100 border-green-200"
    } else {
        "text-red-600 bg-gradient-to-r from-red-100 to-pink-100 border-red-200"
    };

    view! {
        <div class="group relative">
            <div class=format!("absolute inset-0 bg-gradient-to-br {} rounded-3xl blur-lg opacity-0 group-hover:opacity-100 transition-opacity duration-500", gradient_class)></div>
            <div class="relative glass bg-gradient-to-br from-white/95 to-white/85 backdrop-blur-2xl rounded-3xl p-6 border border-white/20 shadow-lg hover:shadow-2xl transition-all duration-500 hover:scale-105 hover:-translate-y-1">
                <div class="flex items-start justify-between mb-4">
                    <div class="relative">
                        <div class="w-14 h-14 bg-gradient-to-br from-gray-100 to-gray-50 rounded-2xl flex items-center justify-center shadow-sm group-hover:shadow-md transition-shadow duration-300">
                            <span class="text-3xl filter group-hover:brightness-110 transition-all duration-300">{icon}</span>
                        </div>
                        <div class="absolute -inset-1 bg-gradient-to-r from-tiffany-blue/30 to-purple-500/30 rounded-2xl blur opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                    </div>
                    <div class=format!(
                        "flex items-center gap-1 text-sm font-semibold px-3 py-1.5 rounded-xl border shadow-sm {}",
                        trend_class
                    )>
                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=if trend_positive { "M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" } else { "M13 17h8m0 0V9m0 8l-8-8-4 4-6-6" }></path>
                        </svg>
                        {trend}
                    </div>
                </div>
                <div class="space-y-2">
                    <p class="text-3xl font-bold bg-gradient-to-r from-gray-900 to-gray-700 bg-clip-text text-transparent group-hover:from-tiffany-dark group-hover:to-blue-700 transition-all duration-300">
                        {value}
                    </p>
                    <p class="text-sm font-medium text-gray-700 uppercase tracking-wide">{title}</p>
                    <p class="text-sm text-gray-500">{subtitle}</p>
                </div>
                
                // Animated background pattern
                <div class="absolute inset-0 opacity-5 group-hover:opacity-10 transition-opacity duration-500">
                    <div class="absolute top-4 right-4 w-20 h-20 bg-gradient-to-br from-tiffany-blue to-purple-500 rounded-full blur-xl"></div>
                    <div class="absolute bottom-4 left-4 w-16 h-16 bg-gradient-to-tr from-pink-400 to-orange-400 rounded-full blur-lg"></div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn EnhancedApplicationCard(application: ApplicationStatus) -> impl IntoView {
    let status_gradient = match application.status_type.as_str() {
        "success" => "from-green-100 to-emerald-100 border-green-200 text-green-800",
        "pending" => "from-yellow-100 to-orange-100 border-yellow-200 text-yellow-800", 
        "info" => "from-blue-100 to-cyan-100 border-blue-200 text-blue-800",
        _ => "from-gray-100 to-slate-100 border-gray-200 text-gray-800",
    };

    let border_gradient = match application.status_type.as_str() {
        "success" => "border-green-400",
        "pending" => "border-yellow-400",
        "info" => "border-blue-400", 
        _ => "border-gray-400",
    };

    view! {
        <div class="group relative floating-card">
            <div class="absolute inset-0 bg-gradient-to-r from-tiffany-blue/5 to-purple-500/5 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
            <div class=format!("relative glass-ultra rounded-2xl p-6 border-l-4 {} hover:shadow-2xl transition-all duration-500 hover:-translate-y-1", border_gradient)>
                <div class="flex justify-between items-start mb-6">
                    <div class="flex-1 space-y-3">
                        <div class="flex items-start gap-3">
                            <div class="w-12 h-12 bg-gradient-to-br from-tiffany-blue to-blue-600 rounded-xl flex items-center justify-center shadow-lg">
                                <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2-2v2m8 0V6a2 2 0 012 2v6a2 2 0 01-2 2H6a2 2 0 01-2-2V8a2 2 0 012-2V6"></path>
                                </svg>
                            </div>
                            <div class="flex-1">
                                <h3 class="font-bold text-xl bg-gradient-to-r from-gray-900 to-gray-700 bg-clip-text text-transparent group-hover:from-tiffany-dark group-hover:to-blue-700 transition-all duration-300">
                                    {application.job_title.clone()}
                                </h3>
                                <p class="text-gray-600 font-medium mt-1">{application.company.clone()}</p>
                                <p class="text-gray-500 text-sm flex items-center gap-2 mt-2">
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                                    </svg>
                                    {application.location.clone()}
                                </p>
                            </div>
                        </div>
                        
                        <div class="flex items-center gap-4 text-sm">
                            <span class="flex items-center gap-2 text-gray-600">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                                </svg>
                                {application.applied_date.clone()}
                            </span>
                            <span class="font-semibold bg-gradient-to-r from-tiffany-dark to-blue-700 bg-clip-text text-transparent">
                                {application.salary_range.clone()}
                            </span>
                        </div>
                    </div>
                    
                    <div class="text-right space-y-3">
                        <div class=format!("px-4 py-2 rounded-xl text-sm font-semibold border bg-gradient-to-r {} shadow-sm", status_gradient)>
                            {application.status.clone()}
                        </div>
                        <div class="text-right">
                            <p class="text-xs text-gray-500 mb-2">"Progress"</p>
                            <div class="w-24 bg-gray-200 rounded-full h-3 shadow-inner">
                                <div 
                                    class="bg-gradient-to-r from-tiffany-blue to-blue-600 h-3 rounded-full transition-all duration-1000 shadow-sm"
                                    style=format!("width: {}%", application.progress_percentage)
                                ></div>
                            </div>
                            <p class="text-xs font-semibold text-tiffany-dark mt-1">{application.progress_percentage}"%"</p>
                        </div>
                    </div>
                </div>
                
                <Show when=move || application.next_action.is_some()>
                    <div class="bg-gradient-to-r from-blue-50 to-cyan-50 rounded-xl p-4 mb-4 border border-blue-200/50">
                        <div class="flex items-start gap-3">
                            <div class="w-8 h-8 bg-gradient-to-br from-blue-500 to-cyan-600 rounded-lg flex items-center justify-center flex-shrink-0">
                                <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                            </div>
                            <div>
                                <p class="text-sm font-semibold text-blue-900 mb-1">"Next Action Required"</p>
                                <p class="text-sm text-blue-700">{application.next_action.unwrap_or_default()}</p>
                            </div>
                        </div>
                    </div>
                </Show>
                
                <div class="flex items-center justify-between pt-4 border-t border-gray-100">
                    <div class="flex items-center gap-4 text-xs text-gray-500">
                        <span class="flex items-center gap-1">
                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                            "Response: " {application.estimated_response_time.clone()}
                        </span>
                        <Show when=move || application.contact_person.is_some()>
                            <span class="flex items-center gap-1">
                                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                                </svg>
                                {application.contact_person.unwrap_or_default()}
                            </span>
                        </Show>
                    </div>
                    <div class="flex gap-2">
                        <button class="btn-magical text-xs px-4 py-2 rounded-lg">
                            "View Details"
                        </button>
                        <button class="btn btn-secondary text-xs px-4 py-2 rounded-lg hover:bg-gray-100 transition-colors">
                            "Follow Up"
                        </button>
                    </div>
                </div>
                
                // Animated background elements
                <div class="absolute inset-0 opacity-5 pointer-events-none">
                    <div class="absolute top-4 right-4 w-16 h-16 bg-gradient-to-br from-tiffany-blue to-purple-500 rounded-full blur-xl"></div>
                    <div class="absolute bottom-4 left-4 w-12 h-12 bg-gradient-to-tr from-pink-400 to-orange-400 rounded-full blur-lg"></div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SmartRecommendationCard(recommendation: JobRecommendation) -> impl IntoView {
    view! {
        <div class="job-card p-4 hover:scale-105 transition-all cursor-pointer">
            <div class="flex justify-between items-start mb-3">
                <div class="flex-1">
                    <h4 class="font-semibold text-gray-900">{recommendation.title.clone()}</h4>
                    <p class="text-sm text-gray-600 mt-1">{recommendation.company.clone()}</p>
                    <p class="text-sm text-gray-500 flex items-center gap-1 mt-1">
                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                        </svg>
                        {recommendation.location.clone()}
                    </p>
                </div>
                <div class="text-center">
                    <div class=format!(
                        "w-12 h-12 rounded-full flex items-center justify-center text-white font-bold text-sm {}",
                        if recommendation.match_score >= 90.0 { "bg-green-500" }
                        else if recommendation.match_score >= 80.0 { "bg-blue-500" }
                        else { "bg-purple-500" }
                    )>
                        {format!("{:.0}%", recommendation.match_score)}
                    </div>
                    <p class="text-xs text-gray-500 mt-1">"Match"</p>
                </div>
            </div>
            
            <div class="flex items-center gap-2 mb-3">
                <Show when=move || recommendation.remote_option>
                    <span class="badge-tiffany text-xs">"Remote"</span>
                </Show>
                <span class="text-xs text-gray-600">{recommendation.job_type.clone()}</span>
            </div>
            
            <div class="flex justify-between items-center text-sm">
                <span class="font-medium text-gray-700">
                    {format!("${}-${}k", recommendation.salary_min / 1000, recommendation.salary_max / 1000)}
                </span>
                <span class="text-gray-500">{recommendation.posted_date.clone()}</span>
            </div>
            
            <div class="mt-3 pt-3 border-t border-gray-100">
                <p class="text-xs text-gray-600">
                    "Skills match: " {recommendation.skills_match.join(", ")}
                </p>
            </div>
        </div>
    }
}

#[component]
fn MarketInsightCard(insight: MarketInsight) -> impl IntoView {
    view! {
        <div class="text-center p-4 bg-gray-50 rounded-xl hover:bg-gray-100 transition-colors">
            <div class="text-2xl mb-2">{insight.icon.clone()}</div>
            <p class="text-2xl font-bold text-gray-900 mb-1">{insight.value.clone()}</p>
            <p class="text-sm font-medium text-gray-700 mb-1">{insight.label.clone()}</p>
            <p class="text-xs text-gray-500">{insight.context.clone()}</p>
            <div class="flex items-center justify-center gap-1 mt-2">
                <span class=format!(
                    "text-xs font-medium {}",
                    if insight.trend == "up" { "text-green-600" } else { "text-red-600" }
                )>
                    {if insight.trend == "up" { "↗" } else { "↘" }} {format!("{:.1}%", insight.change)}
                </span>
            </div>
        </div>
    }
}

#[component]
fn DetailedMarketInsightCard(insight: MarketInsight) -> impl IntoView {
    view! {
        <div class="glass bg-white/90 rounded-2xl p-6 hover:scale-105 transition-transform">
            <div class="flex items-center justify-between mb-4">
                <span class="text-3xl">{insight.icon.clone()}</span>
                <span class=format!(
                    "text-sm font-medium px-2 py-1 rounded-full {}",
                    if insight.trend == "up" { "text-green-600 bg-green-100" } else { "text-red-600 bg-red-100" }
                )>
                    {if insight.trend == "up" { "↗" } else { "↘" }} {format!("{:.1}%", insight.change)}
                </span>
            </div>
            <p class="text-2xl font-bold text-gray-900 mb-1">{insight.value.clone()}</p>
            <p class="text-sm font-medium text-gray-700 mb-2">{insight.label.clone()}</p>
            <p class="text-xs text-gray-500 mb-3">{insight.context.clone()}</p>
            <div class="bg-blue-50 rounded-lg p-3">
                <p class="text-xs font-medium text-blue-900">Recommendation:</p>
                <p class="text-xs text-blue-700">{insight.recommendation.clone()}</p>
            </div>
        </div>
    }
}

#[component]
fn ActivityFeedCard(activity: ActivityFeedItem) -> impl IntoView {
    view! {
        <div class="flex items-start gap-3 p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors">
            <div class=format!(
                "w-8 h-8 rounded-full flex items-center justify-center text-white text-sm {}",
                match activity.priority.as_str() {
                    "high" => "bg-red-500",
                    "medium" => "bg-yellow-500",
                    _ => "bg-green-500",
                }
            )>
                {match activity.activity_type.as_str() {
                    "application" => "📝",
                    "interview" => "🎯",
                    "response" => "📬",
                    "job_alert" => "🔔",
                    _ => "📊",
                }}
            </div>
            <div class="flex-1 min-w-0">
                <p class="font-medium text-gray-900 text-sm">{activity.title.clone()}</p>
                <p class="text-xs text-gray-600 mt-1">{activity.description.clone()}</p>
                <p class="text-xs text-gray-500 mt-2">{activity.timestamp.clone()}</p>
            </div>
            <Show when=move || activity.action_required>
                <span class="w-2 h-2 bg-red-500 rounded-full animate-pulse"></span>
            </Show>
        </div>
    }
}

#[component]
fn QuickActionCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    href: &'static str,
    color: &'static str,
) -> impl IntoView {
    view! {
        <A href=href class="group">
            <div class="glass bg-white/80 backdrop-blur-xl rounded-2xl p-6 hover:scale-105 transition-all cursor-pointer">
                <div class=format!(
                    "w-12 h-12 bg-gradient-to-r {} rounded-xl flex items-center justify-center text-white text-2xl mb-4 group-hover:scale-110 transition-transform",
                    color
                )>
                    {icon}
                </div>
                <h3 class="font-semibold text-gray-900 mb-2">{title}</h3>
                <p class="text-sm text-gray-600">{description}</p>
            </div>
        </A>
    }
}

#[component]
fn CareerGoalCard(goal: CareerGoal) -> impl IntoView {
    view! {
        <div class="glass bg-white/80 backdrop-blur-xl rounded-2xl p-6">
            <div class="flex justify-between items-start mb-4">
                <div>
                    <h3 class="font-bold text-lg text-gray-900">{goal.title.clone()}</h3>
                    <p class="text-gray-600 mt-1">{goal.description.clone()}</p>
                </div>
                <span class=format!(
                    "px-3 py-1 rounded-full text-sm font-medium {}",
                    match goal.priority.as_str() {
                        "high" => "bg-red-100 text-red-800",
                        "medium" => "bg-yellow-100 text-yellow-800",
                        _ => "bg-green-100 text-green-800",
                    }
                )>
                    {goal.priority.clone()}
                </span>
            </div>
            
            <div class="mb-4">
                <div class="flex justify-between items-center mb-2">
                    <span class="text-sm text-gray-600">Progress</span>
                    <span class="text-sm font-medium">{format!("{:.0}%", goal.progress)}</span>
                </div>
                <div class="w-full bg-gray-200 rounded-full h-3">
                    <div 
                        class="bg-gradient-to-r from-tiffany-blue to-tiffany-dark h-3 rounded-full transition-all duration-500"
                        style=format!("width: {}%", goal.progress)
                    ></div>
                </div>
            </div>
            
            <div class="mb-4">
                <p class="text-sm font-medium text-gray-700 mb-2">Target Date:</p>
                <p class="text-sm text-gray-600">{goal.target_date.clone()}</p>
            </div>
            
            <div class="mb-4">
                <p class="text-sm font-medium text-gray-700 mb-2">Next Steps:</p>
                <ul class="space-y-1">
                    {goal.next_steps.iter().take(3).map(|step| {
                        view! {
                            <li class="text-sm text-gray-600 flex items-center gap-2">
                                <span class="w-1 h-1 bg-tiffany-blue rounded-full"></span>
                                {step.clone()}
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>
            
            <button class="btn btn-primary w-full rounded-lg py-2">
                "Update Goal"
            </button>
        </div>
    }
}

// Utility Functions

fn get_time_of_day() -> &'static str {
    let hour = chrono::Local::now().hour();
    match hour {
        5..=11 => "morning",
        12..=17 => "afternoon",
        18..=21 => "evening",
        _ => "night",
    }
}

// Data Generation Functions

fn generate_dashboard_metrics() -> DashboardMetrics {
    DashboardMetrics {
        total_applications: 27,
        applications_this_week: 5,
        pending_interviews: 3,
        job_alerts: 12,
        profile_completion: 85.5,
        avg_response_time: 4.2,
        success_rate: 78.5,
        saved_jobs: 15,
        viewed_jobs: 142,
        application_conversion_rate: 23.5,
    }
}

fn generate_sample_applications() -> Vec<ApplicationStatus> {
    vec![
        ApplicationStatus {
            id: "app-1".to_string(),
            job_title: "Senior Clinical Pharmacist".to_string(),
            company: "Royal Prince Alfred Hospital".to_string(),
            applied_date: "2 days ago".to_string(),
            status: "Interview Scheduled".to_string(),
            status_type: "success".to_string(),
            next_action: Some("Prepare for interview tomorrow at 2 PM".to_string()),
            salary_range: "$110k - $130k".to_string(),
            location: "Camperdown, NSW".to_string(),
            urgency: "high".to_string(),
            progress_percentage: 75,
            estimated_response_time: "Today".to_string(),
            contact_person: Some("Dr. Sarah Chen".to_string()),
            interview_date: Some("Tomorrow, 2:00 PM".to_string()),
        },
        ApplicationStatus {
            id: "app-2".to_string(),
            job_title: "Hospital Pharmacist".to_string(),
            company: "St Vincent's Private Hospital".to_string(),
            applied_date: "5 days ago".to_string(),
            status: "Under Review".to_string(),
            status_type: "pending".to_string(),
            next_action: Some("Await response from HR department".to_string()),
            salary_range: "$95k - $115k".to_string(),
            location: "Darlinghurst, NSW".to_string(),
            urgency: "medium".to_string(),
            progress_percentage: 45,
            estimated_response_time: "2-3 days".to_string(),
            contact_person: Some("Jennifer Martinez".to_string()),
            interview_date: None,
        },
        ApplicationStatus {
            id: "app-3".to_string(),
            job_title: "Community Pharmacist".to_string(),
            company: "TerryWhite Chemmart".to_string(),
            applied_date: "1 week ago".to_string(),
            status: "Application Sent".to_string(),
            status_type: "info".to_string(),
            next_action: Some("Follow up if no response by Friday".to_string()),
            salary_range: "$80k - $95k".to_string(),
            location: "Parramatta, NSW".to_string(),
            urgency: "low".to_string(),
            progress_percentage: 25,
            estimated_response_time: "1 week".to_string(),
            contact_person: None,
            interview_date: None,
        },
    ]
}

fn generate_job_recommendations() -> Vec<JobRecommendation> {
    vec![
        JobRecommendation {
            id: "rec-1".to_string(),
            title: "Senior Clinical Pharmacist - Oncology".to_string(),
            company: "Westmead Hospital".to_string(),
            location: "Westmead, NSW".to_string(),
            match_score: 95.5,
            salary_min: 115000,
            salary_max: 140000,
            job_type: "Full-time".to_string(),
            posted_date: "3 hours ago".to_string(),
            applications_count: 2,
            urgency: "high".to_string(),
            remote_option: false,
            benefits_count: 8,
            employer_rating: 4.8,
            growth_potential: "High".to_string(),
            skills_match: vec!["Clinical Experience".to_string(), "Oncology".to_string(), "Leadership".to_string()],
        },
        JobRecommendation {
            id: "rec-2".to_string(),
            title: "Industrial Pharmacist - Regulatory Affairs".to_string(),
            company: "Pfizer Australia".to_string(),
            location: "North Ryde, NSW".to_string(),
            match_score: 88.2,
            salary_min: 120000,
            salary_max: 145000,
            job_type: "Full-time".to_string(),
            posted_date: "1 day ago".to_string(),
            applications_count: 5,
            urgency: "medium".to_string(),
            remote_option: true,
            benefits_count: 12,
            employer_rating: 4.9,
            growth_potential: "Very High".to_string(),
            skills_match: vec!["Regulatory".to_string(), "Research".to_string(), "Quality Assurance".to_string()],
        },
        JobRecommendation {
            id: "rec-3".to_string(),
            title: "Compounding Pharmacist Specialist".to_string(),
            company: "Specialist Compounding Pharmacy".to_string(),
            location: "Double Bay, NSW".to_string(),
            match_score: 82.7,
            salary_min: 95000,
            salary_max: 115000,
            job_type: "Full-time".to_string(),
            posted_date: "2 days ago".to_string(),
            applications_count: 8,
            urgency: "medium".to_string(),
            remote_option: false,
            benefits_count: 6,
            employer_rating: 4.6,
            growth_potential: "Medium".to_string(),
            skills_match: vec!["Compounding".to_string(), "Formulation".to_string(), "Quality Control".to_string()],
        },
    ]
}

fn generate_market_insights() -> Vec<MarketInsight> {
    vec![
        MarketInsight {
            label: "Average Salary".to_string(),
            value: "$102,000".to_string(),
            change: 12.5,
            trend: "up".to_string(),
            period: "vs last year".to_string(),
            context: "Hospital pharmacists".to_string(),
            icon: "💰".to_string(),
            recommendation: "Consider negotiating higher base salary for hospital roles".to_string(),
        },
        MarketInsight {
            label: "Job Growth".to_string(),
            value: "+18%".to_string(),
            change: 5.2,
            trend: "up".to_string(),
            period: "last 6 months".to_string(),
            context: "Clinical positions".to_string(),
            icon: "📈".to_string(),
            recommendation: "Strong demand for clinical expertise - excellent time to apply".to_string(),
        },
        MarketInsight {
            label: "Competition".to_string(),
            value: "7.2".to_string(),
            change: -8.3,
            trend: "down".to_string(),
            period: "applications per job".to_string(),
            context: "Sydney metro area".to_string(),
            icon: "👥".to_string(),
            recommendation: "Lower competition means better chances of success".to_string(),
        },
        MarketInsight {
            label: "Response Time".to_string(),
            value: "4.8 days".to_string(),
            change: -15.2,
            trend: "up".to_string(),
            period: "average".to_string(),
            context: "Employer response".to_string(),
            icon: "⏱️".to_string(),
            recommendation: "Faster responses indicate urgent hiring needs".to_string(),
        },
    ]
}

fn generate_activity_feed() -> Vec<ActivityFeedItem> {
    vec![
        ActivityFeedItem {
            id: "activity-1".to_string(),
            title: "New job match found".to_string(),
            description: "Senior Clinical Pharmacist at Westmead Hospital matches your criteria".to_string(),
            timestamp: "5 minutes ago".to_string(),
            activity_type: "job_alert".to_string(),
            priority: "high".to_string(),
            action_required: true,
            related_job_id: Some("job-123".to_string()),
            metadata: HashMap::new(),
        },
        ActivityFeedItem {
            id: "activity-2".to_string(),
            title: "Interview reminder".to_string(),
            description: "Interview with Royal Prince Alfred Hospital tomorrow at 2 PM".to_string(),
            timestamp: "2 hours ago".to_string(),
            activity_type: "interview".to_string(),
            priority: "high".to_string(),
            action_required: true,
            related_job_id: Some("job-456".to_string()),
            metadata: HashMap::new(),
        },
        ActivityFeedItem {
            id: "activity-3".to_string(),
            title: "Application status updated".to_string(),
            description: "St Vincent's Hospital application moved to 'Under Review'".to_string(),
            timestamp: "4 hours ago".to_string(),
            activity_type: "application".to_string(),
            priority: "medium".to_string(),
            action_required: false,
            related_job_id: Some("job-789".to_string()),
            metadata: HashMap::new(),
        },
        ActivityFeedItem {
            id: "activity-4".to_string(),
            title: "Profile viewed".to_string(),
            description: "Your profile was viewed by 3 employers today".to_string(),
            timestamp: "6 hours ago".to_string(),
            activity_type: "profile".to_string(),
            priority: "low".to_string(),
            action_required: false,
            related_job_id: None,
            metadata: HashMap::new(),
        },
        ActivityFeedItem {
            id: "activity-5".to_string(),
            title: "Market update".to_string(),
            description: "Hospital pharmacist salaries increased by 12% this quarter".to_string(),
            timestamp: "1 day ago".to_string(),
            activity_type: "market".to_string(),
            priority: "low".to_string(),
            action_required: false,
            related_job_id: None,
            metadata: HashMap::new(),
        },
    ]
}

fn generate_career_goals() -> Vec<CareerGoal> {
    vec![
        CareerGoal {
            id: "goal-1".to_string(),
            title: "Obtain Clinical Pharmacist Certification".to_string(),
            description: "Complete advanced clinical training and certification program".to_string(),
            target_date: "June 2025".to_string(),
            progress: 65.0,
            milestones: vec![
                "Enroll in certification program".to_string(),
                "Complete theoretical modules".to_string(),
                "Finish practical placements".to_string(),
                "Pass certification exam".to_string(),
            ],
            next_steps: vec![
                "Complete Module 3: Pharmacokinetics".to_string(),
                "Schedule practical placement at RPA".to_string(),
                "Review exam preparation materials".to_string(),
            ],
            priority: "high".to_string(),
        },
        CareerGoal {
            id: "goal-2".to_string(),
            title: "Secure Senior Hospital Position".to_string(),
            description: "Land a senior pharmacist role at a major teaching hospital".to_string(),
            target_date: "December 2024".to_string(),
            progress: 40.0,
            milestones: vec![
                "Update resume and portfolio".to_string(),
                "Complete 5 applications".to_string(),
                "Attend networking events".to_string(),
                "Receive job offer".to_string(),
            ],
            next_steps: vec![
                "Apply to Westmead Hospital position".to_string(),
                "Prepare for upcoming interviews".to_string(),
                "Get references from current manager".to_string(),
            ],
            priority: "high".to_string(),
        },
        CareerGoal {
            id: "goal-3".to_string(),
            title: "Complete Masters in Clinical Pharmacy".to_string(),
            description: "Pursue advanced degree to enhance clinical expertise".to_string(),
            target_date: "December 2026".to_string(),
            progress: 15.0,
            milestones: vec![
                "Research universities and programs".to_string(),
                "Submit applications".to_string(),
                "Complete coursework".to_string(),
                "Submit thesis".to_string(),
            ],
            next_steps: vec![
                "Contact University of Sydney admissions".to_string(),
                "Prepare application documents".to_string(),
                "Secure academic references".to_string(),
            ],
            priority: "medium".to_string(),
        },
        CareerGoal {
            id: "goal-4".to_string(),
            title: "Build Professional Network".to_string(),
            description: "Establish connections within the pharmacy community".to_string(),
            target_date: "Ongoing".to_string(),
            progress: 30.0,
            milestones: vec![
                "Join PSA and other professional bodies".to_string(),
                "Attend 6 industry conferences".to_string(),
                "Connect with 50 professionals".to_string(),
                "Speak at industry event".to_string(),
            ],
            next_steps: vec![
                "Register for SHPA Annual Conference".to_string(),
                "Join local pharmacist networking group".to_string(),
                "Update LinkedIn profile".to_string(),
            ],
            priority: "medium".to_string(),
        },
    ]
}

// Enhanced CSS for beautiful animations and effects
const ENHANCED_DASHBOARD_CSS: &str = r#"
/* Advanced Gradient Animations */
@keyframes gradient-x {
    0%, 100% {
        background-size: 200% 200%;
        background-position: left center;
    }
    50% {
        background-size: 200% 200%;
        background-position: right center;
    }
}

@keyframes wave {
    0%, 100% { transform: rotate(0deg); }
    10%, 30%, 50%, 70%, 90% { transform: rotate(-10deg); }
    20%, 40%, 60%, 80% { transform: rotate(10deg); }
}

@keyframes float {
    0%, 100% { transform: translateY(0px); }
    50% { transform: translateY(-10px); }
}

@keyframes shimmer {
    0% { background-position: -200% 0; }
    100% { background-position: 200% 0; }
}

@keyframes glow {
    0%, 100% { box-shadow: 0 0 5px rgba(23, 221, 184, 0.2); }
    50% { box-shadow: 0 0 20px rgba(23, 221, 184, 0.4), 0 0 30px rgba(23, 221, 184, 0.2); }
}

@keyframes pulse-gentle {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.8; }
}

@keyframes slide-up {
    from {
        opacity: 0;
        transform: translateY(30px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

@keyframes scale-in {
    from {
        opacity: 0;
        transform: scale(0.9);
    }
    to {
        opacity: 1;
        transform: scale(1);
    }
}

@keyframes fade-in-up {
    from {
        opacity: 0;
        transform: translateY(20px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

/* Enhanced Utility Classes */
.animate-gradient-x {
    animation: gradient-x 6s ease infinite;
    background-size: 200% 200%;
}

.animate-wave {
    animation: wave 2s ease-in-out infinite;
    transform-origin: 70% 70%;
}

.animate-float {
    animation: float 3s ease-in-out infinite;
}

.animate-shimmer {
    background: linear-gradient(110deg, transparent 40%, rgba(255,255,255,0.5) 50%, transparent 60%);
    background-size: 200% 100%;
    animation: shimmer 2s infinite;
}

.animate-glow {
    animation: glow 2s ease-in-out infinite;
}

.animate-pulse-gentle {
    animation: pulse-gentle 3s ease-in-out infinite;
}

.animate-slide-up {
    animation: slide-up 0.6s ease-out;
}

.animate-scale-in {
    animation: scale-in 0.5s ease-out;
}

.animate-fade-in-up {
    animation: fade-in-up 0.8s ease-out;
}

/* Enhanced Glass Morphism */
.glass-ultra {
    background: rgba(255, 255, 255, 0.85);
    backdrop-filter: blur(40px);
    -webkit-backdrop-filter: blur(40px);
    border: 1px solid rgba(255, 255, 255, 0.2);
    box-shadow: 
        0 8px 32px rgba(0, 0, 0, 0.1),
        inset 0 1px 0 rgba(255, 255, 255, 0.2);
}

.glass-tiffany-ultra {
    background: rgba(23, 221, 184, 0.05);
    backdrop-filter: blur(40px);
    -webkit-backdrop-filter: blur(40px);
    border: 1px solid rgba(23, 221, 184, 0.15);
    box-shadow: 
        0 8px 32px rgba(23, 221, 184, 0.1),
        inset 0 1px 0 rgba(255, 255, 255, 0.2);
}

/* Floating Card Effects */
.floating-card {
    position: relative;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.floating-card:hover {
    transform: translateY(-8px) scale(1.02);
    box-shadow: 
        0 20px 40px rgba(0, 0, 0, 0.1),
        0 0 0 1px rgba(255, 255, 255, 0.05);
}

.floating-card::before {
    content: '';
    position: absolute;
    inset: 0;
    border-radius: inherit;
    padding: 1px;
    background: linear-gradient(135deg, rgba(23, 221, 184, 0.3), rgba(59, 130, 246, 0.3));
    mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
    mask-composite: xor;
    opacity: 0;
    transition: opacity 0.3s;
}

.floating-card:hover::before {
    opacity: 1;
}

/* Advanced Button Styles */
.btn-magical {
    position: relative;
    background: linear-gradient(135deg, #17ddb8, #3b82f6);
    border: none;
    border-radius: 12px;
    color: white;
    cursor: pointer;
    font-weight: 600;
    overflow: hidden;
    padding: 12px 24px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.btn-magical::before {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255,255,255,0.2), transparent);
    transition: left 0.5s;
}

.btn-magical:hover::before {
    left: 100%;
}

.btn-magical:hover {
    transform: translateY(-2px);
    box-shadow: 0 10px 25px rgba(23, 221, 184, 0.4);
}

/* Enhanced Color Palette */
:root {
    --tiffany-blue: #17ddb8;
    --tiffany-dark: #0d9488;
    --tiffany-light: #a7f3d0;
    --shadow-color: 220 3% 15%;
    --shadow-strength: 1%;
}

/* Performance optimizations */
.gpu-accelerated {
    transform: translateZ(0);
    will-change: transform;
}

/* Reduced motion support */
@media (prefers-reduced-motion: reduce) {
    *,
    *::before,
    *::after {
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.01ms !important;
    }
}
"#;