//! `<ArrowLink>` -- a brand link prefixed with a non-underlined "→ "
//! glyph and a single underlined label.
//!
//! This molecule appears verbatim across the documentation list, the
//! project rows, and the getting-started page. External vs. internal
//! routing is auto-detected from the `href`: anything starting with
//! `http://` or `https://` falls through to a plain `<a target="_blank">`,
//! everything else uses `leptos_router::components::A` so client-side
//! navigation is preserved.

use leptos::prelude::*;
use leptos_router::components::A;

/// Visual size of the link, mirroring [`crate::components::ui::LinkSize`].
#[derive(Clone, Copy, Default)]
pub enum ArrowLinkSize {
    /// `link_mobile md:link` -- the default body-link size.
    #[default]
    Standard,
    /// `link_mobile md:link_large` -- the larger size used by the
    /// documentation list.
    Large,
}

impl ArrowLinkSize {
    fn class(self) -> &'static str {
        match self {
            ArrowLinkSize::Standard => "link no-underline",
            ArrowLinkSize::Large => "link_mobile md:link_large no-underline",
        }
    }
}

/// Brand "arrow + label" link. Detects external URLs from the `href`
/// scheme; pass `external=true` to force the external branch when the
/// auto-detection is wrong (e.g. protocol-relative URLs).
#[component]
pub fn ArrowLink(
    href: &'static str,
    title: &'static str,
    #[prop(default = ArrowLinkSize::Standard)] size: ArrowLinkSize,
    #[prop(optional)] external: Option<bool>,
) -> impl IntoView {
    let is_external = external.unwrap_or_else(|| is_external_href(href));
    let class_external = wrapper_class(size, true);
    let class_internal = wrapper_class(size, false);
    view! {
        <Show
            when=move || is_external
            fallback={
                let class_internal = class_internal.clone();
                move || {
                    view! {
                        <div class=class_internal.clone()>
                            <A href=href>
                                <span class="no-underline">{"→ "}</span>
                                <span class="underline">{title}</span>
                            </A>
                        </div>
                    }
                }
            }
        >
            <div class=class_external.clone()>
                <a href=href target="_blank" rel="noopener noreferrer" class="no-underline">
                    <span class="no-underline">{"→ "}</span>
                    <span class="underline">{title}</span>
                </a>
            </div>
        </Show>
    }
}

fn wrapper_class(size: ArrowLinkSize, external: bool) -> String {
    let kind = if external { "external-link" } else { "internal-link" };
    match size {
        ArrowLinkSize::Standard => format!("link {kind} no-underline"),
        ArrowLinkSize::Large => format!("link_mobile md:link_large {kind} no-underline"),
    }
}

fn is_external_href(href: &str) -> bool {
    href.starts_with("http://") || href.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_detection() {
        assert!(is_external_href("https://example.com"));
        assert!(is_external_href("http://example.com"));
        assert!(!is_external_href("/community"));
        assert!(!is_external_href("#anchor"));
    }

    #[test]
    fn wrapper_class_matches_legacy() {
        assert_eq!(
            wrapper_class(ArrowLinkSize::Large, true),
            "link_mobile md:link_large external-link no-underline"
        );
        assert_eq!(
            wrapper_class(ArrowLinkSize::Large, false),
            "link_mobile md:link_large internal-link no-underline"
        );
        assert_eq!(
            wrapper_class(ArrowLinkSize::Standard, true),
            "link external-link no-underline"
        );
    }
}
