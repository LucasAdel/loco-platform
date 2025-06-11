use leptos::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use shared::types::{JobType, AustralianState, CreateJobRequest};
use crate::components::ui::{Button, ButtonVariant, Alert, AlertVariant, LoadingSpinner, SpinnerSize};
use crate::components::forms::Input;
use crate::api::jobs::create_job;
use web_sys::{FormData, HtmlFormElement, Storage};
use gloo_timers::callback::Interval;
use js_sys::Date;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

const ENHANCED_WIZARD_CSS: &str = r#"
/* Ultra-Enhanced Job Creation Wizard CSS Framework */

/* Glass Morphism Base */
.glass {
    background: rgba(255, 255, 255, 0.8);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid rgba(255, 255, 255, 0.3);
}

.glass-tiffany {
    background: rgba(23, 221, 184, 0.1);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid rgba(23, 221, 184, 0.2);
}

/* Text Gradients */
.text-gradient {
    background: linear-gradient(135deg, #17ddb8, #3b82f6);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
}

/* Wizard Step Buttons */
.wizard-step-button {
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    border: 1px solid transparent;
}

.wizard-step-button.active-step {
    background: linear-gradient(135deg, rgba(23, 221, 184, 0.1), rgba(59, 130, 246, 0.1));
    border-color: rgba(23, 221, 184, 0.3);
    transform: translateX(8px);
    box-shadow: 0 8px 25px rgba(23, 221, 184, 0.2);
}

.wizard-step-button.completed-step {
    background: rgba(34, 197, 94, 0.1);
    border-color: rgba(34, 197, 94, 0.2);
}

.wizard-step-button.inactive-step {
    background: rgba(255, 255, 255, 0.5);
    border-color: rgba(229, 231, 235, 0.5);
}

.wizard-step-button:hover {
    transform: translateX(4px);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
}

/* Navigation Buttons */
.wizard-nav-button {
    padding: 12px 24px;
    border-radius: 12px;
    font-weight: 600;
    transition: all 0.3s ease;
    border: none;
    cursor: pointer;
    backdrop-filter: blur(10px);
    position: relative;
    overflow: hidden;
}

.wizard-nav-button::before {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
    transition: left 0.5s ease;
}

.wizard-nav-button:hover::before {
    left: 100%;
}

.previous-button {
    background: rgba(255, 255, 255, 0.8);
    color: #374151;
    border: 1px solid rgba(0, 0, 0, 0.1);
}

.previous-button:hover {
    background: rgba(255, 255, 255, 0.9);
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
}

.next-button {
    background: linear-gradient(135deg, #17ddb8, #3b82f6);
    color: white;
    box-shadow: 0 4px 15px rgba(23, 221, 184, 0.3);
}

.next-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(23, 221, 184, 0.4);
}

.publish-button {
    background: linear-gradient(135deg, #10b981, #059669);
    color: white;
    box-shadow: 0 4px 15px rgba(16, 185, 129, 0.3);
    position: relative;
}

.publish-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(16, 185, 129, 0.4);
}

.publish-button:disabled {
    opacity: 0.7;
    cursor: not-allowed;
    transform: none !important;
}

/* Publish Spinner */
.publish-spinner {
    width: 20px;
    height: 20px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top: 2px solid white;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

/* Enhanced Form Inputs */
.enhanced-input {
    background: rgba(255, 255, 255, 0.9);
    border: 2px solid transparent;
    border-radius: 12px;
    padding: 16px 20px;
    transition: all 0.3s ease;
    backdrop-filter: blur(10px);
    font-size: 16px;
    width: 100%;
}

.enhanced-input:focus {
    border-color: #17ddb8;
    box-shadow: 0 0 0 4px rgba(23, 221, 184, 0.1);
    outline: none;
    background: rgba(255, 255, 255, 1);
}

.enhanced-select {
    background: rgba(255, 255, 255, 0.9);
    border: 2px solid transparent;
    border-radius: 12px;
    padding: 16px 20px;
    transition: all 0.3s ease;
    backdrop-filter: blur(10px);
    font-size: 16px;
    width: 100%;
    cursor: pointer;
}

.enhanced-select:focus {
    border-color: #17ddb8;
    box-shadow: 0 0 0 4px rgba(23, 221, 184, 0.1);
    outline: none;
    background: rgba(255, 255, 255, 1);
}

.enhanced-textarea {
    background: rgba(255, 255, 255, 0.9);
    border: 2px solid transparent;
    border-radius: 12px;
    padding: 16px 20px;
    transition: all 0.3s ease;
    backdrop-filter: blur(10px);
    font-size: 16px;
    width: 100%;
    resize: vertical;
    min-height: 120px;
}

.enhanced-textarea:focus {
    border-color: #17ddb8;
    box-shadow: 0 0 0 4px rgba(23, 221, 184, 0.1);
    outline: none;
    background: rgba(255, 255, 255, 1);
}

/* Form Step Content Animations */
.form-step-content {
    animation: slideInUp 0.5s ease-out;
}

/* Enhanced Checkbox */
.enhanced-checkbox {
    width: 20px;
    height: 20px;
    border: 2px solid #d1d5db;
    border-radius: 6px;
    transition: all 0.3s ease;
    cursor: pointer;
    position: relative;
}

.enhanced-checkbox:checked {
    background: linear-gradient(135deg, #17ddb8, #3b82f6);
    border-color: #17ddb8;
}

.enhanced-checkbox:checked::after {
    content: '✓';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: white;
    font-size: 12px;
    font-weight: bold;
}

/* Badge Styles */
.skill-badge {
    background: linear-gradient(135deg, #17ddb8, #10b981);
    color: white;
    padding: 8px 16px;
    border-radius: 20px;
    font-size: 0.875rem;
    font-weight: 600;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    transition: all 0.3s ease;
}

.skill-badge:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(23, 221, 184, 0.3);
}

.skill-badge .remove-btn {
    background: rgba(255, 255, 255, 0.2);
    border: none;
    border-radius: 50%;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s ease;
}

.skill-badge .remove-btn:hover {
    background: rgba(255, 255, 255, 0.3);
    transform: scale(1.1);
}

/* Preview Card */
.preview-card {
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.9), rgba(249, 250, 251, 0.9));
    border: 1px solid rgba(229, 231, 235, 0.5);
    border-radius: 16px;
    padding: 24px;
    backdrop-filter: blur(10px);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    transition: all 0.3s ease;
}

.preview-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
}

/* Color Variables */
:root {
    --tiffany-blue: #17ddb8;
    --tiffany-dark: #0d9488;
    --tiffany-light: #a7f3d0;
}

.text-tiffany-blue { color: var(--tiffany-blue); }
.text-tiffany-dark { color: var(--tiffany-dark); }
.text-tiffany-light { color: var(--tiffany-light); }
.bg-tiffany-blue { background-color: var(--tiffany-blue); }

