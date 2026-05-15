use crate::components::documentation_training::DocumentationTraining;
use crate::components::project_introduction::ProjectIntroduction;
use crate::components::repo_view::RepositoryGraph;
use crate::components::ui::{ProjectId, ProjectTabs};
use crate::data::projects::EMBEDDED_CONTROLLER;

use leptos::prelude::*;

/// Secure Embedded Controller project page.
#[component]
pub fn EmbeddedController() -> impl IntoView {
    view! {
        <ProjectIntroduction
            project_title=EMBEDDED_CONTROLLER.title
            project_summary=EMBEDDED_CONTROLLER.summary
            project_what=EMBEDDED_CONTROLLER.what
            project_why=EMBEDDED_CONTROLLER.why
            project_who=EMBEDDED_CONTROLLER.team_route
            big_image_url=EMBEDDED_CONTROLLER.big_image_url
            small_image_url=EMBEDDED_CONTROLLER.small_image_url
        />
        <ProjectTabs active=ProjectId::EmbeddedController />
        <RepositoryGraph
            nodes=EMBEDDED_CONTROLLER.nodes_json
            links=EMBEDDED_CONTROLLER.links_json
        />
        <DocumentationTraining />
    }
}
