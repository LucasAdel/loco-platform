// Theme module for Loco Platform
pub mod theme_provider;
pub mod theme_context;
pub mod theme_toggle;

pub use theme_provider::*;
pub use theme_context::*;
pub use theme_toggle::*;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Available themes in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Theme {
    /// Get the CSS class name for the theme
    pub fn class_name(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "auto",
        }
    }
    
    /// Get display name for the theme
    pub fn display_name(&self) -> &'static str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::System => "System",
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::System
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl From<String> for Theme {
    fn from(s: String) -> Self {
        match s.as_str() {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            "system" => Theme::System,
            _ => Theme::System,
        }
    }
}

impl From<Theme> for String {
    fn from(theme: Theme) -> Self {
        theme.class_name().to_string()
    }
}

/// CSS custom properties for theming
pub const THEME_CSS: &str = r#"
:root {
    /* Light theme colours */
    --color-primary: 59 130 246;
    --color-primary-hover: 37 99 235;
    --color-secondary: 107 114 128;
    --color-secondary-hover: 75 85 99;
    --color-success: 34 197 94;
    --color-danger: 239 68 68;
    --color-warning: 251 191 36;
    --color-info: 59 130 246;
    
    /* Background colours */
    --color-bg-primary: 255 255 255;
    --color-bg-secondary: 249 250 251;
    --color-bg-tertiary: 243 244 246;
    
    /* Text colours */
    --color-text-primary: 17 24 39;
    --color-text-secondary: 75 85 99;
    --color-text-tertiary: 156 163 175;
    --color-text-inverse: 255 255 255;
    
    /* Border colours */
    --color-border-primary: 229 231 235;
    --color-border-secondary: 209 213 219;
    
    /* Shadow colours */
    --color-shadow: 0 0 0;
    
    /* Spacing */
    --spacing-xs: 0.25rem;
    --spacing-sm: 0.5rem;
    --spacing-md: 1rem;
    --spacing-lg: 1.5rem;
    --spacing-xl: 2rem;
    --spacing-2xl: 3rem;
    
    /* Border radius */
    --radius-none: 0;
    --radius-sm: 0.25rem;
    --radius-md: 0.375rem;
    --radius-lg: 0.5rem;
    --radius-xl: 0.75rem;
    --radius-2xl: 1rem;
    --radius-full: 9999px;
    
    /* Transitions */
    --transition-fast: 150ms ease-in-out;
    --transition-base: 250ms ease-in-out;
    --transition-slow: 350ms ease-in-out;
    
    /* Font sizes */
    --font-size-xs: 0.75rem;
    --font-size-sm: 0.875rem;
    --font-size-base: 1rem;
    --font-size-lg: 1.125rem;
    --font-size-xl: 1.25rem;
    --font-size-2xl: 1.5rem;
    --font-size-3xl: 1.875rem;
    --font-size-4xl: 2.25rem;
    
    /* Font weights */
    --font-weight-thin: 100;
    --font-weight-light: 300;
    --font-weight-normal: 400;
    --font-weight-medium: 500;
    --font-weight-semibold: 600;
    --font-weight-bold: 700;
    --font-weight-extrabold: 800;
    
    /* Z-index layers */
    --z-index-dropdown: 1000;
    --z-index-sticky: 1020;
    --z-index-fixed: 1030;
    --z-index-modal-backdrop: 1040;
    --z-index-modal: 1050;
    --z-index-popover: 1060;
    --z-index-tooltip: 1070;
}

/* Dark theme */
.dark {
    /* Background colours */
    --color-bg-primary: 17 24 39;
    --color-bg-secondary: 31 41 55;
    --color-bg-tertiary: 55 65 81;
    
    /* Text colours */
    --color-text-primary: 243 244 246;
    --color-text-secondary: 209 213 219;
    --color-text-tertiary: 156 163 175;
    --color-text-inverse: 17 24 39;
    
    /* Border colours */
    --color-border-primary: 75 85 99;
    --color-border-secondary: 55 65 81;
    
    /* Adjusted colours for dark mode */
    --color-primary: 96 165 250;
    --color-primary-hover: 147 197 253;
    --color-secondary: 156 163 175;
    --color-secondary-hover: 209 213 219;
}

/* Apply theme to body */
body {
    background-color: rgb(var(--color-bg-primary));
    color: rgb(var(--color-text-primary));
    transition: background-color var(--transition-base), color var(--transition-base);
}

/* Utility classes using CSS custom properties */
.bg-primary { background-color: rgb(var(--color-bg-primary)); }
.bg-secondary { background-color: rgb(var(--color-bg-secondary)); }
.bg-tertiary { background-color: rgb(var(--color-bg-tertiary)); }

.text-primary { color: rgb(var(--color-text-primary)); }
.text-secondary { color: rgb(var(--color-text-secondary)); }
.text-tertiary { color: rgb(var(--color-text-tertiary)); }

.border-primary { border-color: rgb(var(--color-border-primary)); }
.border-secondary { border-color: rgb(var(--color-border-secondary)); }

.shadow-sm { box-shadow: 0 1px 2px 0 rgba(var(--color-shadow) / 0.05); }
.shadow { box-shadow: 0 1px 3px 0 rgba(var(--color-shadow) / 0.1), 0 1px 2px -1px rgba(var(--color-shadow) / 0.1); }
.shadow-md { box-shadow: 0 4px 6px -1px rgba(var(--color-shadow) / 0.1), 0 2px 4px -2px rgba(var(--color-shadow) / 0.1); }
.shadow-lg { box-shadow: 0 10px 15px -3px rgba(var(--color-shadow) / 0.1), 0 4px 6px -4px rgba(var(--color-shadow) / 0.1); }
.shadow-xl { box-shadow: 0 20px 25px -5px rgba(var(--color-shadow) / 0.1), 0 8px 10px -6px rgba(var(--color-shadow) / 0.1); }

/* Transitions */
.transition-fast { transition-duration: var(--transition-fast); }
.transition-base { transition-duration: var(--transition-base); }
.transition-slow { transition-duration: var(--transition-slow); }

/* System theme detection */
@media (prefers-color-scheme: dark) {
    .auto {
        /* Dark theme colours */
        --color-bg-primary: 17 24 39;
        --color-bg-secondary: 31 41 55;
        --color-bg-tertiary: 55 65 81;
        
        --color-text-primary: 243 244 246;
        --color-text-secondary: 209 213 219;
        --color-text-tertiary: 156 163 175;
        --color-text-inverse: 17 24 39;
        
        --color-border-primary: 75 85 99;
        --color-border-secondary: 55 65 81;
        
        --color-primary: 96 165 250;
        --color-primary-hover: 147 197 253;
        --color-secondary: 156 163 175;
        --color-secondary-hover: 209 213 219;
    }
}
"#;