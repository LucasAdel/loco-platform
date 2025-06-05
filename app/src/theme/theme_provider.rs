use leptos::*;
use leptos_meta::*;
use web_sys::window;
use super::{Theme, ThemeContext, THEME_CSS};

/// Theme provider component that manages the application theme
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    // Get initial theme from localStorage or system preference
    let initial_theme = get_initial_theme();
    
    // Create theme signals
    let (theme, set_theme) = create_signal(initial_theme);
    
    // Provide theme context
    provide_context(ThemeContext(theme, set_theme));
    
    // Apply theme class to document root
    create_effect(move |_| {
        let current_theme = theme.get();
        apply_theme_to_document(current_theme);
        save_theme_preference(current_theme);
    });
    
    view! {
        <>
            // Inject theme CSS
            <Style>{THEME_CSS}</Style>
            
            // Render children
            {children()}
        </>
    }
}

/// Get initial theme from localStorage or system preference
fn get_initial_theme() -> Theme {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(stored_theme)) = storage.get_item("theme") {
                return Theme::from(stored_theme);
            }
        }
        
        // Check system preference
        if let Ok(prefers_dark) = window.match_media("(prefers-color-scheme: dark)") {
            if let Some(media_query) = prefers_dark {
                if media_query.matches() {
                    return Theme::Dark;
                }
            }
        }
    }
    
    Theme::Light
}

/// Apply theme class to document root
fn apply_theme_to_document(theme: Theme) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(root) = document.document_element() {
                // Remove all theme classes
                let _ = root.class_list().remove_3("light", "dark", "auto");
                
                // Add current theme class
                let _ = root.class_list().add_1(theme.class_name());
            }
        }
    }
}

/// Save theme preference to localStorage
fn save_theme_preference(theme: Theme) {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("theme", theme.class_name());
        }
    }
}

/// Hook to get and set the current theme
pub fn use_theme_state() -> (ReadSignal<Theme>, WriteSignal<Theme>) {
    let context = use_context::<ThemeContext>()
        .expect("Theme context not found. Make sure to wrap your app with ThemeProvider");
    (context.0, context.1)
}