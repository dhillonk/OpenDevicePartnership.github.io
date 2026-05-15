//! `<TeamHero>` -- the "Meet the team" header used by every per-team
//! page.
//!
//! Before this extraction, `team_ec.rs`, `team_patina.rs`, and
//! `team_ec_services.rs` each shipped a copy of the same block --
//! same layout, same back-button, same two-column heading + label +
//! description -- with only the team name and tagline differing.
//! The extraction collapses that triplication and migrates the
//! remaining markup onto the [`crate::components::ui`] design system
//! primitives.

use crate::components::section::Section;
use crate::components::themed_icon::ThemedIcon;
use crate::components::ui::{Heading, HeadingLevel, Mono, Text, TextSize};
use leptos::prelude::*;

/// Team-page header. Renders the section background, a back-button
/// (mobile inline / desktop floated to the top-left), and the
/// "Meet the team" / team-name pair.
#[component]
pub fn TeamHero(
    /// Short team name shown above the description (the `mono`
    /// eyebrow label).
    team_name: &'static str,
    /// One-sentence description shown below the team name.
    description: &'static str,
) -> impl IntoView {
    view! {
        <Section class="py-4 md:py-32 relative">
            <div class="block md:hidden mb-4">
                <BackButton />
            </div>
            <div class="hidden md:block absolute left-0 top-0">
                <BackButton />
            </div>
            <div class="flex flex-col md:flex-row gap-20 items-start">
                <div class="flex flex-col items-start w-full md:flex-1">
                    <Heading level=HeadingLevel::H1 class="text-left">
                        {"Meet the team"}
                    </Heading>
                </div>
                <div class="flex flex-col items-start w-full md:flex-1">
                    <Mono class="text-left">{team_name}</Mono>
                    <Text size=TextSize::Lead class="text-left">
                        {description}
                    </Text>
                </div>
            </div>
        </Section>
    }
}

#[component]
fn BackButton() -> impl IntoView {
    view! {
        <a href="javascript:history.back()" class="block m-0 p-0">
            <ThemedIcon
                name="backbutton"
                alt="Back"
                style="margin: 0; padding: 0; display: block;"
            />
        </a>
    }
}
