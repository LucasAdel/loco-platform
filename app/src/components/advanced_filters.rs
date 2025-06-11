use leptos::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use shared::types::{JobType, AustralianState, JobFilters};
use crate::components::ui::{Button, ButtonVariant, Alert, AlertVariant};
use crate::components::forms::Input;
use web_sys::Event;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdvancedFilterState {
    // Location filters
    pub states: Vec<AustralianState>,
    pub suburbs: Vec<String>,
    pub distance_km: Option<f64>,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    
    // Job type filters
    pub job_types: Vec<JobType>,
    pub employment_types: Vec<String>, // Full-time, Part-time, Contract, Casual
    
    // Compensation filters
    pub salary_min: Option<u32>,
    pub salary_max: Option<u32>,
    pub hourly_rate_min: Option<f64>,
    pub hourly_rate_max: Option<f64>,
    pub penalty_rates_only: bool,
    
    // Schedule filters
    pub available_weekdays: Vec<String>,
    pub available_weekends: bool,
    pub shift_types: Vec<String>, // Morning, Afternoon, Evening, Night
    pub flexible_hours_only: bool,
    
    // Experience filters
    pub experience_min: Option<u32>,
    pub experience_max: Option<u32>,
    pub requires_registration: bool,
    pub requires_car: bool,
    
    // Benefits filters
    pub benefits: Vec<String>,
    pub training_provided: bool,
    pub career_development: bool,
    
    // Urgency and date filters
    pub urgent_only: bool,
    pub posted_within_days: Option<u32>,
    pub start_date_from: Option<String>,
    pub start_date_to: Option<String>,
    
    // Company filters
    pub pharmacy_chains: Vec<String>,
    pub independent_pharmacies: bool,
    pub hospital_pharmacies: bool,
    pub clinic_pharmacies: bool,
    
    // Advanced search
    pub keywords: Vec<String>,
    pub exclude_keywords: Vec<String>,
    pub title_contains: String,
    pub description_contains: String,
}

impl AdvancedFilterState {
    pub fn to_job_filters(&self) -> JobFilters {
        JobFilters {
            job_type: self.job_types.first().cloned(),
            state: self.states.first().map(|s| s.to_string()),
            salary_min: self.salary_min.map(|x| x as f64),
            salary_max: self.salary_max.map(|x| x as f64),
            urgent_only: Some(self.urgent_only),
            ..Default::default()
        }
    }

