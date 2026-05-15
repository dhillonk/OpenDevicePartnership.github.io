//! `<Heading>` -- semantic heading element with the brand
//! typography classes.
//!
//! The site declares three heading sizes (`.h1`, `.h2`, `.h3`) that
//! already include their own fluid responsive scaling. This wrapper
//! pairs the visual size with the matching semantic element so call
//! sites stop hand-pairing `<span class="h2">` with the wrong tag.

use leptos::prelude::*;

/// Heading level. Maps both to a semantic `h1`/`h2`/`h3` element
/// and to the `.h1`/`.h2`/`.h3` typography utility class.
#[derive(Clone, Copy, Default)]
pub enum HeadingLevel {
    #[default]
    H1,
    H2,
    H3,
}

impl HeadingLevel {
    fn class(self) -> &'static str {
        match self {
            HeadingLevel::H1 => "h1",
            HeadingLevel::H2 => "h2",
            HeadingLevel::H3 => "h3",
        }
    }
}

/// Brand heading. Renders the matching semantic `<hN>` element with
/// the brand typography class baked in. Pass `class` to layer in
/// alignment, color, or spacing utilities.
#[component]
pub fn Heading(
    #[prop(default = HeadingLevel::H1)] level: HeadingLevel,
    #[prop(into, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let final_class = compose_class(level, &class);
    match level {
        HeadingLevel::H1 => view! { <h1 class=final_class>{children()}</h1> }.into_any(),
        HeadingLevel::H2 => view! { <h2 class=final_class>{children()}</h2> }.into_any(),
        HeadingLevel::H3 => view! { <h3 class=final_class>{children()}</h3> }.into_any(),
    }
}

fn compose_class(level: HeadingLevel, extra: &str) -> String {
    let level_class = level.class();
    if extra.is_empty() {
        level_class.to_string()
    } else {
        format!("{level_class} {extra}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_to_brand_class() {
        assert_eq!(HeadingLevel::H1.class(), "h1");
        assert_eq!(HeadingLevel::H2.class(), "h2");
        assert_eq!(HeadingLevel::H3.class(), "h3");
    }

    #[test]
    fn extra_class_appended() {
        assert_eq!(compose_class(HeadingLevel::H2, "text-left"), "h2 text-left");
    }

    #[test]
    fn empty_extra_is_clean() {
        assert_eq!(compose_class(HeadingLevel::H1, ""), "h1");
    }
}
