use leptos::*;
use leptos::prelude::*;
use leptos::ev::MouseEvent;

/// Neural gradient button with advanced hover effects
#[component]
pub fn NeuralButton(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] on_click: Option<Box<dyn Fn(MouseEvent) + 'static>>,
    #[prop(optional)] variant: Option<String>,
    #[prop(optional)] size: Option<String>,
    #[prop(optional)] full_width: Option<bool>,
    #[prop(optional)] loading: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
) -> impl IntoView {
    let variant = variant.unwrap_or_else(|| "primary".to_string());
    let size = size.unwrap_or_else(|| "md".to_string());
    let full_width = full_width.unwrap_or(false);
    let loading = loading.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);

    let base_classes = "relative overflow-hidden font-semibold rounded-xl transition-all duration-300 transform hover:scale-105 hover:shadow-2xl";
    
    let variant_classes = match variant.as_str() {
        "primary" => "bg-gradient-to-r from-indigo-600 via-purple-600 to-pink-500 text-white hover:from-indigo-700 hover:via-purple-700 hover:to-pink-600",
        "secondary" => "bg-white/10 backdrop-blur-md border border-white/20 text-white hover:bg-white/20",
        "ghost" => "bg-transparent hover:bg-white/10 text-white",
        _ => "bg-gradient-to-r from-indigo-600 via-purple-600 to-pink-500 text-white",
    };
    
    let size_classes = match size.as_str() {
        "sm" => "px-4 py-2 text-sm",
        "md" => "px-6 py-3 text-base",
        "lg" => "px-8 py-4 text-lg",
        _ => "px-6 py-3 text-base",
    };
    
    let width_class = if full_width { "w-full" } else { "" };
    let disabled_class = if disabled || loading { "opacity-50 cursor-not-allowed" } else { "cursor-pointer" };
    
    let classes = format!("{} {} {} {} {}", base_classes, variant_classes, size_classes, width_class, disabled_class);

    view! {
        <button
            class=classes
            on:click=move |ev| {
                if !disabled && !loading {
                    if let Some(handler) = &on_click {
                        handler(ev);
                    }
                }
            }
            disabled=disabled || loading
        >
            // Shimmer effect overlay
            <div class="absolute inset-0 -translate-x-full animate-[shimmer_2s_infinite] bg-gradient-to-r from-transparent via-white/20 to-transparent"></div>
            
            // Button content
            <div class="relative z-10 flex items-center justify-center gap-2">
                {if loading {
                    view! {
                        <div class="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                    }.into_view()
                } else {
                    view! { <></> }.into_view()
                }}
                {children.map(|c| c())}
            </div>
        </button>
    }
}

/// Quantum glass card with depth effects
#[component]
pub fn QuantumCard(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] hover_effect: Option<bool>,
    #[prop(optional)] padding: Option<String>,
) -> impl IntoView {
    let hover_effect = hover_effect.unwrap_or(true);
    let padding = padding.unwrap_or_else(|| "p-6".to_string());
    let custom_class = class.unwrap_or_default();
    
    let base_classes = "relative overflow-hidden rounded-2xl bg-gradient-to-br from-white/5 to-white/10 backdrop-blur-xl border border-white/10";
    let hover_classes = if hover_effect {
        "transition-all duration-300 hover:transform hover:-translate-y-1 hover:shadow-2xl hover:border-white/20"
    } else {
        ""
    };
    
    let classes = format!("{} {} {} {}", base_classes, hover_classes, padding, custom_class);

    view! {
        <div class=classes>
            // Top gradient line
            <div class="absolute top-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-white/50 to-transparent"></div>
            
            // Content
            <div class="relative z-10">
                {children.map(|c| c())}
            </div>
            
            // Glow effect on hover
            {if hover_effect {
                view! {
                    <div class="absolute inset-0 opacity-0 hover:opacity-100 transition-opacity duration-300">
                        <div class="absolute inset-0 bg-gradient-to-br from-indigo-500/10 via-purple-500/10 to-pink-500/10"></div>
                    </div>
                }.into_view()
            } else {
                view! { <></> }.into_view()
            }}
        </div>
    }
}

