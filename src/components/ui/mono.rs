//! `<Mono>` -- monospaced labels.
//!
//! The site uses two monospaced lockups:
//!
//!  * `.mono` -- the small caps eyebrow label (e.g. team
//!    sub-titles), rendered inline.
//!  * `.p_mono` -- the larger display monospaced lockup.
//!
//! Both render as `<span>` so they can sit inside any flow.

use leptos::prelude::*;

/// Monospaced size variant.
#[derive(Clone, Copy, Default)]
pub enum MonoSize {
    /// Small caps eyebrow label.
    #[default]
    Label,
    /// Large display monospaced lockup.
    Display,
}

impl MonoSize {
    fn class(self) -> &'static str {
        match self {
            MonoSize::Label => "mono",
            MonoSize::Display => "p_mono",
        }
    }
}

/// Brand monospaced lockup. Always renders an inline `<span>`.
#[component]
pub fn Mono(
    #[prop(default = MonoSize::Label)] size: MonoSize,
    #[prop(into, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let final_class = compose_class(size, &class);
    view! { <span class=final_class>{children()}</span> }
}

fn compose_class(size: MonoSize, extra: &str) -> String {
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
    fn size_to_brand_class() {
        assert_eq!(MonoSize::Label.class(), "mono");
        assert_eq!(MonoSize::Display.class(), "p_mono");
    }

    #[test]
    fn extra_class_appended() {
        assert_eq!(compose_class(MonoSize::Label, "text-left"), "mono text-left");
    }
}
