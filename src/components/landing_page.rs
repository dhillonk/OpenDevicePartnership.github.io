//! Landing page (`/`) shell.
//!
//! The page is a flat sequence of section components. Each section
//! lives in its own file under [`crate::components::landing`].

use crate::components::landing::{ClosingColumnsSection, HeroSection, ProjectsSection, ValuePropositionSection};
use leptos::prelude::*;

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <HeroSection />
        <ValuePropositionSection />
        <ProjectsSection />
        <ClosingColumnsSection />
    }
}
