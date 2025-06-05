use leptos::*;
use leptos_router::*;
use crate::components::ui::{Button, ButtonVariant, ButtonSize, Alert, AlertVariant};

#[derive(Clone, Debug, PartialEq)]
pub enum UserType {
    JobSeeker,
    Employer,
    Administrator,
}

impl std::fmt::Display for UserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserType::JobSeeker => write!(f, "Job Seeker"),
            UserType::Employer => write!(f, "Employer"),
            UserType::Administrator => write!(f, "Administrator"),
        }
    }
}

#[component]
pub fn Register() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (confirm_password, set_confirm_password) = create_signal(String::new());
    let (first_name, set_first_name) = create_signal(String::new());
    let (last_name, set_last_name) = create_signal(String::new());
    let (user_type, set_user_type) = create_signal(UserType::JobSeeker);
    let (company_name, set_company_name) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(false);
    let (agree_terms, set_agree_terms) = create_signal(false);

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        // Validation
        if password.get() != confirm_password.get() {
            set_error.set(Some("Passwords do not match".to_string()));
            return;
        }
        
        if password.get().len() < 8 {
            set_error.set(Some("Password must be at least 8 characters long".to_string()));
            return;
        }
        
        if !agree_terms.get() {
            set_error.set(Some("You must agree to the terms and conditions".to_string()));
            return;
        }
        
        // TODO: Implement actual registration logic
        set_loading.set(true);
        set_error.set(None);
        
        // Simulate API call
        set_timeout(
            move || {
                set_loading.set(false);
                set_error.set(Some("Registration functionality coming soon!".to_string()));
            },
            std::time::Duration::from_millis(1000),
        );
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        "Create your account"
                    </h2>
                    <p class="mt-2 text-center text-sm text-gray-600">
                        "Already have an account? "
                        <A href="/login" class="font-medium text-blue-600 hover:text-blue-500">
                            "Sign in"
                        </A>
                    </p>
                </div>
                
                <form class="mt-8 space-y-6" on:submit=handle_submit>
                    <Show when=move || error.get().is_some()>
                        <Alert variant=AlertVariant::Error dismissible=true>
                            {move || error.get().unwrap_or_default()}
                        </Alert>
                    </Show>
                    
                    <div class="space-y-4">
                        // User Type Selection
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "I am a..."
                            </label>
                            <div class="grid grid-cols-2 gap-4">
                                <button
                                    type="button"
                                    class=move || {
                                        let base = "relative rounded-lg border p-4 flex flex-col items-center focus:outline-none";
                                        if user_type.get() == UserType::JobSeeker {
                                            format!("{} border-blue-500 ring-2 ring-blue-500 bg-blue-50", base)
                                        } else {
                                            format!("{} border-gray-300 hover:border-gray-400", base)
                                        }
                                    }
                                    on:click=move |_| set_user_type.set(UserType::JobSeeker)
                                >
                                    <svg class="w-8 h-8 mb-2 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                                    </svg>
                                    <span class="text-sm font-medium">"Job Seeker"</span>
                                </button>
                                
                                <button
                                    type="button"
                                    class=move || {
                                        let base = "relative rounded-lg border p-4 flex flex-col items-center focus:outline-none";
                                        if user_type.get() == UserType::Employer {
                                            format!("{} border-blue-500 ring-2 ring-blue-500 bg-blue-50", base)
                                        } else {
                                            format!("{} border-gray-300 hover:border-gray-400", base)
                                        }
                                    }
                                    on:click=move |_| set_user_type.set(UserType::Employer)
                                >
                                    <svg class="w-8 h-8 mb-2 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
                                    </svg>
                                    <span class="text-sm font-medium">"Employer"</span>
                                </button>
                            </div>
                        </div>
                        
                        // Name Fields
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label for="first-name" class="block text-sm font-medium text-gray-700">
                                    "First name"
                                </label>
                                <input
                                    id="first-name"
                                    name="first-name"
                                    type="text"
                                    autocomplete="given-name"
                                    required
                                    class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
                                    placeholder="John"
                                    value=first_name
                                    on:input=move |ev| set_first_name.set(event_target_value(&ev))
                                />
                            </div>
                            <div>
                                <label for="last-name" class="block text-sm font-medium text-gray-700">
                                    "Last name"
                                </label>
                                <input
                                    id="last-name"
                                    name="last-name"
                                    type="text"
                                    autocomplete="family-name"
                                    required
                                    class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
                                    placeholder="Doe"
                                    value=last_name
                                    on:input=move |ev| set_last_name.set(event_target_value(&ev))
                                />
                            </div>
                        </div>
                        
                        // Company Name (only for employers)
                        <Show when=move || user_type.get() == UserType::Employer>
                            <div>
                                <label for="company-name" class="block text-sm font-medium text-gray-700">
                                    "Company name"
                                </label>
                                <input
                                    id="company-name"
                                    name="company-name"
                                    type="text"
                                    autocomplete="organization"
                                    required
                                    class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
                                    placeholder="Acme Corporation"
                                    value=company_name
                                    on:input=move |ev| set_company_name.set(event_target_value(&ev))
                                />
                            </div>
                        </Show>
                        
                        // Email
                        <div>
                            <label for="email-address" class="block text-sm font-medium text-gray-700">
                                "Email address"
                            </label>
                            <input
                                id="email-address"
                                name="email"
                                type="email"
                                autocomplete="email"
                                required
                                class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
                                placeholder="john.doe@example.com"
                                value=email
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                            />
                        </div>
                        
                        // Password
                        <div>
                            <label for="password" class="block text-sm font-medium text-gray-700">
                                "Password"
                            </label>
                            <input
                                id="password"
                                name="password"
                                type="password"
                                autocomplete="new-password"
                                required
                                class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
                                placeholder="••••••••"
                                value=password
                                on:input=move |ev| set_password.set(event_target_value(&ev))
                            />
                            <p class="mt-1 text-sm text-gray-500">
                                "Must be at least 8 characters long"
                            </p>
                        </div>
                        
                        // Confirm Password
                        <div>
                            <label for="confirm-password" class="block text-sm font-medium text-gray-700">
                                "Confirm password"
                            </label>
                            <input
                                id="confirm-password"
                                name="confirm-password"
                                type="password"
                                autocomplete="new-password"
                                required
                                class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
                                placeholder="••••••••"
                                value=confirm_password
                                on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                            />
                        </div>
                        
                        // Terms and Conditions
                        <div class="flex items-start">
                            <div class="flex items-center h-5">
                                <input
                                    id="agree-terms"
                                    name="agree-terms"
                                    type="checkbox"
                                    class="focus:ring-blue-500 h-4 w-4 text-blue-600 border-gray-300 rounded"
                                    checked=agree_terms
                                    on:change=move |ev| set_agree_terms.set(event_target_checked(&ev))
                                />
                            </div>
                            <div class="ml-3 text-sm">
                                <label for="agree-terms" class="font-medium text-gray-700">
                                    "I agree to the "
                                    <A href="/terms" class="text-blue-600 hover:text-blue-500">
                                        "Terms and Conditions"
                                    </A>
                                    " and "
                                    <A href="/privacy" class="text-blue-600 hover:text-blue-500">
                                        "Privacy Policy"
                                    </A>
                                </label>
                            </div>
                        </div>
                    </div>

                    <div>
                        <Button
                            variant=ButtonVariant::Primary
                            size=ButtonSize::Large
                            disabled=loading.get() || !agree_terms.get()
                        >
                            {move || if loading.get() { "Creating account..." } else { "Create account" }}
                        </Button>
                    </div>
                </form>
            </div>
        </div>
    }
}