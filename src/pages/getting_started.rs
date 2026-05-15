use crate::components::landing_page::LandingPage;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn GettingStarted() -> impl IntoView {
    view! { <LandingPage /> }
}
