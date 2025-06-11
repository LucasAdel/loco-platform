use leptos::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use shared::types::{Application, ApplicationStatus};
use crate::components::ui::{Button, ButtonVariant, LoadingSpinner, SpinnerSize, Alert, AlertVariant};
use crate::api::applications::{fetch_applications, update_application_status};
use web_sys::{DragEvent, DataTransfer, HtmlElement};
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationColumn {
    pub status: ApplicationStatus,
    pub title: String,
    pub color: String,
    pub applications: Vec<Application>,
}

impl ApplicationColumn {
    pub fn new(status: ApplicationStatus) -> Self {
        Self {
            title: match status {
                ApplicationStatus::Pending => "Pending Review".to_string(),
                ApplicationStatus::Reviewing => "Under Review".to_string(),
                ApplicationStatus::Shortlisted => "Shortlisted".to_string(),
                ApplicationStatus::Interviewed => "Interviewed".to_string(),
                ApplicationStatus::Offered => "Offered".to_string(),
                ApplicationStatus::Accepted => "Accepted".to_string(),
                ApplicationStatus::Rejected => "Rejected".to_string(),
                ApplicationStatus::Withdrawn => "Withdrawn".to_string(),
            },
            color: match status {
                ApplicationStatus::Pending => "bg-gray-100 border-gray-300".to_string(),
                ApplicationStatus::Reviewing => "bg-blue-100 border-blue-300".to_string(),
                ApplicationStatus::Shortlisted => "bg-yellow-100 border-yellow-300".to_string(),
                ApplicationStatus::Interviewed => "bg-purple-100 border-purple-300".to_string(),
                ApplicationStatus::Offered => "bg-orange-100 border-orange-300".to_string(),
                ApplicationStatus::Accepted => "bg-green-100 border-green-300".to_string(),
                ApplicationStatus::Rejected => "bg-red-100 border-red-300".to_string(),
                ApplicationStatus::Withdrawn => "bg-gray-100 border-gray-400".to_string(),
            },
            status,
            applications: Vec::new(),
        }
    }
}

