use leptos::*;
use leptos_router::*;
use crate::components::ui::{Button, ButtonVariant, ButtonSize};

#[component]
pub fn Team() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            // Header
            <div class="flex justify-between items-center mb-8">
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">
                        "Team Management"
                    </h1>
                    <p class="text-gray-600 mt-2">
                        "Manage your pharmacy team and collaborate on hiring"
                    </p>
                </div>
                <Button variant=ButtonVariant::Primary size=ButtonSize::Medium>
                    "Invite Team Member"
                </Button>
            </div>

            // Team Overview
            <div class="grid md:grid-cols-3 gap-6 mb-8">
                <OverviewCard
                    icon="👥"
                    title="Total Members"
                    value="12"
                    subtext="Active team members"
                />
                <OverviewCard
                    icon="📋"
                    title="Open Positions"
                    value="3"
                    subtext="Currently hiring"
                />
                <OverviewCard
                    icon="📊"
                    title="Applications"
                    value="48"
                    subtext="This month"
                />
            </div>

            // Team Members
            <div class="bg-white rounded-lg shadow-md p-6 mb-8">
                <h2 class="text-xl font-semibold text-gray-900 mb-6">
                    "Team Members"
                </h2>
                <div class="overflow-x-auto">
                    <table class="min-w-full divide-y divide-gray-200">
                        <thead>
                            <tr>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                    "Name"
                                </th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                    "Role"
                                </th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                    "Department"
                                </th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                    "Status"
                                </th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                    "Actions"
                                </th>
                            </tr>
                        </thead>
                        <tbody class="bg-white divide-y divide-gray-200">
                            <TeamMemberRow
                                name="Dr. Michael Chen"
                                role="Pharmacy Manager"
                                department="Management"
                                status="Active"
                                avatar="MC"
                            />
                            <TeamMemberRow
                                name="Sarah Johnson"
                                role="Senior Pharmacist"
                                department="Clinical"
                                status="Active"
                                avatar="SJ"
                            />
                            <TeamMemberRow
                                name="Emma Wilson"
                                role="Pharmacist"
                                department="Retail"
                                status="Active"
                                avatar="EW"
                            />
                            <TeamMemberRow
                                name="James Brown"
                                role="Pharmacy Technician"
                                department="Operations"
                                status="On Leave"
                                avatar="JB"
                            />
                            <TeamMemberRow
                                name="Lisa Anderson"
                                role="Intern Pharmacist"
                                department="Clinical"
                                status="Active"
                                avatar="LA"
                            />
                        </tbody>
                    </table>
                </div>
            </div>

            // Hiring Pipeline
            <div class="bg-white rounded-lg shadow-md p-6">
                <h2 class="text-xl font-semibold text-gray-900 mb-6">
                    "Hiring Pipeline"
                </h2>
                <div class="space-y-4">
                    <HiringPosition
                        title="Clinical Pharmacist"
                        department="Clinical"
                        applications="15"
                        stage="Interviewing"
                        posted="2 weeks ago"
                    />
                    <HiringPosition
                        title="Pharmacy Technician"
                        department="Operations"
                        applications="8"
                        stage="Screening"
                        posted="3 days ago"
                    />
                    <HiringPosition
                        title="Retail Pharmacist"
                        department="Retail"
                        applications="25"
                        stage="Review"
                        posted="1 week ago"
                    />
                </div>
                <div class="mt-6">
                    <A href="/jobs/create">
                        <Button variant=ButtonVariant::Secondary size=ButtonSize::Medium>
                            "Post New Position"
                        </Button>
                    </A>
                </div>
            </div>

            // Team Activity
            <div class="grid md:grid-cols-2 gap-8 mt-8">
                // Recent Activity
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-4">
                        "Recent Activity"
                    </h3>
                    <div class="space-y-3">
                        <ActivityItem
                            action="Interview scheduled"
                            detail="Clinical Pharmacist position"
                            user="Dr. Michael Chen"
                            time="1 hour ago"
                        />
                        <ActivityItem
                            action="Application reviewed"
                            detail="Pharmacy Technician position"
                            user="Sarah Johnson"
                            time="3 hours ago"
                        />
                        <ActivityItem
                            action="Position posted"
                            detail="Retail Pharmacist"
                            user="Emma Wilson"
                            time="Yesterday"
                        />
                    </div>
                </div>

                // Team Performance
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-4">
                        "Team Performance"
                    </h3>
                    <div class="space-y-4">
                        <PerformanceMetric
                            label="Average Time to Hire"
                            value="14 days"
                            trend="down"
                            change="-3 days"
                        />
                        <PerformanceMetric
                            label="Offer Acceptance Rate"
                            value="85%"
                            trend="up"
                            change="+5%"
                        />
                        <PerformanceMetric
                            label="Team Satisfaction"
                            value="4.2/5"
                            trend="same"
                            change="No change"
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn OverviewCard(
    icon: &'static str,
    title: &'static str,
    value: &'static str,
    subtext: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-md p-6">
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-sm text-gray-600">{title}</p>
                    <p class="text-2xl font-bold text-gray-900 mt-1">{value}</p>
                    <p class="text-sm text-gray-500 mt-1">{subtext}</p>
                </div>
                <div class="text-3xl">{icon}</div>
            </div>
        </div>
    }
}

