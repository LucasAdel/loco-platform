pub mod map;
pub mod dashboard;
pub mod jobs;
pub mod profile;
pub mod admin;
pub mod home;
pub mod not_found;
pub mod applications;
pub mod availability;
pub mod create_job;
pub mod job_detail;
pub mod map_enhanced;
pub mod settings;
pub mod team;

// Simple versions for testing
pub mod simple_dashboard;
pub mod simple_jobs;
pub mod simple_map;
pub mod simple_profile;
pub mod simple_admin;

// Export the components with the correct names
pub use dashboard::Dashboard;
pub use jobs::Jobs;
pub use map::Map;
pub use profile::Profile;
pub use admin::Admin;
pub use home::Home;
pub use not_found::NotFound;
pub use applications::Applications;
pub use availability::Availability;
pub use create_job::CreateJob;
pub use job_detail::JobDetail;
pub use map_enhanced::MapEnhanced;
pub use settings::Settings;
pub use team::Team;