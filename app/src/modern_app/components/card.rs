use leptos::*;
use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] hoverable: bool,
    #[prop(optional)] glass: bool,
    children: Children,
) -> impl IntoView {
    let base_classes = if glass {
        "bg-white/70 backdrop-blur-xl border border-white/20 rounded-2xl shadow-xl"
    } else {
        "bg-white rounded-2xl shadow-lg"
    };
    
    let hover_classes = if hoverable {
        "transition-all duration-300 hover:shadow-2xl hover:scale-[1.02] cursor-pointer"
    } else {
        ""
    };
    
    view! {
        <div class=format!("{} {} p-6 {}", base_classes, hover_classes, class)>
            {children()}
        </div>
    }
}

#[component]
pub fn JobCard(job: Job) -> impl IntoView {
    view! {
        <Card hoverable=true glass=true>
            <div class="flex justify-between items-start">
                <div class="flex-1">
                    <h3 class="text-xl font-semibold text-gray-900 mb-2">{&job.title}</h3>
                    <p class="text-gray-600 mb-4">{&job.company}</p>
                    
                    <div class="flex flex-wrap gap-2 mb-4">
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800">
                            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                            </svg>
                            {&job.location}
                        </span>
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-green-100 text-green-800">
                            {&job.job_type}
                        </span>
                        {job.remote_option.unwrap_or(false).then(|| view! {
                            <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-purple-100 text-purple-800">
                                "Remote"
                            </span>
                        })}
                    </div>
                    
                    <p class="text-gray-700 line-clamp-2 mb-4">{&job.description}</p>
                    
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-2xl font-bold text-gray-900">
                                {format_salary(&job.salary_min, &job.salary_max)}
                            </p>
                            <p class="text-sm text-gray-500 mt-1">{&job.posted_date}</p>
                        </div>
                        
                        <a 
                            href=format!("/jobs/{}", &job.id)
                            class="inline-flex items-center px-4 py-2 bg-gradient-to-r from-blue-600 to-blue-700 text-white font-medium rounded-xl hover:from-blue-700 hover:to-blue-800 transition-all duration-200 transform hover:scale-105"
                        >
                            "View Details"
                            <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                            </svg>
                        </a>
                    </div>
                </div>
            </div>
        </Card>
    }
}

#[derive(Clone, Debug)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub description: String,
    pub salary_min: Option<f64>,
    pub salary_max: Option<f64>,
    pub job_type: String,
    pub posted_date: String,
    pub remote_option: Option<bool>,
}

fn format_salary(min: &Option<f64>, max: &Option<f64>) -> String {
    match (min, max) {
        (Some(min), Some(max)) => format!("${:.0}k - ${:.0}k", min / 1000.0, max / 1000.0),
        (Some(min), None) => format!("${:.0}k+", min / 1000.0),
        (None, Some(max)) => format!("Up to ${:.0}k", max / 1000.0),
        (None, None) => "Competitive".to_string(),
    }
}