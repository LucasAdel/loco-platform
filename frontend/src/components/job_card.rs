use dioxus::prelude::*;
use shared::{Job, JobType};
use shared::utils::format_currency;

#[derive(Props, Clone, PartialEq)]
pub struct JobCardProps {
    job: Job,
}

#[component]
pub fn JobCard(props: JobCardProps) -> Element {
    let job_type_color = match props.job.job_type {
        JobType::Intern => "text-blue-600",
        JobType::Student => "text-green-600", 
        JobType::Pharmacist => "text-purple-600",
        JobType::PharmacyAssistant => "text-orange-600",
        JobType::PharmacyTechnician => "text-indigo-600",
    };
    
    rsx! {
        div {
            class: "bg-white rounded-lg shadow-sm border border-gray-200 p-6 hover:shadow-md transition-shadow",
            
            // Job header
            div {
                class: "flex justify-between items-start mb-4",
                
                div {
                    class: "flex-1",
                    h3 {
                        class: "text-lg font-semibold text-gray-900 {job_type_color}",
                        "{props.job.title}"
                    }
                    p {
                        class: "text-gray-600 text-sm",
                        "{props.job.pharmacy_name}"
                    }
                }
                
                div {
                    class: "text-right",
                    span {
                        class: "text-xl font-bold text-teal-600",
                        "{format_currency(props.job.hourly_rate)}"
                    }
                    if props.job.is_urgent {
                        span {
                            class: "inline-block ml-2 px-2 py-1 bg-red-100 text-red-600 text-xs font-medium rounded",
                            "Urgent"
                        }
                    }
                }
            }
            
            // Job details
            div {
                class: "space-y-2 text-sm text-gray-600 mb-4",
                
                div {
                    class: "flex items-center",
                    span {
                        class: "mr-2",
                        "📍"
                    }
                    span {
                        "{props.job.address}"
                        if let Some(distance) = props.job.distance_km {
                            " ({distance:.1} km)"
                        }
                    }
                }
                
                div {
                    class: "flex items-center",
                    span {
                        class: "mr-2",
                        "📅"
                    }
                    span {
                        "{props.job.start_date.format(\"%d/%m/%Y\")}"
                    }
                }
                
                div {
                    class: "flex items-center",
                    span {
                        class: "mr-2",
                        "🕐"
                    }
                    span {
                        "{props.job.start_time} - {props.job.end_time}"
                    }
                }
            }
            
            // Action buttons
            div {
                class: "flex space-x-3",
                
                button {
                    class: "px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors font-medium",
                    onclick: move |_| {
                        tracing::info!("View on Map clicked for job: {}", props.job.id);
                    },
                    "View on Map"
                }
                
                button {
                    class: "px-4 py-2 bg-teal-500 text-white rounded-lg hover:bg-teal-600 transition-colors font-medium",
                    onclick: move |_| {
                        tracing::info!("View Details clicked for job: {}", props.job.id);
                    },
                    "View Details"
                }
            }
        }
    }
}