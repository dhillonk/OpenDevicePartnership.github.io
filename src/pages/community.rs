use crate::components::community_teams::CommunityTeams;
use crate::components::page_layout::PageLayout;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Community() -> impl IntoView {
    view! {
        <PageLayout>
            <CommunityTeams />
        </PageLayout>
    }
}