    pub fn is_active(&self) -> bool {
        !self.states.is_empty() ||
        !self.suburbs.is_empty() ||
        self.distance_km.is_some() ||
        !self.job_types.is_empty() ||
        !self.employment_types.is_empty() ||
        self.salary_min.is_some() ||
        self.salary_max.is_some() ||
        self.hourly_rate_min.is_some() ||
        self.hourly_rate_max.is_some() ||
        self.penalty_rates_only ||
        !self.available_weekdays.is_empty() ||
        self.available_weekends ||
        !self.shift_types.is_empty() ||
        self.flexible_hours_only ||
        self.experience_min.is_some() ||
        self.experience_max.is_some() ||
        self.requires_registration ||
        self.requires_car ||
        !self.benefits.is_empty() ||
        self.training_provided ||
        self.career_development ||
        self.urgent_only ||
        self.posted_within_days.is_some() ||
        self.start_date_from.is_some() ||
        self.start_date_to.is_some() ||
        !self.pharmacy_chains.is_empty() ||
        self.independent_pharmacies ||
        self.hospital_pharmacies ||
        self.clinic_pharmacies ||
        !self.keywords.is_empty() ||
        !self.exclude_keywords.is_empty() ||
        !self.title_contains.is_empty() ||
        !self.description_contains.is_empty()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

#[component]
pub fn AdvancedFilters(
    #[prop(optional)] initial_filters: Option<AdvancedFilterState>,
    on_filters_change: impl Fn(AdvancedFilterState) + 'static,
    #[prop(optional)] on_close: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let (filters, set_filters) = create_signal(initial_filters.unwrap_or_default());
    let (active_tab, set_active_tab) = create_signal("location".to_string());
    let (show_saved_searches, set_show_saved_searches) = create_signal(false);

    // Handle filter changes
    let notify_change = move || {
        on_filters_change(filters.get());
    };

    // Tab switching
    let switch_tab = move |tab: String| {
        move |_| set_active_tab.set(tab.clone())
    };

    // Clear all filters
    let clear_all_filters = move |_| {
        set_filters.update(|f| f.clear());
        notify_change();
    };

    // Apply filters and close
    let apply_filters = move |_| {
        notify_change();
        if let Some(ref close_fn) = on_close {
            close_fn();
        }
    };

    view! {
        <div class="bg-white rounded-lg shadow-lg max-w-4xl w-full max-h-[80vh] overflow-y-auto">
            // Header
            <div class="px-6 py-4 border-b bg-gray-50 rounded-t-lg">
                <div class="flex items-center justify-between">
                    <div>
                        <h2 class="text-xl font-semibold text-gray-900">
                            "Advanced Filters"
                        </h2>
                        <p class="text-sm text-gray-600 mt-1">
                            "Refine your job search with detailed criteria"
                        </p>
                    </div>
                    
                    <div class="flex items-center gap-2">
                        {move || {
                            if filters.get().is_active() {
                                view! {
                                    <span class="px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded-full">
                                        "Filters Active"
                                    </span>
                                }.into_view()
                            } else {
                                view! { <div></div> }.into_view()
                            }
                        }}
                        
                        {on_close.as_ref().map(|_| {
                            view! {
                                <button
                                    class="text-gray-400 hover:text-gray-600"
                                    on:click=move |_| {
                                        if let Some(ref close_fn) = on_close {
                                            close_fn();
                                        }
                                    }
                                >
                                    "✕"
                                </button>
                            }
                        })}
                    </div>
                </div>
            </div>

            // Tab navigation
            <div class="px-6 py-3 border-b bg-gray-50">
                <div class="flex space-x-4 overflow-x-auto">
                    {["location", "job_type", "compensation", "schedule", "experience", "advanced"]
                        .into_iter()
                        .map(|tab| {
                            let tab_name = tab.to_string();
                            let display_name = match tab {
                                "location" => "Location",
                                "job_type" => "Job Type",
                                "compensation" => "Compensation",
                                "schedule" => "Schedule",
                                "experience" => "Experience",
                                "advanced" => "Advanced",
                                _ => tab,
                            };
                            
                            view! {
                                <button
                                    class=move || format!(
                                        "px-4 py-2 text-sm font-medium rounded-md whitespace-nowrap {}",
                                        if active_tab.get() == tab_name {
                                            "bg-blue-100 text-blue-700 border border-blue-300"
                                        } else {
                                            "text-gray-600 hover:text-gray-900 hover:bg-gray-100"
                                        }
                                    )
                                    on:click=switch_tab(tab_name.clone())
                                >
                                    {display_name}
                                </button>
                            }
                        })
                        .collect_view()
                    }
                </div>
            </div>

            // Filter content
            <div class="px-6 py-6">
                {move || match active_tab.get().as_str() {
                    "location" => view! {
                        <LocationFilters filters=filters set_filters=set_filters on_change=notify_change />
                    }.into_view(),
                    "job_type" => view! {
                        <JobTypeFilters filters=filters set_filters=set_filters on_change=notify_change />
                    }.into_view(),
                    "compensation" => view! {
                        <CompensationFilters filters=filters set_filters=set_filters on_change=notify_change />
                    }.into_view(),
                    "schedule" => view! {
                        <ScheduleFilters filters=filters set_filters=set_filters on_change=notify_change />
                    }.into_view(),
                    "experience" => view! {
                        <ExperienceFilters filters=filters set_filters=set_filters on_change=notify_change />
                    }.into_view(),
                    "advanced" => view! {
                        <AdvancedSearchFilters filters=filters set_filters=set_filters on_change=notify_change />
                    }.into_view(),
                    _ => view! { <div>"Select a filter category"</div> }.into_view(),
                }}
            </div>

            // Footer actions
            <div class="px-6 py-4 border-t bg-gray-50 rounded-b-lg">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2">
                        <Button 
                            variant=ButtonVariant::Secondary
                            on_click=clear_all_filters
                        >
                            "Clear All"
                        </Button>
                        
                        <button 
                            class="text-sm text-blue-600 hover:text-blue-800"
                            on:click=move |_| set_show_saved_searches.update(|s| *s = !*s)
                        >
                            "Saved Searches"
                        </button>
                    </div>
                    
                    <div class="flex items-center gap-2">
                        {on_close.as_ref().map(|_| {
                            view! {
                                <Button 
                                    variant=ButtonVariant::Secondary
                                    on_click=move |_| {
                                        if let Some(ref close_fn) = on_close {
                                            close_fn();
                                        }
                                    }
                                >
                                    "Cancel"
                                </Button>
                            }
                        })}
                        
                        <Button 
                            variant=ButtonVariant::Primary
                            on_click=apply_filters
                        >
                            "Apply Filters"
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn LocationFilters(
    filters: ReadSignal<AdvancedFilterState>,
    set_filters: WriteSignal<AdvancedFilterState>,
    on_change: impl Fn() + 'static,
) -> impl IntoView {
    let toggle_state = move |state: AustralianState| {
        move |_| {
            set_filters.update(|f| {
                if f.states.contains(&state) {
                    f.states.retain(|&s| s != state);
                } else {
                    f.states.push(state);
                }
            });
            on_change();
        }
    };

    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-medium text-gray-900 mb-4">
                    "Location Preferences"
                </h3>
                
                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">
                            "States/Territories"
                        </label>
                        <div class="grid grid-cols-2 md:grid-cols-4 gap-2">
                            {[
                                (AustralianState::NSW, "NSW"),
                                (AustralianState::VIC, "VIC"),
                                (AustralianState::QLD, "QLD"),
                                (AustralianState::SA, "SA"),
                                (AustralianState::WA, "WA"),
                                (AustralianState::TAS, "TAS"),
                                (AustralianState::NT, "NT"),
                                (AustralianState::ACT, "ACT"),
                            ].into_iter()
                                .map(|(state, label)| {
                                    view! {
                                        <label class="flex items-center">
                                            <input
                                                type="checkbox"
                                                class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                                                checked=move || filters.get().states.contains(&state)
                                                on:change=toggle_state(state)
                                            />
                                            <span class="text-sm text-gray-700">{label}</span>
                                        </label>
                                    }
                                })
                                .collect_view()
                            }
                        </div>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Distance (km)"
                            </label>
                            <input
                                type="number"
                                min="1"
                                max="500"
                                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                placeholder="e.g. 25"
                                value=move || filters.get().distance_km.map(|d| d.to_string()).unwrap_or_default()
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    let distance = value.parse::<f64>().ok();
                                    set_filters.update(|f| f.distance_km = distance);
                                    on_change();
                                }
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn JobTypeFilters(
    filters: ReadSignal<AdvancedFilterState>,
    set_filters: WriteSignal<AdvancedFilterState>,
    on_change: impl Fn() + 'static,
) -> impl IntoView {
    let toggle_job_type = move |job_type: JobType| {
        move |_| {
            set_filters.update(|f| {
                if f.job_types.contains(&job_type) {
                    f.job_types.retain(|&jt| jt != job_type);
                } else {
                    f.job_types.push(job_type);
                }
            });
            on_change();
        }
    };

    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-medium text-gray-900 mb-4">
                    "Job Types & Employment"
                </h3>
                
                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">
                            "Pharmacy Roles"
                        </label>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                            {[
                                (JobType::Pharmacist, "Pharmacist"),
                                (JobType::PharmacyAssistant, "Pharmacy Assistant"),
                                (JobType::PharmacyTechnician, "Pharmacy Technician"),
                                (JobType::PharmacyManager, "Pharmacy Manager"),
                                (JobType::InternPharmacist, "Intern Pharmacist"),
                                (JobType::ClinicalPharmacist, "Clinical Pharmacist"),
                            ].into_iter()
                                .map(|(job_type, label)| {
                                    view! {
                                        <label class="flex items-center">
                                            <input
                                                type="checkbox"
                                                class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                                                checked=move || filters.get().job_types.contains(&job_type)
                                                on:change=toggle_job_type(job_type)
                                            />
                                            <span class="text-sm text-gray-700">{label}</span>
                                        </label>
                                    }
                                })
                                .collect_view()
                            }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CompensationFilters(
    filters: ReadSignal<AdvancedFilterState>,
    set_filters: WriteSignal<AdvancedFilterState>,
    on_change: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-medium text-gray-900 mb-4">
                    "Compensation & Benefits"
                </h3>
                
                <div class="space-y-4">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Minimum Hourly Rate ($AUD)"
                            </label>
                            <input
                                type="number"
                                min="20"
                                max="100"
                                step="0.50"
                                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                placeholder="e.g. 35.00"
                                value=move || filters.get().hourly_rate_min.map(|r| r.to_string()).unwrap_or_default()
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    let rate = value.parse::<f64>().ok();
                                    set_filters.update(|f| f.hourly_rate_min = rate);
                                    on_change();
                                }
                            />
                        </div>

                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Maximum Hourly Rate ($AUD)"
                            </label>
                            <input
                                type="number"
                                min="20"
                                max="100"
                                step="0.50"
                                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                placeholder="e.g. 50.00"
                                value=move || filters.get().hourly_rate_max.map(|r| r.to_string()).unwrap_or_default()
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    let rate = value.parse::<f64>().ok();
                                    set_filters.update(|f| f.hourly_rate_max = rate);
                                    on_change();
                                }
                            />
                        </div>
                    </div>

                    <div class="space-y-2">
                        <label class="flex items-center">
                            <input
                                type="checkbox"
                                class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                                checked=move || filters.get().penalty_rates_only
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_filters.update(|f| f.penalty_rates_only = checked);
                                    on_change();
                                }
                            />
                            <span class="text-sm text-gray-700">"Penalty rates apply"</span>
                        </label>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ScheduleFilters(
    filters: ReadSignal<AdvancedFilterState>,
    set_filters: WriteSignal<AdvancedFilterState>,
    on_change: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-medium text-gray-900 mb-4">
                    "Schedule & Availability"
                </h3>
                
                <div class="space-y-4">
                    <div class="space-y-2">
                        <label class="flex items-center">
                            <input
                                type="checkbox"
                                class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                                checked=move || filters.get().available_weekends
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_filters.update(|f| f.available_weekends = checked);
                                    on_change();
                                }
                            />
                            <span class="text-sm text-gray-700">"Weekend availability required"</span>
                        </label>

                        <label class="flex items-center">
                            <input
                                type="checkbox"
                                class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                                checked=move || filters.get().flexible_hours_only
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_filters.update(|f| f.flexible_hours_only = checked);
                                    on_change();
                                }
                            />
                            <span class="text-sm text-gray-700">"Flexible hours only"</span>
                        </label>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ExperienceFilters(
    filters: ReadSignal<AdvancedFilterState>,
    set_filters: WriteSignal<AdvancedFilterState>,
    on_change: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-medium text-gray-900 mb-4">
                    "Experience & Requirements"
                </h3>
                
                <div class="space-y-4">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Minimum Experience (years)"
                            </label>
                            <input
                                type="number"
                                min="0"
                                max="20"
                                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                placeholder="e.g. 2"
                                value=move || filters.get().experience_min.map(|e| e.to_string()).unwrap_or_default()
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    let exp = value.parse::<u32>().ok();
                                    set_filters.update(|f| f.experience_min = exp);
                                    on_change();
                                }
                            />
                        </div>
                    </div>

                    <div class="space-y-2">
                        <label class="flex items-center">
                            <input
                                type="checkbox"
                                class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                                checked=move || filters.get().requires_registration
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_filters.update(|f| f.requires_registration = checked);
                                    on_change();
                                }
                            />
                            <span class="text-sm text-gray-700">"AHPRA registration required"</span>
                        </label>

