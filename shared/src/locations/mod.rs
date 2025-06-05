pub mod suburb_coordinates;
pub mod job_location_fixer;

pub use suburb_coordinates::{
    get_suburb_coordinates,
    is_valid_adelaide_location,
    fix_swapped_coordinates,
    get_location_with_fallback,
    ADELAIDE_SUBURBS,
};

pub use job_location_fixer::{
    fix_job_location,
    fix_job_locations,
    fix_job_locations_with_stats,
    job_location_needs_fixing,
    describe_location_fix,
    LocationFixStats,
};