//! `<Stack>` -- one-axis flex layout with a configurable gap.
//!
//! Most page content is structured as columns of evenly-spaced
//! children; this primitive replaces dozens of bespoke
//! `flex flex-col gap-N` `<div>`s with a single declarative wrapper.

use leptos::prelude::*;

/// Stacking direction.
#[derive(Clone, Copy, Default)]
pub enum StackDirection {
    /// Children stack top-to-bottom (`flex-col`).
    #[default]
    Vertical,
    /// Children flow left-to-right (`flex-row`).
    Horizontal,
}

impl StackDirection {
    fn class(self) -> &'static str {
        match self {
            StackDirection::Vertical => "flex flex-col",
            StackDirection::Horizontal => "flex flex-row",
        }
    }
}

/// One-axis flex layout. `gap` is one of the built-in Tailwind
/// spacing tokens (`0`, `1`, `2`, `3`, `4`, `6`, `8`, `10`, `12`,
/// `16`, `20`, `24`, `32`); other values fall back to `gap-4`
/// because Tailwind only sees literal class strings at build time.
#[component]
pub fn Stack(
    #[prop(default = StackDirection::Vertical)] direction: StackDirection,
    #[prop(default = 4)] gap: u8,
    #[prop(into, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let final_class = compose_class(direction, gap, &class);
    view! { <div class=final_class>{children()}</div> }
}

fn compose_class(direction: StackDirection, gap: u8, extra: &str) -> String {
    let dir_class = direction.class();
    let gap_class = gap_class(gap);
    if extra.is_empty() {
        format!("{dir_class} {gap_class}")
    } else {
        format!("{dir_class} {gap_class} {extra}")
    }
}

/// Map a gap token to its Tailwind class. Hard-coded so the JIT
/// always sees the literal string. Unknown values fall back to
/// `gap-4`.
fn gap_class(gap: u8) -> &'static str {
    match gap {
        0 => "gap-0",
        1 => "gap-1",
        2 => "gap-2",
        3 => "gap-3",
        4 => "gap-4",
        6 => "gap-6",
        8 => "gap-8",
        10 => "gap-10",
        12 => "gap-12",
        16 => "gap-16",
        20 => "gap-20",
        24 => "gap-24",
        32 => "gap-32",
        _ => "gap-4",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_to_class() {
        assert_eq!(StackDirection::Vertical.class(), "flex flex-col");
        assert_eq!(StackDirection::Horizontal.class(), "flex flex-row");
    }

    #[test]
    fn known_gap_tokens_map_to_tailwind() {
        assert_eq!(gap_class(0), "gap-0");
        assert_eq!(gap_class(8), "gap-8");
        assert_eq!(gap_class(20), "gap-20");
        assert_eq!(gap_class(32), "gap-32");
    }

    #[test]
    fn unknown_gap_falls_back_to_4() {
        assert_eq!(gap_class(7), "gap-4");
        assert_eq!(gap_class(100), "gap-4");
    }

    #[test]
    fn full_class_string() {
        let c = compose_class(StackDirection::Vertical, 8, "items-start");
        assert!(c.contains("flex flex-col"));
        assert!(c.contains("gap-8"));
        assert!(c.contains("items-start"));
    }
}
