use leptos::*;
use leptos::prelude::*;
use leptos_router::*;
use crate::api::{use_auth, LoginRequest};

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    
    // Form state
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let (show_password, set_show_password) = create_signal(false);
    
    // Handle form submission
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        
        let auth = auth.clone();
        let email_val = email.get();
        let password_val = password.get();
        
        spawn_local(async move {
            set_error.set(None);
            
            match auth.sign_in(email_val, password_val).await {
                Ok(_) => {
                    navigate("/dashboard", Default::default());
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
        });
    };
    
    // OAuth sign in
    let oauth_sign_in = move |provider: &'static str| {
        move |_| {
            spawn_local(async move {
                match crate::api::SupabaseAuth::get_oauth_url(provider).await {
                    Ok(response) => {
                        if let Some(url) = response.get("url").and_then(|v| v.as_str()) {
                            let window = web_sys::window().unwrap();
                            let _ = window.location().set_href(url);
                        }
                    }
                    Err(e) => {
                        set_error.set(Some(format!("OAuth error: {}", e)));
                    }
                }
            });
        }
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 via-white to-purple-50">
            <div class="w-full max-w-md">
                // Glass morphism card
                <div class="bg-white/70 backdrop-blur-xl rounded-3xl shadow-2xl border border-white/20 p-8">
                    // Logo and title
                    <div class="text-center mb-8">
                        <div class="inline-flex items-center justify-center w-20 h-20 bg-gradient-to-br from-blue-600 to-purple-600 rounded-2xl mb-4">
                            <svg class="w-12 h-12 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
                            </svg>
                        </div>
                        <h1 class="text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                            "Welcome Back"
                        </h1>
                        <p class="text-gray-600 mt-2">"Sign in to your Loco Platform account"</p>
                    </div>

                    // Error message
                    {move || error.get().map(|e| view! {
                        <div class="mb-6 p-4 bg-red-50 border border-red-200 rounded-xl">
                            <p class="text-red-600 text-sm">{e}</p>
                        </div>
                    })}

                    // Login form
                    <form on:submit=on_submit class="space-y-6">
                        // Email input
                        <div>
                            <label for="email" class="block text-sm font-medium text-gray-700 mb-2">
                                "Email Address"
                            </label>
                            <div class="relative">
                                <input
                                    type="email"
                                    id="email"
                                    class="w-full px-4 py-3 pl-12 bg-white/50 border border-gray-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
                                    placeholder="you@example.com"
                                    required
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                />
                                <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                                    <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207" />
                                    </svg>
                                </div>
                            </div>
                        </div>

                        // Password input
                        <div>
                            <label for="password" class="block text-sm font-medium text-gray-700 mb-2">
                                "Password"
                            </label>
                            <div class="relative">
                                <input
                                    type=move || if show_password.get() { "text" } else { "password" }
                                    id="password"
                                    class="w-full px-4 py-3 pl-12 pr-12 bg-white/50 border border-gray-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
                                    placeholder="••••••••"
                                    required
                                    on:input=move |ev| set_password.set(event_target_value(&ev))
                                />
                                <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                                    <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                                    </svg>
                                </div>
                                <button
                                    type="button"
                                    class="absolute inset-y-0 right-0 pr-4 flex items-center"
                                    on:click=move |_| set_show_password.update(|v| *v = !*v)
                                >
                                    <svg class="w-5 h-5 text-gray-400 hover:text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        {move || if show_password.get() {
                                            view! {
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
                                            }
                                        } else {
                                            view! {
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                                            }
                                        }}
                                    </svg>
                                </button>
                            </div>
                        </div>

                        // Remember me and forgot password
                        <div class="flex items-center justify-between">
                            <div class="flex items-center">
                                <input
                                    id="remember-me"
                                    type="checkbox"
                                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                                />
                                <label for="remember-me" class="ml-2 block text-sm text-gray-700">
                                    "Remember me"
                                </label>
                            </div>
                            <a href="/auth/forgot-password" class="text-sm text-blue-600 hover:text-blue-700">
                                "Forgot password?"
                            </a>
                        </div>

                        // Submit button
                        <button
                            type="submit"
                            class="w-full py-3 px-4 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-xl font-semibold hover:shadow-lg transform hover:-translate-y-0.5 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
                            disabled=move || auth.loading.get()
                        >
                            {move || if auth.loading.get() {
                                view! {
                                    <span class="flex items-center justify-center">
                                        <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
                                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                        </svg>
                                        "Signing in..."
                                    </span>
                                }
                            } else {
                                view! { <span>"Sign In"</span> }
                            }}
                        </button>
                    </form>

                    // Divider
                    <div class="relative my-8">
                        <div class="absolute inset-0 flex items-center">
                            <div class="w-full border-t border-gray-200"></div>
                        </div>
                        <div class="relative flex justify-center text-sm">
                            <span class="px-4 bg-white/70 text-gray-500">"Or continue with"</span>
                        </div>
                    </div>

                    // OAuth buttons
                    <div class="grid grid-cols-3 gap-3">
                        <button
                            on:click=oauth_sign_in("google")
                            class="flex items-center justify-center py-2.5 px-4 border border-gray-200 rounded-xl hover:bg-gray-50 transition-colors"
                        >
                            <svg class="w-5 h-5" viewBox="0 0 24 24">
                                <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
                                <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
                                <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
                                <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
                            </svg>
                        </button>

                        <button
                            on:click=oauth_sign_in("github")
                            class="flex items-center justify-center py-2.5 px-4 border border-gray-200 rounded-xl hover:bg-gray-50 transition-colors"
                        >
                            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                            </svg>
                        </button>

                        <button
                            on:click=oauth_sign_in("linkedin")
                            class="flex items-center justify-center py-2.5 px-4 border border-gray-200 rounded-xl hover:bg-gray-50 transition-colors"
                        >
                            <svg class="w-5 h-5 text-[#0A66C2]" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
                            </svg>
                        </button>
                    </div>

                    // Sign up link
                    <p class="mt-8 text-center text-sm text-gray-600">
                        "Don't have an account? "
                        <a href="/auth/register" class="font-medium text-blue-600 hover:text-blue-700">
                            "Sign up for free"
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}