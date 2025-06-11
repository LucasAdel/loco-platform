use leptos::*;
use wasm_bindgen_futures::spawn_local;
use leptos::prelude::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use crate::components::{AnalyticsDashboard, LoadingSpinner, SpinnerSize, Modal, Card, Toast};
use crate::components::ui::{Button, ButtonVariant, Alert, AlertVariant};
use web_sys::{window, Storage};
use js_sys::Date;
use wasm_bindgen::JsCast;
use gloo_timers::callback::Timeout;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub last_login: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminJob {
    pub id: String,
    pub title: String,
    pub company: String,
    pub status: String,
    pub applications: u32,
    pub created_at: String,
    pub expires_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub total_users: u32,
    pub active_jobs: u32,
    pub total_employers: u32,
    pub applications_today: u32,
    pub revenue_monthly: f64,
    pub system_health: f64,
    pub storage_used: f64,
    pub api_calls_today: u32,
}

fn get_sample_metrics() -> SystemMetrics {
    SystemMetrics {
        total_users: 10482,
        active_jobs: 524,
        total_employers: 186,
        applications_today: 142,
        revenue_monthly: 45680.50,
        system_health: 98.5,
        storage_used: 67.3,
        api_calls_today: 15647,
    }
}

fn get_sample_users() -> Vec<AdminUser> {
    vec![
        AdminUser {
            id: "1".to_string(),
            name: "Sarah Johnson".to_string(),
            email: "sarah.johnson@email.com".to_string(),
            role: "Pharmacist".to_string(),
            status: "Active".to_string(),
            last_login: "2 hours ago".to_string(),
            created_at: "2024-01-15".to_string(),
        },
        AdminUser {
            id: "2".to_string(),
            name: "Michael Brown".to_string(),
            email: "m.brown@pharmacy.com.au".to_string(),
            role: "Employer".to_string(),
            status: "Active".to_string(),
            last_login: "1 day ago".to_string(),
            created_at: "2024-02-20".to_string(),
        },
        AdminUser {
            id: "3".to_string(),
            name: "Emily Davis".to_string(),
            email: "emily.davis@email.com".to_string(),
            role: "Pharmacist".to_string(),
            status: "Suspended".to_string(),
            last_login: "1 week ago".to_string(),
            created_at: "2024-03-10".to_string(),
        },
    ]
}

fn get_sample_jobs() -> Vec<AdminJob> {
    vec![
        AdminJob {
            id: "1".to_string(),
            title: "Senior Clinical Pharmacist".to_string(),
            company: "RPA Hospital".to_string(),
            status: "Active".to_string(),
            applications: 23,
            created_at: "2024-12-01".to_string(),
            expires_at: "2024-12-31".to_string(),
        },
        AdminJob {
            id: "2".to_string(),
            title: "Weekend Locum Pharmacist".to_string(),
            company: "Chemist Warehouse".to_string(),
            status: "Pending Review".to_string(),
            applications: 8,
            created_at: "2024-12-05".to_string(),
            expires_at: "2024-12-19".to_string(),
        },
    ]
}

const ENHANCED_ADMIN_CSS: &str = r#"
/* Ultra-Enhanced Admin Dashboard CSS Framework */

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

