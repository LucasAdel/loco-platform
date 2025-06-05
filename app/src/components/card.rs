use leptos::*;

/// Card elevation levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardElevation {
    None,
    Small,
    Medium,
    Large,
}

impl CardElevation {
    fn class(&self) -> &'static str {
        match self {
            CardElevation::None => "",
            CardElevation::Small => "shadow-sm",
            CardElevation::Medium => "shadow-md",
            CardElevation::Large => "shadow-lg",
        }
    }
}

/// Apple-style card component
#[component]
pub fn Card(
    /// Card content
    children: Children,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
    
    /// Whether the card is clickable
    #[prop(default = false)]
    clickable: bool,
    
    /// Card elevation
    #[prop(default = CardElevation::Small)]
    elevation: CardElevation,
    
    /// Whether to add padding
    #[prop(default = true)]
    padded: bool,
    
    /// Whether to show a border
    #[prop(default = true)]
    bordered: bool,
    
    /// Click handler
    #[prop(optional)]
    on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let base_classes = "bg-primary rounded-xl transition-all duration-300";
    
    let elevation_class = elevation.class();
    
    let interactive_classes = if clickable || on_click.is_some() {
        "cursor-pointer hover:shadow-xl hover:scale-[1.02] active:scale-[0.98]"
    } else {
        ""
    };
    
    let padding_class = if padded { "p-6" } else { "" };
    
    let border_class = if bordered { "border border-primary" } else { "" };
    
    let custom_class = class.unwrap_or("");
    
    let combined_classes = format!(
        "{} {} {} {} {} {}",
        base_classes,
        elevation_class,
        interactive_classes,
        padding_class,
        border_class,
        custom_class
    );
    
    let handle_click = move |_| {
        if let Some(handler) = &on_click {
            handler();
        }
    };
    
    view! {
        <div
            class=combined_classes
            on:click=handle_click
        >
            {children()}
        </div>
    }
}

/// Feature card with icon
#[component]
pub fn FeatureCard(
    /// Card title
    title: &'static str,
    
    /// Card description
    description: &'static str,
    
    /// Icon name (FontAwesome)
    #[prop(optional)]
    icon: Option<&'static str>,
    
    /// Icon colour class
    #[prop(default = "text-blue-600")]
    icon_color: &'static str,
    
    /// Additional content
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <Card class="h-full">
            <div class="flex items-start space-x-4">
                {icon.map(|icon_name| view! {
                    <div class=format!("flex-shrink-0 {}", icon_color)>
                        <i class=format!("fas fa-{} text-2xl", icon_name)></i>
                    </div>
                })}
                <div class="flex-1">
                    <h3 class="text-lg font-semibold text-primary mb-2">{title}</h3>
                    <p class="text-secondary mb-4">{description}</p>
                    {children.map(|c| c())}
                </div>
            </div>
        </Card>
    }
}

/// Stat card component
#[component]
pub fn StatCard(
    /// Stat label
    label: &'static str,
    
    /// Stat value
    value: String,
    
    /// Change percentage (optional)
    #[prop(optional)]
    change: Option<f64>,
    
    /// Icon name (optional)
    #[prop(optional)]
    icon: Option<&'static str>,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    let change_color = move || {
        if let Some(change_val) = change {
            if change_val > 0.0 {
                "text-green-600"
            } else if change_val < 0.0 {
                "text-red-600"
            } else {
                "text-gray-600"
            }
        } else {
            ""
        }
    };
    
    let change_icon = move || {
        if let Some(change_val) = change {
            if change_val > 0.0 {
                "arrow-up"
            } else if change_val < 0.0 {
                "arrow-down"
            } else {
                "minus"
            }
        } else {
            ""
        }
    };
    
    view! {
        <Card class=class>
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-sm text-secondary mb-1">{label}</p>
                    <p class="text-2xl font-bold text-primary">{value}</p>
                    {change.map(|change_val| view! {
                        <div class=format!("flex items-center mt-2 text-sm {}", change_color())>
                            <i class=format!("fas fa-{} mr-1", change_icon())></i>
                            <span>{format!("{:.1}%", change_val.abs())}</span>
                        </div>
                    })}
                </div>
                {icon.map(|icon_name| view! {
                    <div class="text-3xl text-secondary opacity-20">
                        <i class=format!("fas fa-{}", icon_name)></i>
                    </div>
                })}
            </div>
        </Card>
    }
}

