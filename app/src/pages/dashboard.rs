use leptos::*;
use leptos_router::*;
use crate::components::ui::{Button, ButtonVariant, ButtonSize};

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            // Welcome Section
            <div class="mb-8">
                <h1 class="text-3xl font-bold text-gray-900">
                    "Welcome back, Sarah!"
                </h1>
                <p class="text-gray-600 mt-2">
                    "Here's what's happening with your job search today."
                </p>
            </div>

            // Quick Actions
            <div class="grid md:grid-cols-4 gap-4 mb-8">
                <QuickAction
                    icon="🔍"
                    title="Search Jobs"
                    count="12 new"
                    href="/jobs"
                    color="bg-blue-500"
                />
                <QuickAction
                    icon="📄"
                    title="Applications"
                    count="3 pending"
                    href="/applications"
                    color="bg-green-500"
                />
                <QuickAction
                    icon="💬"
                    title="Messages"
                    count="2 unread"
                    href="/messages"
                    color="bg-purple-500"
                />
                <QuickAction
                    icon="🔔"
                    title="Notifications"
                    count="5 new"
                    href="/notifications"
                    color="bg-orange-500"
                />
            </div>

            <div class="grid md:grid-cols-3 gap-8">
                // Recent Applications
                <div class="md:col-span-2 bg-white rounded-lg shadow-md p-6">
                    <div class="flex justify-between items-center mb-4">
                        <h2 class="text-xl font-semibold text-gray-900">
                            "Recent Applications"
                        </h2>
                        <A href="/applications">
                            <Button variant=ButtonVariant::Secondary size=ButtonSize::Small>
                                "View All"
                            </Button>
                        </A>
                    </div>
                    <div class="space-y-4">
                        <ApplicationCard
                            title="Senior Clinical Pharmacist"
                            company="Sydney Hospital"
                            location="Sydney, NSW"
                            status="Interview Scheduled"
                            date="Tomorrow, 2:00 PM"
                        />
                        <ApplicationCard
                            title="Pharmacy Manager"
                            company="TerryWhite Chemmart"
                            location="Melbourne, VIC"
                            status="Under Review"
                            date="Applied 3 days ago"
                        />
                        <ApplicationCard
                            title="Hospital Pharmacist"
                            company="Royal Brisbane Hospital"
                            location="Brisbane, QLD"
                            status="Application Sent"
                            date="Applied 1 week ago"
                        />
                    </div>
                </div>

                // Job Recommendations
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-xl font-semibold text-gray-900 mb-4">
                        "Recommended Jobs"
                    </h2>
                    <div class="space-y-4">
                        <RecommendedJob
                            title="Clinical Pharmacist"
                            company="St Vincent's Hospital"
                            location="Sydney, NSW"
                            match_score="95%"
                        />
                        <RecommendedJob
                            title="Senior Pharmacist"
                            company="Chemist Warehouse"
                            location="Parramatta, NSW"
                            match_score="88%"
                        />
                        <RecommendedJob
                            title="Pharmacy Specialist"
                            company="NSW Health"
                            location="Newcastle, NSW"
                            match_score="82%"
                        />
                    </div>
                    <div class="mt-4">
                        <A href="/jobs">
                            <Button variant=ButtonVariant::Primary size=ButtonSize::Small class="w-full">
                                "View More Jobs"
                            </Button>
                        </A>
                    </div>
                </div>
            </div>

            // Profile Completion
            <div class="bg-blue-50 rounded-lg p-6 mt-8">
                <div class="flex items-center justify-between">
                    <div>
                        <h3 class="text-lg font-semibold text-gray-900">
                            "Complete Your Profile"
                        </h3>
                        <p class="text-gray-600 mt-1">
                            "Your profile is 75% complete. Add more details to get better job matches."
                        </p>
                    </div>
                    <A href="/profile">
                        <Button variant=ButtonVariant::Primary size=ButtonSize::Medium>
                            "Complete Profile"
                        </Button>
                    </A>
                </div>
                <div class="mt-4 bg-white rounded-full h-2 overflow-hidden">
                    <div class="bg-blue-500 h-full" style="width: 75%"></div>
                </div>
            </div>

            // Job Market Insights
            <div class="bg-white rounded-lg shadow-md p-6 mt-8">
                <h2 class="text-xl font-semibold text-gray-900 mb-4">
                    "Job Market Insights"
                </h2>
                <div class="grid md:grid-cols-3 gap-6">
                    <InsightCard
                        label="Average Salary"
                        value="$95,000"
                        subtext="For Senior Pharmacist roles"
                    />
                    <InsightCard
                        label="Job Growth"
                        value="+15%"
                        subtext="In the last 3 months"
                    />
                    <InsightCard
                        label="Top Location"
                        value="Sydney"
                        subtext="Most opportunities"
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn QuickAction(
    icon: &'static str,
    title: &'static str,
    count: &'static str,
    href: &'static str,
    color: &'static str,
) -> impl IntoView {
    view! {
        <A href=href class="block bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
            <div class=format!("w-12 h-12 {} rounded-lg flex items-center justify-center text-white text-2xl mb-4", color)>
                {icon}
            </div>
            <h3 class="font-semibold text-gray-900">{title}</h3>
            <p class="text-sm text-gray-600 mt-1">{count}</p>
        </A>
    }
}

#[component]
fn ApplicationCard(
    title: &'static str,
    company: &'static str,
    location: &'static str,
    status: &'static str,
    date: &'static str,
) -> impl IntoView {
    let status_class = match status {
        "Interview Scheduled" => "bg-green-100 text-green-800",
        "Under Review" => "bg-yellow-100 text-yellow-800",
        "Application Sent" => "bg-blue-100 text-blue-800",
        _ => "bg-gray-100 text-gray-800",
    };
    
    view! {
        <div class="border rounded-lg p-4 hover:bg-gray-50 transition-colors">
            <div class="flex justify-between items-start">
                <div>
                    <h3 class="font-semibold text-gray-900">{title}</h3>
                    <p class="text-gray-600">{company} " • " {location}</p>
                    <p class="text-sm text-gray-500 mt-2">{date}</p>
                </div>
                <span class=format!("px-3 py-1 rounded-full text-sm {}", status_class)>
                    {status}
                </span>
            </div>
        </div>
    }
}

#[component]
fn RecommendedJob(
    title: &'static str,
    company: &'static str,
    location: &'static str,
    match_score: &'static str,
) -> impl IntoView {
    view! {
        <A href="/jobs/1" class="block border rounded-lg p-4 hover:bg-gray-50 transition-colors">
            <div class="flex justify-between items-start">
                <div>
                    <h3 class="font-semibold text-gray-900">{title}</h3>
                    <p class="text-sm text-gray-600">{company}</p>
                    <p class="text-sm text-gray-500">{location}</p>
                </div>
                <span class="bg-green-100 text-green-800 px-2 py-1 rounded text-sm">
                    {match_score}
                </span>
            </div>
        </A>
    }
}

#[component]
fn InsightCard(
    label: &'static str,
    value: &'static str,
    subtext: &'static str,
) -> impl IntoView {
    view! {
        <div class="text-center">
            <p class="text-3xl font-bold text-blue-600">{value}</p>
            <p class="text-gray-900 mt-1">{label}</p>
            <p class="text-sm text-gray-600 mt-1">{subtext}</p>
        </div>
    }
}