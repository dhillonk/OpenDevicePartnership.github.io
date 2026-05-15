//! `<Link>` -- inline underlined link.
//!
//! There are three declared link sizes (`.link`, `.link_mobile`,
//! `.link_large`). [`LinkSize::Standard`] picks up the responsive
//! pairing automatically; [`LinkSize::Large`] is the lone larger
//! size used in CTAs.
//!
//! For client-side router navigation use
//! [`leptos_router::components::A`] directly -- this primitive is for
//! plain `<a>` elements (external URLs, in-page anchors, mailto, …).

use leptos::prelude::*;

/// Link size.
#[derive(Clone, Copy, Default)]
pub enum LinkSize {
    /// Standard inline link, responsive across breakpoints.
    #[default]
    Standard,
    /// Larger CTA link.
    Large,
}

impl LinkSize {
    fn class(self) -> &'static str {
        match self {
            LinkSize::Standard => "link_mobile md:link",
            LinkSize::Large => "link_large",
        }
    }
}

/// Brand link. Wraps a plain `<a>` element. External URLs are
/// detected automatically (anything starting with `http://` or
/// `https://`) and get `target="_blank"` plus
/// `rel="noopener noreferrer"`.
#[component]
pub fn Link(
    href: &'static str,
    #[prop(default = LinkSize::Standard)] size: LinkSize,
    #[prop(into, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let final_class = compose_class(size, &class);
    let external = is_external(href);
    let target = if external { Some("_blank") } else { None };
    let rel = if external { Some("noopener noreferrer") } else { None };
    view! {
        <a class=final_class href=href target=target rel=rel>
            {children()}
        </a>
    }
}

fn compose_class(size: LinkSize, extra: &str) -> String {
    let size_class = size.class();
    if extra.is_empty() {
        size_class.to_string()
    } else {
        format!("{size_class} {extra}")
    }
}

fn is_external(href: &str) -> bool {
    href.starts_with("http://") || href.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_to_brand_class() {
        assert_eq!(LinkSize::Standard.class(), "link_mobile md:link");
        assert_eq!(LinkSize::Large.class(), "link_large");
    }

    #[test]
    fn external_detection() {
        assert!(is_external("https://github.com/x"));
        assert!(is_external("http://example.com"));
        assert!(!is_external("/projects"));
        assert!(!is_external("#anchor"));
        assert!(!is_external("mailto:foo@example.com"));
    }
}