#[component]
fn TeamMemberRow(
    name: &'static str,
    role: &'static str,
    department: &'static str,
    status: &'static str,
    avatar: &'static str,
) -> impl IntoView {
    let status_class = match status {
        "Active" => "bg-green-100 text-green-800",
        "On Leave" => "bg-yellow-100 text-yellow-800",
        _ => "bg-gray-100 text-gray-800",
    };
    
    view! {
        <tr>
            <td class="px-6 py-4 whitespace-nowrap">
                <div class="flex items-center">
                    <div class="w-10 h-10 bg-blue-500 rounded-full flex items-center justify-center text-white font-semibold">
                        {avatar}
                    </div>
                    <div class="ml-4">
                        <div class="text-sm font-medium text-gray-900">{name}</div>
                    </div>
                </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm text-gray-900">{role}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm text-gray-900">{department}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
                <span class=format!("px-2 inline-flex text-xs leading-5 font-semibold rounded-full {}", status_class)>
                    {status}
                </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                <button class="text-blue-600 hover:text-blue-900 mr-3">"Edit"</button>
                <button class="text-red-600 hover:text-red-900">"Remove"</button>
            </td>
        </tr>
    }
}

#[component]
fn HiringPosition(
    title: &'static str,
    department: &'static str,
    applications: &'static str,
    stage: &'static str,
    posted: &'static str,
) -> impl IntoView {
    let stage_class = match stage {
        "Interviewing" => "bg-purple-100 text-purple-800",
        "Screening" => "bg-blue-100 text-blue-800",
        "Review" => "bg-yellow-100 text-yellow-800",
        _ => "bg-gray-100 text-gray-800",
    };
    
    view! {
        <div class="border rounded-lg p-4">
            <div class="flex justify-between items-start">
                <div>
                    <h3 class="font-semibold text-gray-900">{title}</h3>
                    <p class="text-sm text-gray-600">{department} " • Posted " {posted}</p>
                    <p class="text-sm text-gray-500 mt-2">{applications} applications</p>
                </div>
                <span class=format!("px-3 py-1 rounded-full text-sm {}", stage_class)>
                    {stage}
                </span>
            </div>
            <div class="mt-4 flex space-x-2">
                <button class="text-sm text-blue-600 hover:text-blue-800">"View Applications"</button>
                <span class="text-gray-400">"•"</span>
                <button class="text-sm text-blue-600 hover:text-blue-800">"Edit Position"</button>
            </div>
        </div>
    }
}

#[component]
fn ActivityItem(
    action: &'static str,
    detail: &'static str,
    user: &'static str,
    time: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex justify-between items-start">
            <div>
                <p class="text-sm text-gray-900">{action}</p>
                <p class="text-sm text-gray-600">{detail}</p>
                <p class="text-xs text-gray-500 mt-1">by {user}</p>
            </div>
            <span class="text-xs text-gray-500">{time}</span>
        </div>
    }
}

#[component]
fn PerformanceMetric(
    label: &'static str,
    value: &'static str,
    trend: &'static str,
    change: &'static str,
) -> impl IntoView {
    let trend_icon = match trend {
        "up" => "↑",
        "down" => "↓",
        _ => "→",
    };
    
    let trend_class = match trend {
        "up" => "text-green-600",
        "down" => "text-red-600",
        _ => "text-gray-600",
    };
    
    view! {
        <div class="flex justify-between items-center">
            <div>
                <p class="text-sm text-gray-600">{label}</p>
                <p class="text-lg font-semibold text-gray-900">{value}</p>
            </div>
            <div class=format!("text-sm {}", trend_class)>
                <span>{trend_icon}</span>
                <span class="ml-1">{change}</span>
            </div>
        </div>
    }
}