/* Enhanced Metric Cards */
.metric-card {
    position: relative;
    overflow: hidden;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.metric-card::before {
    content: '';
    position: absolute;
    top: -50%;
    left: -50%;
    width: 200%;
    height: 200%;
    background: linear-gradient(
        45deg,
        transparent,
        rgba(64, 169, 255, 0.1),
        transparent
    );
    transform: rotate(0deg);
    transition: transform 0.6s;
}

.metric-card:hover::before {
    transform: rotate(180deg);
}

/* Enhanced Data Table */
.data-table {
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(10px);
    border-radius: 12px;
    overflow: hidden;
}

.data-table th {
    background: linear-gradient(135deg, #17ddb8, #3b82f6);
    color: white;
    font-weight: 600;
    text-transform: uppercase;
    font-size: 0.875rem;
    letter-spacing: 0.05em;
}

.data-table tbody tr {
    transition: all 0.2s;
    border-bottom: 1px solid rgba(0, 0, 0, 0.05);
}

.data-table tbody tr:hover {
    background: rgba(64, 169, 255, 0.05);
    transform: translateX(2px);
}

/* Charts Enhancement */
.chart-container {
    background: rgba(255, 255, 255, 0.95);
    border-radius: 16px;
    padding: 24px;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.08);
}

/* Status Badges */
.status-badge {
    display: inline-flex;
    align-items: center;
    padding: 4px 12px;
    border-radius: 20px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.status-badge.active {
    background: linear-gradient(135deg, #10b981, #34d399);
    color: white;
}

.status-badge.pending {
    background: linear-gradient(135deg, #f59e0b, #fbbf24);
    color: white;
}

.status-badge.inactive {
    background: linear-gradient(135deg, #ef4444, #f87171);
    color: white;
}

/* Action Buttons */
.action-button {
    position: relative;
    overflow: hidden;
    transition: all 0.3s;
}

.action-button::before {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    width: 0;
    height: 0;
    background: rgba(255, 255, 255, 0.3);
    border-radius: 50%;
    transform: translate(-50%, -50%);
    transition: width 0.6s, height 0.6s;
}

.action-button:hover::before {
    width: 300px;
    height: 300px;
}

/* Quick Stats */
.quick-stat {
    position: relative;
    padding: 20px;
    background: linear-gradient(135deg, #f0f9ff, #e0f2fe);
    border-radius: 12px;
    overflow: hidden;
}

.quick-stat::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 4px;
    background: linear-gradient(90deg, #17ddb8, #3b82f6);
    transform: scaleX(0);
    transform-origin: left;
    transition: transform 0.4s;
}

.quick-stat:hover::after {
    transform: scaleX(1);
}

/* User Avatar */
.user-avatar {
    position: relative;
    display: inline-block;
}

.user-avatar::after {
    content: '';
    position: absolute;
    bottom: 0;
    right: 0;
    width: 12px;
    height: 12px;
    background: #10b981;
    border: 2px solid white;
    border-radius: 50%;
}

/* Notification Bell */
.notification-bell {
    position: relative;
    cursor: pointer;
    transition: transform 0.2s;
}

.notification-bell:hover {
    transform: rotate(-15deg);
}

.notification-bell::after {
    content: '';
    position: absolute;
    top: 0;
    right: 0;
    width: 8px;
    height: 8px;
    background: #ef4444;
    border-radius: 50%;
    animation: pulse 2s infinite;
}

/* Animations */
@keyframes pulse {
    0% {
        transform: scale(1);
        opacity: 1;
    }
    50% {
        transform: scale(1.2);
        opacity: 0.8;
    }
    100% {
        transform: scale(1);
        opacity: 1;
    }
}

@keyframes slideInRight {
    from {
        transform: translateX(100%);
        opacity: 0;
    }
    to {
        transform: translateX(0);
        opacity: 1;
    }
}

/* Enhanced Scrollbar */
::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.05);
    border-radius: 4px;
}

::-webkit-scrollbar-thumb {
    background: linear-gradient(135deg, #17ddb8, #3b82f6);
    border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
    background: linear-gradient(135deg, #14c7a5, #2563eb);
}

/* Dark Mode Support */
@media (prefers-color-scheme: dark) {
    .glass {
        background: rgba(20, 20, 30, 0.8);
        border: 1px solid rgba(255, 255, 255, 0.1);
    }
    
    .data-table {
        background: rgba(20, 20, 30, 0.9);
    }
    
    .chart-container {
        background: rgba(20, 20, 30, 0.95);
    }
    
    .quick-stat {
        background: linear-gradient(135deg, #1e293b, #334155);
    }
}

/* Responsive Enhancements */
@media (max-width: 768px) {
    .metric-card {
        margin-bottom: 16px;
    }
    
    .data-table {
        font-size: 0.875rem;
    }
    
    .chart-container {
        padding: 16px;
    }
}
"#;

#[component]
pub fn Admin() -> impl IntoView {
    // Enhanced state management
    let (active_tab, set_active_tab) = create_signal("overview".to_string());
    let (metrics_data, set_metrics_data) = create_signal(get_sample_metrics());
    let (users_data, set_users_data) = create_signal(get_sample_users());
    let (jobs_data, set_jobs_data) = create_signal(get_sample_jobs());
    let (loading, set_loading) = create_signal(false);
    let (show_user_modal, set_show_user_modal) = create_signal(false);
    let (show_job_modal, set_show_job_modal) = create_signal(false);
    let (selected_user, set_selected_user) = create_signal(None::<AdminUser>);
    let (selected_job, set_selected_job) = create_signal(None::<AdminJob>);
    let (toast_message, set_toast_message) = create_signal(None::<String>);

    // Real-time metrics update simulation
    Effect::new({
        let set_metrics_data = set_metrics_data.clone();
        move |_| {
            spawn_local(async move {
                loop {
                    gloo_timers::future::TimeoutFuture::new(30000).await; // Update every 30s
                    set_metrics_data.update(|metrics| {
                        metrics.applications_today += 1;
                        metrics.api_calls_today += rand::random::<u32>() % 100;
                        metrics.system_health = 95.0 + (rand::random::<f64>() * 5.0);
                    });
                }
            });
        }
    });

    let approve_job = move |job_id: String| {
        set_jobs_data.update(|jobs| {
            if let Some(job) = jobs.iter_mut().find(|j| j.id == job_id) {
                job.status = "Active".to_string();
            }
        });
        set_toast_message.set(Some("Job approved successfully!".to_string()));
    };

    let suspend_user = move |user_id: String| {
        set_users_data.update(|users| {
            if let Some(user) = users.iter_mut().find(|u| u.id == user_id) {
                user.status = if user.status == "Active" { "Suspended".to_string() } else { "Active".to_string() };
            }
        });
        set_toast_message.set(Some("User status updated successfully!".to_string()));
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
                                "🛡️ Admin Command Center"
                            </h1>
                            <p class="text-xl text-gray-700">
                                "Complete platform administration and analytics"
                            </p>
                            <div class="flex items-center gap-6 mt-4 text-sm text-gray-600">
                                <div class="flex items-center gap-2">
                                    <span class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></span>
                                    <span>"System Health: " {move || format!("{:.1}%", metrics_data.get().system_health)}</span>
                                </div>
                                <div class="flex items-center gap-2">
                                    <span class="w-2 h-2 bg-blue-500 rounded-full animate-pulse"></span>
                                    <span>"Live Users: " {move || metrics_data.get().total_users}</span>
                                </div>
                                <div class="flex items-center gap-2">
                                    <span class="w-2 h-2 bg-purple-500 rounded-full animate-pulse"></span>
                                    <span>"API Calls Today: " {move || metrics_data.get().api_calls_today}</span>
                                </div>
                            </div>
                        </div>
                        
                        // Quick Actions
                        <div class="flex items-center gap-4">
                            <button class="admin-quick-action-btn">
                                <span class="text-xl">"🔄"</span>
                                "Refresh Data"
                            </button>
                            <button class="admin-quick-action-btn">
                                <span class="text-xl">"📊"</span>
                                "Generate Report"
                            </button>
                            <button class="admin-quick-action-btn">
                                <span class="text-xl">"⚙️"</span>
                                "System Settings"
                            </button>
                        </div>
                    </div>
                </div>

                // Enhanced Metrics Cards
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                    <EnhancedMetricCard
                        title="Total Users"
                        value=move || metrics_data.get().total_users.to_string()
                        change="+12%"
                        positive=true
                        icon="👥"
                        color="blue"
                    />
                    <EnhancedMetricCard
                        title="Active Jobs"
                        value=move || metrics_data.get().active_jobs.to_string()
                        change="+8%"
                        positive=true
                        icon="💼"
                        color="green"
                    />
                    <EnhancedMetricCard
                        title="Monthly Revenue"
                        value=move || format!("${:.0}k", metrics_data.get().revenue_monthly / 1000.0)
                        change="+23%"
                        positive=true
                        icon="💰"
                        color="purple"
                    />
                    <EnhancedMetricCard
                        title="Applications Today"
                        value=move || metrics_data.get().applications_today.to_string()
                        change="-5%"
                        positive=false
                        icon="📝"
                        color="orange"
                    />
                </div>

                // Enhanced Navigation Tabs
                <div class="glass bg-white/90 rounded-2xl mb-8 shadow-xl">
                    <AdminTabs active_tab=active_tab set_active_tab=set_active_tab />
                </div>

                // Enhanced Content Area
                <div class="glass bg-white/95 rounded-2xl shadow-2xl overflow-hidden">
                    {move || match active_tab.get().as_str() {
                        "overview" => view! {
                            <AdminOverview 
                                metrics_data=metrics_data 
                                users_data=users_data 
                                jobs_data=jobs_data
                            />
                        }.into_view(),
                        "users" => view! {
                            <UserManagement 
                                users_data=users_data 
                                set_users_data=set_users_data
                                set_selected_user=set_selected_user
                                set_show_user_modal=set_show_user_modal
                                suspend_user=suspend_user
                            />
                        }.into_view(),
                        "jobs" => view! {
                            <JobManagement 
                                jobs_data=jobs_data 
                                set_jobs_data=set_jobs_data
                                set_selected_job=set_selected_job
                                set_show_job_modal=set_show_job_modal
                                approve_job=approve_job
                            />
                        }.into_view(),
                        "analytics" => view! {
                            <AdminAnalytics metrics_data=metrics_data />
                        }.into_view(),
                        "settings" => view! {
                            <SystemSettings />
                        }.into_view(),
                        _ => view! {
                            <AdminOverview 
                                metrics_data=metrics_data 
                                users_data=users_data 
                                jobs_data=jobs_data
                            />
                        }.into_view(),
                    }}
                </div>
            </div>

            // Enhanced Modals
            <Show when=move || show_user_modal.get()>
                <UserDetailModal 
                    user=selected_user 
                    on_close=move || set_show_user_modal.set(false)
                />
            </Show>

            <Show when=move || show_job_modal.get()>
                <JobDetailModal 
                    job=selected_job 
                    on_close=move || set_show_job_modal.set(false)
                />
            </Show>

            // Toast Notifications
            <Show when=move || toast_message.get().is_some()>
                <div class="fixed top-4 right-4 z-50">
                    <Toast 
                        message=move || toast_message.get().unwrap_or_default()
                        on_close=move || set_toast_message.set(None)
                    />
                </div>
            </Show>

            // Enhanced CSS Framework
            <style>
                {ENHANCED_ADMIN_CSS}
            </style>
        </div>
    }
}

#[component]
fn EnhancedMetricCard(
    title: &'static str,
    value: Signal<String>,
    change: &'static str,
    positive: bool,
    icon: &'static str,
    color: &'static str,
) -> impl IntoView {
    let card_class = format!("enhanced-metric-card-{}", color);
    let change_class = if positive { "positive-change" } else { "negative-change" };
    
    view! {
        <div class=format!("enhanced-metric-card {}", card_class)>
            <div class="metric-card-header">
                <div class="metric-icon">
                    <span class="text-2xl">{icon}</span>
                </div>
                <div class="metric-details">
                    <h3 class="metric-title">{title}</h3>
                    <div class="metric-value-container">
                        <p class="metric-value">{value}</p>
                        <p class=format!("metric-change {}", change_class)>{change}</p>
                    </div>
                </div>
            </div>
            <div class="metric-chart-preview">
                <div class="mini-chart"></div>
            </div>
        </div>
    }
}

#[component]
fn AdminTabs(
    active_tab: ReadSignal<String>,
    set_active_tab: WriteSignal<String>,
) -> impl IntoView {
    let tabs = vec![
        ("overview", "🏠", "Overview"),
        ("users", "👥", "Users"),
        ("jobs", "💼", "Jobs"),
        ("analytics", "📊", "Analytics"),
        ("settings", "⚙️", "Settings"),
    ];

    view! {
        <div class="admin-tabs-container">
            <div class="admin-tabs">
                {tabs.into_iter().map(|(tab_id, icon, label)| {
                    let tab_id_clone = tab_id.to_string();
                    view! {
                        <button
                            class=move || format!(
                                "admin-tab-button {}",
                                if active_tab.get() == tab_id { "active" } else { "" }
                            )
                            on:click=move |_| set_active_tab.set(tab_id_clone.clone())
                        >
                            <span class="tab-icon">{icon}</span>
                            <span class="tab-label">{label}</span>
                        </button>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn AdminOverview(
    metrics_data: ReadSignal<SystemMetrics>,
    users_data: ReadSignal<Vec<AdminUser>>,
    jobs_data: ReadSignal<Vec<AdminJob>>,
) -> impl IntoView {
    view! {
        <div class="admin-overview-content">
            <div class="overview-section">
                <h2 class="section-title">"🚀 Platform Performance"</h2>
                <div class="performance-grid">
                    <div class="performance-card">
                        <h3>"System Health"</h3>
                        <div class="health-indicator">
                            <div class="health-circle">
                                <span class="health-percentage">{move || format!("{:.1}%", metrics_data.get().system_health)}</span>
                            </div>
                        </div>
                    </div>
                    <div class="performance-card">
                        <h3>"Storage Usage"</h3>
                        <div class="storage-bar">
                            <div class="storage-fill" style:width=move || format!("{}%", metrics_data.get().storage_used)></div>
                            <span class="storage-text">{move || format!("{:.1}% Used", metrics_data.get().storage_used)}</span>
                        </div>
                    </div>
                </div>
            </div>

            <div class="overview-section">
                <h2 class="section-title">"📈 Recent Activity"</h2>
                <div class="activity-timeline">
                    <ActivityTimelineItem 
                        icon="👤"
                        title="New user registered"
                        description="sarah.johnson@email.com joined as Pharmacist"
                        time="2 minutes ago"
                        type_="success"
                    />
                    <ActivityTimelineItem 
                        icon="💼"
                        title="Job listing approved"
                        description="Senior Clinical Pharmacist - RPA Hospital"
                        time="15 minutes ago"
                        type_="info"
                    />
                    <ActivityTimelineItem 
                        icon="⚠️"
                        title="System maintenance"
                        description="Scheduled backup completed successfully"
                        time="1 hour ago"
                        type_="warning"
                    />
                </div>
            </div>

            <div class="overview-section">
                <h2 class="section-title">"🔥 Quick Actions"</h2>
                <div class="quick-actions-grid">
                    <QuickActionCard 
                        icon="👥"
                        title="Manage Users"
                        description="Review and moderate user accounts"
                        action="View Users"
                    />
                    <QuickActionCard 
                        icon="💼"
                        title="Approve Jobs"
                        description=move || format!("{} jobs pending review", jobs_data.get().iter().filter(|j| j.status == "Pending Review").count())
                        action="Review Jobs"
                    />
                    <QuickActionCard 
                        icon="📊"
                        title="Generate Report"
                        description="Create comprehensive analytics report"
                        action="Generate"
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn UserManagement(
    users_data: ReadSignal<Vec<AdminUser>>,
    set_users_data: WriteSignal<Vec<AdminUser>>,
    set_selected_user: WriteSignal<Option<AdminUser>>,
    set_show_user_modal: WriteSignal<bool>,
    suspend_user: impl Fn(String) + Copy + 'static,
) -> impl IntoView {
    view! {
        <div class="user-management-content">
            <div class="management-header">
                <h2 class="section-title">"👥 User Management"</h2>
                <div class="management-actions">
                    <button class="admin-action-btn primary">
                        <span>"+"</span>
                        "Add User"
                    </button>
                    <button class="admin-action-btn secondary">
                        <span>"📥"</span>
                        "Import Users"
                    </button>
                    <button class="admin-action-btn secondary">
                        <span>"📤"</span>
                        "Export Data"
                    </button>
                </div>
            </div>

            <div class="users-table-container">
                <table class="enhanced-admin-table">
                    <thead>
                        <tr>
                            <th>"User"</th>
                            <th>"Role"</th>
                            <th>"Status"</th>
                            <th>"Last Login"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || users_data.get().into_iter().map(|user| {
                            let user_clone = user.clone();
                            let user_id = user.id.clone();
                            view! {
                                <tr class="table-row">
                                    <td class="user-cell">
                                        <div class="user-info">
                                            <div class="user-avatar">
                                                {user.name.chars().next().unwrap_or('U').to_string()}
                                            </div>
                                            <div class="user-details">
                                                <div class="user-name">{user.name.clone()}</div>
                                                <div class="user-email">{user.email.clone()}</div>
                                            </div>
                                        </div>
                                    </td>
                                    <td>
                                        <span class="role-badge">{user.role.clone()}</span>
                                    </td>
                                    <td>
                                        <span class=format!("status-badge {}", if user.status == "Active" { "active" } else { "suspended" })>
                                            {user.status.clone()}
                                        </span>
                                    </td>
                                    <td class="last-login">{user.last_login.clone()}</td>
                                    <td class="actions-cell">
                                        <div class="action-buttons">
                                            <button 
                                                class="action-btn view"
                                                on:click=move |_| {
                                                    set_selected_user.set(Some(user_clone.clone()));
                                                    set_show_user_modal.set(true);
                                                }
                                            >
                                                "👁️"
                                            </button>
                                            <button 
                                                class="action-btn suspend"
                                                on:click=move |_| suspend_user(user_id.clone())
                                            >
                                                {if user.status == "Active" { "⏸️" } else { "▶️" }}
                                            </button>
                                        </div>
                                    </td>
                                </tr>
                            }
                        }).collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
fn JobManagement(
    jobs_data: ReadSignal<Vec<AdminJob>>,
    set_jobs_data: WriteSignal<Vec<AdminJob>>,
    set_selected_job: WriteSignal<Option<AdminJob>>,
    set_show_job_modal: WriteSignal<bool>,
    approve_job: impl Fn(String) + Copy + 'static,
) -> impl IntoView {
    view! {
        <div class="job-management-content">
            <div class="management-header">
                <h2 class="section-title">"💼 Job Management"</h2>
                <div class="management-stats">
                    <div class="stat-item">
                        <span class="stat-value">{move || jobs_data.get().len()}</span>
                        <span class="stat-label">"Total Jobs"</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-value">{move || jobs_data.get().iter().filter(|j| j.status == "Pending Review").count()}</span>
                        <span class="stat-label">"Pending Review"</span>
                    </div>
                </div>
            </div>

            <div class="jobs-grid">
                {move || jobs_data.get().into_iter().map(|job| {
                    let job_clone = job.clone();
                    let job_id = job.id.clone();
                    view! {
                        <div class="job-management-card">
                            <div class="job-card-header">
                                <div class="job-title">{job.title.clone()}</div>
                                <span class=format!("job-status-badge {}", if job.status == "Active" { "active" } else { "pending" })>
                                    {job.status.clone()}
                                </span>
                            </div>
                            <div class="job-card-content">
                                <div class="job-company">"🏢 " {job.company.clone()}</div>
                                <div class="job-applications">"📝 " {job.applications} " applications"</div>
                                <div class="job-dates">
                                    <span>"📅 Created: " {job.created_at.clone()}</span>
                                    <span>"⏰ Expires: " {job.expires_at.clone()}</span>
                                </div>
                            </div>
                            <div class="job-card-actions">
                                <button 
                                    class="job-action-btn view"
                                    on:click=move |_| {
                                        set_selected_job.set(Some(job_clone.clone()));
                                        set_show_job_modal.set(true);
                                    }
                                >
                                    "View Details"
                                </button>
                                {if job.status == "Pending Review" {
                                    view! {
                                        <button 
                                            class="job-action-btn approve"
                                            on:click=move |_| approve_job(job_id.clone())
                                        >
                                            "✅ Approve"
                                        </button>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }}
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn AdminAnalytics(metrics_data: ReadSignal<SystemMetrics>) -> impl IntoView {
    view! {
        <div class="analytics-content">
            <h2 class="section-title">"📊 Advanced Analytics"</h2>
            <div class="analytics-grid">
                <div class="analytics-card">
                    <h3>"User Growth"</h3>
                    <div class="chart-placeholder">
                        <div class="chart-bars">
                            <div class="chart-bar" style="height: 60%"></div>
                            <div class="chart-bar" style="height: 80%"></div>
                            <div class="chart-bar" style="height: 90%"></div>
                            <div class="chart-bar" style="height: 100%"></div>
                            <div class="chart-bar" style="height: 85%"></div>
                        </div>
                    </div>
                </div>
                <div class="analytics-card">
                    <h3>"Job Posting Trends"</h3>
                    <div class="trend-indicator positive">
                        <span class="trend-arrow">"↗️"</span>
                        <span class="trend-value">"+23% this month"</span>
                    </div>
                </div>
                <div class="analytics-card">
                    <h3>"Revenue Analytics"</h3>
                    <div class="revenue-display">
                        <span class="revenue-amount">{move || format!("${:.2}", metrics_data.get().revenue_monthly)}</span>
                        <span class="revenue-period">"Monthly"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SystemSettings() -> impl IntoView {
    view! {
        <div class="settings-content">
            <h2 class="section-title">"⚙️ System Settings"</h2>
            <div class="settings-sections">
                <div class="settings-section">
                    <h3>"Platform Configuration"</h3>
                    <div class="settings-group">
                        <div class="setting-item">
                            <label>"Site Name"</label>
                            <input type="text" value="Loco Platform" class="setting-input" />
                        </div>
                        <div class="setting-item">
                            <label>"Maintenance Mode"</label>
                            <input type="checkbox" class="setting-checkbox" />
                        </div>
                    </div>
                </div>
                <div class="settings-section">
                    <h3>"Security Settings"</h3>
                    <div class="settings-group">
                        <div class="setting-item">
                            <label>"Two-Factor Authentication"</label>
                            <input type="checkbox" checked=true class="setting-checkbox" />
                        </div>
                        <div class="setting-item">
                            <label>"Session Timeout (minutes)"</label>
                            <input type="number" value="30" class="setting-input" />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ActivityTimelineItem(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    time: &'static str,
    type_: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("timeline-item {}", type_)>
            <div class="timeline-icon">{icon}</div>
            <div class="timeline-content">
                <div class="timeline-title">{title}</div>
                <div class="timeline-description">{description}</div>
                <div class="timeline-time">{time}</div>
            </div>
        </div>
    }
}

#[component]
fn QuickActionCard(
    icon: &'static str,
    title: &'static str,
    description: String,
    action: &'static str,
) -> impl IntoView {
    view! {
        <div class="quick-action-card">
            <div class="action-icon">{icon}</div>
            <div class="action-content">
                <h4 class="action-title">{title}</h4>
                <p class="action-description">{description}</p>
            </div>
            <button class="action-button">{action}</button>
        </div>
    }
}

#[component]
fn UserDetailModal(
    user: ReadSignal<Option<AdminUser>>,
    on_close: impl Fn() + Copy + 'static,
) -> impl IntoView {
    view! {
        <div class="modal-overlay" on:click=move |_| on_close()>
            <div class="modal-content user-modal" on:click=|e| e.stop_propagation()>
                {move || if let Some(user_data) = user.get() {
                    view! {
                        <div class="modal-header">
                            <h2>"User Details"</h2>
                            <button class="modal-close" on:click=move |_| on_close()>"×"</button>
                        </div>
                        <div class="modal-body">
                            <div class="user-detail-grid">
                                <div class="detail-item">
                                    <label>"Name"</label>
                                    <span>{user_data.name}</span>
                                </div>
                                <div class="detail-item">
                                    <label>"Email"</label>
                                    <span>{user_data.email}</span>
                                </div>
                                <div class="detail-item">
                                    <label>"Role"</label>
                                    <span>{user_data.role}</span>
                                </div>
                                <div class="detail-item">
                                    <label>"Status"</label>
                                    <span>{user_data.status}</span>
                                </div>
                                <div class="detail-item">
                                    <label>"Last Login"</label>
                                    <span>{user_data.last_login}</span>
                                </div>
                                <div class="detail-item">
                                    <label>"Created"</label>
                                    <span>{user_data.created_at}</span>
                                </div>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }}
            </div>
        </div>
    }
}

#[component]
fn JobDetailModal(
    job: ReadSignal<Option<AdminJob>>,
    on_close: impl Fn() + Copy + 'static,
) -> impl IntoView {
    view! {
        <div class="modal-overlay" on:click=move |_| on_close()>
            <div class="modal-content job-modal" on:click=|e| e.stop_propagation()>
                {move || if let Some(job_data) = job.get() {
                    view! {
                        <div class="modal-header">
                            <h2>"Job Details"</h2>
                            <button class="modal-close" on:click=move |_| on_close()>"×"</button>
                        </div>
                        <div class="modal-body">
                            <div class="job-detail-grid">
                                <div class="detail-item">
                                    <label>"Title"</label>
                                    <span>{job_data.title}</span>
                                </div>
                                <div class="detail-item">
                                    <label>"Company"</label>
                                    <span>{job_data.company}</span>
                                </div>
                                <div class="detail-item">
                                    <label>"Status"</label>
                                    <span>{job_data.status}</span>
                                </div>
                                <div class="detail-item">
                                    <label>"Applications"</label>
                                    <span>{job_data.applications}</span>
                                </div>
                                <div class="detail-item">
                                    <label>"Created"</label>
                                    <span>{job_data.created_at}</span>
                                </div>
                                <div class="detail-item">
                                    <label>"Expires"</label>
                                    <span>{job_data.expires_at}</span>
                                </div>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }}
            </div>
        </div>
    }
}

