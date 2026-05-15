//! `<TwoColumnIntro>` -- the recurring "stack-on-mobile,
//! side-by-side-on-md+" intro layout used by both
//! [`crate::components::projects_component::ProjectsComponent`] and
//! [`crate::components::project_introduction::ProjectIntroduction`].
//!
//! Provides the outer [`Section`] shell + the
//! `flex flex-col md:flex-row` row. Callers pass two slot children
//! (`left` and `right`); each slot becomes a column.

use crate::components::section::Section;
use leptos::prelude::*;

/// Two-column intro layout.
#[component]
pub fn TwoColumnIntro(
    /// Left column content. Accepts any `FnOnce() -> impl IntoView`.
    #[prop(into)]
    left: ViewFnOnce,
    /// Right column content. Accepts any `FnOnce() -> impl IntoView`.
    #[prop(into)]
    right: ViewFnOnce,
    /// Whether [`Section`] applies its default page padding.
    #[prop(default = true)]
    padded: bool,
    /// Extra Tailwind classes appended to the section.
    #[prop(into, default = String::new())]
    section_class: String,
    /// Tailwind class string for the inner row's `gap`. Defaults to
    /// the `gap-10 md:gap-20` pairing used by the projects page.
    #[prop(default = "gap-10 md:gap-20")]
    gap: &'static str,
) -> impl IntoView {
    let row_class = format!("flex flex-col md:flex-row {gap}");
    view! {
        <Section padded=padded class=section_class>
            <div class=row_class>
                <div class="flex flex-col items-start w-full md:flex-1">{left.run()}</div>
                <div class="flex flex-col items-start w-full md:flex-1 mt-8 md:mt-0">
                    {right.run()}
                </div>
            </div>
        </Section>
    }
}