#[component]
pub fn ApplicationBoard(
    #[prop(optional)] job_id: Option<uuid::Uuid>,
    #[prop(optional)] user_id: Option<uuid::Uuid>,
) -> impl IntoView {
    let (columns, set_columns) = create_signal(Vec::<ApplicationColumn>::new());
    let (selected_applications, set_selected_applications) = create_signal(Vec::<uuid::Uuid>::new());
    let (is_loading, set_is_loading) = create_signal(true);
    let (error_message, set_error_message) = create_signal(None::<String>);
    let (dragging_application, set_dragging_application) = create_signal(None::<uuid::Uuid>);

    // Initialize columns
    let init_columns = move || {
        let mut cols = vec![
            ApplicationColumn::new(ApplicationStatus::Pending),
            ApplicationColumn::new(ApplicationStatus::Reviewing),
            ApplicationColumn::new(ApplicationStatus::Shortlisted),
            ApplicationColumn::new(ApplicationStatus::Interviewed),
            ApplicationColumn::new(ApplicationStatus::Offered),
            ApplicationColumn::new(ApplicationStatus::Accepted),
            ApplicationColumn::new(ApplicationStatus::Rejected),
        ];
        set_columns.set(cols);
    };

    // Load applications
    let load_applications = move || {
        set_is_loading.set(true);
        spawn_local(async move {
            match fetch_applications(job_id, user_id).await {
                Ok(applications) => {
                    // Group applications by status
                    let mut cols = columns.get();
                    for col in &mut cols {
                        col.applications.clear();
                    }
                    
                    for app in applications {
                        if let Some(col) = cols.iter_mut().find(|c| c.status == app.status) {
                            col.applications.push(app);
                        }
                    }
                    
                    set_columns.set(cols);
                    set_error_message.set(None);
                }
                Err(e) => {
                    set_error_message.set(Some(format!("Failed to load applications: {}", e)));
                }
            }
            set_is_loading.set(false);
        });
    };

    // Initialize board
    create_effect(move |_| {
        init_columns();
        load_applications();
    });

    // Handle drag start
    let handle_drag_start = move |app_id: uuid::Uuid| {
        move |ev: DragEvent| {
            set_dragging_application.set(Some(app_id));
            
            if let Some(data_transfer) = ev.data_transfer() {
                data_transfer.set_data("text/plain", &app_id.to_string()).ok();
                data_transfer.set_effect_allowed("move");
            }
        }
    };

    // Handle drag over
    let handle_drag_over = move |_status: ApplicationStatus| {
        move |ev: DragEvent| {
            ev.prevent_default();
            if let Some(data_transfer) = ev.data_transfer() {
                data_transfer.set_drop_effect("move");
            }
        }
    };

    // Handle drop
    let handle_drop = move |new_status: ApplicationStatus| {
        move |ev: DragEvent| {
            ev.prevent_default();
            
            if let Some(data_transfer) = ev.data_transfer() {
                if let Ok(app_id_str) = data_transfer.get_data("text/plain") {
                    if let Ok(app_id) = uuid::Uuid::parse_str(&app_id_str) {
                        // Update application status
                        spawn_local(async move {
                            match update_application_status(app_id, new_status).await {
                                Ok(_) => {
                                    // Refresh the board
                                    load_applications();
                                }
                                Err(e) => {
                                    set_error_message.set(Some(format!("Failed to update application: {}", e)));
                                }
                            }
                        });
                    }
                }
            }
            
            set_dragging_application.set(None);
        }
    };

    // Toggle application selection
    let toggle_selection = move |app_id: uuid::Uuid| {
        move |_| {
            set_selected_applications.update(|selected| {
                if let Some(pos) = selected.iter().position(|&id| id == app_id) {
                    selected.remove(pos);
                } else {
                    selected.push(app_id);
                }
            });
        }
    };

    // Bulk action handlers
    let bulk_approve = move |_| {
        let selected = selected_applications.get();
        if !selected.is_empty() {
            spawn_local(async move {
                for app_id in selected {
                    update_application_status(app_id, ApplicationStatus::Shortlisted).await.ok();
                }
                load_applications();
                set_selected_applications.set(Vec::new());
            });
        }
    };

    let bulk_reject = move |_| {
        let selected = selected_applications.get();
        if !selected.is_empty() {
            spawn_local(async move {
                for app_id in selected {
                    update_application_status(app_id, ApplicationStatus::Rejected).await.ok();
                }
                load_applications();
                set_selected_applications.set(Vec::new());
            });
        }
    };

    view! {
        <div class="p-6">
            <div class="mb-6">
                <h1 class="text-2xl font-bold text-gray-900 mb-2">
                    "Application Board"
                </h1>
                <p class="text-gray-600">
                    "Drag and drop applications to change their status"
                </p>
                
                // Bulk actions toolbar
                {move || {
                    let selected_count = selected_applications.get().len();
                    if selected_count > 0 {
                        view! {
                            <div class="mt-4 p-3 bg-blue-50 border border-blue-200 rounded-lg">
                                <div class="flex items-center justify-between">
                                    <span class="text-blue-800 font-medium">
                                        {selected_count} " applications selected"
                                    </span>
                                    <div class="flex gap-2">
                                        <Button 
                                            variant=ButtonVariant::Secondary
                                            on_click=bulk_approve
                                        >
                                            "Shortlist Selected"
                                        </Button>
                                        <Button 
                                            variant=ButtonVariant::Danger
                                            on_click=bulk_reject
                                        >
                                            "Reject Selected"
                                        </Button>
                                        <Button 
                                            variant=ButtonVariant::Secondary
                                            on_click=move |_| set_selected_applications.set(Vec::new())
                                        >
                                            "Clear Selection"
                                        </Button>
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                }}
            </div>

            // Error message
            {move || {
                if let Some(error) = error_message.get() {
                    view! {
                        <div class="mb-4">
                            <Alert variant=AlertVariant::Error>
                                {error}
                            </Alert>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}

            // Loading state
            {move || {
                if is_loading.get() {
                    view! {
                        <div class="flex justify-center py-12">
                            <LoadingSpinner size=SpinnerSize::Large />
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-4 gap-6">
                            {columns.get()
                                .into_iter()
                                .map(|column| {
                                    let status = column.status;
                                    view! {
                                        <ApplicationColumn
                                            column=column
                                            on_drag_over=handle_drag_over(status)
                                            on_drop=handle_drop(status)
                                            on_drag_start=handle_drag_start
                                            on_toggle_selection=toggle_selection
                                            selected_applications=selected_applications
                                            dragging_application=dragging_application
                                        />
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
fn ApplicationColumn(
    column: ApplicationColumn,
    on_drag_over: Box<dyn Fn(DragEvent) + 'static>,
    on_drop: Box<dyn Fn(DragEvent) + 'static>,
    on_drag_start: Box<dyn Fn(uuid::Uuid) -> Box<dyn Fn(DragEvent) + 'static> + 'static>,
    on_toggle_selection: Box<dyn Fn(uuid::Uuid) -> Box<dyn Fn() + 'static> + 'static>,
    selected_applications: ReadSignal<Vec<uuid::Uuid>>,
    dragging_application: ReadSignal<Option<uuid::Uuid>>,
) -> impl IntoView {
    view! {
        <div class={format!("border-2 border-dashed rounded-lg p-4 min-h-96 {}", column.color)}>
            <div class="mb-4">
                <h3 class="font-semibold text-gray-900 mb-1">{column.title}</h3>
                <span class="text-sm text-gray-600">
                    {column.applications.len()} " applications"
                </span>
            </div>

            <div 
                class="space-y-3 min-h-48"
                on:dragover=move |ev| on_drag_over(ev)
                on:drop=move |ev| on_drop(ev)
            >
                {column.applications
                    .into_iter()
                    .map(|app| {
                        let app_id = app.id;
                        view! {
                            <ApplicationCard
                                application=app
                                is_selected=move || selected_applications.get().contains(&app_id)
                                is_dragging=move || dragging_application.get() == Some(app_id)
                                on_drag_start=move |ev| (on_drag_start)(app_id)(ev)
                                on_toggle_selection=move || (on_toggle_selection)(app_id)()
                            />
                        }
                    })
                    .collect_view()
                }
            </div>
        </div>
    }
}

#[component]
fn ApplicationCard(
    application: Application,
    is_selected: impl Fn() -> bool + 'static,
    is_dragging: impl Fn() -> bool + 'static,
    on_drag_start: impl Fn(DragEvent) + 'static,
    on_toggle_selection: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <div 
            class=move || format!(
                "bg-white rounded-lg p-4 shadow-sm border cursor-move transition-all duration-200 {}{}",
                if is_selected() { " ring-2 ring-blue-500 border-blue-300" } else { " border-gray-200 hover:shadow-md" },
                if is_dragging() { " opacity-50 transform scale-95" } else { "" }
            )
            draggable="true"
            on:dragstart=move |ev| on_drag_start(ev)
            on:click=move |_| on_toggle_selection()
        >
            <div class="flex items-start justify-between mb-2">
                <div class="flex items-center">
                    <input
                        type="checkbox"
                        class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                        checked=is_selected
                        on:change=move |_| on_toggle_selection()
                        on:click=move |ev| ev.stop_propagation()
                    />
                    <h4 class="font-medium text-gray-900 truncate">
                        {application.job_title.unwrap_or_else(|| "Job Application".to_string())}
                    </h4>
                </div>
                
                <span class="text-xs px-2 py-1 bg-gray-100 text-gray-600 rounded-full">
                    {format!("{:?}", application.status)}
                </span>
            </div>

            <div class="space-y-1 text-sm text-gray-600">
                <p>
                    <span class="font-medium">"Applicant: "</span>
                    {application.user_name.unwrap_or_else(|| "Unknown".to_string())}
                </p>
                
                <p>
                    <span class="font-medium">"Applied: "</span>
                    {application.created_at.format("%d/%m/%Y").to_string()}
                </p>
                
                {application.cover_letter.as_ref().map(|letter| {
                    let preview = if letter.len() > 100 {
                        format!("{}...", &letter[..100])
                    } else {
                        letter.clone()
                    };
                    view! {
                        <p class="text-gray-500 text-xs mt-2 italic">
                            {preview}
                        </p>
                    }
                })}
            </div>

            <div class="mt-3 flex justify-end">
                <button class="text-blue-600 hover:text-blue-800 text-sm font-medium">
                    "View Details"
                </button>
            </div>
        </div>
    }
}