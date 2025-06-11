use leptos::*;
use leptos::prelude::*;
use leptos_router::*;
use crate::contexts::use_auth;
use crate::components::{Button, ButtonVariant};

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);
    let loading = auth.loading;
    
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        let auth = auth.clone();
        let email_val = email.get();
        let password_val = password.get();
        
        spawn_local(async move {
            error.set(None);
            match auth.login(email_val, password_val).await {
                Ok(_) => {
                    navigate("/dashboard", Default::default());
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
        });
    };
    
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 via-white to-purple-50">
            // Animated background elements
            <div class="absolute inset-0 overflow-hidden">
                <div class="absolute -top-40 -right-40 w-80 h-80 bg-purple-300 rounded-full mix-blend-multiply filter blur-xl opacity-70 animate-blob"></div>
                <div class="absolute -bottom-40 -left-40 w-80 h-80 bg-blue-300 rounded-full mix-blend-multiply filter blur-xl opacity-70 animate-blob animation-delay-2000"></div>
                <div class="absolute top-40 left-40 w-80 h-80 bg-pink-300 rounded-full mix-blend-multiply filter blur-xl opacity-70 animate-blob animation-delay-4000"></div>
            </div>
            
            <div class="relative z-10 w-full max-w-md">
                <div class="bg-white/80 backdrop-blur-xl rounded-3xl shadow-2xl p-8">
                    <div class="text-center mb-8">
                        <div class="w-20 h-20 bg-gradient-to-br from-blue-600 to-purple-600 rounded-2xl flex items-center justify-center text-white font-bold text-3xl mx-auto mb-4">
                            "L"
                        </div>
                        <h2 class="text-3xl font-bold text-gray-900">"Welcome Back"</h2>
                        <p class="text-gray-600 mt-2">"Sign in to your account to continue"</p>
                    </div>
                    
                    <form on:submit=on_submit class="space-y-6">
                        {move || error.get().map(|e| view! {
                            <div class="p-4 bg-red-50 border border-red-200 rounded-xl">
                                <p class="text-sm text-red-600">{e}</p>
                            </div>
                        })}
                        
                        <div>
                            <label for="email" class="block text-sm font-medium text-gray-700 mb-2">
                                "Email Address"
                            </label>
                            <input
                                type="email"
                                id="email"
                                class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                                placeholder="you@example.com"
                                required
                                on:input=move |ev| email.set(event_target_value(&ev))
                            />
                        </div>
                        
                        <div>
                            <label for="password" class="block text-sm font-medium text-gray-700 mb-2">
                                "Password"
                            </label>
                            <input
                                type="password"
                                id="password"
                                class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                                placeholder="••••••••"
                                required
                                on:input=move |ev| password.set(event_target_value(&ev))
                            />
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <label class="flex items-center">
                                <input type="checkbox" class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"/>
                                <span class="ml-2 text-sm text-gray-600">"Remember me"</span>
                            </label>
                            <a href="/forgot-password" class="text-sm text-blue-600 hover:text-blue-700">
                                "Forgot password?"
                            </a>
                        </div>
                        
                        <Button 
                            variant=ButtonVariant::Primary
                            full_width=true
                            disabled=loading.get()
                        >
                            {move || if loading.get() {
                                "Signing in..."
                            } else {
                                "Sign In"
                            }}
                        </Button>
                        
                        <div class="relative">
                            <div class="absolute inset-0 flex items-center">
                                <div class="w-full border-t border-gray-300"></div>
                            </div>
                            <div class="relative flex justify-center text-sm">
                                <span class="px-2 bg-white text-gray-500">"Or continue with"</span>
                            </div>
                        </div>
                        
                        <div class="grid grid-cols-2 gap-3">
                            <button type="button" class="w-full inline-flex justify-center py-2 px-4 border border-gray-300 rounded-xl shadow-sm bg-white text-sm font-medium text-gray-500 hover:bg-gray-50">
                                <svg class="w-5 h-5" viewBox="0 0 24 24">
                                    <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
                                    <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
                                    <path fill="#FBBC04" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
                                    <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
                                </svg>
                                <span class="ml-2">"Google"</span>
                            </button>
                            <button type="button" class="w-full inline-flex justify-center py-2 px-4 border border-gray-300 rounded-xl shadow-sm bg-white text-sm font-medium text-gray-500 hover:bg-gray-50">
                                <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                                </svg>
                                <span class="ml-2">"GitHub"</span>
                            </button>
                        </div>
                    </form>
                    
                    <p class="mt-6 text-center text-sm text-gray-600">
                        "Don't have an account? "
                        <a href="/register" class="font-medium text-blue-600 hover:text-blue-700">
                            "Sign up"
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}