use crate::components::documentation_training::DocumentationTraining;
use crate::components::main::Main;
use crate::components::page_layout::PageLayout;
use crate::components::partners_grid::PartnersGrid;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <PageLayout>
            <Main />
            <PartnersGrid />
            <DocumentationTraining />
        </PageLayout>
    }
}
