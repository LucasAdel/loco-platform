use leptos::*;
use leptos_router::*;
use crate::components::ui::{Button, ButtonVariant, ButtonSize, Alert, AlertVariant};

#[component]
pub fn ForgotPassword() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(false);
    let (loading, set_loading) = create_signal(false);

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        // Basic email validation
        if !email.get().contains('@') {
            set_error.set(Some("Please enter a valid email address".to_string()));
            return;
        }
        
        // TODO: Implement actual password reset logic
        set_loading.set(true);
        set_error.set(None);
        set_success.set(false);
        
        // Simulate API call
        set_timeout(
            move || {
                set_loading.set(false);
                set_success.set(true);
            },
            std::time::Duration::from_millis(1000),
        );
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        "Reset your password"
                    </h2>
                    <p class="mt-2 text-center text-sm text-gray-600">
                        "Enter your email address and we'll send you a link to reset your password."
                    </p>
                </div>
                
                <Show
                    when=move || !success.get()
                    fallback=move || view! {
                        <div class="rounded-md bg-green-50 p-4">
                            <div class="flex">
                                <div class="flex-shrink-0">
                                    <svg class="h-5 w-5 text-green-400" viewBox="0 0 20 20" fill="currentColor">
                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                                    </svg>
                                </div>
                                <div class="ml-3">
                                    <h3 class="text-sm font-medium text-green-800">
                                        "Check your email"
                                    </h3>
                                    <div class="mt-2 text-sm text-green-700">
                                        <p>
                                            "We've sent a password reset link to "
                                            <span class="font-medium">{move || email.get()}</span>
                                            ". The link will expire in 24 hours."
                                        </p>
                                        <p class="mt-2">
                                            "Didn't receive the email? Check your spam folder or "
                                            <button
                                                type="button"
                                                class="font-medium underline text-green-800 hover:text-green-900"
                                                on:click=move |_| {
                                                    set_success.set(false);
                                                    set_email.set(String::new());
                                                }
                                            >
                                                "try again"
                                            </button>
                                            "."
                                        </p>
                                    </div>
                                    <div class="mt-4">
                                        <A 
                                            href="/login" 
                                            class="text-sm font-medium text-green-800 hover:text-green-900"
                                        >
                                            "← Back to sign in"
                                        </A>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                >
                    <form class="mt-8 space-y-6" on:submit=handle_submit>
                        <Show when=move || error.get().is_some()>
                            <Alert variant=AlertVariant::Error dismissible=true>
                                {move || error.get().unwrap_or_default()}
                            </Alert>
                        </Show>
                        
                        <div>
                            <label for="email-address" class="block text-sm font-medium text-gray-700">
                                "Email address"
                            </label>
                            <div class="mt-1">
                                <input
                                    id="email-address"
                                    name="email"
                                    type="email"
                                    autocomplete="email"
                                    required
                                    class="appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
                                    placeholder="john.doe@example.com"
                                    value=email
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                />
                            </div>
                        </div>

                        <div class="space-y-4">
                            <Button
                                variant=ButtonVariant::Primary
                                size=ButtonSize::Large
                                disabled=loading.get()
                            >
                                {move || if loading.get() { "Sending reset link..." } else { "Send reset link" }}
                            </Button>
                            
                            <div class="text-center">
                                <A 
                                    href="/login" 
                                    class="text-sm font-medium text-blue-600 hover:text-blue-500"
                                >
                                    "← Back to sign in"
                                </A>
                            </div>
                        </div>
                    </form>
                </Show>
            </div>
        </div>
    }
}