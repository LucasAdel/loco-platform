use leptos::*;
use leptos_router::*;
// use crate::components::ui::{Button, ButtonVariant, ButtonSize};

#[component]
pub fn Admin() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">
                "Admin Dashboard"
            </h1>

            // Quick Stats
            <div class="grid md:grid-cols-4 gap-6 mb-8">
                <StatCard
                    title="Total Users"
                    value="10,482"
                    change="+12%"
                    positive=true
                />
                <StatCard
                    title="Active Jobs"
                    value="524"
                    change="+8%"
                    positive=true
                />
                <StatCard
                    title="Total Employers"
                    value="186"
                    change="+3%"
                    positive=true
                />
                <StatCard
                    title="Applications Today"
                    value="142"
                    change="-5%"
                    positive=false
                />
            </div>

            // Admin Sections
            <div class="grid md:grid-cols-2 gap-8">
                // User Management
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-xl font-semibold text-gray-900 mb-4">
                        "User Management"
                    </h2>
                    <div class="space-y-4">
                        <AdminAction
                            icon="👥"
                            title="Manage Users"
                            description="View, edit, and manage user accounts"
                            href="/admin/users"
                        />
                        <AdminAction
                            icon="🔐"
                            title="User Permissions"
                            description="Configure roles and permissions"
                            href="/admin/permissions"
                        />
                        <AdminAction
                            icon="📊"
                            title="User Analytics"
                            description="View user activity and engagement"
                            href="/admin/analytics/users"
                        />
                    </div>
                </div>

                // Job Management
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-xl font-semibold text-gray-900 mb-4">
                        "Job Management"
                    </h2>
                    <div class="space-y-4">
                        <AdminAction
                            icon="💼"
                            title="Manage Jobs"
                            description="Review, approve, and manage job listings"
                            href="/admin/jobs"
                        />
                        <AdminAction
                            icon="🏢"
                            title="Employer Accounts"
                            description="Manage employer registrations"
                            href="/admin/employers"
                        />
                        <AdminAction
                            icon="📝"
                            title="Job Analytics"
                            description="View job posting trends and metrics"
                            href="/admin/analytics/jobs"
                        />
                    </div>
                </div>

                // System Management
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-xl font-semibold text-gray-900 mb-4">
                        "System Management"
                    </h2>
                    <div class="space-y-4">
                        <AdminAction
                            icon="⚙️"
                            title="System Settings"
                            description="Configure platform settings"
                            href="/admin/settings"
                        />
                        <AdminAction
                            icon="📧"
                            title="Email Templates"
                            description="Manage email notifications"
                            href="/admin/emails"
                        />
                        <AdminAction
                            icon="🔔"
                            title="Notifications"
                            description="Configure system notifications"
                            href="/admin/notifications"
                        />
                    </div>
                </div>

                // Reports
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-xl font-semibold text-gray-900 mb-4">
                        "Reports & Analytics"
                    </h2>
                    <div class="space-y-4">
                        <AdminAction
                            icon="📈"
                            title="Platform Reports"
                            description="Generate comprehensive reports"
                            href="/admin/reports"
                        />
                        <AdminAction
                            icon="💰"
                            title="Revenue Analytics"
                            description="Track platform revenue and growth"
                            href="/admin/revenue"
                        />
                        <AdminAction
                            icon="📊"
                            title="Export Data"
                            description="Export platform data for analysis"
                            href="/admin/export"
                        />
                    </div>
                </div>
            </div>

            // Recent Activity
            <div class="bg-white rounded-lg shadow-md p-6 mt-8">
                <h2 class="text-xl font-semibold text-gray-900 mb-4">
                    "Recent Activity"
                </h2>
                <div class="space-y-3">
                    <ActivityItem
                        action="New user registration"
                        detail="john.doe@example.com"
                        time="2 minutes ago"
                    />
                    <ActivityItem
                        action="Job listing approved"
                        detail="Senior Pharmacist - Melbourne CBD"
                        time="15 minutes ago"
                    />
                    <ActivityItem
                        action="Employer verification"
                        detail="Sydney Pharmacy Group"
                        time="1 hour ago"
                    />
                    <ActivityItem
                        action="System backup completed"
                        detail="Daily backup successful"
                        time="3 hours ago"
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn StatCard(
    title: &'static str,
    value: &'static str,
    change: &'static str,
    positive: bool,
) -> impl IntoView {
    let change_class = if positive {
        "text-green-600"
    } else {
        "text-red-600"
    };
    
    view! {
        <div class="bg-white rounded-lg shadow-md p-6">
            <h3 class="text-sm font-medium text-gray-500">{title}</h3>
            <div class="mt-2 flex items-baseline">
                <p class="text-2xl font-semibold text-gray-900">{value}</p>
                <p class=format!("ml-2 text-sm {}", change_class)>{change}</p>
            </div>
        </div>
    }
}

#[component]
fn AdminAction(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    href: &'static str,
) -> impl IntoView {
    view! {
        <A href=href class="block hover:bg-gray-50 p-3 rounded-lg transition-colors">
            <div class="flex items-start space-x-3">
                <span class="text-2xl">{icon}</span>
                <div>
                    <h3 class="font-medium text-gray-900">{title}</h3>
                    <p class="text-sm text-gray-600 mt-1">{description}</p>
                </div>
            </div>
        </A>
    }
}

#[component]
fn ActivityItem(
    action: &'static str,
    detail: &'static str,
    time: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex justify-between items-start py-2">
            <div>
                <p class="text-gray-900">{action}</p>
                <p class="text-sm text-gray-600">{detail}</p>
            </div>
            <span class="text-sm text-gray-500">{time}</span>
        </div>
    }
}