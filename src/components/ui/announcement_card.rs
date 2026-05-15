//! `<AnnouncementCard>` -- the styled detail panel that renders one
//! selected announcement on the announcements page.
//!
//! Bakes in the responsive padding, the title typography, and the
//! body wrapper. Callers pass a `title` string and arbitrary children
//! (the rendered press-release content).

use leptos::prelude::*;

/// One announcement detail card.
#[component]
pub fn AnnouncementCard(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <div class="p_mobile md:p">
            <div class="h2 pb-4 md:pb-6">{title}</div>
            <div class="p2">{children()}</div>
        </div>
    }
}
