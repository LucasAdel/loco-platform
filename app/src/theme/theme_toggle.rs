use leptos::*;
use super::{Theme, use_theme_state};

/// Theme toggle component that allows users to switch between themes
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (theme, set_theme) = use_theme_state();
    
    let toggle_theme = move |_| {
        set_theme.update(|current| {
            *current = match current {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::System,
                Theme::System => Theme::Light,
            }
        });
    };
    
    view! {
        <button
            on:click=toggle_theme
            class="relative inline-flex items-center justify-center p-2 rounded-lg text-secondary hover:bg-secondary transition-base"
            aria-label="Toggle theme"
        >
            {move || match theme.get() {
                Theme::Light => view! {
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                            d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                    </svg>
                },
                Theme::Dark => view! {
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                            d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                    </svg>
                },
                Theme::System => view! {
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                            d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                    </svg>
                },
            }}
        </button>
    }
}

/// Theme dropdown component with all theme options
#[component]
pub fn ThemeDropdown() -> impl IntoView {
    let (theme, set_theme) = use_theme_state();
    let (is_open, set_is_open) = create_signal(false);
    
    let toggle_dropdown = move |_| {
        set_is_open.update(|open| *open = !*open);
    };
    
    let select_theme = move |new_theme: Theme| {
        set_theme.set(new_theme);
        set_is_open.set(false);
    };
    
    view! {
        <div class="relative">
            <button
                on:click=toggle_dropdown
                class="flex items-center gap-2 px-3 py-2 rounded-lg border border-primary bg-primary hover:bg-secondary transition-base"
            >
                <span class="text-sm font-medium">{move || theme.get().display_name()}</span>
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
            </button>
            
            <Show when=move || is_open.get()>
                <div class="absolute right-0 mt-2 w-48 rounded-lg border border-primary bg-primary shadow-lg z-50">
                    <div class="py-1">
                        <ThemeOption theme=Theme::Light current_theme=theme on_select=select_theme />
                        <ThemeOption theme=Theme::Dark current_theme=theme on_select=select_theme />
                        <ThemeOption theme=Theme::System current_theme=theme on_select=select_theme />
                    </div>
                </div>
            </Show>
        </div>
    }
}

/// Individual theme option in the dropdown
#[component]
fn ThemeOption<F>(
    theme: Theme,
    current_theme: ReadSignal<Theme>,
    on_select: F,
) -> impl IntoView
where
    F: Fn(Theme) + 'static,
{
    let is_selected = move || current_theme.get() == theme;
    let handle_click = move |_| on_select(theme);
    
    view! {
        <button
            on:click=handle_click
            class="w-full flex items-center gap-3 px-4 py-2 text-sm hover:bg-secondary transition-base"
            class:bg-secondary=is_selected
        >
            <span class="w-5">
                {match theme {
                    Theme::Light => view! {
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                        </svg>
                    },
                    Theme::Dark => view! {
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                        </svg>
                    },
                    Theme::System => view! {
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                        </svg>
                    },
                }}
            </span>
            <span class="flex-1 text-left">{theme.display_name()}</span>
            <Show when=is_selected>
                <svg class="w-4 h-4 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
            </Show>
        </button>
    }
}