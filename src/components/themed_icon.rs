use leptos::prelude::*;

/// A `<picture>` that swaps between `/images/light/{name}.svg` and
/// `/images/dark/{name}.svg` based on `prefers-color-scheme`.
///
/// Replaces the dozen-plus hand-rolled `<picture><source.../><img/></picture>`
/// blocks scattered through the components and pages. The light asset is the
/// fallback that is rendered when no `<source>` matches.
///
/// `class` and `style` are forwarded to the `<img>` element so callers can
/// keep the exact sizing they had before.
#[component]
pub fn ThemedIcon(
    /// Asset basename (without extension or directory). The component looks up
    /// `/images/light/{name}.svg` and `/images/dark/{name}.svg`.
    #[prop(into)]
    name: String,
    /// `alt` text for the `<img>`.
    #[prop(into)]
    alt: String,
    /// Optional Tailwind / utility classes for the `<img>`.
    #[prop(into, optional)]
    class: String,
    /// Optional inline style string for the `<img>`.
    #[prop(into, optional)]
    style: String,
) -> impl IntoView {
    let dark_src = format!("/images/dark/{}.svg", name);
    let light_src = format!("/images/light/{}.svg", name);
    view! {
        <picture>
            <source srcset=dark_src media="(prefers-color-scheme: dark)" />
            <img src=light_src alt=alt class=class style=style />
        </picture>
    }
}
