//! `<Container>` -- centered max-width content wrapper.
//!
//! Sits inside a [`super::Section`] and clamps body content to a
//! readable measure on very wide screens while still letting the
//! section's brand background bleed to the viewport edges.

use leptos::prelude::*;

/// Centered, max-width content wrapper. Defaults to a comfortable
/// reading measure (`max-w-screen-xl`).
#[component]
pub fn Container(
    /// Tailwind `max-w-*` class. Defaults to `max-w-screen-xl`.
    #[prop(default = "max-w-screen-xl")]
    max_width: &'static str,
    #[prop(into, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let final_class = compose_class(max_width, &class);
    view! { <div class=final_class>{children()}</div> }
}

fn compose_class(max_width: &str, extra: &str) -> String {
    if extra.is_empty() {
        format!("{max_width} mx-auto w-full")
    } else {
        format!("{max_width} mx-auto w-full {extra}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_class_includes_centering() {
        let c = compose_class("max-w-screen-xl", "");
        assert!(c.contains("max-w-screen-xl"));
        assert!(c.contains("mx-auto"));
        assert!(c.contains("w-full"));
    }

    #[test]
    fn extra_class_appended() {
        let c = compose_class("max-w-3xl", "py-8");
        assert!(c.contains("max-w-3xl"));
        assert!(c.contains("py-8"));
    }
}
