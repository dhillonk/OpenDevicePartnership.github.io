//! `<LabeledSection>` -- a small caps "MONO" label followed by body
//! content. Used to build the WHAT / WHY / WHO panels on the
//! projects page and on every project introduction.

use crate::components::ui::Mono;
use leptos::prelude::*;

/// Mono label + arbitrary body content.
#[component]
pub fn LabeledSection(label: &'static str, children: Children) -> impl IntoView {
    view! {
        <Mono class="text-left">{label}</Mono>
        {children()}
    }
}
