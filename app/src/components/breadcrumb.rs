use leptos::*;
use leptos_router::*;

/// Breadcrumb item
#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<String>,
}

impl BreadcrumbItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
            icon: None,
        }
    }
    
    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }
    
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Breadcrumb navigation component
#[component]
pub fn Breadcrumb(
    /// List of breadcrumb items
    items: Vec<BreadcrumbItem>,
    
    /// Separator character or component
    #[prop(default = "/")]
    separator: &'static str,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    let items_len = items.len();
    
    view! {
        <nav 
            aria-label="Breadcrumb"
            class=format!("flex items-center space-x-2 text-sm {}", class.unwrap_or(""))
        >
            <ol class="flex items-center space-x-2">
                {items.into_iter().enumerate().map(|(index, item)| {
                    let is_last = index == items_len - 1;
                    
                    view! {
                        <li class="flex items-center">
                            {if index > 0 {
                                view! {
                                    <span class="mx-2 text-gray-400">{separator}</span>
                                }
                            } else {
                                view! { <span></span> }
                            }}
                            
                            {if let Some(href) = item.href {
                                if !is_last {
                                    view! {
                                        <A 
                                            href=href
                                            class="flex items-center text-gray-600 hover:text-blue-600 transition-colors"
                                        >
                                            {item.icon.map(|icon| view! {
                                                <i class=format!("fas fa-{} mr-1.5", icon)></i>
                                            })}
                                            {item.label}
                                        </A>
                                    }.into_view()
                                } else {
                                    view! {
                                        <span class="flex items-center text-gray-900 font-medium">
                                            {item.icon.map(|icon| view! {
                                                <i class=format!("fas fa-{} mr-1.5", icon)></i>
                                            })}
                                            {item.label}
                                        </span>
                                    }.into_view()
                                }
                            } else {
                                view! {
                                    <span class=format!(
                                        "flex items-center {}",
                                        if is_last { "text-gray-900 font-medium" } else { "text-gray-600" }
                                    )>
                                        {item.icon.map(|icon| view! {
                                            <i class=format!("fas fa-{} mr-1.5", icon)></i>
                                        })}
                                        {item.label}
                                    </span>
                                }.into_view()
                            }}
                        </li>
                    }
                }).collect_view()}
            </ol>
        </nav>
    }
}

/// Auto-breadcrumb component that generates breadcrumbs from current route
#[component]
pub fn AutoBreadcrumb(
    /// Custom label mapping for routes
    #[prop(optional)]
    route_labels: Option<Vec<(&'static str, &'static str)>>,
    
    /// Whether to show home as first item
    #[prop(default = true)]
    show_home: bool,
    
    /// Home label
    #[prop(default = "Home")]
    home_label: &'static str,
    
    /// Home icon
    #[prop(optional)]
    home_icon: Option<&'static str>,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    let location = use_location();
    
    let breadcrumbs = create_memo(move |_| {
        let pathname = location.pathname.get();
        let segments: Vec<&str> = pathname
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        
        let mut items = Vec::new();
        
        // Add home item if requested
        if show_home {
            items.push(
                BreadcrumbItem::new(home_label)
                    .with_href("/")
                    .with_icon(home_icon.unwrap_or("home"))
            );
        }
        
        // Build breadcrumb items from route segments
        let mut current_path = String::new();
        for (index, segment) in segments.iter().enumerate() {
            current_path.push('/');
            current_path.push_str(segment);
            
            // Get custom label or format the segment
            let label = if let Some(labels) = &route_labels {
                labels.iter()
                    .find(|(path, _)| path == segment)
                    .map(|(_, label)| label.to_string())
                    .unwrap_or_else(|| format_segment(segment))
            } else {
                format_segment(segment)
            };
            
            let is_last = index == segments.len() - 1;
            let mut item = BreadcrumbItem::new(label);
            
            if !is_last {
                item.href = Some(current_path.clone());
            }
            
            items.push(item);
        }
        
        items
    });
    
    view! {
        <Show 
            when=move || breadcrumbs.get().len() > if show_home { 1 } else { 0 }
        >
            <Breadcrumb items=breadcrumbs.get() class=class />
        </Show>
    }
}

/// Format a route segment into a readable label
fn format_segment(segment: &str) -> String {
    // Handle common patterns
    match segment {
        "admin" => "Admin".to_string(),
        "settings" => "Settings".to_string(),
        "profile" => "Profile".to_string(),
        "dashboard" => "Dashboard".to_string(),
        "jobs" => "Jobs".to_string(),
        "applications" => "Applications".to_string(),
        _ => {
            // Convert kebab-case to Title Case
            segment
                .split('-')
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().chain(chars).collect(),
                    }
                })
                .collect::<Vec<String>>()
                .join(" ")
        }
    }
}

/// Structured breadcrumb component with schema.org microdata
#[component]
pub fn StructuredBreadcrumb(
    /// List of breadcrumb items
    items: Vec<BreadcrumbItem>,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    let items_len = items.len();
    let base_url = "https://locoplatform.com.au"; // Replace with your actual domain
    
    view! {
        <nav 
            aria-label="Breadcrumb"
            class=format!("flex items-center space-x-2 text-sm {}", class.unwrap_or(""))
        >
            <ol 
                itemscope
                itemtype="https://schema.org/BreadcrumbList"
                class="flex items-center space-x-2"
            >
                {items.into_iter().enumerate().map(|(index, item)| {
                    let position = index + 1;
                    let is_last = index == items_len - 1;
                    
                    view! {
                        <li 
                            itemprop="itemListElement"
                            itemscope
                            itemtype="https://schema.org/ListItem"
                            class="flex items-center"
                        >
                            {if index > 0 {
                                view! {
                                    <span class="mx-2 text-gray-400">
                                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
                                        </svg>
                                    </span>
                                }
                            } else {
                                view! { <span></span> }
                            }}
                            
                            {if let Some(href) = &item.href {
                                if !is_last {
                                    view! {
                                        <A 
                                            href=href.clone()
                                            itemprop="item"
                                            class="flex items-center text-gray-600 hover:text-blue-600 transition-colors"
                                        >
                                            {item.icon.as_ref().map(|icon| view! {
                                                <i class=format!("fas fa-{} mr-1.5", icon)></i>
                                            })}
                                            <span itemprop="name">{&item.label}</span>
                                        </A>
                                    }.into_view()
                                } else {
                                    view! {
                                        <span 
                                            itemprop="item"
                                            class="flex items-center text-gray-900 font-medium"
                                        >
                                            {item.icon.as_ref().map(|icon| view! {
                                                <i class=format!("fas fa-{} mr-1.5", icon)></i>
                                            })}
                                            <span itemprop="name">{&item.label}</span>
                                        </span>
                                    }.into_view()
                                }
                            } else {
                                view! {
                                    <span 
                                        itemprop="item"
                                        class=format!(
                                            "flex items-center {}",
                                            if is_last { "text-gray-900 font-medium" } else { "text-gray-600" }
                                        )
                                    >
                                        {item.icon.as_ref().map(|icon| view! {
                                            <i class=format!("fas fa-{} mr-1.5", icon)></i>
                                        })}
                                        <span itemprop="name">{&item.label}</span>
                                    </span>
                                }.into_view()
                            }}
                            
                            <meta itemprop="position" content=position.to_string() />
                        </li>
                    }
                }).collect_view()}
            </ol>
        </nav>
    }
}