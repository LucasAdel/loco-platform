use leptos::*;
use leptos::prelude::*;
use crate::components::ui::{Button, ButtonVariant, ButtonSize};

#[component]
pub fn Settings() -> impl IntoView {
    // Form state
    let (save_loading, set_save_loading) = create_signal(false);
    let (success_message, set_success_message) = create_signal(None::<String>);

    let save_settings = move |_| {
        set_save_loading.set(true);
        set_success_message.set(None);
        
        // Simulate save operation
        set_timeout(
            move || {
                set_save_loading.set(false);
                set_success_message.set(Some("Settings saved successfully!".to_string()));
            },
            std::time::Duration::from_secs(1),
        );
    };

    view! {
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">
                "Settings"
            </h1>

            // Success Message
            {move || success_message.get().map(|msg| view! {
                <div class="bg-green-50 border border-green-200 text-green-800 px-4 py-3 rounded-lg mb-6">
                    {msg}
                </div>
            })}

            // Settings Sections
            <div class="space-y-8">
                // Profile Settings
                <SettingsSection title="Profile Settings">
                    <div class="grid md:grid-cols-2 gap-6">
                        <FormField label="First Name" input_type="text" value="Sarah" />
                        <FormField label="Last Name" input_type="text" value="Johnson" />
                        <FormField label="Email" input_type="email" value="sarah.johnson@example.com" />
                        <FormField label="Phone" input_type="tel" value="+61 412 345 678" />
                        <div class="md:col-span-2">
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Bio"
                            </label>
                            <textarea
                                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                rows="4"
                                placeholder="Tell us about yourself..."
                            >
                                "Senior pharmacist with 8 years of experience in hospital and clinical settings."
                            </textarea>
                        </div>
                    </div>
                </SettingsSection>

                // Professional Settings
                <SettingsSection title="Professional Information">
                    <div class="grid md:grid-cols-2 gap-6">
                        <FormField label="AHPRA Number" input_type="text" value="PHA0001234567" />
                        <FormField label="Years of Experience" input_type="number" value="8" />
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Current Role"
                            </label>
                            <select class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500">
                                <option>"Senior Pharmacist"</option>
                                <option>"Pharmacist"</option>
                                <option>"Pharmacy Manager"</option>
                                <option>"Clinical Pharmacist"</option>
                            </select>
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Preferred Location"
                            </label>
                            <select class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500">
                                <option>"Sydney, NSW"</option>
                                <option>"Melbourne, VIC"</option>
                                <option>"Brisbane, QLD"</option>
                                <option>"Perth, WA"</option>
                                <option>"Adelaide, SA"</option>
                            </select>
                        </div>
                    </div>
                </SettingsSection>

                // Notification Settings
                <SettingsSection title="Notification Preferences">
                    <div class="space-y-4">
                        <ToggleSetting
                            label="Email Notifications"
                            description="Receive email updates about new job matches"
                            checked=true
                        />
                        <ToggleSetting
                            label="Application Updates"
                            description="Get notified when your applications are viewed or updated"
                            checked=true
                        />
                        <ToggleSetting
                            label="Newsletter"
                            description="Receive our weekly newsletter with job market insights"
                            checked=false
                        />
                        <ToggleSetting
                            label="SMS Notifications"
                            description="Receive SMS alerts for urgent updates"
                            checked=false
                        />
                    </div>
                </SettingsSection>

                // Privacy Settings
                <SettingsSection title="Privacy Settings">
                    <div class="space-y-4">
                        <ToggleSetting
                            label="Profile Visible to Employers"
                            description="Allow employers to find and view your profile"
                            checked=true
                        />
                        <ToggleSetting
                            label="Show Full Name"
                            description="Display your full name on your public profile"
                            checked=true
                        />
                        <ToggleSetting
                            label="Show Contact Details"
                            description="Make your contact information visible to employers"
                            checked=false
                        />
                    </div>
                </SettingsSection>

                // Account Settings
                <SettingsSection title="Account Settings">
                    <div class="space-y-4">
                        <div>
                            <Button variant=ButtonVariant::Secondary size=ButtonSize::Medium>
                                "Change Password"
                            </Button>
                        </div>
                        <div>
                            <Button variant=ButtonVariant::Secondary size=ButtonSize::Medium>
                                "Download My Data"
                            </Button>
                        </div>
                        <div class="pt-4 border-t">
                            <p class="text-sm text-gray-600 mb-4">
                                "Deleting your account will permanently remove all your data. This action cannot be undone."
                            </p>
                            <button class="text-red-600 hover:text-red-700 font-medium">
                                "Delete Account"
                            </button>
                        </div>
                    </div>
                </SettingsSection>

                // Save Button
                <div class="flex justify-end pt-6">
                    <Button
                        variant=ButtonVariant::Primary
                        size=ButtonSize::Large
                        on_click=Callback::new(save_settings)
                        disabled=save_loading.get()
                    >
                        {move || if save_loading.get() { "Saving..." } else { "Save Settings" }}
                    </Button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SettingsSection(
    title: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-md p-6">
            <h2 class="text-xl font-semibold text-gray-900 mb-6">{title}</h2>
            {children()}
        </div>
    }
}

#[component]
fn FormField(
    label: &'static str,
    input_type: &'static str,
    value: &'static str,
) -> impl IntoView {
    view! {
        <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
                {label}
            </label>
            <input
                type=input_type
                value=value
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
        </div>
    }
}

#[component]
fn ToggleSetting(
    label: &'static str,
    description: &'static str,
    checked: bool,
) -> impl IntoView {
    let (is_checked, set_is_checked) = create_signal(checked);

    view! {
        <div class="flex items-center justify-between">
            <div>
                <h3 class="font-medium text-gray-900">{label}</h3>
                <p class="text-sm text-gray-600">{description}</p>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
                <input
                    type="checkbox"
                    class="sr-only peer"
                    checked=checked
                    on:change=move |_| set_is_checked.set(!is_checked.get())
                />
                <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
            </label>
        </div>
    }
}