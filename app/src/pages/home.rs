use leptos::*;
use leptos_router::*;
use crate::components::modern_ui::*;

#[component]
pub fn Home() -> impl IntoView {
    // Animation signals
    let (hero_loaded, set_hero_loaded) = create_signal(false);
    
    // Trigger animations on mount
    create_effect(move |_| {
        set_timeout(
            move || set_hero_loaded(true),
            std::time::Duration::from_millis(100),
        );
    });

    view! {
        <div class="min-h-screen bg-black text-white relative overflow-hidden">
            // Neural background effect
            <div class="fixed inset-0 z-0">
                <div class="absolute inset-0 bg-gradient-to-br from-indigo-900/20 via-purple-900/20 to-pink-900/20"></div>
                <div class="absolute top-0 left-0 w-96 h-96 bg-indigo-500/20 rounded-full blur-3xl animate-pulse"></div>
                <div class="absolute bottom-0 right-0 w-96 h-96 bg-purple-500/20 rounded-full blur-3xl animate-pulse" style="animation-delay: 2s;"></div>
            </div>
            
            // Hero Section
            <section class="relative z-10 min-h-screen flex items-center justify-center px-4">
                <div class="max-w-7xl mx-auto text-center">
                    <div class=move || if hero_loaded() { "opacity-100 transform translate-y-0 transition-all duration-1000" } else { "opacity-0 transform translate-y-10" }>
                        <h1 class="text-6xl md:text-7xl lg:text-8xl font-black mb-8">
                            "Find Your Perfect"
                            <span class="block mt-2">
                                <HolographicText size="text-6xl md:text-7xl lg:text-8xl">
                                    "Pharmacy Role"
                                </HolographicText>
                            </span>
                        </h1>
                        
                        <p class="text-xl md:text-2xl text-gray-300 mb-12 max-w-3xl mx-auto">
                            "Australia's premier platform connecting pharmacy professionals with their dream careers. 
                            Powered by AI, designed for excellence."
                        </p>
                        
                        <div class="flex flex-col sm:flex-row gap-6 justify-center">
                            <A href="/jobs">
                                <NeuralButton size="lg" full_width=true>
                                    <span class="flex items-center gap-2">
                                        <i class="fas fa-briefcase"></i>
                                        "Browse 500+ Jobs"
                                    </span>
                                </NeuralButton>
                            </A>
                            <A href="/register">
                                <NeuralButton variant="secondary" size="lg" full_width=true>
                                    <span class="flex items-center gap-2">
                                        <i class="fas fa-rocket"></i>
                                        "Post a Job - Free"
                                    </span>
                                </NeuralButton>
                            </A>
                        </div>
                    </div>
                    
                    // Stats
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mt-20">
                        <FloatingElement duration=6.0 delay=0.0>
                            <MetricCard
                                title="Active Professionals"
                                value=10000
                                icon="fa-users"
                                change=12.5
                                color="blue"
                            />
                        </FloatingElement>
                        <FloatingElement duration=6.0 delay=0.5>
                            <MetricCard
                                title="Partner Pharmacies"
                                value=2500
                                icon="fa-building"
                                change=8.3
                                color="purple"
                            />
                        </FloatingElement>
                        <FloatingElement duration=6.0 delay=1.0>
                            <MetricCard
                                title="Success Rate"
                                value=98
                                icon="fa-trophy"
                                change=2.1
                                color="green"
                            />
                        </FloatingElement>
                    </div>
                </div>
                
                // Scroll indicator
                <div class="absolute bottom-8 left-1/2 transform -translate-x-1/2 animate-bounce">
                    <i class="fas fa-chevron-down text-2xl text-gray-400"></i>
                </div>
            </section>
            
            // Features Section
            <section class="relative z-10 py-20 px-4">
                <div class="max-w-7xl mx-auto">
                    <div class="text-center mb-16">
                        <h2 class="text-5xl font-bold mb-4">
                            <HolographicText>"Revolutionary Features"</HolographicText>
                        </h2>
                        <p class="text-xl text-gray-400">Everything you need to transform your career</p>
                    </div>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        <QuantumCard hover_effect=true>
                            <div class="text-center">
                                <div class="w-20 h-20 mx-auto mb-6 rounded-2xl bg-gradient-to-br from-indigo-500 to-purple-500 flex items-center justify-center">
                                    <i class="fas fa-brain text-3xl text-white"></i>
                                </div>
                                <h3 class="text-2xl font-bold mb-4">AI-Powered Matching</h3>
                                <p class="text-gray-400">
                                    "Our neural network analyses your skills and preferences to find perfect job matches instantly."
                                </p>
                            </div>
                        </QuantumCard>
                        
                        <QuantumCard hover_effect=true>
                            <div class="text-center">
                                <div class="w-20 h-20 mx-auto mb-6 rounded-2xl bg-gradient-to-br from-blue-500 to-cyan-500 flex items-center justify-center">
                                    <i class="fas fa-bolt text-3xl text-white"></i>
                                </div>
                                <h3 class="text-2xl font-bold mb-4">Real-time Updates</h3>
                                <p class="text-gray-400">
                                    "Get instant notifications when new opportunities match your profile. Never miss out."
                                </p>
                            </div>
                        </QuantumCard>
                        
                        <QuantumCard hover_effect=true>
                            <div class="text-center">
                                <div class="w-20 h-20 mx-auto mb-6 rounded-2xl bg-gradient-to-br from-purple-500 to-pink-500 flex items-center justify-center">
                                    <i class="fas fa-chart-line text-3xl text-white"></i>
                                </div>
                                <h3 class="text-2xl font-bold mb-4">Smart Analytics</h3>
                                <p class="text-gray-400">
                                    "Track your application success, salary trends, and market insights with powerful dashboards."
                                </p>
                            </div>
                        </QuantumCard>
                        
                        <QuantumCard hover_effect=true>
                            <div class="text-center">
                                <div class="w-20 h-20 mx-auto mb-6 rounded-2xl bg-gradient-to-br from-green-500 to-teal-500 flex items-center justify-center">
                                    <i class="fas fa-map-marked-alt text-3xl text-white"></i>
                                </div>
                                <h3 class="text-2xl font-bold mb-4">Interactive Map Search</h3>
                                <p class="text-gray-400">
                                    "Visualise opportunities near you with our advanced mapping technology and commute calculator."
                                </p>
                            </div>
                        </QuantumCard>
                        
                        <QuantumCard hover_effect=true>
                            <div class="text-center">
                                <div class="w-20 h-20 mx-auto mb-6 rounded-2xl bg-gradient-to-br from-orange-500 to-red-500 flex items-center justify-center">
                                    <i class="fas fa-clock text-3xl text-white"></i>
                                </div>
                                <h3 class="text-2xl font-bold mb-4">One-Click Apply</h3>
                                <p class="text-gray-400">
                                    "Apply to multiple positions instantly with your saved profile. No repetitive forms."
                                </p>
                            </div>
                        </QuantumCard>
                        
                        <QuantumCard hover_effect=true>
                            <div class="text-center">
                                <div class="w-20 h-20 mx-auto mb-6 rounded-2xl bg-gradient-to-br from-indigo-500 to-purple-500 flex items-center justify-center">
                                    <i class="fas fa-shield-alt text-3xl text-white"></i>
                                </div>
                                <h3 class="text-2xl font-bold mb-4">Bank-Level Security</h3>
                                <p class="text-gray-400">
                                    "Your data is protected with military-grade encryption and Australian privacy compliance."
                                </p>
                            </div>
                        </QuantumCard>
                    </div>
                </div>
            </section>
            
            // How It Works
            <section class="relative z-10 py-20 px-4 bg-white/5">
                <div class="max-w-7xl mx-auto">
                    <div class="text-center mb-16">
                        <h2 class="text-5xl font-bold mb-4">
                            "How " <HolographicText>"Loco"</HolographicText> " Works"
                        </h2>
                        <p class="text-xl text-gray-400">Get started in just 3 simple steps</p>
                    </div>
                    
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 relative">
                        // Connection line
                        <div class="hidden md:block absolute top-1/2 left-1/4 right-1/4 h-0.5 bg-gradient-to-r from-blue-500 to-purple-500 transform -translate-y-1/2"></div>
                        
                        <div class="relative z-10">
                            <QuantumCard hover_effect=true class="text-center">
                                <div class="w-24 h-24 mx-auto mb-6 rounded-full bg-gradient-to-br from-indigo-500 to-purple-500 flex items-center justify-center text-3xl font-bold">
                                    "1"
                                </div>
                                <h3 class="text-2xl font-bold mb-4">Create Your Profile</h3>
                                <p class="text-gray-400">
                                    "Tell us about your experience, qualifications, and career goals in just 2 minutes."
                                </p>
                            </QuantumCard>
                        </div>
                        
                        <div class="relative z-10">
                            <QuantumCard hover_effect=true class="text-center">
                                <div class="w-24 h-24 mx-auto mb-6 rounded-full bg-gradient-to-br from-purple-500 to-pink-500 flex items-center justify-center text-3xl font-bold">
                                    "2"
                                </div>
                                <h3 class="text-2xl font-bold mb-4">Get Matched</h3>
                                <p class="text-gray-400">
                                    "Our AI instantly matches you with relevant opportunities from top pharmacies."
                                </p>
                            </QuantumCard>
                        </div>
                        
                        <div class="relative z-10">
                            <QuantumCard hover_effect=true class="text-center">
                                <div class="w-24 h-24 mx-auto mb-6 rounded-full bg-gradient-to-br from-pink-500 to-red-500 flex items-center justify-center text-3xl font-bold">
                                    "3"
                                </div>
                                <h3 class="text-2xl font-bold mb-4">Land Your Dream Job</h3>
                                <p class="text-gray-400">
                                    "Apply with one click and track your applications. It's that simple!"
                                </p>
                            </QuantumCard>
                        </div>
                    </div>
                </div>
            </section>
            
            // CTA Section
            <section class="relative z-10 py-20 px-4">
                <div class="max-w-4xl mx-auto text-center">
                    <QuantumCard hover_effect=false class="p-12 bg-gradient-to-br from-indigo-600/20 to-purple-600/20">
                        <h2 class="text-4xl md:text-5xl font-bold mb-6">
                            "Ready to Transform Your"
                            <span class="block mt-2">
                                <HolographicText size="text-4xl md:text-5xl">"Pharmacy Career?"</HolographicText>
                            </span>
                        </h2>
                        <p class="text-xl text-gray-300 mb-8">
                            "Join thousands of pharmacy professionals already using Loco to advance their careers."
                        </p>
                        <div class="flex flex-col sm:flex-row gap-4 justify-center">
                            <A href="/register">
                                <NeuralButton size="lg">
                                    <span class="flex items-center gap-2">
                                        <i class="fas fa-rocket"></i>
                                        "Start Free Today"
                                    </span>
                                </NeuralButton>
                            </A>
                            <A href="/jobs">
                                <NeuralButton variant="secondary" size="lg">
                                    <span class="flex items-center gap-2">
                                        <i class="fas fa-search"></i>
                                        "Browse Jobs First"
                                    </span>
                                </NeuralButton>
                            </A>
                        </div>
                    </QuantumCard>
                </div>
            </section>
        </div>
    }
}