/// Holographic text effect
#[component]
pub fn HolographicText(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] size: Option<String>,
    #[prop(optional)] weight: Option<String>,
) -> impl IntoView {
    let size = size.unwrap_or_else(|| "text-4xl".to_string());
    let weight = weight.unwrap_or_else(|| "font-bold".to_string());
    
    let classes = format!(
        "{} {} bg-gradient-to-r from-blue-400 via-purple-500 to-pink-500 bg-clip-text text-transparent animate-gradient-x bg-300%",
        size, weight
    );

    view! {
        <span class=classes>
            {children.map(|c| c())}
        </span>
    }
}

/// Neural input field with glow effects
#[component]
pub fn NeuralInput(
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static>>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] input_type: Option<String>,
    #[prop(optional)] icon: Option<String>,
    #[prop(optional)] error: Option<RwSignal<Option<String>>>,
) -> impl IntoView {
    let input_type = input_type.unwrap_or_else(|| "text".to_string());
    let placeholder = placeholder.unwrap_or_default();
    
    let base_classes = "w-full px-4 py-3 bg-white/5 border-2 border-white/10 rounded-xl text-white placeholder-gray-400 transition-all duration-300 focus:outline-none focus:border-indigo-500 focus:bg-white/10 focus:shadow-[0_0_20px_rgba(99,102,241,0.3)]";
    let icon_padding = if icon.is_some() { "pl-12" } else { "" };
    
    let classes = format!("{} {}", base_classes, icon_padding);

    view! {
        <div class="relative">
            {icon.map(|i| view! {
                <div class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400">
                    <i class=format!("fas {}", i)></i>
                </div>
            })}
            
            <input
                type=input_type
                class=classes
                placeholder=placeholder
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    if let Some(signal) = value {
                        signal.set(val.clone());
                    }
                    if let Some(handler) = &on_input {
                        handler(val);
                    }
                }
                prop:value=move || value.map(|v| v.get()).unwrap_or_default()
            />
            
            {move || error.and_then(|e| e.get()).map(|err| view! {
                <p class="mt-2 text-sm text-red-400">{err}</p>
            })}
        </div>
    }
}

/// Floating animation wrapper
#[component]
pub fn FloatingElement(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] duration: Option<f32>,
    #[prop(optional)] delay: Option<f32>,
) -> impl IntoView {
    let duration = duration.unwrap_or(6.0);
    let delay = delay.unwrap_or(0.0);
    
    let style = format!(
        "animation: float {}s ease-in-out {}s infinite",
        duration, delay
    );

    view! {
        <div style=style class="floating-element">
            {children.map(|c| c())}
        </div>
    }
}

/// Metric card with animated counter
#[component]
pub fn MetricCard(
    title: String,
    value: i32,
    #[prop(optional)] icon: Option<String>,
    #[prop(optional)] change: Option<f32>,
    #[prop(optional)] color: Option<String>,
) -> impl IntoView {
    let color = color.unwrap_or_else(|| "blue".to_string());
    let (current_value, set_current_value) = create_signal(0);
    
    // Animate counter on mount
    create_effect(move |_| {
        let increment = value / 60; // 60 frames for ~1 second animation
        let mut current = 0;
        
        let interval = set_interval_with_handle(
            move || {
                current += increment;
                if current >= value {
                    current = value;
                    set_current_value(current);
                    // Clear interval when done
                } else {
                    set_current_value(current);
                }
            },
            std::time::Duration::from_millis(16),
        );
        
        on_cleanup(move || {
            interval.clear();
        });
    });

    let icon_bg = format!("bg-{}-500/20", color);
    let icon_color = format!("text-{}-500", color);
    let change_color = if change.unwrap_or(0.0) >= 0.0 { "text-green-400" } else { "text-red-400" };

    view! {
        <QuantumCard hover_effect=true>
            <div class="flex items-center justify-between mb-4">
                {icon.map(|i| view! {
                    <div class=format!("w-12 h-12 rounded-lg {} flex items-center justify-center", icon_bg)>
                        <i class=format!("fas {} {}", i, icon_color)></i>
                    </div>
                })}
                
                {change.map(|c| view! {
                    <span class=format!("text-sm {}", change_color)>
                        {if c >= 0.0 { "+" } else { "" }}{c}"%"
                    </span>
                })}
            </div>
            
            <h3 class="text-3xl font-bold mb-1">
                {move || current_value.get()}
            </h3>
            <p class="text-gray-400">{title}</p>
        </QuantumCard>
    }
}

