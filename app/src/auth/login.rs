use leptos::*;
use leptos_router::*;
use crate::components::ui::{Button, ButtonVariant, ButtonSize, Alert, AlertVariant};

#[component]
pub fn Login() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(false);

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        // TODO: Implement actual login logic
        set_loading.set(true);
        set_error.set(None);
        
        // Simulate API call
        set_timeout(
            move || {
                set_loading.set(false);
                set_error.set(Some("Login functionality coming soon!".to_string()));
            },
            std::time::Duration::from_millis(1000),
        );
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        "Sign in to your account"
                    </h2>
                    <p class="mt-2 text-center text-sm text-gray-600">
                        "Or "
                        <A href="/register" class="font-medium text-blue-600 hover:text-blue-500">
                            "create a new account"
                        </A>
                    </p>
                </div>
                
                <form class="mt-8 space-y-6" on:submit=handle_submit>
                    <Show when=move || error.get().is_some()>
                        <Alert variant=AlertVariant::Error dismissible=true>
                            {move || error.get().unwrap_or_default()}
                        </Alert>
                    </Show>
                    
                    <div class="rounded-md shadow-sm -space-y-px">
                        <div>
                            <label for="email-address" class="sr-only">"Email address"</label>
                            <input
                                id="email-address"
                                name="email"
                                type="email"
                                autocomplete="email"
                                required
                                class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
                                placeholder="Email address"
                                value=email
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                            />
                        </div>
                        <div>
                            <label for="password" class="sr-only">"Password"</label>
                            <input
                                id="password"
                                name="password"
                                type="password"
                                autocomplete="current-password"
                                required
                                class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
                                placeholder="Password"
                                value=password
                                on:input=move |ev| set_password.set(event_target_value(&ev))
                            />
                        </div>
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="flex items-center">
                            <input
                                id="remember-me"
                                name="remember-me"
                                type="checkbox"
                                class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                            />
                            <label for="remember-me" class="ml-2 block text-sm text-gray-900">
                                "Remember me"
                            </label>
                        </div>

                        <div class="text-sm">
                            <A href="/forgot-password" class="font-medium text-blue-600 hover:text-blue-500">
                                "Forgot your password?"
                            </A>
                        </div>
                    </div>

                    <div>
                        <Button
                            variant=ButtonVariant::Primary
                            size=ButtonSize::Large
                            disabled=loading.get()
                        >
                            {move || if loading.get() { "Signing in..." } else { "Sign in" }}
                        </Button>
                    </div>
                </form>
            </div>
        </div>
    }
}