/* Animations */
@keyframes slideInUp {
    from {
        opacity: 0;
        transform: translateY(30px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

@keyframes float {
    0%, 100% { transform: translateY(0px) rotate(0deg); }
    50% { transform: translateY(-20px) rotate(5deg); }
}

@keyframes float-delayed {
    0%, 100% { transform: translateY(0px) rotate(0deg); }
    50% { transform: translateY(-15px) rotate(-3deg); }
}

@keyframes shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
}

@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}

.animate-float { animation: float 6s ease-in-out infinite; }
.animate-float-delayed { animation: float-delayed 8s ease-in-out infinite 2s; }
.animate-shimmer { animation: shimmer 2s infinite; }

/* Responsive Design */
@media (max-width: 1024px) {
    .wizard-step-button.active-step {
        transform: none;
    }
    
    .wizard-step-button:hover {
        transform: none;
    }
}

@media (max-width: 768px) {
    .enhanced-input,
    .enhanced-select,
    .enhanced-textarea {
        padding: 14px 16px;
        font-size: 16px; /* Prevent zoom on iOS */
    }
    
    .wizard-nav-button {
        padding: 14px 20px;
        font-size: 14px;
    }
}

/* Focus Indicators for Accessibility */
.wizard-step-button:focus-visible,
.wizard-nav-button:focus-visible,
.enhanced-input:focus-visible,
.enhanced-select:focus-visible,
.enhanced-textarea:focus-visible {
    outline: 2px solid #17ddb8;
    outline-offset: 2px;
}

/* High Contrast Mode Support */
@media (prefers-contrast: high) {
    .glass,
    .glass-tiffany {
        background: rgba(255, 255, 255, 0.95);
        border: 2px solid #000;
    }
    
    .text-gradient {
        background: none;
        -webkit-text-fill-color: initial;
        color: #000;
    }
}

/* Reduced Motion Support */
@media (prefers-reduced-motion: reduce) {
    * {
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.01ms !important;
    }
}
"#;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WizardStep {
    BasicInfo,
    JobDetails,
    Requirements,
    Compensation,
    WorkConditions,
    Application,
    Preview,
}

