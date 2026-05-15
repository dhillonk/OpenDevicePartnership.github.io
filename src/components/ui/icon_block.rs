//! `<IconBlock>` -- the "icon stacked above content" molecule.
//!
//! Recurs across the landing page (value-prop cards), the
//! documentation footer, and the team hero. A [`ThemedIcon`] sits at
//! the top of a left-aligned column; the children render below it.
//! Sizing of the icon is parameterised so callers can pick between
//! the two house defaults.

use crate::components::themed_icon::ThemedIcon;
use leptos::prelude::*;

/// Icon size preset.
#[derive(Clone, Copy, Default)]
pub enum IconBlockSize {
    /// `.icon` -- the standard value-prop / community card size.
    #[default]
    Standard,
    /// `w-[80px] h-[80px] md:w-[150px] md:h-[150px]` -- the larger
    /// hero / documentation-footer icon.
    Hero,
}

impl IconBlockSize {
    fn icon_class(self) -> &'static str {
        match self {
            IconBlockSize::Standard => "icon",
            IconBlockSize::Hero => "w-[80px] h-[80px] md:w-[150px] md:h-[150px] object-contain mb-4 block",
        }
    }
}

/// Vertical "icon over content" column.
#[component]
pub fn IconBlock(
    /// Icon basename in `/public/icons/{light,dark}/{name}.svg`.
    name: &'static str,
    /// Accessible label for the icon.
    alt: &'static str,
    #[prop(default = IconBlockSize::Standard)] size: IconBlockSize,
    #[prop(into, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let wrapper_class = compose_wrapper_class(&class);
    view! {
        <div class=wrapper_class>
            <ThemedIcon name=name alt=alt class=size.icon_class() />
            {children()}
        </div>
    }
}

fn compose_wrapper_class(extra: &str) -> String {
    let base = "flex flex-col items-start w-full";
    if extra.is_empty() {
        base.to_string()
    } else {
        format!("{base} {extra}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_class_presets() {
        assert_eq!(IconBlockSize::Standard.icon_class(), "icon");
        assert!(IconBlockSize::Hero.icon_class().contains("w-[80px]"));
    }

    #[test]
    fn wrapper_class_composes_extras() {
        assert_eq!(compose_wrapper_class(""), "flex flex-col items-start w-full");
        assert_eq!(
            compose_wrapper_class("md:flex-1"),
            "flex flex-col items-start w-full md:flex-1"
        );
    }
}
