use crate::components::landing_page::LandingPage;
use crate::components::page_layout::PageLayout;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn GettingStarted() -> impl IntoView {
    view! {
        <PageLayout scrollable_x=true>
            <LandingPage />
        </PageLayout>
    }
}