impl WizardStep {
    fn title(&self) -> &'static str {
        match self {
            WizardStep::BasicInfo => "Basic Information",
            WizardStep::JobDetails => "Job Details",
            WizardStep::Requirements => "Requirements & Skills",
            WizardStep::Compensation => "Compensation & Benefits",
            WizardStep::WorkConditions => "Work Conditions",
            WizardStep::Application => "Application Process",
            WizardStep::Preview => "Review & Publish",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            WizardStep::BasicInfo => "Start with the fundamentals of your job posting",
            WizardStep::JobDetails => "Define the role type and core responsibilities",
            WizardStep::Requirements => "Specify qualifications and essential skills",
            WizardStep::Compensation => "Set attractive compensation packages",
            WizardStep::WorkConditions => "Detail schedule and working arrangements",
            WizardStep::Application => "Configure how candidates can apply",
            WizardStep::Preview => "Final review before publishing your position",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            WizardStep::BasicInfo => "🏢",
            WizardStep::JobDetails => "📋",
            WizardStep::Requirements => "🎯",
            WizardStep::Compensation => "💰",
            WizardStep::WorkConditions => "⏰",
            WizardStep::Application => "📧",
            WizardStep::Preview => "👁️",
        }
    }

    fn next(&self) -> Option<WizardStep> {
        match self {
            WizardStep::BasicInfo => Some(WizardStep::JobDetails),
            WizardStep::JobDetails => Some(WizardStep::Requirements),
            WizardStep::Requirements => Some(WizardStep::Compensation),
            WizardStep::Compensation => Some(WizardStep::WorkConditions),
            WizardStep::WorkConditions => Some(WizardStep::Application),
            WizardStep::Application => Some(WizardStep::Preview),
            WizardStep::Preview => None,
        }
    }

    fn previous(&self) -> Option<WizardStep> {
        match self {
            WizardStep::BasicInfo => None,
            WizardStep::JobDetails => Some(WizardStep::BasicInfo),
            WizardStep::Requirements => Some(WizardStep::JobDetails),
            WizardStep::Compensation => Some(WizardStep::Requirements),
            WizardStep::WorkConditions => Some(WizardStep::Compensation),
            WizardStep::Application => Some(WizardStep::WorkConditions),
            WizardStep::Preview => Some(WizardStep::Application),
        }
    }

    fn step_number(&self) -> usize {
        match self {
            WizardStep::BasicInfo => 1,
            WizardStep::JobDetails => 2,
            WizardStep::Requirements => 3,
            WizardStep::Compensation => 4,
            WizardStep::WorkConditions => 5,
            WizardStep::Application => 6,
            WizardStep::Preview => 7,
        }
    }

    fn total_steps() -> usize {
        7
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobFormData {
    // Basic Info
    pub title: String,
    pub pharmacy_name: String,
    pub address: String,
    pub suburb: String,
    pub postcode: String,
    pub state: Option<AustralianState>,
    pub job_type: Option<JobType>,
    
    // Requirements
    pub requirements: Vec<String>,
    pub certifications: Vec<String>,
    pub experience_years: Option<u32>,
    pub must_have_car: bool,
    pub must_speak_english: bool,
    
    // Compensation
    pub hourly_rate: Option<f64>,
    pub salary_min: Option<u32>,
    pub salary_max: Option<u32>,
    pub benefits: Vec<String>,
    pub penalty_rates: bool,
    
    // Schedule
    pub start_date: String,
    pub end_date: String,
    pub start_time: String,
    pub end_time: String,
    pub days_of_week: Vec<String>,
    pub flexible_hours: bool,
    
    // Description
    pub description: String,
    pub special_instructions: String,
    pub contact_name: String,
    pub contact_phone: String,
    pub contact_email: String,
    
    // Additional
    pub is_urgent: bool,
    pub requires_references: bool,
}

impl JobFormData {
    pub fn to_create_request(&self) -> CreateJobRequest {
        CreateJobRequest {
            title: self.title.clone(),
            description: self.description.clone(),
            pharmacy_name: self.pharmacy_name.clone(),
            hourly_rate: self.hourly_rate.unwrap_or(0.0),
            address: self.address.clone(),
            suburb: self.suburb.clone(),
            postcode: self.postcode.clone(),
            state: self.state.clone().unwrap_or(AustralianState::NewSouthWales).to_string(),
            latitude: None, // Will be geocoded on backend
            longitude: None,
            start_date: chrono::NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d")
                .map(|date| date.and_hms_opt(0, 0, 0).unwrap().and_utc())
                .unwrap_or_else(|_| chrono::Utc::now()),
            end_date: chrono::NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d")
                .map(|date| date.and_hms_opt(23, 59, 59).unwrap().and_utc())
                .unwrap_or_else(|_| chrono::Utc::now()),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
            job_type: self.job_type.clone().unwrap_or(JobType::Pharmacist),
            is_urgent: self.is_urgent,
        }
    }

    pub fn validate_step(&self, step: &WizardStep) -> Vec<String> {
        let mut errors = Vec::new();

        match step {
            WizardStep::BasicInfo => {
                if self.title.trim().is_empty() {
                    errors.push("Job title is required".to_string());
                }
                if self.pharmacy_name.trim().is_empty() {
                    errors.push("Pharmacy name is required".to_string());
                }
                if self.address.trim().is_empty() {
                    errors.push("Address is required".to_string());
                }
                if self.suburb.trim().is_empty() {
                    errors.push("Suburb is required".to_string());
                }
                if self.postcode.trim().is_empty() {
                    errors.push("Postcode is required".to_string());
                }
                if self.state.is_none() {
                    errors.push("State is required".to_string());
                }
                if self.job_type.is_none() {
                    errors.push("Job type is required".to_string());
                }
            }
            WizardStep::Compensation => {
                if self.hourly_rate.is_none() && self.salary_min.is_none() {
                    errors.push("Either hourly rate or salary range is required".to_string());
                }
                if let (Some(min), Some(max)) = (self.salary_min, self.salary_max) {
                    if min >= max {
                        errors.push("Maximum salary must be greater than minimum".to_string());
                    }
                }
            }
            WizardStep::Schedule => {
                if self.start_date.trim().is_empty() {
                    errors.push("Start date is required".to_string());
                }
                if self.end_date.trim().is_empty() {
                    errors.push("End date is required".to_string());
                }
                if self.start_time.trim().is_empty() {
                    errors.push("Start time is required".to_string());
                }
                if self.end_time.trim().is_empty() {
                    errors.push("End time is required".to_string());
                }
            }
            WizardStep::Description => {
                if self.description.trim().is_empty() {
                    errors.push("Job description is required".to_string());
                }
                if self.contact_name.trim().is_empty() {
                    errors.push("Contact name is required".to_string());
                }
                if self.contact_email.trim().is_empty() {
                    errors.push("Contact email is required".to_string());
                }
            }
            _ => {} // No validation for other steps
        }

        errors
    }
}

#[component]
pub fn JobCreationWizard() -> impl IntoView {
    let (current_step, set_current_step) = create_signal(WizardStep::BasicInfo);
    let (form_data, set_form_data) = create_signal(JobFormData::default());
    let (errors, set_errors) = create_signal(Vec::<String>::new());
    let (is_submitting, set_is_submitting) = create_signal(false);
    let (success_message, set_success_message) = create_signal(None::<String>);
    let (auto_save_enabled, set_auto_save_enabled) = create_signal(true);
    let (last_saved, set_last_saved) = create_signal(None::<String>);
    let (completion_percentage, set_completion_percentage) = create_signal(0);

    // Auto-save functionality
    Effect::new({
        let form_data = form_data.clone();
        let auto_save_enabled = auto_save_enabled.clone();
        let set_last_saved = set_last_saved.clone();
        
        move |_| {
            if auto_save_enabled.get() {
                let data = form_data.get();
                if let Ok(json) = serde_json::to_string(&data) {
                    if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                        let _ = storage.set_item("job_wizard_draft", &json);
                        let now = Date::new_0().to_iso_string();
                        set_last_saved.set(Some(format!("Draft saved at {}", 
                            now.as_string().unwrap_or_default().split('T').nth(1).unwrap_or_default()[..5].to_string())));
                    }
                }
            }
        }
    });

    // Calculate completion percentage
    Effect::new({
        let form_data = form_data.clone();
        let set_completion_percentage = set_completion_percentage.clone();
        
        move |_| {
            let data = form_data.get();
            let mut completed_fields = 0;
            let total_fields = 15; // Adjust based on your requirements
            
            if !data.title.is_empty() { completed_fields += 1; }
            if !data.pharmacy_name.is_empty() { completed_fields += 1; }
            if !data.address.is_empty() { completed_fields += 1; }
            if !data.suburb.is_empty() { completed_fields += 1; }
            if !data.postcode.is_empty() { completed_fields += 1; }
            if data.state.is_some() { completed_fields += 1; }
            if data.job_type.is_some() { completed_fields += 1; }
            if !data.description.is_empty() { completed_fields += 1; }
            if data.hourly_rate.is_some() { completed_fields += 1; }
            if !data.start_date.is_empty() { completed_fields += 1; }
            if !data.end_date.is_empty() { completed_fields += 1; }
            if !data.start_time.is_empty() { completed_fields += 1; }
            if !data.end_time.is_empty() { completed_fields += 1; }
            if !data.contact_name.is_empty() { completed_fields += 1; }
            if !data.contact_email.is_empty() { completed_fields += 1; }
            
            let percentage = (completed_fields * 100) / total_fields;
            set_completion_percentage.set(percentage);
        }
    });

    let next_step = move |_| {
        let current = current_step.get();
        let data = form_data.get();
        
        // Validate current step
        let step_errors = data.validate_step(&current);
        if !step_errors.is_empty() {
            set_errors.set(step_errors);
            return;
        }
        
        set_errors.set(Vec::new());
        
        if let Some(next) = current.next() {
            set_current_step.set(next);
        }
    };

    let previous_step = move |_| {
        if let Some(prev) = current_step.get().previous() {
            set_current_step.set(prev);
            set_errors.set(Vec::new());
        }
    };

    let jump_to_step = move |step: WizardStep| {
        set_current_step.set(step);
        set_errors.set(Vec::new());
    };

    let submit_job = move |_| {
        let data = form_data.get();
        let create_request = data.to_create_request();
        
        set_is_submitting.set(true);
        
        spawn_local(async move {
            // Add 2 second delay for dramatic effect
            gloo_timers::future::TimeoutFuture::new(2000).await;
            
            match create_job(create_request).await {
                Ok(_) => {
                    set_success_message.set(Some("🎉 Job posted successfully! Your position is now live and attracting top pharmacy talent.".to_string()));
                    set_form_data.set(JobFormData::default());
                    set_current_step.set(WizardStep::BasicInfo);
                    
                    // Clear saved draft
                    if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                        let _ = storage.remove_item("job_wizard_draft");
                    }
                }
                Err(e) => {
                    set_errors.set(vec![format!("Failed to create job: {}", e)]);
                }
            }
            set_is_submitting.set(false);
        });
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-tiffany-light/20 via-white to-blue-50 relative overflow-hidden">
            // Animated Background Elements
            <div class="absolute inset-0 overflow-hidden pointer-events-none">
                <div class="absolute -top-40 -right-40 w-80 h-80 bg-gradient-to-br from-tiffany-blue/20 to-purple-400/20 rounded-full blur-3xl animate-pulse"></div>
                <div class="absolute -bottom-40 -left-40 w-80 h-80 bg-gradient-to-tr from-blue-400/20 to-tiffany-blue/20 rounded-full blur-3xl animate-pulse"></div>
                <div class="absolute top-1/2 left-1/2 w-60 h-60 bg-gradient-to-br from-tiffany-blue/10 to-transparent rounded-full blur-2xl animate-bounce"></div>
            </div>

            <div class="relative z-10 max-w-7xl mx-auto px-4 py-8">
                <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
                    // Ultra-Enhanced Sidebar Navigation
                    <div class="lg:col-span-1">
                        <div class="glass-tiffany rounded-2xl p-6 sticky top-8 space-y-6">
                            // Header with animated icon
                            <div class="text-center mb-8">
                                <div class="wizard-icon-container">
                                    <span class="text-4xl animate-bounce">"✨"</span>
                                </div>
                                <h2 class="text-xl font-bold text-gradient mt-4">Job Creation Wizard</h2>
                                <p class="text-sm text-gray-600 mt-2">Create the perfect job posting</p>
                            </div>

                            // Completion Circle
                            <div class="flex items-center justify-center mb-6">
                                <div class="relative w-24 h-24">
                                    <svg class="w-24 h-24 transform -rotate-90" viewBox="0 0 100 100">
                                        // Background circle
                                        <circle cx="50" cy="50" r="40" stroke="#e5e7eb" stroke-width="8" fill="none"/>
                                        // Progress circle
                                        <circle 
                                            cx="50" cy="50" r="40" 
                                            stroke="url(#gradient)" 
                                            stroke-width="8" 
                                            fill="none"
                                            stroke-linecap="round"
                                            stroke-dasharray="251.2"
                                            stroke-dashoffset=move || format!("{}", 251.2 - (251.2 * completion_percentage.get() / 100))
                                            class="transition-all duration-1000 ease-out"
                                        />
                                        <defs>
                                            <linearGradient id="gradient" x1="0%" y1="0%" x2="100%" y2="100%">
                                                <stop offset="0%" stop-color="#17ddb8"/>
                                                <stop offset="100%" stop-color="#3b82f6"/>
                                            </linearGradient>
                                        </defs>
                                    </svg>
                                    <div class="absolute inset-0 flex items-center justify-center">
                                        <span class="text-lg font-bold text-tiffany-dark">
                                            {move || completion_percentage.get()}"%"
                                        </span>
                                    </div>
                                </div>
                            </div>

                            // Step Navigation
                            <div class="space-y-3">
                                {[
                                    WizardStep::BasicInfo,
                                    WizardStep::JobDetails,
                                    WizardStep::Requirements,
                                    WizardStep::Compensation,
                                    WizardStep::WorkConditions,
                                    WizardStep::Application,
                                    WizardStep::Preview,
                                ].into_iter().map(|step| {
                                    let step_clone = step.clone();
                                    view! {
                                        <button
                                            class=move || format!(
                                                "wizard-step-button w-full text-left p-4 rounded-xl transition-all duration-300 {}",
                                                if current_step.get() == step_clone {
                                                    "active-step"
                                                } else if step_clone.step_number() < current_step.get().step_number() {
                                                    "completed-step"
                                                } else {
                                                    "inactive-step"
                                                }
                                            )
                                            on:click=move |_| jump_to_step(step_clone.clone())
                                        >
                                            <div class="flex items-center gap-3">
                                                <div class=move || format!(
                                                    "step-icon w-10 h-10 rounded-full flex items-center justify-center text-sm font-bold transition-all duration-300 {}",
                                                    if current_step.get() == step_clone {
                                                        "bg-gradient-to-r from-tiffany-blue to-blue-500 text-white"
                                                    } else if step_clone.step_number() < current_step.get().step_number() {
                                                        "bg-green-500 text-white"
                                                    } else {
                                                        "bg-gray-200 text-gray-600"
                                                    }
                                                )>
                                                    {move || if step_clone.step_number() < current_step.get().step_number() {
                                                        "✓".to_string()
                                                    } else {
                                                        step_clone.step_number().to_string()
                                                    }}
                                                </div>
                                                <div class="flex-1">
                                                    <div class="font-semibold text-sm">{step.title()}</div>
                                                    <div class="text-xs text-gray-500 truncate">{step.description()}</div>
                                                </div>
                                                <div class="text-lg">{step.icon()}</div>
                                            </div>
                                        </button>
                                    }
                                }).collect_view()}
                            </div>

                            // Auto-save indicator
                            <div class="auto-save-indicator mt-6 p-3 bg-gray-50 rounded-lg">
                                <div class="flex items-center gap-2">
                                    <div class=move || format!(
                                        "w-2 h-2 rounded-full {}",
                                        if auto_save_enabled.get() { "bg-green-500 animate-pulse" } else { "bg-gray-400" }
                                    )></div>
                                    <span class="text-xs text-gray-600">
                                        {move || if auto_save_enabled.get() {
                                            "Auto-save enabled"
                                        } else {
                                            "Auto-save disabled"
                                        }}
                                    </span>
                                </div>
                                {move || if let Some(saved_time) = last_saved.get() {
                                    view! {
                                        <div class="text-xs text-gray-500 mt-1">{saved_time}</div>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }}
                            </div>
                        </div>
                    </div>

                    // Ultra-Enhanced Main Content Area
                    <div class="lg:col-span-3">
                        <div class="glass bg-white/90 rounded-2xl overflow-hidden shadow-2xl">
                            // Enhanced Header with Animated Progress
                            <div class="relative overflow-hidden">
                                <div class="bg-gradient-to-r from-tiffany-blue via-blue-500 to-purple-600 px-8 py-6">
                                    // Floating particles background
                                    <div class="absolute inset-0 overflow-hidden">
                                        <div class="absolute w-20 h-20 bg-white/10 rounded-full -top-10 -left-10 animate-float"></div>
                                        <div class="absolute w-32 h-32 bg-white/5 rounded-full top-1/2 right-0 animate-float-delayed"></div>
                                        <div class="absolute w-16 h-16 bg-white/10 rounded-full bottom-0 left-1/3 animate-float"></div>
                                    </div>
                                    
                                    <div class="relative z-10">
                                        <div class="flex items-center justify-between text-white">
                                            <div>
                                                <h1 class="text-3xl font-bold flex items-center gap-3">
                                                    <span class="text-4xl animate-bounce">{move || current_step.get().icon()}</span>
                                                    {move || current_step.get().title()}
                                                </h1>
                                                <p class="text-blue-100 mt-2 text-lg">{move || current_step.get().description()}</p>
                                            </div>
                                            <div class="text-right">
                                                <div class="text-2xl font-bold">
                                                    {move || current_step.get().step_number()} "/" {WizardStep::total_steps()}
                                                </div>
                                                <div class="text-blue-200 text-sm">Steps Complete</div>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                // Enhanced animated progress bar
                                <div class="relative h-2 bg-gray-200">
                                    <div 
                                        class="absolute top-0 left-0 h-full bg-gradient-to-r from-tiffany-blue to-purple-500 transition-all duration-1000 ease-out shadow-lg"
                                        style:width=move || format!("{}%", (current_step.get().step_number() * 100) / WizardStep::total_steps())
                                    ></div>
                                    <div class="absolute top-0 left-0 h-full bg-gradient-to-r from-transparent via-white/20 to-transparent animate-shimmer"></div>
                                </div>
                            </div>

                            // Success message with celebration
                            {move || {
                                if let Some(msg) = success_message.get() {
                                    view! {
                                        <div class="px-8 py-6 bg-gradient-to-r from-green-50 to-emerald-50 border-l-4 border-green-500">
                                            <div class="flex items-center gap-4">
                                                <div class="text-4xl animate-bounce">"🎉"</div>
                                                <div>
                                                    <div class="text-lg font-semibold text-green-800">Success!</div>
                                                    <div class="text-green-700">{msg}</div>
                                                </div>
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }
                            }}

                            // Enhanced error messages
                            {move || {
                                let errors_list = errors.get();
                                if !errors_list.is_empty() {
                                    view! {
                                        <div class="px-8 py-6 bg-gradient-to-r from-red-50 to-rose-50 border-l-4 border-red-500">
                                            <div class="flex items-start gap-4">
                                                <div class="text-2xl">"⚠️"</div>
                                                <div>
                                                    <div class="text-lg font-semibold text-red-800 mb-2">Please fix these issues:</div>
                                                    <ul class="space-y-1">
                                                        {errors_list.into_iter()
                                                            .map(|error| view! { 
                                                                <li class="text-red-700 flex items-center gap-2">
                                                                    <span class="w-1 h-1 bg-red-500 rounded-full"></span>
                                                                    {error}
                                                                </li> 
                                                            })
                                                            .collect_view()
                                                        }
                                                    </ul>
                                                </div>
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }
                            }}

                            // Enhanced form content with animations
                            <div class="px-8 py-8">
                                <div class="form-step-content">
                                    {move || match current_step.get() {
                                        WizardStep::BasicInfo => view! {
                                            <BasicInfoStep form_data=form_data set_form_data=set_form_data />
                                        }.into_view(),
                                        WizardStep::JobDetails => view! {
                                            <JobDetailsStep form_data=form_data set_form_data=set_form_data />
                                        }.into_view(),
                                        WizardStep::Requirements => view! {
                                            <RequirementsStep form_data=form_data set_form_data=set_form_data />
                                        }.into_view(),
                                        WizardStep::Compensation => view! {
                                            <CompensationStep form_data=form_data set_form_data=set_form_data />
                                        }.into_view(),
                                        WizardStep::WorkConditions => view! {
                                            <WorkConditionsStep form_data=form_data set_form_data=set_form_data />
                                        }.into_view(),
                                        WizardStep::Application => view! {
                                            <ApplicationStep form_data=form_data set_form_data=set_form_data />
                                        }.into_view(),
                                        WizardStep::Preview => view! {
                                            <PreviewStep form_data=form_data />
                                        }.into_view(),
                                    }}
                                </div>
                            </div>

                            // Ultra-Enhanced Navigation Footer
                            <div class="px-8 py-6 bg-gradient-to-r from-gray-50 to-blue-50 border-t border-gray-200">
                                <div class="flex justify-between items-center">
                                    <div>
                                        {move || {
                                            if current_step.get().previous().is_some() {
                                                view! {
                                                    <button
                                                        class="wizard-nav-button previous-button"
                                                        on:click=previous_step
                                                    >
                                                        <span class="flex items-center gap-2">
                                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                                                            </svg>
                                                            "Previous"
                                                        </span>
                                                    </button>
                                                }.into_view()
                                            } else {
                                                view! { 
                                                    <div class="flex items-center gap-2 text-gray-400">
                                                        <span class="text-2xl">"🚀"</span>
                                                        <span class="text-sm">Ready to create something amazing?</span>
                                                    </div>
                                                }.into_view()
                                            }
                                        }}
                                    </div>

                                    <div class="flex items-center gap-4">
                                        // Progress indicators (dots)
                                        <div class="hidden md:flex items-center gap-2">
                                            {(1..=WizardStep::total_steps()).map(|step_num| {
                                                view! {
                                                    <div class=move || format!(
                                                        "w-2 h-2 rounded-full transition-all duration-300 {}",
                                                        if step_num <= current_step.get().step_number() {
                                                            "bg-tiffany-blue"
                                                        } else {
                                                            "bg-gray-300"
                                                        }
                                                    )></div>
                                                }
                                            }).collect_view()}
                                        </div>

                                        {move || {
                                            if let Some(_) = current_step.get().next() {
                                                view! {
                                                    <button
                                                        class="wizard-nav-button next-button"
                                                        on:click=next_step
                                                    >
                                                        <span class="flex items-center gap-2">
                                                            "Continue"
                                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                                            </svg>
                                                        </span>
                                                    </button>
                                                }.into_view()
                                            } else {
                                                view! {
                                                    <button
                                                        class="wizard-nav-button publish-button"
                                                        on:click=submit_job
                                                        disabled=is_submitting
                                                    >
                                                        {move || if is_submitting.get() {
                                                            view! {
                                                                <span class="flex items-center gap-3">
                                                                    <div class="publish-spinner"></div>
                                                                    <span>"Publishing Your Job..."</span>
                                                                    <span class="text-xl animate-bounce">"🚀"</span>
                                                                </span>
                                                            }.into_view()
                                                        } else {
                                                            view! { 
                                                                <span class="flex items-center gap-2">
                                                                    <span class="text-xl">"🎯"</span>
                                                                    "Publish Job"
                                                                </span>
                                                            }.into_view()
                                                        }}
                                                    </button>
                                                }.into_view()
                                            }
                                        }}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Enhanced CSS Framework
            <style>
                {ENHANCED_WIZARD_CSS}
            </style>
        </div>
    }
}

#[component]
fn BasicInfoStep(
    form_data: ReadSignal<JobFormData>,
    set_form_data: WriteSignal<JobFormData>,
) -> impl IntoView {
    let update_field = move |field: &'static str, value: String| {
        set_form_data.update(|data| {
            match field {
                "title" => data.title = value,
                "pharmacy_name" => data.pharmacy_name = value,
                "address" => data.address = value,
                "suburb" => data.suburb = value,
                "postcode" => data.postcode = value,
                _ => {}
            }
        });
    };

    let update_state = move |value: String| {
        let state = match value.as_str() {
            "NSW" => Some(AustralianState::NSW),
            "VIC" => Some(AustralianState::VIC),
            "QLD" => Some(AustralianState::QLD),
            "SA" => Some(AustralianState::SA),
            "WA" => Some(AustralianState::WA),
            "TAS" => Some(AustralianState::TAS),
            "NT" => Some(AustralianState::NT),
            "ACT" => Some(AustralianState::ACT),
            _ => None,
        };
        set_form_data.update(|data| data.state = state);
    };

    let update_job_type = move |value: String| {
        let job_type = match value.as_str() {
            "Pharmacist" => Some(JobType::Pharmacist),
            "PharmacyAssistant" => Some(JobType::PharmacyAssistant),
            "PharmacyTechnician" => Some(JobType::PharmacyTechnician),
            "PharmacyManager" => Some(JobType::PharmacyManager),
            "InternPharmacist" => Some(JobType::InternPharmacist),
            "ClinicalPharmacist" => Some(JobType::ClinicalPharmacist),
            _ => None,
        };
        set_form_data.update(|data| data.job_type = job_type);
    };

    view! {
        <div class="space-y-8">
            // Enhanced Header
            <div class="text-center mb-8">
                <h3 class="text-2xl font-bold text-gray-900 mb-2">{"Let's start with the basics"}</h3>
                <p class="text-gray-600">Tell us about your pharmacy and the location for this position</p>
            </div>

            // Enhanced Form Fields
            <div class="space-y-6">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="form-group">
                        <label class="block text-lg font-semibold text-gray-800 mb-3">
                            "Position Title *"
                        </label>
                        <input
                            class="enhanced-input"
                            value=move || form_data.get().title
                            on:input=move |ev| update_field("title", event_target_value(&ev))
                            placeholder="e.g. Senior Pharmacist - Weekend Relief"
                        />
                        <p class="text-sm text-gray-500 mt-2">Make it specific and attractive to candidates</p>
                    </div>

                    <div class="form-group">
                        <label class="block text-lg font-semibold text-gray-800 mb-3">
                            "Pharmacy Name *"
                        </label>
                        <input
                            class="enhanced-input"
                            value=move || form_data.get().pharmacy_name
                            on:input=move |ev| update_field("pharmacy_name", event_target_value(&ev))
                            placeholder="e.g. Priceline Pharmacy Bondi"
                        />
                        <p class="text-sm text-gray-500 mt-2">Include your brand and location if helpful</p>
                    </div>
                </div>

                <div class="form-group">
                    <label class="block text-lg font-semibold text-gray-800 mb-3">
                        "Complete Address *"
                    </label>
                    <input
                        class="enhanced-input"
                        value=move || form_data.get().address
                        on:input=move |ev| update_field("address", event_target_value(&ev))
                        placeholder="e.g. 123 Collins Street, Melbourne VIC 3000"
                    />
                    <p class="text-sm text-gray-500 mt-2">This helps candidates understand commute and location</p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <div class="form-group">
                        <label class="block text-lg font-semibold text-gray-800 mb-3">
                            "Suburb *"
                        </label>
                        <input
                            class="enhanced-input"
                            value=move || form_data.get().suburb
                            on:input=move |ev| update_field("suburb", event_target_value(&ev))
                            placeholder="e.g. Melbourne"
                        />
                    </div>

                    <div class="form-group">
                        <label class="block text-lg font-semibold text-gray-800 mb-3">
                            "Postcode *"
                        </label>
                        <input
                            class="enhanced-input"
                            value=move || form_data.get().postcode
                            on:input=move |ev| update_field("postcode", event_target_value(&ev))
                            placeholder="e.g. 3000"
                        />
                    </div>

                    <div class="form-group">
                        <label class="block text-lg font-semibold text-gray-800 mb-3">
                            "State *"
                        </label>
                        <select
                            class="enhanced-select"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                update_state(value);
                            }
                        >
                            <option value="">"Select State"</option>
                            <option value="NSW">"NSW"</option>
                            <option value="VIC">"VIC"</option>
                            <option value="QLD">"QLD"</option>
                            <option value="SA">"SA"</option>
                            <option value="WA">"WA"</option>
                            <option value="TAS">"TAS"</option>
                            <option value="NT">"NT"</option>
                            <option value="ACT">"ACT"</option>
                        </select>
                    </div>
                </div>

                // Enhanced Job Type Selection
                <div class="form-group">
                    <label class="block text-lg font-semibold text-gray-800 mb-3">
                        "Position Type *"
                    </label>
                    <div class="grid grid-cols-2 md:grid-cols-3 gap-4">
                        {[
                            ("Pharmacist", "👨‍⚕️ Pharmacist"),
                            ("PharmacyAssistant", "🏥 Pharmacy Assistant"),
                            ("PharmacyTechnician", "🔬 Pharmacy Technician"),
                            ("PharmacyManager", "👔 Pharmacy Manager"),
                            ("InternPharmacist", "🎓 Intern Pharmacist"),
                            ("ClinicalPharmacist", "⚕️ Clinical Pharmacist"),
                        ].into_iter().map(|(value, label)| {
                            let value_clone = value.to_string();
                            view! {
                                <button
                                    type="button"
                                    class=move || format!(
                                        "job-type-card p-4 rounded-xl border-2 transition-all duration-300 text-left {}",
                                        if form_data.get().job_type.is_some() && 
                                           format!("{:?}", form_data.get().job_type.unwrap()) == value {
                                            "border-tiffany-blue bg-tiffany-blue/10 text-tiffany-dark"
                                        } else {
                                            "border-gray-200 hover:border-tiffany-blue/50 hover:bg-gray-50"
                                        }
                                    )
                                    on:click=move |_| update_job_type(value_clone.clone())
                                >
                                    <div class="font-semibold">{label}</div>
                                </button>
                            }
                        }).collect_view()}
                    </div>
                    <p class="text-sm text-gray-500 mt-3">Choose the primary role for this position</p>
                </div>

                // Helpful Tips Card
                <div class="bg-gradient-to-r from-blue-50 to-tiffany-blue/10 border border-blue-200 rounded-xl p-6">
                    <div class="flex items-start gap-4">
                        <div class="text-2xl">"💡"</div>
                        <div>
                            <h4 class="font-semibold text-blue-900 mb-2">Pro Tips for Better Applications</h4>
                            <ul class="text-sm text-blue-800 space-y-1">
                                <li>"• Use specific job titles (e.g., 'Senior Pharmacist' vs 'Pharmacist')"</li>
                                <li>"• Include pharmacy brand if it's well-known"</li>
                                <li>"• Mention if it's a busy/quiet location"</li>
                                <li>"• Add nearby landmarks for easy recognition"</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn JobDetailsStep(
    form_data: ReadSignal<JobFormData>,
    set_form_data: WriteSignal<JobFormData>,
) -> impl IntoView {
    view! {
        <div class="space-y-8">
            // Enhanced Header
            <div class="text-center mb-8">
                <h3 class="text-2xl font-bold text-gray-900 mb-2">Define the role details</h3>
                <p class="text-gray-600">Help candidates understand exactly what this position involves</p>
            </div>

            <div class="space-y-6">
                // Employment Type
                <div class="form-group">
                    <label class="block text-lg font-semibold text-gray-800 mb-3">
                        "Employment Type"
                    </label>
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                        {[
                            ("Full-time", "🕘 Full-time"),
                            ("Part-time", "⏰ Part-time"),
                            ("Casual", "🔄 Casual"),
                            ("Locum", "🚀 Locum"),
                        ].into_iter().map(|(value, label)| {
                            view! {
                                <button
                                    type="button"
                                    class="employment-type-card p-4 rounded-xl border-2 transition-all duration-300 text-center hover:border-tiffany-blue/50 hover:bg-gray-50"
                                >
                                    <div class="font-semibold">{label}</div>
                                </button>
                            }
                        }).collect_view()}
                    </div>
                </div>

                // Urgency Level
                <div class="form-group">
                    <label class="block text-lg font-semibold text-gray-800 mb-3">
                        "Position Urgency"
                    </label>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div class="urgency-card p-4 rounded-xl border-2 border-gray-200 hover:border-green-400 transition-all duration-300">
                            <div class="flex items-center gap-3">
                                <input type="radio" name="urgency" class="enhanced-checkbox" />
                                <div>
                                    <div class="font-semibold text-green-700">"🟢 Standard"</div>
                                    <div class="text-sm text-gray-600">Normal hiring timeline</div>
                                </div>
                            </div>
                        </div>
                        <div class="urgency-card p-4 rounded-xl border-2 border-gray-200 hover:border-yellow-400 transition-all duration-300">
                            <div class="flex items-center gap-3">
                                <input type="radio" name="urgency" class="enhanced-checkbox" />
                                <div>
                                    <div class="font-semibold text-yellow-700">"🟡 Priority"</div>
                                    <div class="text-sm text-gray-600">Preferred start within 2 weeks</div>
                                </div>
                            </div>
                        </div>
                        <div class="urgency-card p-4 rounded-xl border-2 border-gray-200 hover:border-red-400 transition-all duration-300">
                            <div class="flex items-center gap-3">
                                <input 
                                    type="radio" 
                                    name="urgency" 
                                    class="enhanced-checkbox"
                                    checked=move || form_data.get().is_urgent
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        set_form_data.update(|data| data.is_urgent = checked);
                                    }
                                />
                                <div>
                                    <div class="font-semibold text-red-700">"🔴 Urgent"</div>
                                    <div class="text-sm text-gray-600">Immediate start required</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                // Job Description Preview
                <div class="form-group">
                    <label class="block text-lg font-semibold text-gray-800 mb-3">
                        "Brief Role Description"
                    </label>
                    <textarea
                        class="enhanced-textarea"
                        placeholder="Provide a brief overview of the role and key responsibilities..."
                        prop:value=move || form_data.get().description
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_form_data.update(|data| data.description = value);
                        }
                    ></textarea>
                    <p class="text-sm text-gray-500 mt-2">You can expand on this in later steps</p>
                </div>

                // Helpful Tips Card
                <div class="bg-gradient-to-r from-purple-50 to-blue-50 border border-purple-200 rounded-xl p-6">
                    <div class="flex items-start gap-4">
                        <div class="text-2xl">"🎯"</div>
                        <div>
                            <h4 class="font-semibold text-purple-900 mb-2">Role Definition Tips</h4>
                            <ul class="text-sm text-purple-800 space-y-1">
                                <li>"• Be honest about urgency - it affects candidate expectations"</li>
                                <li>"• Casual roles attract different candidates than full-time"</li>
                                <li>"• Locum positions often command higher rates"</li>
                                <li>"• Clear descriptions get better quality applications"</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn WorkConditionsStep(
    form_data: ReadSignal<JobFormData>,
    set_form_data: WriteSignal<JobFormData>,
) -> impl IntoView {
    view! {
        <div class="space-y-8">
            // Enhanced Header
            <div class="text-center mb-8">
                <h3 class="text-2xl font-bold text-gray-900 mb-2">Work schedule and conditions</h3>
                <p class="text-gray-600">Define when and how the work will be performed</p>
            </div>

            <div class="space-y-6">
                // Date Range
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="form-group">
                        <label class="block text-lg font-semibold text-gray-800 mb-3">
                            "Start Date *"
                        </label>
                        <input
                            type="date"
                            class="enhanced-input"
                            value=move || form_data.get().start_date
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_form_data.update(|data| data.start_date = value);
                            }
                        />
                    </div>

                    <div class="form-group">
                        <label class="block text-lg font-semibold text-gray-800 mb-3">
                            "End Date *"
                        </label>
                        <input
                            type="date"
                            class="enhanced-input"
                            value=move || form_data.get().end_date
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_form_data.update(|data| data.end_date = value);
                            }
                        />
                    </div>
                </div>

                // Time Range
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="form-group">
                        <label class="block text-lg font-semibold text-gray-800 mb-3">
                            "Start Time *"
                        </label>
                        <input
                            type="time"
                            class="enhanced-input"
                            value=move || form_data.get().start_time
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_form_data.update(|data| data.start_time = value);
                            }
                        />
                    </div>

                    <div class="form-group">
                        <label class="block text-lg font-semibold text-gray-800 mb-3">
                            "End Time *"
                        </label>
                        <input
                            type="time"
                            class="enhanced-input"
                            value=move || form_data.get().end_time
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_form_data.update(|data| data.end_time = value);
                            }
                        />
                    </div>
                </div>

                // Flexibility Options
                <div class="form-group">
                    <label class="block text-lg font-semibold text-gray-800 mb-3">
                        "Schedule Flexibility"
                    </label>
                    <div class="space-y-3">
                        <label class="flex items-center gap-3 p-4 border border-gray-200 rounded-xl hover:bg-gray-50 transition-all duration-300">
                            <input 
                                type="checkbox" 
                                class="enhanced-checkbox"
                                checked=move || form_data.get().flexible_hours
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_form_data.update(|data| data.flexible_hours = checked);
                                }
                            />
                            <div>
                                <div class="font-semibold text-gray-900">Flexible Hours Available</div>
                                <div class="text-sm text-gray-600">Can accommodate different start/end times</div>
                            </div>
                        </label>
                    </div>
                </div>

                // Special Instructions
                <div class="form-group">
                    <label class="block text-lg font-semibold text-gray-800 mb-3">
                        "Special Instructions or Notes"
                    </label>
                    <textarea
                        class="enhanced-textarea"
                        placeholder="Any special working conditions, equipment, or requirements..."
                        prop:value=move || form_data.get().special_instructions
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_form_data.update(|data| data.special_instructions = value);
                        }
                    ></textarea>
                </div>

                // Working Conditions Tips
                <div class="bg-gradient-to-r from-green-50 to-teal-50 border border-green-200 rounded-xl p-6">
                    <div class="flex items-start gap-4">
                        <div class="text-2xl">"⏰"</div>
                        <div>
                            <h4 class="font-semibold text-green-900 mb-2">Schedule Best Practices</h4>
                            <ul class="text-sm text-green-800 space-y-1">
                                <li>"• Be realistic about start and end dates"</li>
                                <li>"• Mention if weekend or evening work is required"</li>
                                <li>"• Flexibility attracts more candidates"</li>
                                <li>"• Include any break times or special requirements"</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ApplicationStep(
    form_data: ReadSignal<JobFormData>,
    set_form_data: WriteSignal<JobFormData>,
) -> impl IntoView {
    view! {
        <div class="space-y-8">
            // Enhanced Header
            <div class="text-center mb-8">
                <h3 class="text-2xl font-bold text-gray-900 mb-2">Application process</h3>
                <p class="text-gray-600">How candidates can apply and contact you</p>
            </div>

            <div class="space-y-6">
                // Contact Information
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="form-group">
                        <label class="block text-lg font-semibold text-gray-800 mb-3">
                            "Contact Person *"
                        </label>
                        <input
                            class="enhanced-input"
                            value=move || form_data.get().contact_name
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_form_data.update(|data| data.contact_name = value);
                            }
                            placeholder="e.g. Sarah Johnson, Pharmacy Manager"
                        />
                    </div>

                    <div class="form-group">
                        <label class="block text-lg font-semibold text-gray-800 mb-3">
                            "Contact Email *"
                        </label>
                        <input
                            type="email"
                            class="enhanced-input"
                            value=move || form_data.get().contact_email
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_form_data.update(|data| data.contact_email = value);
                            }
                            placeholder="e.g. careers@pharmacy.com.au"
                        />
                    </div>
                </div>

                <div class="form-group">
                    <label class="block text-lg font-semibold text-gray-800 mb-3">
                        "Contact Phone (Optional)"
                    </label>
                    <input
                        type="tel"
                        class="enhanced-input"
                        value=move || form_data.get().contact_phone
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_form_data.update(|data| data.contact_phone = value);
                        }
                        placeholder="e.g. (02) 1234 5678"
                    />
                </div>

                // Application Requirements
                <div class="form-group">
                    <label class="block text-lg font-semibold text-gray-800 mb-3">
                        "Application Requirements"
                    </label>
                    <div class="space-y-3">
                        <label class="flex items-center gap-3 p-4 border border-gray-200 rounded-xl hover:bg-gray-50 transition-all duration-300">
                            <input 
                                type="checkbox" 
                                class="enhanced-checkbox"
                                checked=move || form_data.get().requires_references
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_form_data.update(|data| data.requires_references = checked);
                                }
                            />
                            <div>
                                <div class="font-semibold text-gray-900">References Required</div>
                                <div class="text-sm text-gray-600">Candidates must provide professional references</div>
                            </div>
                        </label>
                    </div>
                </div>

                // Application Preview
                <div class="preview-card">
                    <h4 class="font-semibold text-gray-900 mb-4">Application Preview</h4>
                    <div class="space-y-3 text-sm text-gray-700">
                        <div>
                            <strong>How to Apply:</strong>
                            <p>Interested candidates can apply by sending their CV and cover letter to {move || form_data.get().contact_email}</p>
                        </div>
                        {move || if !form_data.get().contact_name.is_empty() {
                            view! {
                                <div>
                                    <strong>Contact Person:</strong> {form_data.get().contact_name}
                                </div>
                            }.into_view()
                        } else {
                            view! { <div></div> }.into_view()
                        }}
                        {move || if form_data.get().requires_references {
                            view! {
                                <div>
                                    <strong>Requirements:</strong> Professional references required
                                </div>
                            }.into_view()
                        } else {
                            view! { <div></div> }.into_view()
                        }}
                    </div>
                </div>

                // Application Tips
                <div class="bg-gradient-to-r from-indigo-50 to-purple-50 border border-indigo-200 rounded-xl p-6">
                    <div class="flex items-start gap-4">
                        <div class="text-2xl">"📧"</div>
                        <div>
                            <h4 class="font-semibold text-indigo-900 mb-2">Application Best Practices</h4>
                            <ul class="text-sm text-indigo-800 space-y-1">
                                <li>"• Use a professional email address for applications"</li>
                                <li>"• Respond to candidates within 48 hours"</li>
                                <li>"• Be clear about what documents are required"</li>
                                <li>"• Consider phone interviews for out-of-town candidates"</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn RequirementsStep(
    form_data: ReadSignal<JobFormData>,
    set_form_data: WriteSignal<JobFormData>,
) -> impl IntoView {
    let (new_requirement, set_new_requirement) = create_signal(String::new());
    let (new_certification, set_new_certification) = create_signal(String::new());

    let add_requirement = move |_| {
        let req = new_requirement.get().trim().to_string();
        if !req.is_empty() {
            set_form_data.update(|data| data.requirements.push(req));
            set_new_requirement.set(String::new());
        }
    };

    let remove_requirement = move |index: usize| {
        set_form_data.update(|data| data.requirements.remove(index));
    };

    let add_certification = move |_| {
        let cert = new_certification.get().trim().to_string();
        if !cert.is_empty() {
            set_form_data.update(|data| data.certifications.push(cert));
            set_new_certification.set(String::new());
        }
    };

    let remove_certification = move |index: usize| {
        set_form_data.update(|data| data.certifications.remove(index));
    };

    view! {
        <div class="space-y-6">
            <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">
                    "Experience Required (years)"
                </label>
                <input
                    type="number"
                    min="0"
                    max="50"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    value=move || form_data.get().experience_years.map(|x| x.to_string()).unwrap_or_default()
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        let years = value.parse::<u32>().ok();
                        set_form_data.update(|data| data.experience_years = years);
                    }
                />
            </div>

            <div>
                <label class="block text-sm font-medium text-gray-700 mb-4">
                    "Requirements"
                </label>
                <div class="flex gap-2 mb-3">
                    <Input
                        value=new_requirement
                        on_input=set_new_requirement
                        placeholder="Add a job requirement..."
                    />
                    <Button variant=ButtonVariant::Secondary on_click=add_requirement>
                        "Add"
                    </Button>
                </div>
                <div class="space-y-2">
                    {move || {
                        form_data.get().requirements
                            .into_iter()
                            .enumerate()
                            .map(|(index, req)| view! {
                                <div class="flex items-center justify-between p-2 bg-gray-50 rounded">
                                    <span>{req}</span>
                                    <button
                                        class="text-red-600 hover:text-red-800 text-sm"
                                        on:click=move |_| remove_requirement(index)
                                    >
                                        "Remove"
                                    </button>
                                </div>
                            })
                            .collect_view()
                    }}
                </div>
            </div>

            <div>
                <label class="block text-sm font-medium text-gray-700 mb-4">
                    "Certifications"
                </label>
                <div class="flex gap-2 mb-3">
                    <Input
                        value=new_certification
                        on_input=set_new_certification
                        placeholder="Add required certification..."
                    />
                    <Button variant=ButtonVariant::Secondary on_click=add_certification>
                        "Add"
                    </Button>
                </div>
                <div class="space-y-2">
                    {move || {
                        form_data.get().certifications
                            .into_iter()
                            .enumerate()
                            .map(|(index, cert)| view! {
                                <div class="flex items-center justify-between p-2 bg-gray-50 rounded">
                                    <span>{cert}</span>
                                    <button
                                        class="text-red-600 hover:text-red-800 text-sm"
                                        on:click=move |_| remove_certification(index)
                                    >
                                        "Remove"
                                    </button>
                                </div>
                            })
                            .collect_view()
                    }}
                </div>
            </div>

            <div class="space-y-4">
                <label class="flex items-center">
                    <input
                        type="checkbox"
                        class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                        checked=move || form_data.get().must_have_car
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_form_data.update(|data| data.must_have_car = checked);
                        }
                    />
                    <span class="text-gray-700">"Must have own vehicle"</span>
                </label>

                <label class="flex items-center">
                    <input
                        type="checkbox"
                        class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                        checked=move || form_data.get().must_speak_english
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_form_data.update(|data| data.must_speak_english = checked);
                        }
                    />
                    <span class="text-gray-700">"Must speak fluent English"</span>
                </label>
            </div>
        </div>
    }
}