/// Loading skeleton with shimmer effect
#[component]
pub fn SkeletonLoader(
    #[prop(optional)] width: Option<String>,
    #[prop(optional)] height: Option<String>,
    #[prop(optional)] rounded: Option<String>,
) -> impl IntoView {
    let width = width.unwrap_or_else(|| "w-full".to_string());
    let height = height.unwrap_or_else(|| "h-4".to_string());
    let rounded = rounded.unwrap_or_else(|| "rounded".to_string());
    
    let classes = format!(
        "{} {} {} bg-gradient-to-r from-gray-800 via-gray-700 to-gray-800 bg-size-200 animate-shimmer",
        width, height, rounded
    );

    view! {
        <div class=classes></div>
    }
}

/// Notification toast component
#[component]
pub fn Toast(
    message: String,
    #[prop(optional)] variant: Option<String>,
    #[prop(optional)] duration: Option<u32>,
    #[prop(optional)] on_close: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let variant = variant.unwrap_or_else(|| "info".to_string());
    let duration = duration.unwrap_or(5000);
    let (visible, set_visible) = create_signal(true);
    
    // Auto-hide after duration
    if duration > 0 {
        set_timeout(
            move || {
                set_visible(false);
                if let Some(handler) = on_close {
                    handler();
                }
            },
            std::time::Duration::from_millis(duration as u64),
        );
    }
    
    let (icon, bg_color) = match variant.as_str() {
        "success" => ("fa-check-circle", "from-green-500/20 to-green-600/20 border-green-500/30"),
        "error" => ("fa-exclamation-circle", "from-red-500/20 to-red-600/20 border-red-500/30"),
        "warning" => ("fa-exclamation-triangle", "from-yellow-500/20 to-yellow-600/20 border-yellow-500/30"),
        _ => ("fa-info-circle", "from-blue-500/20 to-blue-600/20 border-blue-500/30"),
    };

    view! {
        <Show when=move || visible.get()>
            <div class=format!(
                "fixed bottom-4 right-4 z-50 max-w-sm bg-gradient-to-r {} backdrop-blur-xl border rounded-xl p-4 shadow-2xl transform transition-all duration-300 hover:scale-105",
                bg_color
            )>
                <div class="flex items-start gap-3">
                    <i class=format!("fas {} text-xl", icon)></i>
                    <div class="flex-1">
                        <p class="text-white">{message}</p>
                    </div>
                    <button
                        class="text-gray-400 hover:text-white transition-colors"
                        on:click=move |_| {
                            set_visible(false);
                            if let Some(handler) = &on_close {
                                handler();
                            }
                        }
                    >
                        <i class="fas fa-times"></i>
                    </button>
                </div>
            </div>
        </Show>
    }
}

// Helper function for set_interval (would need to be implemented properly)
fn set_interval_with_handle<F>(mut f: F, duration: std::time::Duration) -> IntervalHandle
where
    F: FnMut() + 'static,
{
    // This is a simplified implementation
    // In a real app, you'd use wasm-bindgen to call window.setInterval
    IntervalHandle { id: 0 }
}

struct IntervalHandle {
    id: i32,
}

impl IntervalHandle {
    fn clear(self) {
        // Clear the interval
    }
}