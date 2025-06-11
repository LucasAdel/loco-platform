use leptos::*;
use leptos::prelude::*;
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
    let (show_password, set_show_password) = create_signal(false);
    let (show_confirm_password, set_show_confirm_password) = create_signal(false);

    // Password strength calculation
    let password_strength = create_memo(move |_| {
        let pwd = password.get();
        if pwd.is_empty() { return (0, "Enter a password".to_string()); }
        
        let mut score = 0;
        let mut requirements = Vec::new();
        
        if pwd.len() >= 8 { score += 1; } else { requirements.push("8+ characters"); }
        if pwd.chars().any(|c| c.is_uppercase()) { score += 1; } else { requirements.push("uppercase letter"); }
        if pwd.chars().any(|c| c.is_lowercase()) { score += 1; } else { requirements.push("lowercase letter"); }
        if pwd.chars().any(|c| c.is_numeric()) { score += 1; } else { requirements.push("number"); }
        if pwd.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) { score += 1; } else { requirements.push("special character"); }
        
        let message = if score == 5 {
            "Strong password".to_string()
        } else if score >= 3 {
            "Good password".to_string()
        } else if score >= 2 {
            "Weak password".to_string()
        } else {
            format!("Need: {}", requirements.join(", "))
        };
        
        (score, message)
    });

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        // Validation
        if first_name.get().trim().is_empty() || last_name.get().trim().is_empty() {
            set_error.set(Some("Please enter your full name".to_string()));
            return;
        }
        
        if !email.get().contains('@') {
            set_error.set(Some("Please enter a valid email address".to_string()));
            return;
        }
        
        if password.get() != confirm_password.get() {
            set_error.set(Some("Passwords do not match".to_string()));
            return;
        }
        
        if password_strength.get().0 < 3 {
            set_error.set(Some("Please choose a stronger password".to_string()));
            return;
        }
        
        if user_type.get() == UserType::Employer && company_name.get().trim().is_empty() {
            set_error.set(Some("Please enter your company name".to_string()));
            return;
        }
        
        if !agree_terms.get() {
            set_error.set(Some("You must agree to the terms and conditions".to_string()));
            return;
        }
        
        // TODO: Implement actual registration logic with backend API
        set_loading.set(true);
        set_error.set(None);
        
        // Simulate API call
        set_timeout(
            move || {
                set_loading.set(false);
                // TODO: Navigate to email verification page or dashboard
                set_error.set(Some("Registration system integration coming soon!".to_string()));
            },
            std::time::Duration::from_millis(2000),
        );
    };

    let handle_social_signup = move |provider: &str| {
        // TODO: Implement social signup
        set_error.set(Some(format!("{} signup coming soon!", provider)));
    };

    view! {
        <div class="min-h-screen relative overflow-hidden">
            // Animated Background
            <div class="absolute inset-0 bg-gradient-to-br from-emerald-400/20 via-tiffany-500/10 to-blue-600/20">
                <div class="absolute inset-0 opacity-40" style="background-image: url('data:image/svg+xml,%3Csvg width=%2260%22 height=%2260%22 viewBox=%220 0 60 60%22 xmlns=%22http://www.w3.org/2000/svg%22%3E%3Cg fill=%22none%22 fill-rule=%22evenodd%22%3E%3Cg fill=%22%229C92AC%22 fill-opacity=%220.04%22%3E%3Ccircle cx=%2230%22 cy=%2230%22 r=%224%22/%3E%3C/g%3E%3C/g%3E%3C/svg%3E');"></div>
                
                // Floating Elements
                <div class="absolute top-1/4 right-1/4 w-72 h-72 bg-emerald-300/10 rounded-full blur-3xl animate-float"></div>
                <div class="absolute bottom-1/4 left-1/4 w-96 h-96 bg-tiffany-300/10 rounded-full blur-3xl animate-float-delayed"></div>
                <div class="absolute top-3/4 right-1/3 w-80 h-80 bg-blue-300/10 rounded-full blur-3xl animate-pulse-slow"></div>
            </div>

            // Main Content
            <div class="relative z-10 min-h-screen flex items-center justify-center py-8 px-4 sm:px-6 lg:px-8">
                <div class="max-w-lg w-full space-y-8">
                    // Glass Morphism Card
                    <div class="bg-white/70 backdrop-blur-xl border border-white/20 rounded-3xl shadow-2xl p-8 transform hover:scale-[1.01] transition-all duration-500">
                        
                        // Header
                        <div class="text-center mb-8">
                            <div class="mx-auto h-20 w-20 bg-gradient-to-br from-emerald-500 to-tiffany-600 rounded-2xl flex items-center justify-center mb-6 transform hover:rotate-6 transition-transform duration-300">
                                <svg class="h-10 w-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
                                </svg>
                            </div>
                            <h2 class="text-3xl font-bold bg-gradient-to-r from-gray-900 to-gray-700 bg-clip-text text-transparent">
                                "Join Our Platform"
                            </h2>
                            <p class="mt-2 text-gray-600">
                                "Already have an account? "
                                <A href="/login" class="font-semibold text-tiffany-600 hover:text-tiffany-700 transition-colors duration-200">
                                    "Sign in here"
                                </A>
                            </p>
                        </div>

                        // Social Signup Buttons
                        <div class="space-y-3 mb-6">
                            <button
                                type="button"
                                class="w-full flex items-center justify-center px-4 py-3 border border-gray-300/50 rounded-xl shadow-sm bg-white/50 backdrop-blur-sm text-sm font-medium text-gray-700 hover:bg-white/70 hover:shadow-md transition-all duration-200 transform hover:scale-[1.02]"
                                on:click=move |_| handle_social_signup("Google")
                            >
                                <svg class="w-5 h-5 mr-3" viewBox="0 0 24 24">
                                    <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
                                    <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
                                    <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
                                    <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
                                </svg>
                                "Sign up with Google"
                            </button>
                            
                            <button
                                type="button"
                                class="w-full flex items-center justify-center px-4 py-3 border border-gray-300/50 rounded-xl shadow-sm bg-white/50 backdrop-blur-sm text-sm font-medium text-gray-700 hover:bg-white/70 hover:shadow-md transition-all duration-200 transform hover:scale-[1.02]"
                                on:click=move |_| handle_social_signup("LinkedIn")
                            >
                                <svg class="w-5 h-5 mr-3" fill="#0077B5" viewBox="0 0 24 24">
                                    <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
                                </svg>
                                "Sign up with LinkedIn"
                            </button>
                        </div>

                        // Divider
                        <div class="relative my-6">
                            <div class="absolute inset-0 flex items-center">
                                <div class="w-full border-t border-gray-300/50"></div>
                            </div>
                            <div class="relative flex justify-center text-sm">
                                <span class="px-4 bg-white/50 text-gray-500 rounded-full">or create account manually</span>
                            </div>
                        </div>
                
                        <form class="space-y-6" on:submit=handle_submit>
                            <Show when=move || error.get().is_some()>
                                <div class="transform animate-shake">
                                    <Alert variant=AlertVariant::Error dismissible=true>
                                        {move || error.get().unwrap_or_default()}
                                    </Alert>
                                </div>
                            </Show>
                            
                            // User Type Selection
                            <div class="space-y-3">
                                <label class="block text-sm font-medium text-gray-700">
                                    "I want to..."
                                </label>
                                <div class="grid grid-cols-2 gap-3">
                                    <button
                                        type="button"
                                        class=move || {
                                            let base = "relative rounded-xl border p-4 flex flex-col items-center focus:outline-none transition-all duration-200 transform hover:scale-[1.02]";
                                            if user_type.get() == UserType::JobSeeker {
                                                format!("{} border-tiffany-500 ring-2 ring-tiffany-500/20 bg-tiffany-50/70", base)
                                            } else {
                                                format!("{} border-gray-300/50 hover:border-gray-400 bg-white/30", base)
                                            }
                                        }
                                        on:click=move |_| set_user_type.set(UserType::JobSeeker)
                                    >
                                        <svg class="w-8 h-8 mb-2 text-tiffany-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2-2v2m8 0V6a2 2 0 012 2v6a2 2 0 01-2 2H6a2 2 0 01-2-2V8a2 2 0 012-2V6"></path>
                                        </svg>
                                        <span class="text-sm font-medium text-gray-700">"Find Jobs"</span>
                                    </button>
                                    
                                    <button
                                        type="button"
                                        class=move || {
                                            let base = "relative rounded-xl border p-4 flex flex-col items-center focus:outline-none transition-all duration-200 transform hover:scale-[1.02]";
                                            if user_type.get() == UserType::Employer {
                                                format!("{} border-emerald-500 ring-2 ring-emerald-500/20 bg-emerald-50/70", base)
                                            } else {
                                                format!("{} border-gray-300/50 hover:border-gray-400 bg-white/30", base)
                                            }
                                        }
                                        on:click=move |_| set_user_type.set(UserType::Employer)
                                    >
                                        <svg class="w-8 h-8 mb-2 text-emerald-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
                                        </svg>
                                        <span class="text-sm font-medium text-gray-700">"Hire Talent"</span>
                                    </button>
                                </div>
                            </div>
                            
                            // Name Fields
                            <div class="grid grid-cols-2 gap-4">
                                <div class="space-y-2">
                                    <label for="first-name" class="block text-sm font-medium text-gray-700">
                                        "First name"
                                    </label>
                                    <div class="relative group">
                                        <input
                                            id="first-name"
                                            name="first-name"
                                            type="text"
                                            autocomplete="given-name"
                                            required
                                            class="block w-full px-3 py-3 border border-gray-300/50 rounded-xl shadow-sm placeholder-gray-400 bg-white/50 backdrop-blur-sm focus:outline-none focus:ring-2 focus:ring-tiffany-500 focus:border-transparent transition-all duration-200 hover:bg-white/70"
                                            placeholder="John"
                                            value=first_name
                                            on:input=move |ev| set_first_name.set(event_target_value(&ev))
                                        />
                                    </div>
                                </div>
                                <div class="space-y-2">
                                    <label for="last-name" class="block text-sm font-medium text-gray-700">
                                        "Last name"
                                    </label>
                                    <div class="relative group">
                                        <input
                                            id="last-name"
                                            name="last-name"
                                            type="text"
                                            autocomplete="family-name"
                                            required
                                            class="block w-full px-3 py-3 border border-gray-300/50 rounded-xl shadow-sm placeholder-gray-400 bg-white/50 backdrop-blur-sm focus:outline-none focus:ring-2 focus:ring-tiffany-500 focus:border-transparent transition-all duration-200 hover:bg-white/70"
                                            placeholder="Doe"
                                            value=last_name
                                            on:input=move |ev| set_last_name.set(event_target_value(&ev))
                                        />
                                    </div>
                                </div>
                            </div>
                            
                            // Company Name (only for employers)
                            <Show when=move || user_type.get() == UserType::Employer>
                                <div class="space-y-2 animate-fadeIn">
                                    <label for="company-name" class="block text-sm font-medium text-gray-700">
                                        "Company name"
                                    </label>
                                    <div class="relative group">
                                        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                            <svg class="h-5 w-5 text-gray-400 group-focus-within:text-emerald-500 transition-colors duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
                                            </svg>
                                        </div>
                                        <input
                                            id="company-name"
                                            name="company-name"
                                            type="text"
                                            autocomplete="organization"
                                            required
                                            class="block w-full pl-10 pr-3 py-3 border border-gray-300/50 rounded-xl shadow-sm placeholder-gray-400 bg-white/50 backdrop-blur-sm focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-all duration-200 hover:bg-white/70"
                                            placeholder="Acme Corporation"
                                            value=company_name
                                            on:input=move |ev| set_company_name.set(event_target_value(&ev))
                                        />
                                    </div>
                                </div>
                            </Show>
                            
                            // Email
                            <div class="space-y-2">
                                <label for="email-address" class="block text-sm font-medium text-gray-700">
                                    "Email address"
                                </label>
                                <div class="relative group">
                                    <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                        <svg class="h-5 w-5 text-gray-400 group-focus-within:text-tiffany-500 transition-colors duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207" />
                                        </svg>
                                    </div>
                                    <input
                                        id="email-address"
                                        name="email"
                                        type="email"
                                        autocomplete="email"
                                        required
                                        class="block w-full pl-10 pr-3 py-3 border border-gray-300/50 rounded-xl shadow-sm placeholder-gray-400 bg-white/50 backdrop-blur-sm focus:outline-none focus:ring-2 focus:ring-tiffany-500 focus:border-transparent transition-all duration-200 hover:bg-white/70"
                                        placeholder="john.doe@example.com"
                                        value=email
                                        on:input=move |ev| set_email.set(event_target_value(&ev))
                                    />
                                </div>
                            </div>
                            
                            // Password
                            <div class="space-y-2">
                                <label for="password" class="block text-sm font-medium text-gray-700">
                                    "Password"
                                </label>
                                <div class="relative group">
                                    <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                        <svg class="h-5 w-5 text-gray-400 group-focus-within:text-tiffany-500 transition-colors duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                                        </svg>
                                    </div>
                                    <input
                                        id="password"
                                        name="password"
                                        type=move || if show_password.get() { "text" } else { "password" }
                                        autocomplete="new-password"
                                        required
                                        class="block w-full pl-10 pr-12 py-3 border border-gray-300/50 rounded-xl shadow-sm placeholder-gray-400 bg-white/50 backdrop-blur-sm focus:outline-none focus:ring-2 focus:ring-tiffany-500 focus:border-transparent transition-all duration-200 hover:bg-white/70"
                                        placeholder="••••••••"
                                        value=password
                                        on:input=move |ev| set_password.set(event_target_value(&ev))
                                    />
                                    <button
                                        type="button"
                                        class="absolute inset-y-0 right-0 pr-3 flex items-center"
                                        on:click=move |_| set_show_password.set(!show_password.get())
                                    >
                                        <Show
                                            when=move || show_password.get()
                                            fallback=move || view! {
                                                <svg class="h-5 w-5 text-gray-400 hover:text-gray-600 transition-colors duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                                                </svg>
                                            }
                                        >
                                            <svg class="h-5 w-5 text-gray-400 hover:text-gray-600 transition-colors duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L3 3m6.878 6.878L21 21" />
                                            </svg>
                                        </Show>
                                    </button>
                                </div>
                                
                                // Password Strength Indicator
                                <Show when=move || !password.get().is_empty()>
                                    <div class="mt-2">
                                        <div class="flex items-center space-x-2">
                                            <div class="flex-1 h-2 bg-gray-200 rounded-full overflow-hidden">
                                                <div 
                                                    class=move || {
                                                        let (score, _) = password_strength.get();
                                                        let width = (score as f32 / 5.0 * 100.0) as u32;
                                                        let color = match score {
                                                            5 => "bg-green-500",
                                                            3..=4 => "bg-yellow-500",
                                                            1..=2 => "bg-red-500",
                                                            _ => "bg-gray-300",
                                                        };
                                                        format!("{} h-full transition-all duration-300", color)
                                                    }
                                                    style=move || {
                                                        let (score, _) = password_strength.get();
                                                        let width = (score as f32 / 5.0 * 100.0) as u32;
                                                        format!("width: {}%", width)
                                                    }
                                                ></div>
                                            </div>
                                            <span class=move || {
                                                let (score, _) = password_strength.get();
                                                let color = match score {
                                                    5 => "text-green-600",
                                                    3..=4 => "text-yellow-600",
                                                    1..=2 => "text-red-600",
                                                    _ => "text-gray-500",
                                                };
                                                format!("text-xs font-medium {}", color)
                                            }>
                                                {move || password_strength.get().1}
                                            </span>
                                        </div>
                                    </div>
                                </Show>
                            </div>
                            
                            // Confirm Password
                            <div class="space-y-2">
                                <label for="confirm-password" class="block text-sm font-medium text-gray-700">
                                    "Confirm password"
                                </label>
                                <div class="relative group">
                                    <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                        <svg class="h-5 w-5 text-gray-400 group-focus-within:text-tiffany-500 transition-colors duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                                        </svg>
                                    </div>
                                    <input
                                        id="confirm-password"
                                        name="confirm-password"
                                        type=move || if show_confirm_password.get() { "text" } else { "password" }
                                        autocomplete="new-password"
                                        required
                                        class=move || {
                                            let base = "block w-full pl-10 pr-12 py-3 border rounded-xl shadow-sm placeholder-gray-400 bg-white/50 backdrop-blur-sm focus:outline-none focus:ring-2 focus:border-transparent transition-all duration-200 hover:bg-white/70";
                                            if !confirm_password.get().is_empty() && password.get() != confirm_password.get() {
                                                format!("{} border-red-300 focus:ring-red-500", base)
                                            } else if !confirm_password.get().is_empty() && password.get() == confirm_password.get() {
                                                format!("{} border-green-300 focus:ring-green-500", base)
                                            } else {
                                                format!("{} border-gray-300/50 focus:ring-tiffany-500", base)
                                            }
                                        }
                                        placeholder="••••••••"
                                        value=confirm_password
                                        on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                                    />
                                    <button
                                        type="button"
                                        class="absolute inset-y-0 right-0 pr-3 flex items-center"
                                        on:click=move |_| set_show_confirm_password.set(!show_confirm_password.get())
                                    >
                                        <Show
                                            when=move || show_confirm_password.get()
                                            fallback=move || view! {
                                                <svg class="h-5 w-5 text-gray-400 hover:text-gray-600 transition-colors duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                                                </svg>
                                            }
                                        >
                                            <svg class="h-5 w-5 text-gray-400 hover:text-gray-600 transition-colors duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L3 3m6.878 6.878L21 21" />
                                            </svg>
                                        </Show>
                                    </button>
                                </div>
                                
                                // Password Match Indicator
                                <Show when=move || !confirm_password.get().is_empty()>
                                    <div class="flex items-center space-x-2 mt-1">
                                        <Show
                                            when=move || password.get() == confirm_password.get()
                                            fallback=move || view! {
                                                <div class="flex items-center text-red-600">
                                                    <svg class="h-4 w-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                                    </svg>
                                                    <span class="text-xs">"Passwords don't match"</span>
                                                </div>
                                            }
                                        >
                                            <div class="flex items-center text-green-600">
                                                <svg class="h-4 w-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                                                </svg>
                                                <span class="text-xs">"Passwords match"</span>
                                            </div>
                                        </Show>
                                    </div>
                                </Show>
                            </div>
                            
                            // Terms and Conditions
                            <div class="flex items-start space-x-3 p-4 bg-gray-50/50 rounded-xl">
                                <div class="flex items-center h-5 mt-0.5">
                                    <input
                                        id="agree-terms"
                                        name="agree-terms"
                                        type="checkbox"
                                        class="h-4 w-4 text-tiffany-600 focus:ring-tiffany-500 border-gray-300 rounded transition-colors duration-200"
                                        checked=agree_terms
                                        on:change=move |ev| set_agree_terms.set(event_target_checked(&ev))
                                    />
                                </div>
                                <div class="text-sm">
                                    <label for="agree-terms" class="font-medium text-gray-700">
                                        "I agree to the "
                                        <A href="/terms" class="text-tiffany-600 hover:text-tiffany-700 transition-colors duration-200 underline">
                                            "Terms and Conditions"
                                        </A>
                                        " and "
                                        <A href="/privacy" class="text-tiffany-600 hover:text-tiffany-700 transition-colors duration-200 underline">
                                            "Privacy Policy"
                                        </A>
                                        ". I understand that my information will be used to provide job matching services."
                                    </label>
                                </div>
                            </div>

                            // Submit Button
                            <div>
                                <button
                                    type="submit"
                                    disabled=loading.get() || !agree_terms.get() || password_strength.get().0 < 3
                                    class="group relative w-full flex justify-center py-3 px-4 border border-transparent text-sm font-medium rounded-xl text-white bg-gradient-to-r from-emerald-600 to-tiffany-600 hover:from-emerald-700 hover:to-tiffany-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-emerald-500 disabled:opacity-50 disabled:cursor-not-allowed transform hover:scale-[1.02] transition-all duration-200 shadow-lg hover:shadow-xl"
                                >
                                    <Show
                                        when=move || loading.get()
                                        fallback=move || view! {
                                            <span class="flex items-center">
                                                <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
                                                </svg>
                                                "Create Account"
                                            </span>
                                        }
                                    >
                                        <span class="flex items-center">
                                            <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
                                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                            </svg>
                                            "Creating account..."
                                        </span>
                                    </Show>
                                </button>
                            </div>
                        </form>

                        // Footer
                        <div class="mt-6 text-center">
                            <p class="text-xs text-gray-500">
                                "By creating an account, you'll receive notifications about new job opportunities and platform updates."
                            </p>
                        </div>
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
                
                .animate-float { animation: float 6s ease-in-out infinite; }
                .animate-float-delayed { animation: float-delayed 8s ease-in-out infinite; }
                .animate-pulse-slow { animation: pulse-slow 4s ease-in-out infinite; }
                .animate-shake { animation: shake 0.5s ease-in-out; }
                .animate-fadeIn { animation: fadeIn 0.3s ease-out; }
            "
            </style>
        </div>
    }
}