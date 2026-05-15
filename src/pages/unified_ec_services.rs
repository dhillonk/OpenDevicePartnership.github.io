use crate::components::documentation_training::DocumentationTraining;
use crate::components::page_layout::PageLayout;
use crate::components::project_introduction::ProjectIntroduction;
use crate::components::repo_view::RepositoryGraph;
use crate::data::projects::EC_SERVICES;

use leptos::prelude::*;

/// Unified EC Services project page.
#[component]
pub fn WindowsEcServices() -> impl IntoView {
    view! {
        <PageLayout scrollable_x=true>
            <ProjectIntroduction
                project_title=EC_SERVICES.title
                project_summary=EC_SERVICES.summary
                project_what=EC_SERVICES.what
                project_why=EC_SERVICES.why
                project_who=EC_SERVICES.team_route
                big_image_url=EC_SERVICES.big_image_url
                small_image_url=EC_SERVICES.small_image_url
            />
            <RepositoryGraph nodes=EC_SERVICES.nodes_json links=EC_SERVICES.links_json />
            <DocumentationTraining />
        </PageLayout>
    }
}