                        <label class="flex items-center">
                            <input
                                type="checkbox"
                                class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                                checked=move || filters.get().requires_car
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_filters.update(|f| f.requires_car = checked);
                                    on_change();
                                }
                            />
                            <span class="text-sm text-gray-700">"Own vehicle required"</span>
                        </label>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn AdvancedSearchFilters(
    filters: ReadSignal<AdvancedFilterState>,
    set_filters: WriteSignal<AdvancedFilterState>,
    on_change: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-medium text-gray-900 mb-4">
                    "Advanced Search"
                </h3>
                
                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">
                            "Title Contains"
                        </label>
                        <Input
                            value=move || filters.get().title_contains
                            on_input=move |value| {
                                set_filters.update(|f| f.title_contains = value);
                                on_change();
                            }
                            placeholder="e.g. manager, senior, weekend"
                        />
                    </div>

                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">
                            "Description Contains"
                        </label>
                        <Input
                            value=move || filters.get().description_contains
                            on_input=move |value| {
                                set_filters.update(|f| f.description_contains = value);
                                on_change();
                            }
                            placeholder="e.g. clinical, training, team"
                        />
                    </div>

                    <div class="space-y-2">
                        <label class="flex items-center">
                            <input
                                type="checkbox"
                                class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                                checked=move || filters.get().urgent_only
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_filters.update(|f| f.urgent_only = checked);
                                    on_change();
                                }
                            />
                            <span class="text-sm text-gray-700">"Urgent positions only"</span>
                        </label>
                    </div>

                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">
                            "Posted Within (days)"
                        </label>
                        <select
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                let days = value.parse::<u32>().ok();
                                set_filters.update(|f| f.posted_within_days = days);
                                on_change();
                            }
                        >
                            <option value="">"Any time"</option>
                            <option value="1">"Last 24 hours"</option>
                            <option value="3">"Last 3 days"</option>
                            <option value="7">"Last week"</option>
                            <option value="14">"Last 2 weeks"</option>
                            <option value="30">"Last month"</option>
                        </select>
                    </div>
                </div>
            </div>
        </div>
    }
}