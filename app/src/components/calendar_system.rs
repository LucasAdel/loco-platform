use leptos::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Weekday, Duration as ChronoDuration, Utc, Datelike};
use std::collections::HashMap;
use crate::components::ui::{Button, ButtonVariant, Alert, AlertVariant, LoadingSpinner, SpinnerSize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CalendarView {
    Month,
    Week,
    Day,
    Year,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AvailabilityType {
    Available,
    Busy,
    Preferred,
    Unavailable,
}

impl AvailabilityType {
    pub fn color_class(&self) -> &'static str {
        match self {
            AvailabilityType::Available => "bg-green-100 border-green-300 text-green-800",
            AvailabilityType::Busy => "bg-red-100 border-red-300 text-red-800",
            AvailabilityType::Preferred => "bg-blue-100 border-blue-300 text-blue-800",
            AvailabilityType::Unavailable => "bg-gray-100 border-gray-300 text-gray-800",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AvailabilityType::Available => "Available",
            AvailabilityType::Busy => "Busy",
            AvailabilityType::Preferred => "Preferred",
            AvailabilityType::Unavailable => "Unavailable",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilitySlot {
    pub id: uuid::Uuid,
    pub date: NaiveDate,
    pub start_time: String, // "09:00"
    pub end_time: String,   // "17:00"
    pub availability_type: AvailabilityType,
    pub notes: Option<String>,
    pub job_type_preference: Option<String>,
    pub hourly_rate: Option<f64>,
    pub is_recurring: bool,
    pub recurring_pattern: Option<RecurringPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringPattern {
    pub pattern_type: RecurringType,
    pub end_date: Option<NaiveDate>,
    pub days_of_week: Vec<Weekday>,
    pub interval: u32, // Every N weeks/months
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecurringType {
    Daily,
    Weekly,
    Monthly,
    Custom,
}

#[component]
pub fn CalendarSystem(
    #[prop(optional)] user_id: Option<uuid::Uuid>,
    #[prop(optional)] on_availability_change: Option<Box<dyn Fn(Vec<AvailabilitySlot>) + 'static>>,
) -> impl IntoView {
    let (current_view, set_current_view) = create_signal(CalendarView::Month);
    let (current_date, set_current_date) = create_signal(Utc::now().date_naive());
    let (availability_slots, set_availability_slots) = create_signal(HashMap::<NaiveDate, Vec<AvailabilitySlot>>::new());
    let (selected_date, set_selected_date) = create_signal(None::<NaiveDate>);
    let (is_creating_slot, set_is_creating_slot) = create_signal(false);
    let (drag_start_date, set_drag_start_date) = create_signal(None::<NaiveDate>);
    let (drag_end_date, set_drag_end_date) = create_signal(None::<NaiveDate>);
    let (show_recurring_modal, set_show_recurring_modal) = create_signal(false);
    let (is_loading, set_is_loading) = create_signal(false);

    // Initialize with sample data
    create_effect(move |_| {
        let mut slots = HashMap::new();
        
        // Add some sample availability slots
        let today = Utc::now().date_naive();
        for i in 0..30 {
            let date = today + ChronoDuration::days(i);
            let mut date_slots = Vec::new();
            
            // Add random availability
            if i % 3 == 0 {
                date_slots.push(AvailabilitySlot {
                    id: uuid::Uuid::new_v4(),
                    date,
                    start_time: "09:00".to_string(),
                    end_time: "17:00".to_string(),
                    availability_type: AvailabilityType::Available,
                    notes: Some("Available for any pharmacy work".to_string()),
                    job_type_preference: None,
                    hourly_rate: Some(45.0),
                    is_recurring: false,
                    recurring_pattern: None,
                });
            } else if i % 4 == 0 {
                date_slots.push(AvailabilitySlot {
                    id: uuid::Uuid::new_v4(),
                    date,
                    start_time: "10:00".to_string(),
                    end_time: "14:00".to_string(),
                    availability_type: AvailabilityType::Preferred,
                    notes: Some("Prefer morning shifts".to_string()),
                    job_type_preference: Some("Weekend Relief".to_string()),
                    hourly_rate: Some(50.0),
                    is_recurring: false,
                    recurring_pattern: None,
                });
            }
            
            if !date_slots.is_empty() {
                slots.insert(date, date_slots);
            }
        }
        
        set_availability_slots.set(slots);
    });

    // Navigation functions
    let navigate_previous = move |_| {
        match current_view.get() {
            CalendarView::Month => {
                set_current_date.update(|date| {
                    *date = date.with_day(1).unwrap().checked_sub_months(chrono::Months::new(1)).unwrap();
                });
            }
            CalendarView::Week => {
                set_current_date.update(|date| {
                    *date = *date - ChronoDuration::weeks(1);
                });
            }
            CalendarView::Day => {
                set_current_date.update(|date| {
                    *date = *date - ChronoDuration::days(1);
                });
            }
            CalendarView::Year => {
                set_current_date.update(|date| {
                    *date = date.with_year(date.year() - 1).unwrap();
                });
            }
        }
    };

    let navigate_next = move |_| {
        match current_view.get() {
            CalendarView::Month => {
                set_current_date.update(|date| {
                    *date = date.with_day(1).unwrap().checked_add_months(chrono::Months::new(1)).unwrap();
                });
            }
            CalendarView::Week => {
                set_current_date.update(|date| {
                    *date = *date + ChronoDuration::weeks(1);
                });
            }
            CalendarView::Day => {
                set_current_date.update(|date| {
                    *date = *date + ChronoDuration::days(1);
                });
            }
            CalendarView::Year => {
                set_current_date.update(|date| {
                    *date = date.with_year(date.year() + 1).unwrap();
                });
            }
        }
    };

    let go_to_today = move |_| {
        set_current_date.set(Utc::now().date_naive());
    };

    // Create availability slot
    let create_availability_slot = move |date: NaiveDate, availability_type: AvailabilityType| {
        let new_slot = AvailabilitySlot {
            id: uuid::Uuid::new_v4(),
            date,
            start_time: "09:00".to_string(),
            end_time: "17:00".to_string(),
            availability_type,
            notes: None,
            job_type_preference: None,
            hourly_rate: Some(45.0),
            is_recurring: false,
            recurring_pattern: None,
        };

        set_availability_slots.update(|slots| {
            slots.entry(date).or_insert_with(Vec::new).push(new_slot.clone());
        });

        if let Some(ref callback) = on_availability_change {
            let all_slots: Vec<AvailabilitySlot> = availability_slots.get()
                .values()
                .flatten()
                .cloned()
                .collect();
            callback(all_slots);
        }
    };

    // Handle date click
    let handle_date_click = move |date: NaiveDate| {
        move |_| {
            set_selected_date.set(Some(date));
            if is_creating_slot.get() {
                create_availability_slot(date, AvailabilityType::Available);
                set_is_creating_slot.set(false);
            }
        }
    };

    // View switcher
    let switch_view = move |view: CalendarView| {
        move |_| set_current_view.set(view.clone())
    };

    view! {
        <div class="bg-white rounded-lg shadow-lg">
            // Header
            <div class="px-6 py-4 border-b bg-gray-50 rounded-t-lg">
                <div class="flex items-center justify-between">
                    <div>
                        <h2 class="text-xl font-semibold text-gray-900">
                            "Availability Calendar"
                        </h2>
                        <p class="text-sm text-gray-600 mt-1">
                            "Manage your work availability and preferences"
                        </p>
                    </div>
                    
                    <div class="flex items-center gap-2">
                        <Button 
                            variant=ButtonVariant::Secondary
                            on_click=move |_| set_is_creating_slot.update(|creating| *creating = !*creating)
                        >
                            {move || if is_creating_slot.get() { "Cancel" } else { "Add Availability" }}
                        </Button>
                        
                        <Button 
                            variant=ButtonVariant::Secondary
                            on_click=move |_| set_show_recurring_modal.set(true)
                        >
                            "Recurring Pattern"
                        </Button>
                    </div>
                </div>

                // View controls
                <div class="flex items-center justify-between mt-4">
                    <div class="flex items-center gap-2">
                        <Button 
                            variant=ButtonVariant::Secondary
                            on_click=navigate_previous
                        >
                            "←"
                        </Button>
                        
                        <Button 
                            variant=ButtonVariant::Secondary
                            on_click=go_to_today
                        >
                            "Today"
                        </Button>
                        
                        <Button 
                            variant=ButtonVariant::Secondary
                            on_click=navigate_next
                        >
                            "→"
                        </Button>
                        
                        <h3 class="text-lg font-medium text-gray-900 ml-4">
                            {move || match current_view.get() {
                                CalendarView::Month => current_date.get().format("%B %Y").to_string(),
                                CalendarView::Week => format!("Week of {}", current_date.get().format("%B %d, %Y")),
                                CalendarView::Day => current_date.get().format("%A, %B %d, %Y").to_string(),
                                CalendarView::Year => current_date.get().year().to_string(),
                            }}
                        </h3>
                    </div>
                    
                    <div class="flex gap-1">
                        {[
                            (CalendarView::Month, "Month"),
                            (CalendarView::Week, "Week"),
                            (CalendarView::Day, "Day"),
                            (CalendarView::Year, "Year"),
                        ].into_iter()
                            .map(|(view, label)| {
                                view! {
                                    <button
                                        class=move || format!(
                                            "px-3 py-1 text-sm rounded {}",
                                            if current_view.get() == view {
                                                "bg-blue-100 text-blue-700 border border-blue-300"
                                            } else {
                                                "text-gray-600 hover:bg-gray-100"
                                            }
                                        )
                                        on:click=switch_view(view.clone())
                                    >
                                        {label}
                                    </button>
                                }
                            })
                            .collect_view()
                        }
                    </div>
                </div>

                // Legend
                <div class="flex items-center gap-4 mt-4 text-sm">
                    {[
                        AvailabilityType::Available,
                        AvailabilityType::Preferred,
                        AvailabilityType::Busy,
                        AvailabilityType::Unavailable,
                    ].into_iter()
                        .map(|availability_type| {
                            view! {
                                <div class="flex items-center gap-1">
                                    <div class={format!("w-3 h-3 rounded border {}", availability_type.color_class())}></div>
                                    <span class="text-gray-600">{availability_type.display_name()}</span>
                                </div>
                            }
                        })
                        .collect_view()
                    }
                </div>
            </div>

            // Calendar content
            <div class="p-6">
                {move || {
                    if is_loading.get() {
                        view! {
                            <div class="flex justify-center py-12">
                                <LoadingSpinner size=SpinnerSize::Large />
                            </div>
                        }.into_view()
                    } else {
                        match current_view.get() {
                            CalendarView::Month => view! {
                                <MonthView
                                    current_date=current_date
                                    availability_slots=availability_slots
                                    selected_date=selected_date
                                    on_date_click=handle_date_click
                                    is_creating_slot=is_creating_slot
                                />
                            }.into_view(),
                            CalendarView::Week => view! {
                                <WeekView
                                    current_date=current_date
                                    availability_slots=availability_slots
                                    on_date_click=handle_date_click
                                />
                            }.into_view(),
                            CalendarView::Day => view! {
                                <DayView
                                    current_date=current_date
                                    availability_slots=availability_slots
                                />
                            }.into_view(),
                            CalendarView::Year => view! {
                                <YearView
                                    current_date=current_date
                                    availability_slots=availability_slots
                                    on_date_click=handle_date_click
                                />
                            }.into_view(),
                        }
                    }
                }}
            </div>

            // Availability details panel
            {move || {
                if let Some(date) = selected_date.get() {
                    let slots = availability_slots.get().get(&date).cloned().unwrap_or_default();
                    view! {
                        <AvailabilityDetailsPanel
                            date=date
                            slots=slots
                            on_close=move || set_selected_date.set(None)
                        />
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}
        </div>
    }
}

#[component]
fn MonthView(
    current_date: ReadSignal<NaiveDate>,
    availability_slots: ReadSignal<HashMap<NaiveDate, Vec<AvailabilitySlot>>>,
    selected_date: ReadSignal<Option<NaiveDate>>,
    on_date_click: impl Fn(NaiveDate) -> Box<dyn Fn() + 'static> + 'static,
    is_creating_slot: ReadSignal<bool>,
) -> impl IntoView {
    // Generate calendar days for the month
    let calendar_days = move || {
        let date = current_date.get();
        let start_of_month = date.with_day(1).unwrap();
        let start_of_calendar = start_of_month - ChronoDuration::days(start_of_month.weekday().num_days_from_monday() as i64);
        
        let mut days = Vec::new();
        for i in 0..42 { // 6 weeks * 7 days
            let day = start_of_calendar + ChronoDuration::days(i);
            days.push(day);
        }
        days
    };

    view! {
        <div class="grid grid-cols-7 gap-1">
            // Day headers
            {["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
                .into_iter()
                .map(|day| {
                    view! {
                        <div class="p-2 text-center text-sm font-medium text-gray-600 bg-gray-50">
                            {day}
                        </div>
                    }
                })
                .collect_view()
            }

            // Calendar days
            {move || {
                calendar_days()
                    .into_iter()
                    .map(|day| {
                        let is_current_month = day.month() == current_date.get().month();
                        let is_today = day == Utc::now().date_naive();
                        let is_selected = selected_date.get() == Some(day);
                        let slots = availability_slots.get().get(&day).cloned().unwrap_or_default();
                        let has_availability = !slots.is_empty();
                        
                        view! {
                            <div
                                class=move || format!(
                                    "min-h-24 p-2 border cursor-pointer transition-colors {}{}{}{}{}",
                                    if is_current_month { "bg-white" } else { "bg-gray-50 text-gray-400" },
                                    if is_today { " ring-2 ring-blue-500" } else { "" },
                                    if is_selected { " bg-blue-50 border-blue-300" } else { " border-gray-200" },
                                    if has_availability { " border-l-4 border-l-green-500" } else { "" },
                                    if is_creating_slot.get() { " hover:bg-yellow-50" } else { " hover:bg-gray-50" }
                                )
                                on:click=(on_date_click)(day)
                            >
                                <div class="flex justify-between items-start">
                                    <span class=format!(
                                        "text-sm {}",
                                        if is_today { "font-bold text-blue-600" } else { "text-gray-900" }
                                    )>
                                        {day.day()}
                                    </span>
                                    
                                    {if has_availability {
                                        view! {
                                            <span class="text-xs bg-green-100 text-green-800 px-1 rounded">
                                                {slots.len()}
                                            </span>
                                        }.into_view()
                                    } else {
                                        view! { <div></div> }.into_view()
                                    }}
                                </div>
                                
                                // Show availability indicators
                                <div class="mt-1 space-y-1">
                                    {slots.into_iter()
                                        .take(3) // Show max 3 slots
                                        .map(|slot| {
                                            view! {
                                                <div class={format!(
                                                    "text-xs px-1 py-0.5 rounded border {}",
                                                    slot.availability_type.color_class()
                                                )}>
                                                    {format!("{}-{}", slot.start_time, slot.end_time)}
                                                </div>
                                            }
                                        })
                                        .collect_view()
                                    }
                                </div>
                            </div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

#[component]
fn WeekView(
    current_date: ReadSignal<NaiveDate>,
    availability_slots: ReadSignal<HashMap<NaiveDate, Vec<AvailabilitySlot>>>,
    on_date_click: impl Fn(NaiveDate) -> Box<dyn Fn() + 'static> + 'static,
) -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="text-center text-gray-600">
                "Week view - Coming soon"
            </div>
            <div class="grid grid-cols-7 gap-4">
                // Simplified week view for now
                {(0..7).map(|i| {
                    let day = current_date.get() - ChronoDuration::days(current_date.get().weekday().num_days_from_monday() as i64) + ChronoDuration::days(i);
                    let slots = availability_slots.get().get(&day).cloned().unwrap_or_default();
                    
                    view! {
                        <div class="bg-gray-50 p-4 rounded-lg">
                            <h3 class="font-medium text-gray-900 mb-2">
                                {day.format("%a %d").to_string()}
                            </h3>
                            <div class="space-y-1">
                                {slots.into_iter()
                                    .map(|slot| {
                                        view! {
                                            <div class={format!(
                                                "text-xs px-2 py-1 rounded {}",
                                                slot.availability_type.color_class()
                                            )}>
                                                {format!("{}-{}", slot.start_time, slot.end_time)}
                                            </div>
                                        }
                                    })
                                    .collect_view()
                                }
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn DayView(
    current_date: ReadSignal<NaiveDate>,
    availability_slots: ReadSignal<HashMap<NaiveDate, Vec<AvailabilitySlot>>>,
) -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="text-center">
                <h3 class="text-lg font-medium text-gray-900">
                    {move || current_date.get().format("%A, %B %d, %Y").to_string()}
                </h3>
            </div>
            
            {move || {
                let slots = availability_slots.get().get(&current_date.get()).cloned().unwrap_or_default();
                if slots.is_empty() {
                    view! {
                        <div class="text-center py-12 text-gray-500">
                            "No availability set for this day"
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="space-y-3">
                            {slots.into_iter()
                                .map(|slot| {
                                    view! {
                                        <div class={format!(
                                            "p-4 rounded-lg border {}",
                                            slot.availability_type.color_class()
                                        )}>
                                            <div class="flex justify-between items-start">
                                                <div>
                                                    <h4 class="font-medium">
                                                        {format!("{} - {}", slot.start_time, slot.end_time)}
                                                    </h4>
                                                    <p class="text-sm opacity-75 mt-1">
                                                        {slot.availability_type.display_name()}
                                                    </p>
                                                    {slot.notes.as_ref().map(|notes| {
                                                        view! {
                                                            <p class="text-sm mt-2 opacity-75">
                                                                {notes}
                                                            </p>
                                                        }
                                                    })}
                                                </div>
                                                {slot.hourly_rate.map(|rate| {
                                                    view! {
                                                        <span class="text-sm font-medium">
                                                            {format!("${:.2}/hr", rate)}
                                                        </span>
                                                    }
                                                })}
                                            </div>
                                        </div>
                                    }
                                })
                                .collect_view()
                            }
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

#[component]
fn YearView(
    current_date: ReadSignal<NaiveDate>,
    availability_slots: ReadSignal<HashMap<NaiveDate, Vec<AvailabilitySlot>>>,
    on_date_click: impl Fn(NaiveDate) -> Box<dyn Fn() + 'static> + 'static,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-3 md:grid-cols-4 gap-6">
            {(1..=12).map(|month| {
                let month_date = current_date.get().with_month(month).unwrap().with_day(1).unwrap();
                view! {
                    <div class="bg-gray-50 p-3 rounded-lg">
                        <h3 class="text-sm font-medium text-gray-900 mb-2 text-center">
                            {month_date.format("%B").to_string()}
                        </h3>
                        <div class="grid grid-cols-7 gap-1 text-xs">
                            // Mini month view would go here
                            <div class="text-center py-12 text-gray-400">
                                "Mini calendar"
                            </div>
                        </div>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
fn AvailabilityDetailsPanel(
    date: NaiveDate,
    slots: Vec<AvailabilitySlot>,
    on_close: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <div class="border-t bg-gray-50 p-6">
            <div class="flex items-center justify-between mb-4">
                <h3 class="text-lg font-medium text-gray-900">
                    {date.format("%A, %B %d, %Y").to_string()}
                </h3>
                <button
                    class="text-gray-400 hover:text-gray-600"
                    on:click=move |_| on_close()
                >
                    "✕"
                </button>
            </div>
            
            {if slots.is_empty() {
                view! {
                    <div class="text-center py-8 text-gray-500">
                        "No availability set for this day"
                        <div class="mt-4">
                            <Button variant=ButtonVariant::Primary>
                                "Add Availability"
                            </Button>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {
                    <div class="space-y-4">
                        {slots.into_iter()
                            .map(|slot| {
                                view! {
                                    <div class={format!(
                                        "p-4 rounded-lg border {}",
                                        slot.availability_type.color_class()
                                    )}>
                                        <div class="flex justify-between items-start mb-2">
                                            <div>
                                                <h4 class="font-medium">
                                                    {format!("{} - {}", slot.start_time, slot.end_time)}
                                                </h4>
                                                <p class="text-sm opacity-75">
                                                    {slot.availability_type.display_name()}
                                                </p>
                                            </div>
                                            {slot.hourly_rate.map(|rate| {
                                                view! {
                                                    <span class="text-sm font-medium">
                                                        {format!("${:.2}/hr", rate)}
                                                    </span>
                                                }
                                            })}
                                        </div>
                                        
                                        {slot.job_type_preference.as_ref().map(|pref| {
                                            view! {
                                                <p class="text-sm opacity-75 mb-2">
                                                    <span class="font-medium">"Preferred: "</span>
                                                    {pref}
                                                </p>
                                            }
                                        })}
                                        
                                        {slot.notes.as_ref().map(|notes| {
                                            view! {
                                                <p class="text-sm opacity-75">
                                                    {notes}
                                                </p>
                                            }
                                        })}
                                        
                                        <div class="flex justify-end mt-3 gap-2">
                                            <button class="text-sm text-blue-600 hover:text-blue-800">
                                                "Edit"
                                            </button>
                                            <button class="text-sm text-red-600 hover:text-red-800">
                                                "Delete"
                                            </button>
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()
                        }
                    </div>
                }.into_view()
            }}
        </div>
    }
}