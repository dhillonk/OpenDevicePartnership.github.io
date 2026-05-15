//! Per-section components for the landing page (`/`).
//!
//! [`crate::components::landing_page::LandingPage`] composes these
//! sections in order. Each lives in its own file so the landing
//! shell stays a flat, readable list of `<...Section />` calls
//! instead of a 100-line view block.

mod closing_columns_section;
mod hero_section;
mod projects_section;
mod value_proposition_section;

pub use closing_columns_section::ClosingColumnsSection;
pub use hero_section::HeroSection;
pub use projects_section::ProjectsSection;
pub use value_proposition_section::ValuePropositionSection;
