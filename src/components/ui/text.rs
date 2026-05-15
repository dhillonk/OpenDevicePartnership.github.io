//! `<Text>` -- body copy with the brand paragraph classes.
//!
//! The site declares three body sizes:
//!
//!  * `.p` -- standard body copy. Has a `.p_mobile` twin pulled in
//!    automatically when [`TextSize::Body`] is used.
//!  * `.p1` -- emphasised lead size.
//!  * `.p2` -- larger lead size used in hero blocks.
//!
//! `Text` always renders a `<p>` element. For inline runs of body
//! copy inside another paragraph, prefer a raw `<span>` -- nesting
//! `<p>` inside `<p>` is invalid HTML.

use leptos::prelude::*;

/// Paragraph size. [`TextSize::Body`] resolves to the responsive
/// `.p_mobile md:p` pairing; the larger sizes use a single class
/// because they only have one declared size.
#[derive(Clone, Copy, Default)]
pub enum TextSize {
    /// Standard body copy. Responsive: small on mobile, scales up
    /// at the `md` breakpoint.
    #[default]
    Body,
    /// Emphasised lead size (`.p1`).
    Lead,
    /// Larger lead size (`.p2`) used in hero blocks.
    Large,
}

impl TextSize {
    fn class(self) -> &'static str {
        match self {
            TextSize::Body => "p_mobile md:p",
            TextSize::Lead => "p1",
            TextSize::Large => "p2",
        }
    }
}

/// Brand paragraph. Renders a `<p>` with the matching brand body
/// class. Pass `class` to layer in alignment, color, max-width, or
/// spacing utilities.
#[component]
pub fn Text(
    #[prop(default = TextSize::Body)] size: TextSize,
    #[prop(into, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let final_class = compose_class(size, &class);
    view! { <p class=final_class>{children()}</p> }
}

fn compose_class(size: TextSize, extra: &str) -> String {
    let size_class = size.class();
    if extra.is_empty() {
        size_class.to_string()
    } else {
        format!("{size_class} {extra}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_uses_responsive_pairing() {
        assert_eq!(TextSize::Body.class(), "p_mobile md:p");
    }

    #[test]
    fn lead_and_large_use_single_class() {
        assert_eq!(TextSize::Lead.class(), "p1");
        assert_eq!(TextSize::Large.class(), "p2");
    }

    #[test]
    fn extra_class_appended() {
        assert_eq!(compose_class(TextSize::Lead, "text-left"), "p1 text-left");
    }
}
