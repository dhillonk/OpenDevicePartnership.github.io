//! Shared page chrome.
//!
//! Wraps every standard page in:
//!
//! * an `ErrorBoundary` that renders a generic fallback list,
//! * the outer `w-full min-h-screen` container,
//! * the site `<Header/>`,
//! * the page `children`,
//! * and the site `<Footer/>`.
//!
//! Pages that need horizontal scrolling on overflow (typically those that
//! embed the repository graph SVG, the project introduction overlay, or
//! other wide content) opt in via `scrollable_x=true`. The default is
//! `overflow-x-hidden` to match the home / community / team pages.

use crate::components::footer::Footer;
use crate::components::header::Header;
use leptos::prelude::*;

#[component]
pub fn PageLayout(
    /// If `true`, the outer container allows horizontal scrolling
    /// (`overflow-x: auto`). Default is `overflow-x-hidden`.
    #[prop(optional)]
    scrollable_x: bool,
    children: Children,
) -> impl IntoView {
    let outer_class = if scrollable_x {
        "w-full min-h-screen overflow-x-auto"
    } else {
        "w-full min-h-screen overflow-x-hidden"
    };

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>
            <div class=outer_class>
                <Header />
                {children()}
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
