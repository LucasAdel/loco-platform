use leptos::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::ui::{LoadingSpinner, SpinnerSize, Alert, AlertVariant, Button, ButtonVariant};
use crate::api::applications::get_application_stats;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsData {
    pub job_metrics: JobMetrics,
    pub application_metrics: ApplicationMetrics,
    pub user_metrics: UserMetrics,
    pub performance_metrics: PerformanceMetrics,
    pub financial_metrics: FinancialMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobMetrics {
    pub total_jobs: u32,
    pub active_jobs: u32,
    pub filled_jobs: u32,
    pub urgent_jobs: u32,
    pub jobs_this_week: u32,
    pub jobs_by_type: HashMap<String, u32>,
    pub jobs_by_location: HashMap<String, u32>,
    pub average_time_to_fill: f64, // days
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMetrics {
    pub total_applications: u32,
    pub pending_applications: u32,
    pub successful_applications: u32,
    pub rejected_applications: u32,
    pub applications_this_week: u32,
    pub application_success_rate: f64, // percentage
    pub average_response_time: f64, // hours
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMetrics {
    pub total_professionals: u32,
    pub total_employers: u32,
    pub active_professionals: u32,
    pub active_employers: u32,
    pub new_registrations_this_week: u32,
    pub user_engagement_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub average_page_load_time: f64, // milliseconds
    pub api_response_time: f64, // milliseconds
    pub uptime_percentage: f64,
    pub error_rate: f64, // percentage
    pub user_satisfaction_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialMetrics {
    pub revenue_this_month: f64,
    pub revenue_last_month: f64,
    pub average_job_value: f64,
    pub cost_per_acquisition: f64,
    pub lifetime_value: f64,
    pub revenue_growth_rate: f64, // percentage
}

impl Default for AnalyticsData {
    fn default() -> Self {
        Self {
            job_metrics: JobMetrics {
                total_jobs: 1247,
                active_jobs: 89,
                filled_jobs: 1158,
                urgent_jobs: 12,
                jobs_this_week: 45,
                jobs_by_type: {
                    let mut map = HashMap::new();
                    map.insert("Pharmacist".to_string(), 567);
                    map.insert("Pharmacy Assistant".to_string(), 324);
                    map.insert("Pharmacy Technician".to_string(), 198);
                    map.insert("Pharmacy Manager".to_string(), 89);
                    map.insert("Intern Pharmacist".to_string(), 69);
                    map
                },
                jobs_by_location: {
                    let mut map = HashMap::new();
                    map.insert("NSW".to_string(), 445);
                    map.insert("VIC".to_string(), 378);
                    map.insert("QLD".to_string(), 234);
                    map.insert("SA".to_string(), 89);
                    map.insert("WA".to_string(), 78);
                    map.insert("TAS".to_string(), 23);
                    map
                },
                average_time_to_fill: 8.5,
            },
            application_metrics: ApplicationMetrics {
                total_applications: 3456,
                pending_applications: 156,
                successful_applications: 1823,
                rejected_applications: 1477,
                applications_this_week: 234,
                application_success_rate: 52.7,
                average_response_time: 36.4,
            },
            user_metrics: UserMetrics {
                total_professionals: 2345,
                total_employers: 567,
                active_professionals: 1234,
                active_employers: 234,
                new_registrations_this_week: 45,
                user_engagement_score: 7.8,
            },
            performance_metrics: PerformanceMetrics {
                average_page_load_time: 1250.0,
                api_response_time: 89.0,
                uptime_percentage: 99.94,
                error_rate: 0.12,
                user_satisfaction_score: 8.4,
            },
            financial_metrics: FinancialMetrics {
                revenue_this_month: 45670.50,
                revenue_last_month: 42340.75,
                average_job_value: 156.78,
                cost_per_acquisition: 23.45,
                lifetime_value: 567.89,
                revenue_growth_rate: 7.85,
            },
        }
    }
}

#[component]
pub fn AnalyticsDashboard() -> impl IntoView {
    let (analytics_data, set_analytics_data) = create_signal(None::<AnalyticsData>);
    let (selected_timeframe, set_selected_timeframe) = create_signal("7days".to_string());
    let (is_loading, set_is_loading) = create_signal(true);
    let (error_message, set_error_message) = create_signal(None::<String>);

    // Load analytics data
    create_effect(move |_| {
        let timeframe = selected_timeframe.get();
        set_is_loading.set(true);
        
        spawn_local(async move {
            // Simulate API call delay
            gloo_timers::future::TimeoutFuture::new(1000).await;
            
            // For now, use mock data
            let data = AnalyticsData::default();
            set_analytics_data.set(Some(data));
            set_is_loading.set(false);
        });
    });

    // Refresh data
    let refresh_data = move |_| {
        set_is_loading.set(true);
        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(500).await;
            let data = AnalyticsData::default();
            set_analytics_data.set(Some(data));
            set_is_loading.set(false);
        });
    };

    view! {
        <div class="space-y-6">
            // Header
            <div class="bg-white rounded-lg shadow-sm p-6">
                <div class="flex items-center justify-between">
                    <div>
                        <h1 class="text-2xl font-bold text-gray-900">
                            "Analytics Dashboard"
                        </h1>
                        <p class="text-gray-600 mt-1">
                            "Comprehensive insights into your pharmacy job platform"
                        </p>
                    </div>
                    
                    <div class="flex items-center gap-4">
                        // Timeframe selector
                        <select
                            class="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_selected_timeframe.set(value);
                            }
                        >
                            <option value="7days">"Last 7 days"</option>
                            <option value="30days">"Last 30 days"</option>
                            <option value="90days">"Last 90 days"</option>
                            <option value="1year">"Last year"</option>
                        </select>
                        
                        // Refresh button
                        <Button 
                            variant=ButtonVariant::Secondary
                            on_click=refresh_data
                            disabled=is_loading
                        >
                            {move || if is_loading.get() {
                                view! {
                                    <div class="flex items-center">
                                        <LoadingSpinner size=SpinnerSize::Small />
                                        <span class="ml-2">"Refreshing..."</span>
                                    </div>
                                }.into_view()
                            } else {
                                view! { "Refresh Data" }.into_view()
                            }}
                        </Button>
                    </div>
                </div>
            </div>

            // Error message
            {move || {
                if let Some(error) = error_message.get() {
                    view! {
                        <Alert variant=AlertVariant::Error>
                            {error}
                        </Alert>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}

            // Loading state
            {move || {
                if is_loading.get() && analytics_data.get().is_none() {
                    view! {
                        <div class="flex justify-center py-12">
                            <LoadingSpinner size=SpinnerSize::Large />
                        </div>
                    }.into_view()
                } else if let Some(data) = analytics_data.get() {
                    view! {
                        <div class="space-y-6">
                            // Key Performance Indicators
                            <KPISection data=data.clone() />
                            
                            // Job Analytics
                            <JobAnalyticsSection job_metrics=data.job_metrics.clone() />
                            
                            // Application Analytics
                            <ApplicationAnalyticsSection application_metrics=data.application_metrics.clone() />
                            
                            // User Analytics
                            <UserAnalyticsSection user_metrics=data.user_metrics.clone() />
                            
                            // Performance Analytics
                            <PerformanceAnalyticsSection performance_metrics=data.performance_metrics.clone() />
                            
                            // Financial Analytics
                            <FinancialAnalyticsSection financial_metrics=data.financial_metrics.clone() />
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="text-center py-12">
                            <p class="text-gray-500">"No analytics data available"</p>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

#[component]
fn KPISection(data: AnalyticsData) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-sm p-6">
            <h2 class="text-lg font-semibold text-gray-900 mb-4">
                "Key Performance Indicators"
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <KPICard
                    title="Total Jobs"
                    value=data.job_metrics.total_jobs
                    change=Some(12.5)
                    is_positive=true
                    icon="💼"
                />
                
                <KPICard
                    title="Applications"
                    value=data.application_metrics.total_applications
                    change=Some(8.2)
                    is_positive=true
                    icon="📝"
                />
                
                <KPICard
                    title="Success Rate"
                    value=format!("{:.1}%", data.application_metrics.application_success_rate)
                    change=Some(3.1)
                    is_positive=true
                    icon="✅"
                />
                
                <KPICard
                    title="Avg. Fill Time"
                    value=format!("{:.1} days", data.job_metrics.average_time_to_fill)
                    change=Some(-1.2)
                    is_positive=true
                    icon="⏱️"
                />
            </div>
        </div>
    }
}

#[component]
fn KPICard(
    title: &'static str,
    value: impl ToString + 'static,
    change: Option<f64>,
    is_positive: bool,
    icon: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-gray-50 rounded-lg p-4">
            <div class="flex items-center justify-between mb-2">
                <span class="text-2xl">{icon}</span>
                {change.map(|ch| {
                    view! {
                        <span class={format!(
                            "text-sm font-medium {}",
                            if (ch > 0.0) == is_positive { "text-green-600" } else { "text-red-600" }
                        )}>
                            {if ch > 0.0 { "+" } else { "" }}{format!("{:.1}%", ch)}
                        </span>
                    }
                })}
            </div>
            <h3 class="text-sm font-medium text-gray-600 mb-1">{title}</h3>
            <p class="text-2xl font-bold text-gray-900">{value.to_string()}</p>
        </div>
    }
}

#[component]
fn JobAnalyticsSection(job_metrics: JobMetrics) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-sm p-6">
            <h2 class="text-lg font-semibold text-gray-900 mb-4">
                "Job Analytics"
            </h2>
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                // Job Status Distribution
                <div>
                    <h3 class="text-md font-medium text-gray-800 mb-3">
                        "Job Status Distribution"
                    </h3>
                    <div class="space-y-2">
                        <div class="flex justify-between items-center">
                            <span class="text-sm text-gray-600">"Active"</span>
                            <div class="flex items-center">
                                <div class="w-32 bg-gray-200 rounded-full h-2 mr-2">
                                    <div 
                                        class="bg-green-500 h-2 rounded-full"
                                        style:width=format!("{}%", (job_metrics.active_jobs as f64 / job_metrics.total_jobs as f64) * 100.0)
                                    ></div>
                                </div>
                                <span class="text-sm font-medium">{job_metrics.active_jobs}</span>
                            </div>
                        </div>
                        
                        <div class="flex justify-between items-center">
                            <span class="text-sm text-gray-600">"Filled"</span>
                            <div class="flex items-center">
                                <div class="w-32 bg-gray-200 rounded-full h-2 mr-2">
                                    <div 
                                        class="bg-blue-500 h-2 rounded-full"
                                        style:width=format!("{}%", (job_metrics.filled_jobs as f64 / job_metrics.total_jobs as f64) * 100.0)
                                    ></div>
                                </div>
                                <span class="text-sm font-medium">{job_metrics.filled_jobs}</span>
                            </div>
                        </div>
                        
                        <div class="flex justify-between items-center">
                            <span class="text-sm text-gray-600">"Urgent"</span>
                            <div class="flex items-center">
                                <div class="w-32 bg-gray-200 rounded-full h-2 mr-2">
                                    <div 
                                        class="bg-red-500 h-2 rounded-full"
                                        style:width=format!("{}%", (job_metrics.urgent_jobs as f64 / job_metrics.total_jobs as f64) * 100.0)
                                    ></div>
                                </div>
                                <span class="text-sm font-medium">{job_metrics.urgent_jobs}</span>
                            </div>
                        </div>
                    </div>
                </div>
                
                // Job Types Distribution
                <div>
                    <h3 class="text-md font-medium text-gray-800 mb-3">
                        "Job Types Distribution"
                    </h3>
                    <div class="space-y-2">
                        {job_metrics.jobs_by_type
                            .into_iter()
                            .map(|(job_type, count)| {
                                let percentage = (count as f64 / job_metrics.total_jobs as f64) * 100.0;
                                view! {
                                    <div class="flex justify-between items-center">
                                        <span class="text-sm text-gray-600">{job_type}</span>
                                        <div class="flex items-center">
                                            <div class="w-24 bg-gray-200 rounded-full h-2 mr-2">
                                                <div 
                                                    class="bg-purple-500 h-2 rounded-full"
                                                    style:width=format!("{}%", percentage)
                                                ></div>
                                            </div>
                                            <span class="text-sm font-medium">{count}</span>
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ApplicationAnalyticsSection(application_metrics: ApplicationMetrics) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-sm p-6">
            <h2 class="text-lg font-semibold text-gray-900 mb-4">
                "Application Analytics"
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div class="text-center">
                    <div class="text-3xl font-bold text-blue-600 mb-1">
                        {application_metrics.total_applications}
                    </div>
                    <div class="text-sm text-gray-600">"Total Applications"</div>
                </div>
                
                <div class="text-center">
                    <div class="text-3xl font-bold text-green-600 mb-1">
                        {format!("{:.1}%", application_metrics.application_success_rate)}
                    </div>
                    <div class="text-sm text-gray-600">"Success Rate"</div>
                </div>
                
                <div class="text-center">
                    <div class="text-3xl font-bold text-orange-600 mb-1">
                        {format!("{:.1}h", application_metrics.average_response_time)}
                    </div>
                    <div class="text-sm text-gray-600">"Avg Response Time"</div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn UserAnalyticsSection(user_metrics: UserMetrics) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-sm p-6">
            <h2 class="text-lg font-semibold text-gray-900 mb-4">
                "User Analytics"
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <div class="text-center">
                    <div class="text-2xl font-bold text-blue-600 mb-1">
                        {user_metrics.total_professionals}
                    </div>
                    <div class="text-sm text-gray-600">"Total Professionals"</div>
                </div>
                
                <div class="text-center">
                    <div class="text-2xl font-bold text-green-600 mb-1">
                        {user_metrics.total_employers}
                    </div>
                    <div class="text-sm text-gray-600">"Total Employers"</div>
                </div>
                
                <div class="text-center">
                    <div class="text-2xl font-bold text-purple-600 mb-1">
                        {user_metrics.new_registrations_this_week}
                    </div>
                    <div class="text-sm text-gray-600">"New This Week"</div>
                </div>
                
                <div class="text-center">
                    <div class="text-2xl font-bold text-orange-600 mb-1">
                        {format!("{:.1}/10", user_metrics.user_engagement_score)}
                    </div>
                    <div class="text-sm text-gray-600">"Engagement Score"</div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn PerformanceAnalyticsSection(performance_metrics: PerformanceMetrics) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-sm p-6">
            <h2 class="text-lg font-semibold text-gray-900 mb-4">
                "Performance Analytics"
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <div class="text-center">
                    <div class="text-2xl font-bold text-green-600 mb-1">
                        {format!("{:.2}%", performance_metrics.uptime_percentage)}
                    </div>
                    <div class="text-sm text-gray-600">"Uptime"</div>
                </div>
                
                <div class="text-center">
                    <div class="text-2xl font-bold text-blue-600 mb-1">
                        {format!("{:.0}ms", performance_metrics.api_response_time)}
                    </div>
                    <div class="text-sm text-gray-600">"API Response Time"</div>
                </div>
                
                <div class="text-center">
                    <div class="text-2xl font-bold text-orange-600 mb-1">
                        {format!("{:.1}/10", performance_metrics.user_satisfaction_score)}
                    </div>
                    <div class="text-sm text-gray-600">"User Satisfaction"</div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn FinancialAnalyticsSection(financial_metrics: FinancialMetrics) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-sm p-6">
            <h2 class="text-lg font-semibold text-gray-900 mb-4">
                "Financial Analytics"
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <div class="text-center">
                    <div class="text-2xl font-bold text-green-600 mb-1">
                        {"$"}{format!("{:.0}", financial_metrics.revenue_this_month)}
                    </div>
                    <div class="text-sm text-gray-600">"Revenue This Month"</div>
                    <div class="text-xs text-green-600 mt-1">
                        {format!("+{:.1}%", financial_metrics.revenue_growth_rate)}
                    </div>
                </div>
                
                <div class="text-center">
                    <div class="text-2xl font-bold text-blue-600 mb-1">
                        {"$"}{format!("{:.2}", financial_metrics.average_job_value)}
                    </div>
                    <div class="text-sm text-gray-600">"Avg Job Value"</div>
                </div>
                
                <div class="text-center">
                    <div class="text-2xl font-bold text-purple-600 mb-1">
                        {"$"}{format!("{:.2}", financial_metrics.lifetime_value)}
                    </div>
                    <div class="text-sm text-gray-600">"Customer LTV"</div>
                </div>
            </div>
        </div>
    }
}