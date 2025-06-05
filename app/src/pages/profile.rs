use leptos::*;
use leptos_router::*;
use crate::components::ui::{Button, ButtonVariant, ButtonSize};

#[component]
pub fn Profile() -> impl IntoView {
    // Mock user data - in a real app, this would come from auth context
    let user_name = "Sarah Johnson";
    let user_email = "sarah.johnson@example.com";
    let user_role = "Senior Pharmacist";
    let user_location = "Sydney, NSW";
    
    view! {
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            // Profile Header
            <div class="bg-white rounded-lg shadow-md p-8 mb-8">
                <div class="flex items-center space-x-6">
                    <div class="w-24 h-24 bg-blue-500 rounded-full flex items-center justify-center text-white text-3xl font-bold">
                        "SJ"
                    </div>
                    <div class="flex-1">
                        <h1 class="text-3xl font-bold text-gray-900">{user_name}</h1>
                        <p class="text-gray-600 mt-1">{user_role}</p>
                        <p class="text-gray-500 text-sm mt-1">{user_location}</p>
                    </div>
                    <div>
                        <A href="/settings">
                            <Button variant=ButtonVariant::Secondary size=ButtonSize::Medium>
                                "Edit Profile"
                            </Button>
                        </A>
                    </div>
                </div>
            </div>

            // Profile Details
            <div class="grid md:grid-cols-2 gap-8">
                // Contact Information
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-xl font-semibold text-gray-900 mb-4">
                        "Contact Information"
                    </h2>
                    <div class="space-y-3">
                        <ProfileField label="Email" value=user_email />
                        <ProfileField label="Phone" value="+61 412 345 678" />
                        <ProfileField label="LinkedIn" value="linkedin.com/in/sarahjohnson" />
                    </div>
                </div>

                // Professional Details
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-xl font-semibold text-gray-900 mb-4">
                        "Professional Details"
                    </h2>
                    <div class="space-y-3">
                        <ProfileField label="AHPRA Number" value="PHA0001234567" />
                        <ProfileField label="Experience" value="8 years" />
                        <ProfileField label="Specialisations" value="Hospital, Clinical" />
                    </div>
                </div>

                // Application History
                <div class="bg-white rounded-lg shadow-md p-6 md:col-span-2">
                    <h2 class="text-xl font-semibold text-gray-900 mb-4">
                        "Recent Applications"
                    </h2>
                    <div class="space-y-4">
                        <ApplicationItem
                            title="Senior Clinical Pharmacist"
                            company="Sydney Hospital"
                            date="Applied 2 days ago"
                            status="Under Review"
                        />
                        <ApplicationItem
                            title="Pharmacy Manager"
                            company="TerryWhite Chemmart"
                            date="Applied 1 week ago"
                            status="Interview Scheduled"
                        />
                        <ApplicationItem
                            title="Hospital Pharmacist"
                            company="Royal Melbourne Hospital"
                            date="Applied 2 weeks ago"
                            status="Application Viewed"
                        />
                    </div>
                    <div class="mt-6">
                        <A href="/applications">
                            <Button variant=ButtonVariant::Primary size=ButtonSize::Medium>
                                "View All Applications"
                            </Button>
                        </A>
                    </div>
                </div>
            </div>
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