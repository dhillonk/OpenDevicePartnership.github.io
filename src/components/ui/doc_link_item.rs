//! `<DocLinkItem>` -- one row of the "Documentation" footer list.
//!
//! Wraps an [`ArrowLink`] (size [`ArrowLinkSize::Large`]) inside a
//! semantic `<li>`. Extracted so the documentation list (and any
//! future "list of brand links" variant) doesn't have to reach into
//! the wrapper-`<li>` + `<ArrowLink>` shape every time.

use crate::components::ui::{ArrowLink, ArrowLinkSize};
use leptos::prelude::*;

/// One `<li>` containing a large arrow-link. `external` overrides
/// the [`ArrowLink`] auto-detection (the documentation list mixes
/// in-app routes with external URLs and tracks the distinction
/// per-item in [`crate::components::documentation_training::DocLink`]).
#[component]
pub fn DocLinkItem(href: &'static str, title: &'static str, external: bool) -> impl IntoView {
    view! {
        <li>
            <ArrowLink href=href title=title size=ArrowLinkSize::Large external=external />
        </li>
    }
}