/// Image card component
#[component]
pub fn ImageCard(
    /// Image source URL
    image_src: &'static str,
    
    /// Image alt text
    image_alt: &'static str,
    
    /// Card title
    title: &'static str,
    
    /// Card description
    #[prop(optional)]
    description: Option<&'static str>,
    
    /// Badge text (optional)
    #[prop(optional)]
    badge: Option<&'static str>,
    
    /// Additional content
    #[prop(optional)]
    children: Option<Children>,
    
    /// Click handler
    #[prop(optional)]
    on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    view! {
        <Card 
            padded=false 
            clickable=on_click.is_some()
            on_click=on_click
            class="overflow-hidden group"
        >
            <div class="relative aspect-video overflow-hidden">
                <img 
                    src=image_src 
                    alt=image_alt
                    class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
                />
                {badge.map(|text| view! {
                    <div class="absolute top-4 right-4">
                        <span class="px-3 py-1 text-xs font-semibold text-white bg-blue-600 rounded-full">
                            {text}
                        </span>
                    </div>
                })}
            </div>
            <div class="p-6">
                <h3 class="text-lg font-semibold text-primary mb-2">{title}</h3>
                {description.map(|desc| view! {
                    <p class="text-secondary mb-4">{desc}</p>
                })}
                {children.map(|c| c())}
            </div>
        </Card>
    }
}

/// Profile card component
#[component]
pub fn ProfileCard(
    /// Profile name
    name: &'static str,
    
    /// Profile role/title
    role: &'static str,
    
    /// Avatar URL (optional)
    #[prop(optional)]
    avatar_url: Option<&'static str>,
    
    /// Avatar initials (used if no URL)
    #[prop(optional)]
    initials: Option<&'static str>,
    
    /// Additional info items
    #[prop(optional)]
    info_items: Vec<(&'static str, &'static str)>,
    
    /// Action buttons
    #[prop(optional)]
    actions: Option<View>,
) -> impl IntoView {
    let display_initials = initials.unwrap_or_else(|| {
        name.split_whitespace()
            .take(2)
            .map(|word| word.chars().next().unwrap_or_default())
            .collect::<String>()
            .leak()
    });
    
    view! {
        <Card>
            <div class="flex items-start space-x-4">
                // Avatar
                <div class="flex-shrink-0">
                    {if let Some(url) = avatar_url {
                        view! {
                            <img 
                                src=url 
                                alt=name
                                class="w-16 h-16 rounded-full object-cover"
                            />
                        }
                    } else {
                        view! {
                            <div class="w-16 h-16 rounded-full bg-blue-100 flex items-center justify-center">
                                <span class="text-xl font-semibold text-blue-600">
                                    {display_initials}
                                </span>
                            </div>
                        }
                    }}
                </div>
                
                // Profile info
                <div class="flex-1">
                    <h3 class="text-lg font-semibold text-primary">{name}</h3>
                    <p class="text-secondary mb-3">{role}</p>
                    
                    {if !info_items.is_empty() {
                        view! {
                            <div class="space-y-1">
                                {info_items.into_iter().map(|(label, value)| view! {
                                    <div class="flex items-center text-sm">
                                        <span class="text-secondary w-24">{label}:</span>
                                        <span class="text-primary">{value}</span>
                                    </div>
                                }).collect_view()}
                            </div>
                        }
                    } else {
                        view! { <div></div> }
                    }}
                    
                    {actions.map(|a| view! {
                        <div class="mt-4">
                            {a}
                        </div>
                    })}
                </div>
            </div>
        </Card>
    }
}