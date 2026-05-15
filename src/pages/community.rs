use crate::components::community_teams::CommunityTeams;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Community() -> impl IntoView {
    view! { <CommunityTeams /> }
}
