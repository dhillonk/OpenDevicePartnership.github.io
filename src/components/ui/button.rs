//! `<Button>` and `<IconButton>` -- branded button surfaces.
//!
//! The brand declares two button looks:
//!
//!  * `.odp-btn` + `.odp-btn-text` -- the primary CTA pill used in
//!    landing surfaces.
//!  * `.odp-header-btn` + `.odp-header-btn-text` -- the slimmer
//!    header navigation button (used by [`crate::components::header`]
//!    via the router `<A>` component, kept out of `<Button>` so the
//!    primitive stays focused on imperative actions).
//!
//! `Button` is for in-page actions (`type="button"` by default).
//! For navigation, use a router link or [`super::Link`]. `IconButton`
//! is the same shell with an icon child instead of text.

use leptos::prelude::*;

/// Visual variant for [`Button`] / [`IconButton`].
#[derive(Clone, Copy, Default)]
pub enum ButtonVariant {
    /// Primary CTA pill.
    #[default]
    Primary,
}

impl ButtonVariant {
    fn class(self) -> &'static str {
        match self {
            ButtonVariant::Primary => "odp-btn odp-btn-text",
        }
    }
}

/// Brand button for in-page actions. Defaults to `type="button"`
/// so it never accidentally submits a form.
#[component]
pub fn Button(
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let final_class = compose_class(variant, &class);
    view! {
        <button
            type="button"
            class=final_class
            on:click=move |_| {
                if let Some(cb) = on_click {
                    cb.run(());
                }
            }
        >
            {children()}
        </button>
    }
}

/// Icon-only variant of [`Button`]. The caller is responsible for
/// supplying an `aria-label` so the button has an accessible name.
#[component]
pub fn IconButton(
    aria_label: &'static str,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let final_class = compose_class(variant, &class);
    view! {
        <button
            type="button"
            class=final_class
            aria-label=aria_label
            on:click=move |_| {
                if let Some(cb) = on_click {
                    cb.run(());
                }
            }
        >
            {children()}
        </button>
    }
}

fn compose_class(variant: ButtonVariant, extra: &str) -> String {
    let variant_class = variant.class();
    if extra.is_empty() {
        variant_class.to_string()
    } else {
        format!("{variant_class} {extra}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_to_brand_class() {
        assert_eq!(ButtonVariant::Primary.class(), "odp-btn odp-btn-text");
    }

    #[test]
    fn extra_class_appended() {
        assert_eq!(
            compose_class(ButtonVariant::Primary, "w-full"),
            "odp-btn odp-btn-text w-full"
        );
    }
}
