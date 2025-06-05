use leptos::*;

/// Validation result type
pub type ValidationResult = Result<(), String>;

/// Validation function type
pub type ValidationFn = Box<dyn Fn(&str) -> ValidationResult>;

/// Input component with built-in validation
#[component]
pub fn Input(
    /// Input type (text, email, password, etc.)
    #[prop(default = "text")]
    input_type: &'static str,
    
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<&'static str>,
    
    /// Label for the input
    #[prop(optional)]
    label: Option<&'static str>,
    
    /// Input value signal
    value: RwSignal<String>,
    
    /// Validation functions
    #[prop(optional)]
    validators: Vec<ValidationFn>,
    
    /// Whether the input is required
    #[prop(default = false)]
    required: bool,
    
    /// Whether the input is disabled
    #[prop(default = false)]
    disabled: bool,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
    
    /// Icon to display inside the input
    #[prop(optional)]
    icon: Option<&'static str>,
    
    /// Helper text
    #[prop(optional)]
    helper_text: Option<&'static str>,
) -> impl IntoView {
    let (error, set_error) = create_signal(None::<String>);
    let (touched, set_touched) = create_signal(false);
    
    // Validate input
    let validate = move |value: &str| {
        if required && value.trim().is_empty() {
            set_error.set(Some("This field is required".to_string()));
            return false;
        }
        
        for validator in &validators {
            if let Err(msg) = validator(value) {
                set_error.set(Some(msg));
                return false;
            }
        }
        
        set_error.set(None);
        true
    };
    
    // Handle input change
    let on_input = move |ev| {
        let new_value = event_target_value(&ev);
        value.set(new_value.clone());
        
        if touched.get() {
            validate(&new_value);
        }
    };
    
    // Handle blur
    let on_blur = move |_| {
        set_touched.set(true);
        validate(&value.get());
    };
    
    let input_classes = move || {
        let base_classes = "block w-full rounded-lg border transition-base focus:outline-none focus:ring-2";
        let state_classes = if error.get().is_some() && touched.get() {
            "border-red-500 focus:border-red-500 focus:ring-red-500"
        } else {
            "border-primary focus:border-blue-500 focus:ring-blue-500"
        };
        let padding_classes = if icon.is_some() {
            "pl-10 pr-3 py-2"
        } else {
            "px-3 py-2"
        };
        let custom_classes = class.unwrap_or("");
        
        format!("{} {} {} {}", base_classes, state_classes, padding_classes, custom_classes)
    };
    
    view! {
        <div class="mb-4">
            {label.map(|text| view! {
                <label class="block text-sm font-medium text-primary mb-1">
                    {text}
                    {if required {
                        view! { <span class="text-red-500 ml-1">"*"</span> }
                    } else {
                        view! { <span></span> }
                    }}
                </label>
            })}
            
            <div class="relative">
                {icon.map(|icon_name| view! {
                    <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                        <i class=format!("fas fa-{} text-secondary", icon_name)></i>
                    </div>
                })}
                
                <input
                    type=input_type
                    placeholder=placeholder.unwrap_or("")
                    value=move || value.get()
                    on:input=on_input
                    on:blur=on_blur
                    disabled=disabled
                    class=input_classes
                />
            </div>
            
            {helper_text.map(|text| view! {
                <p class="mt-1 text-sm text-secondary">{text}</p>
            })}
            
            <Show when=move || error.get().is_some() && touched.get()>
                <p class="mt-1 text-sm text-red-600">
                    {move || error.get().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}

/// TextArea component with built-in validation
#[component]
pub fn TextArea(
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<&'static str>,
    
    /// Label for the textarea
    #[prop(optional)]
    label: Option<&'static str>,
    
    /// TextArea value signal
    value: RwSignal<String>,
    
    /// Validation functions
    #[prop(optional)]
    validators: Vec<ValidationFn>,
    
    /// Whether the textarea is required
    #[prop(default = false)]
    required: bool,
    
    /// Whether the textarea is disabled
    #[prop(default = false)]
    disabled: bool,
    
    /// Number of rows
    #[prop(default = 4)]
    rows: i32,
    
    /// Maximum character length
    #[prop(optional)]
    max_length: Option<usize>,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
    
    /// Helper text
    #[prop(optional)]
    helper_text: Option<&'static str>,
    
    /// Whether to show character count
    #[prop(default = false)]
    show_count: bool,
) -> impl IntoView {
    let (error, set_error) = create_signal(None::<String>);
    let (touched, set_touched) = create_signal(false);
    
    // Validate input
    let validate = move |value: &str| {
        if required && value.trim().is_empty() {
            set_error.set(Some("This field is required".to_string()));
            return false;
        }
        
        if let Some(max) = max_length {
            if value.len() > max {
                set_error.set(Some(format!("Maximum {} characters allowed", max)));
                return false;
            }
        }
        
        for validator in &validators {
            if let Err(msg) = validator(value) {
                set_error.set(Some(msg));
                return false;
            }
        }
        
        set_error.set(None);
        true
    };
    
    // Handle input change
    let on_input = move |ev| {
        let new_value = event_target_value(&ev);
        
        // Enforce max length
        let final_value = if let Some(max) = max_length {
            new_value.chars().take(max).collect()
        } else {
            new_value
        };
        
        value.set(final_value.clone());
        
        if touched.get() {
            validate(&final_value);
        }
    };
    
    // Handle blur
    let on_blur = move |_| {
        set_touched.set(true);
        validate(&value.get());
    };
    
    let textarea_classes = move || {
        let base_classes = "block w-full rounded-lg border transition-base focus:outline-none focus:ring-2 px-3 py-2 resize-none";
        let state_classes = if error.get().is_some() && touched.get() {
            "border-red-500 focus:border-red-500 focus:ring-red-500"
        } else {
            "border-primary focus:border-blue-500 focus:ring-blue-500"
        };
        let custom_classes = class.unwrap_or("");
        
        format!("{} {} {}", base_classes, state_classes, custom_classes)
    };
    
    view! {
        <div class="mb-4">
            {label.map(|text| view! {
                <label class="block text-sm font-medium text-primary mb-1">
                    {text}
                    {if required {
                        view! { <span class="text-red-500 ml-1">"*"</span> }
                    } else {
                        view! { <span></span> }
                    }}
                </label>
            })}
            
            <textarea
                placeholder=placeholder.unwrap_or("")
                value=move || value.get()
                on:input=on_input
                on:blur=on_blur
                disabled=disabled
                rows=rows
                class=textarea_classes
            />
            
            <div class="mt-1 flex justify-between">
                <div>
                    {helper_text.map(|text| view! {
                        <p class="text-sm text-secondary">{text}</p>
                    })}
                    
                    <Show when=move || error.get().is_some() && touched.get()>
                        <p class="text-sm text-red-600">
                            {move || error.get().unwrap_or_default()}
                        </p>
                    </Show>
                </div>
                
                {if show_count {
                    view! {
                        <div class="text-sm text-secondary">
                            {move || {
                                let len = value.get().len();
                                if let Some(max) = max_length {
                                    format!("{}/{}", len, max)
                                } else {
                                    format!("{}", len)
                                }
                            }}
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }}
            </div>
        </div>
    }
}

/// Common validators
pub mod validators {
    use super::ValidationResult;
    
    /// Email validator
    pub fn email(value: &str) -> ValidationResult {
        if value.contains('@') && value.contains('.') {
            Ok(())
        } else {
            Err("Please enter a valid email address".to_string())
        }
    }
    
    /// Minimum length validator
    pub fn min_length(min: usize) -> Box<dyn Fn(&str) -> ValidationResult> {
        Box::new(move |value: &str| {
            if value.len() >= min {
                Ok(())
            } else {
                Err(format!("Minimum {} characters required", min))
            }
        })
    }
    
    /// Maximum length validator
    pub fn max_length(max: usize) -> Box<dyn Fn(&str) -> ValidationResult> {
        Box::new(move |value: &str| {
            if value.len() <= max {
                Ok(())
            } else {
                Err(format!("Maximum {} characters allowed", max))
            }
        })
    }
    
    /// Pattern validator
    pub fn pattern(regex: &'static str, message: &'static str) -> Box<dyn Fn(&str) -> ValidationResult> {
        Box::new(move |value: &str| {
            // Simple pattern matching without regex crate for now
            // In production, use regex crate
            if value.is_empty() {
                return Ok(());
            }
            
            // Example patterns
            match regex {
                r"^\d+$" => {
                    if value.chars().all(|c| c.is_numeric()) {
                        Ok(())
                    } else {
                        Err(message.to_string())
                    }
                },
                _ => Ok(())
            }
        })
    }
    
    /// Australian postcode validator
    pub fn australian_postcode(value: &str) -> ValidationResult {
        if value.len() == 4 && value.chars().all(|c| c.is_numeric()) {
            let postcode: u32 = value.parse().unwrap_or(0);
            if postcode >= 200 && postcode <= 9999 {
                Ok(())
            } else {
                Err("Please enter a valid Australian postcode".to_string())
            }
        } else {
            Err("Postcode must be 4 digits".to_string())
        }
    }
    
    /// Australian phone number validator
    pub fn australian_phone(value: &str) -> ValidationResult {
        let digits: String = value.chars().filter(|c| c.is_numeric()).collect();
        
        if digits.len() == 10 && (digits.starts_with("04") || digits.starts_with("02") || digits.starts_with("03") || digits.starts_with("07") || digits.starts_with("08")) {
            Ok(())
        } else {
            Err("Please enter a valid Australian phone number".to_string())
        }
    }
}