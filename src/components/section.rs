//! `<Section>` -- the standard section shell.
//!
//! The site has a recurring pattern for top-level page sections:
//!
//! ```text
//! <section class="background_<surface> px-4 sm:px-8 md:px-16 lg:px-32 py-..">
//!     ...
//! </section>
//! ```
//!
//! That long horizontal-padding ladder (and the brand surface class)
//! was duplicated at every section. `Section` bakes both in and lets
//! the caller add anything else -- typically a vertical padding -- via
//! `class`.

use leptos::prelude::*;

/// Surface palette token. Maps to the `.background_*` utility classes
/// declared in `style/tailwind.css`.
#[derive(Clone, Copy, Default)]
#[allow(dead_code)]
pub enum Surface {
    #[default]
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
}

impl Surface {
    fn class(self) -> &'static str {
        match self {
            Surface::Primary => "background_primary",
            Surface::Secondary => "background_secondary",
            Surface::Tertiary => "background_tertiary",
            Surface::Quaternary => "background_quaternary",
        }
    }
}

/// A page-level section with the standard responsive horizontal
/// padding ladder (`px-4 sm:px-8 md:px-16 lg:px-32`) and a brand
/// surface background. Vertical padding and any other utilities go
/// through `class`. Pass `padded=false` for full-bleed sections that
/// want their children to extend to the viewport edge (e.g. hero
/// images that bleed left).
#[component]
pub fn Section(
    /// Brand surface for the section background. Defaults to
    /// [`Surface::Primary`].
    #[prop(default = Surface::Primary)]
    surface: Surface,
    /// If `true` (default), apply the standard horizontal padding
    /// ladder. Set to `false` for full-bleed sections.
    #[prop(default = true)]
    padded: bool,
    /// Extra utility classes (vertical padding, gap, etc.).
    #[prop(into, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let class = section_class(surface, padded, &class);
    view! { <section class=class>{children()}</section> }
}

/// The standard horizontal padding ladder applied when `padded=true`.
const PADDING_LADDER: &str = "px-4 sm:px-8 md:px-16 lg:px-32";

/// Compose the final `class` attribute for a [`Section`] from its
/// `surface`, `padded` flag, and caller-supplied `extra` classes.
///
/// Extracted as a pure function so the composition rules can be
/// covered by host-side unit tests without standing up a renderer.
fn section_class(surface: Surface, padded: bool, extra: &str) -> String {
    let surface_class = surface.class();
    let padding_class = if padded { PADDING_LADDER } else { "" };
    format!("{surface_class} {padding_class} {extra}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_maps_to_brand_class() {
        assert_eq!(Surface::Primary.class(), "background_primary");
        assert_eq!(Surface::Secondary.class(), "background_secondary");
        assert_eq!(Surface::Tertiary.class(), "background_tertiary");
        assert_eq!(Surface::Quaternary.class(), "background_quaternary");
    }

    #[test]
    fn padded_section_includes_horizontal_ladder() {
        let class = section_class(Surface::Primary, true, "py-8");
        assert!(class.contains("background_primary"));
        assert!(class.contains("px-4 sm:px-8 md:px-16 lg:px-32"));
        assert!(class.contains("py-8"));
    }

    #[test]
    fn unpadded_section_omits_horizontal_ladder() {
        let class = section_class(Surface::Secondary, false, "");
        assert!(class.contains("background_secondary"));
        assert!(!class.contains("px-4"));
        assert!(!class.contains("sm:px-8"));
        assert!(!class.contains("md:px-16"));
        assert!(!class.contains("lg:px-32"));
    }

    #[test]
    fn empty_extra_class_does_not_break_composition() {
        // Regression: `Section` is used many times with no extra
        // classes. Make sure we never produce a malformed `class=""`
        // (we tolerate trailing whitespace, which the browser ignores).
        let class = section_class(Surface::Primary, true, "");
        assert!(class.starts_with("background_primary"));
    }
}
