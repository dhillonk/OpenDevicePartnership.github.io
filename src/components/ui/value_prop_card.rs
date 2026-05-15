//! `<ValuePropCard>` -- the icon + H3 + body card used on the
//! landing page's "Value Proposition" row.
//!
//! Composes [`IconBlock`] with [`Heading`] (H3) and [`Text`]
//! (Large). The wrapper grows to fill an equal share of its
//! `md:flex-row` parent, matching the legacy Tailwind shape.

use crate::components::ui::{Heading, HeadingLevel, IconBlock, Text, TextSize};
use leptos::prelude::*;

/// Single "Value Proposition" card.
#[component]
pub fn ValuePropCard(
    /// Icon basename in `/public/icons/{light,dark}/{name}.svg`.
    icon: &'static str,
    /// Accessible label for the icon.
    icon_alt: &'static str,
    title: &'static str,
    body: &'static str,
) -> impl IntoView {
    view! {
        <IconBlock name=icon alt=icon_alt class="md:flex-1">
            <Heading level=HeadingLevel::H3 class="break-words w-full text-left">
                {title}
            </Heading>
            <Text size=TextSize::Large class="break-words w-full text-left">
                {body}
            </Text>
        </IconBlock>
    }
}
