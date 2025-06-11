use leptos::*;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme = RwSignal::new(Theme::Light);
    
    // Load theme from localStorage
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(stored_theme)) = storage.get_item("theme") {
                    if stored_theme == "dark" {
                        theme.set(Theme::Dark);
                    }
                }
            }
        }
    });
    
    // Apply theme class to document
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(root) = document.document_element() {
                    match theme.get() {
                        Theme::Dark => {
                            let _ = root.class_list().add_1("dark");
                        }
                        Theme::Light => {
                            let _ = root.class_list().remove_1("dark");
                        }
                    }
                }
            }
        }
    });
    
    provide_context(theme);
    
    children()
}

/// Hook to access the current theme
pub fn use_theme() -> RwSignal<Theme> {
    use_context::<RwSignal<Theme>>().expect("ThemeProvider not found")
}