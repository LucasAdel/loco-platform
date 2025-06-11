use leptos::*;
use leptos::prelude::*;
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
        
        // Enhanced email validation
        if email.get().trim().is_empty() {
            set_error.set(Some("Please enter your email address".to_string()));
            return;
        }
        
        if !email.get().contains('@') || !email.get().contains('.') {
            set_error.set(Some("Please enter a valid email address".to_string()));
            return;
        }
        
        // TODO: Implement actual password reset logic with backend API
        set_loading.set(true);
        set_error.set(None);
        set_success.set(false);
        
        // Simulate API call
        set_timeout(
            move || {
                set_loading.set(false);
                set_success.set(true);
            },
            std::time::Duration::from_millis(1500),
        );
    };

    let handle_resend = move |_| {
        set_loading.set(true);
        set_timeout(
            move || {
                set_loading.set(false);
                // TODO: Implement resend logic
            },
            std::time::Duration::from_millis(1000),
        );
    };

    view! {
        <div class="min-h-screen relative overflow-hidden">
            // Animated Background
            <div class="absolute inset-0 bg-gradient-to-br from-purple-400/20 via-blue-500/10 to-tiffany-600/20">
                <div class="absolute inset-0 opacity-40" style="background-image: repeating-linear-gradient(45deg, transparent, transparent 35px, rgba(255,255,255,.05) 35px, rgba(255,255,255,.05) 70px)"></div>
                
                // Floating Elements
                <div class="absolute top-1/4 left-1/4 w-64 h-64 bg-purple-300/10 rounded-full blur-3xl animate-float"></div>
                <div class="absolute bottom-1/4 right-1/4 w-96 h-96 bg-blue-300/10 rounded-full blur-3xl animate-float-delayed"></div>
                <div class="absolute top-3/4 left-1/3 w-80 h-80 bg-tiffany-300/10 rounded-full blur-3xl animate-pulse-slow"></div>
            </div>

            // Main Content
            <div class="relative z-10 min-h-screen flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
                <div class="max-w-md w-full space-y-8">
                    // Glass Morphism Card
                    <div class="bg-white/70 backdrop-blur-xl border border-white/20 rounded-3xl shadow-2xl p-8 transform hover:scale-[1.02] transition-all duration-500">
                        
                        <Show
                            when=move || !success.get()
                            fallback=move || view! {
                                // Success State
                                <div class="text-center animate-fadeIn">
                                    // Success Icon
                                    <div class="mx-auto h-20 w-20 bg-gradient-to-br from-green-500 to-emerald-600 rounded-2xl flex items-center justify-center mb-6 transform animate-bounce">
                                        <svg class="h-10 w-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 7.89a2 2 0 002.83 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                                        </svg>
                                    </div>
                                    
                                    <h2 class="text-3xl font-bold bg-gradient-to-r from-gray-900 to-gray-700 bg-clip-text text-transparent mb-4">
                                        "Check Your Email"
                                    </h2>
                                    
                                    <div class="bg-green-50/50 rounded-2xl p-6 mb-6">
                                        <div class="flex items-start space-x-4">
                                            <div class="flex-shrink-0">
                                                <svg class="h-6 w-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                                </svg>
                                            </div>
                                            <div class="text-left">
                                                <h3 class="text-lg font-semibold text-green-800 mb-2">
                                                    "Reset Link Sent!"
                                                </h3>
                                                <p class="text-sm text-green-700 mb-3">
                                                    "We've sent a password reset link to:"
                                                </p>
                                                <p class="text-sm font-medium text-green-900 bg-white/50 rounded-lg px-3 py-2 mb-3">
                                                    {move || email.get()}
                                                </p>
                                                <p class="text-sm text-green-700">
                                                    "The link will expire in 24 hours for security reasons."
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    // Help Section
                                    <div class="bg-gray-50/50 rounded-2xl p-4 mb-6">
                                        <h4 class="font-medium text-gray-900 mb-2">"Didn't receive the email?"</h4>
                                        <ul class="text-sm text-gray-600 space-y-1 mb-4">
                                            <li>"• Check your spam/junk folder"</li>
                                            <li>"• Make sure the email address is correct"</li>
                                            <li>"• Wait a few minutes for delivery"</li>
                                        </ul>
                                        
                                        <button
                                            type="button"
                                            class="w-full py-2 px-4 bg-white border border-gray-300 rounded-xl text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-tiffany-500 transition-all duration-200 transform hover:scale-[1.02]"
                                            on:click=handle_resend
                                            disabled=loading.get()
                                        >
                                            {move || if loading.get() { "Resending..." } else { "Resend Email" }}
                                        </button>
                                    </div>
                                    
                                    // Action Buttons
                                    <div class="space-y-3">
                                        <button
                                            type="button"
                                            class="w-full py-3 px-4 bg-gradient-to-r from-tiffany-600 to-blue-600 hover:from-tiffany-700 hover:to-blue-700 text-white font-medium rounded-xl focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-tiffany-500 transition-all duration-200 transform hover:scale-[1.02] shadow-lg hover:shadow-xl"
                                            on:click=move |_| {
                                                set_success.set(false);
                                                set_email.set(String::new());
                                            }
                                        >
                                            "Try Different Email"
                                        </button>
                                        
                                        <A 
                                            href="/login" 
                                            class="block w-full py-3 px-4 bg-white border border-gray-300 text-gray-700 font-medium rounded-xl text-center hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-tiffany-500 transition-all duration-200 transform hover:scale-[1.02]"
                                        >
                                            "← Back to Sign In"
                                        </A>
                                    </div>
                                </div>
                            }
                        >
                            // Form State
                            <div class="text-center mb-8">
                                <div class="mx-auto h-20 w-20 bg-gradient-to-br from-purple-500 to-blue-600 rounded-2xl flex items-center justify-center mb-6 transform hover:rotate-6 transition-transform duration-300">
                                    <svg class="h-10 w-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
                                    </svg>
                                </div>
                                <h2 class="text-3xl font-bold bg-gradient-to-r from-gray-900 to-gray-700 bg-clip-text text-transparent">
                                    "Reset Password"
                                </h2>
                                <p class="mt-3 text-gray-600 leading-relaxed">
                                    "Enter your email address and we'll send you a secure link to reset your password."
                                </p>
                            </div>

                            <form class="space-y-6" on:submit=handle_submit>
                                <Show when=move || error.get().is_some()>
                                    <div class="transform animate-shake">
                                        <Alert variant=AlertVariant::Error dismissible=true>
                                            {move || error.get().unwrap_or_default()}
                                        </Alert>
                                    </div>
                                </Show>
                                
                                // Email Field
                                <div class="space-y-2">
                                    <label for="email-address" class="block text-sm font-medium text-gray-700">
                                        "Email address"
                                    </label>
                                    <div class="relative group">
                                        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                            <svg class="h-5 w-5 text-gray-400 group-focus-within:text-purple-500 transition-colors duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207" />
                                            </svg>
                                        </div>
                                        <input
                                            id="email-address"
                                            name="email"
                                            type="email"
                                            autocomplete="email"
                                            required
                                            class="block w-full pl-10 pr-3 py-3 border border-gray-300/50 rounded-xl shadow-sm placeholder-gray-400 bg-white/50 backdrop-blur-sm focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all duration-200 hover:bg-white/70"
                                            placeholder="john.doe@example.com"
                                            value=email
                                            on:input=move |ev| set_email.set(event_target_value(&ev))
                                        />
                                    </div>
                                    <p class="text-xs text-gray-500 mt-1">
                                        "We'll send a secure reset link to this email address"
                                    </p>
                                </div>

                                // Submit Button
                                <div class="space-y-4">
                                    <button
                                        type="submit"
                                        disabled=loading.get()
                                        class="group relative w-full flex justify-center py-3 px-4 border border-transparent text-sm font-medium rounded-xl text-white bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-purple-500 disabled:opacity-50 disabled:cursor-not-allowed transform hover:scale-[1.02] transition-all duration-200 shadow-lg hover:shadow-xl"
                                    >
                                        <Show
                                            when=move || loading.get()
                                            fallback=move || view! {
                                                <span class="flex items-center">
                                                    <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
                                                    </svg>
                                                    "Send Reset Link"
                                                </span>
                                            }
                                        >
                                            <span class="flex items-center">
                                                <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
                                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                                </svg>
                                                "Sending..."
                                            </span>
                                        </Show>
                                    </button>
                                    
                                    // Back to Login
                                    <div class="text-center">
                                        <A 
                                            href="/login" 
                                            class="inline-flex items-center text-sm font-medium text-purple-600 hover:text-purple-700 transition-colors duration-200"
                                        >
                                            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                            </svg>
                                            "Back to Sign In"
                                        </A>
                                    </div>
                                </div>
                            </form>

                            // Security Notice
                            <div class="mt-8 p-4 bg-blue-50/50 border border-blue-200/50 rounded-xl">
                                <div class="flex items-start space-x-3">
                                    <svg class="h-5 w-5 text-blue-600 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                                    </svg>
                                    <div class="text-sm">
                                        <h4 class="font-medium text-blue-900 mb-1">"Security Notice"</h4>
                                        <p class="text-blue-700">
                                            "For your security, reset links expire after 24 hours and can only be used once."
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </Show>
                    </div>
                </div>
            </div>
            
            // Custom Animations
            <style>
                "@keyframes float {
                    0%, 100% { transform: translateY(0px) rotate(0deg); }
                    33% { transform: translateY(-10px) rotate(1deg); }
                    66% { transform: translateY(5px) rotate(-1deg); }
                }
                
                @keyframes float-delayed {
                    0%, 100% { transform: translateY(0px) rotate(0deg); }
                    33% { transform: translateY(8px) rotate(-1deg); }
                    66% { transform: translateY(-12px) rotate(1deg); }
                }
                
                @keyframes pulse-slow {
                    0%, 100% { opacity: 0.3; transform: scale(1); }
                    50% { opacity: 0.5; transform: scale(1.05); }
                }
                
                @keyframes shake {
                    0%, 100% { transform: translateX(0); }
                    10%, 30%, 50%, 70%, 90% { transform: translateX(-2px); }
                    20%, 40%, 60%, 80% { transform: translateX(2px); }
                }
                
                @keyframes fadeIn {
                    from { opacity: 0; transform: translateY(-10px); }
                    to { opacity: 1; transform: translateY(0); }
                }
                
                @keyframes bounce {
                    0%, 20%, 53%, 80%, 100% { transform: translate3d(0,0,0); }
                    40%, 43% { transform: translate3d(0,-10px,0); }
                    70% { transform: translate3d(0,-5px,0); }
                    90% { transform: translate3d(0,-2px,0); }
                }
                
                .animate-float { animation: float 6s ease-in-out infinite; }
                .animate-float-delayed { animation: float-delayed 8s ease-in-out infinite; }
                .animate-pulse-slow { animation: pulse-slow 4s ease-in-out infinite; }
                .animate-shake { animation: shake 0.5s ease-in-out; }
                .animate-fadeIn { animation: fadeIn 0.3s ease-out; }
                .animate-bounce { animation: bounce 2s infinite; }
            "
            </style>
        </div>
    }
}