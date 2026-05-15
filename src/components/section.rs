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
    let surface_class = surface.class();
    let padding_class = if padded {
        "px-4 sm:px-8 md:px-16 lg:px-32"
    } else {
        ""
    };
    view! { <section class=format!("{surface_class} {padding_class} {class}")>{children()}</section> }
}