// We'll continue with CompensationStep, ScheduleStep, DescriptionStep, and PreviewStep components...
// For now, let's create simplified versions to get the wizard working

#[component]
fn CompensationStep(
    form_data: ReadSignal<JobFormData>,
    set_form_data: WriteSignal<JobFormData>,
) -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                        "Hourly Rate ($AUD)"
                    </label>
                    <input
                        type="number"
                        step="0.01"
                        min="0"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value=move || form_data.get().hourly_rate.map(|x| x.to_string()).unwrap_or_default()
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            let rate = value.parse::<f64>().ok();
                            set_form_data.update(|data| data.hourly_rate = rate);
                        }
                    />
                </div>
            </div>

            <label class="flex items-center">
                <input
                    type="checkbox"
                    class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=move || form_data.get().penalty_rates
                    on:change=move |ev| {
                        let checked = event_target_checked(&ev);
                        set_form_data.update(|data| data.penalty_rates = checked);
                    }
                />
                <span class="text-gray-700">"Penalty rates apply"</span>
            </label>
        </div>
    }
}

#[component]
fn ScheduleStep(
    form_data: ReadSignal<JobFormData>,
    set_form_data: WriteSignal<JobFormData>,
) -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                        "Start Date *"
                    </label>
                    <input
                        type="date"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value=move || form_data.get().start_date
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_form_data.update(|data| data.start_date = value);
                        }
                    />
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                        "End Date *"
                    </label>
                    <input
                        type="date"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value=move || form_data.get().end_date
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_form_data.update(|data| data.end_date = value);
                        }
                    />
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                        "Start Time *"
                    </label>
                    <input
                        type="time"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value=move || form_data.get().start_time
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_form_data.update(|data| data.start_time = value);
                        }
                    />
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                        "End Time *"
                    </label>
                    <input
                        type="time"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value=move || form_data.get().end_time
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_form_data.update(|data| data.end_time = value);
                        }
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn DescriptionStep(
    form_data: ReadSignal<JobFormData>,
    set_form_data: WriteSignal<JobFormData>,
) -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">
                    "Job Description *"
                </label>
                <textarea
                    rows="6"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    placeholder="Describe the role, responsibilities, and what makes this position attractive..."
                    prop:value=move || form_data.get().description
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        set_form_data.update(|data| data.description = value);
                    }
                ></textarea>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                        "Contact Name *"
                    </label>
                    <Input
                        value=move || form_data.get().contact_name
                        on_input=move |value| {
                            set_form_data.update(|data| data.contact_name = value);
                        }
                        placeholder="Contact person name"
                    />
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                        "Contact Email *"
                    </label>
                    <Input
                        value=move || form_data.get().contact_email
                        on_input=move |value| {
                            set_form_data.update(|data| data.contact_email = value);
                        }
                        placeholder="contact@pharmacy.com.au"
                    />
                </div>
            </div>

            <label class="flex items-center">
                <input
                    type="checkbox"
                    class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=move || form_data.get().is_urgent
                    on:change=move |ev| {
                        let checked = event_target_checked(&ev);
                        set_form_data.update(|data| data.is_urgent = checked);
                    }
                />
                <span class="text-gray-700">"This is an urgent position"</span>
            </label>
        </div>
    }
}

