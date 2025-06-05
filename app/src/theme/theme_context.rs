use leptos::*;
use super::Theme;

/// Context key for the theme
#[derive(Clone, Copy, Debug)]
pub struct ThemeContext(pub ReadSignal<Theme>, pub WriteSignal<Theme>);

/// Hook to use the theme context
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
        .expect("Theme context not found. Make sure to wrap your app with ThemeProvider")
}