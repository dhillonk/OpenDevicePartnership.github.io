//! Router-level page chrome.
//!
//! Most pages share the same scaffolding: an error boundary, an
//! outer `min-h-screen` container, the site `<Header/>` at the top,
//! the page-specific content in the middle, and the `<Footer/>` at
//! the bottom. Before t23 every page pulled this in via a manual
//! `<PageLayout>` wrapper, so each page started and ended with
//! identical boilerplate.
//!
//! Now the chrome lives at the router level: the route tree wraps
//! each page's view in a [`SiteShell`] (or [`SiteShellScrollable`])
//! [`ParentRoute`], and pages render only their content. Routes
//! whose content can be wider than the viewport (the repository
//! graph in particular) live under `SiteShellScrollable`; everything
//! else lives under `SiteShell`.

use crate::components::footer::Footer;
use crate::components::header::Header;
use leptos::prelude::*;
use leptos_router::components::Outlet;

/// Shell used by routes whose content is guaranteed to fit the
/// viewport horizontally. The outer container clips horizontal
/// overflow so a runaway grid never produces a body-level scrollbar.
#[component]
pub fn SiteShell() -> impl IntoView {
    chrome("w-full min-h-screen overflow-x-hidden")
}

/// Shell used by routes that may overflow horizontally (e.g. the
/// project pages that embed the wide repository graph SVG). The
/// outer container exposes a horizontal scrollbar when needed.
#[component]
pub fn SiteShellScrollable() -> impl IntoView {
    chrome("w-full min-h-screen overflow-x-auto")
}

fn chrome(outer_class: &'static str) -> impl IntoView {
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
                <Outlet />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