#[component]
fn PreviewStep(form_data: ReadSignal<JobFormData>) -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div class="bg-gray-50 rounded-lg p-6">
                <h3 class="text-lg font-semibold mb-4">"Job Preview"</h3>
                
                <div class="space-y-4">
                    <div>
                        <h4 class="font-medium text-gray-900">{move || form_data.get().title}</h4>
                        <p class="text-gray-600">{move || form_data.get().pharmacy_name}</p>
                        <p class="text-gray-600">{move || format!("{}, {} {}", 
                            form_data.get().suburb, 
                            form_data.get().state.map(|s| format!("{:?}", s)).unwrap_or_default(),
                            form_data.get().postcode
                        )}</p>
                    </div>

                    {move || {
                        if let Some(rate) = form_data.get().hourly_rate {
                            view! {
                                <div>
                                    <span class="text-green-600 font-semibold">
                                        {format!("${:.2}/hour", rate)}
                                    </span>
                                </div>
                            }.into_view()
                        } else {
                            view! { <div></div> }.into_view()
                        }
                    }}

                    <div>
                        <h5 class="font-medium mb-2">"Schedule"</h5>
                        <p class="text-gray-600">
                            {move || format!("{} to {} | {} - {}", 
                                form_data.get().start_date,
                                form_data.get().end_date,
                                form_data.get().start_time,
                                form_data.get().end_time
                            )}
                        </p>
                    </div>

                    <div>
                        <h5 class="font-medium mb-2">"Description"</h5>
                        <p class="text-gray-600">{move || form_data.get().description}</p>
                    </div>

                    <div>
                        <h5 class="font-medium mb-2">"Contact"</h5>
                        <p class="text-gray-600">
                            {move || format!("{} - {}", 
                                form_data.get().contact_name,
                                form_data.get().contact_email
                            )}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}