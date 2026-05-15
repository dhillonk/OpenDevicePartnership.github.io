use crate::components::documentation_training::DocumentationTraining;
use crate::components::page_layout::PageLayout;
use crate::components::projects_component::ProjectsComponent;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <PageLayout scrollable_x=true>
            <ProjectsComponent />
            <DocumentationTraining />
        </PageLayout>
    }
}
