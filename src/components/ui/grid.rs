//! `<Grid>` -- responsive grid layout.
//!
//! Stacks to a single column on small screens and expands to the
//! requested column count at the `md` breakpoint. The supported
//! column counts are hard-coded so Tailwind's JIT always sees the
//! literal class strings.

use leptos::prelude::*;

/// Responsive grid. `cols` is the target column count at the `md`
/// breakpoint and above; below that the grid collapses to a single
/// column. Supported values: `1`, `2`, `3`, `4`, `5`, `6`. Other
/// values fall back to `1`. `gap` follows the same token list as
/// [`super::Stack`].
#[component]
pub fn Grid(
    #[prop(default = 3)] cols: u8,
    #[prop(default = 4)] gap: u8,
    #[prop(into, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let final_class = compose_class(cols, gap, &class);
    view! { <div class=final_class>{children()}</div> }
}

fn compose_class(cols: u8, gap: u8, extra: &str) -> String {
    let cols_class = cols_class(cols);
    let gap_class = gap_class(gap);
    if extra.is_empty() {
        format!("grid grid-cols-1 {cols_class} {gap_class}")
    } else {
        format!("grid grid-cols-1 {cols_class} {gap_class} {extra}")
    }
}

fn cols_class(cols: u8) -> &'static str {
    match cols {
        1 => "md:grid-cols-1",
        2 => "md:grid-cols-2",
        3 => "md:grid-cols-3",
        4 => "md:grid-cols-4",
        5 => "md:grid-cols-5",
        6 => "md:grid-cols-6",
        _ => "md:grid-cols-1",
    }
}

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
    fn always_collapses_to_one_column_on_mobile() {
        let c = compose_class(3, 4, "");
        assert!(c.contains("grid-cols-1"));
        assert!(c.contains("md:grid-cols-3"));
    }

    #[test]
    fn unsupported_cols_falls_back_to_one() {
        assert_eq!(cols_class(0), "md:grid-cols-1");
        assert_eq!(cols_class(7), "md:grid-cols-1");
    }

    #[test]
    fn extra_class_appended() {
        let c = compose_class(2, 8, "items-start");
        assert!(c.contains("md:grid-cols-2"));
        assert!(c.contains("gap-8"));
        assert!(c.contains("items-start"));
    }
}
