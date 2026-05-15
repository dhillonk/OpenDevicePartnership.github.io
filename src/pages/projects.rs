use crate::components::documentation_training::DocumentationTraining;
use crate::components::projects_component::ProjectsComponent;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <ProjectsComponent />
        <DocumentationTraining />
    }
}
