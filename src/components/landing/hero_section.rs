//! Landing page section: hero introduction.

use crate::components::section::Section;
use crate::components::ui::{Heading, HeadingLevel, Text, TextSize};
use leptos::prelude::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <Section>
            <div class="flex flex-col md:flex-row gap-20 items-start">
                <div class="flex flex-col items-start min-w-0 w-full md:flex-1">
                    <Heading level=HeadingLevel::H1 class="break-words w-full text-left">
                        {"An Open Collaboration for Secure, Modern Devices"}
                    </Heading>
                </div>
                <div class="flex flex-col items-start min-w-0 w-full md:flex-1">
                    <Text size=TextSize::Lead class="break-words w-full text-left">
                        {"The Open Device Partnership (ODP) is a global initiative to make it easier for developers and device makers to build secure, efficient, and reliable client devices for cross-platform needs and certified environments."}
                        <br />
                        <br />
                        {"By combining open standards with collaborative development practices, ODP reduces complexity, improves security, and accelerates innovation across diverse silicon and hardware platforms."}
                    </Text>
                </div>
            </div>
        </Section>
    }
}
