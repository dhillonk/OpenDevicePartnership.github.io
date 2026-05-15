use crate::components::documentation_training::DocumentationTraining;
use crate::components::project_introduction::ProjectIntroduction;
use crate::components::repo_view::RepositoryGraph;
use crate::data::projects::PATINA;

use leptos::prelude::*;

/// Patina (boot firmware) project page.
#[component]
pub fn BootFirmware() -> impl IntoView {
    view! {
        <ProjectIntroduction
            project_title=PATINA.title
            project_summary=PATINA.summary
            project_what=PATINA.what
            project_why=PATINA.why
            project_who=PATINA.team_route
            big_image_url=PATINA.big_image_url
            small_image_url=PATINA.small_image_url
        />
        <RepositoryGraph nodes=PATINA.nodes_json links=PATINA.links_json />
        <DocumentationTraining />
    }
}
