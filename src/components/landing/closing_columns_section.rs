//! Landing page section: closing two-column "Partner-Oriented Vision"
//! / "Get Involved!" block.

use crate::components::section::Section;
use crate::components::ui::{Heading, HeadingLevel, Text, TextSize};
use leptos::prelude::*;

#[component]
pub fn ClosingColumnsSection() -> impl IntoView {
    view! {
        <Section class="py-20">
            <div class="flex flex-col md:flex-row gap-16">
                <div class="flex flex-col items-start flex-1">
                    <Heading level=HeadingLevel::H3 class="text-left">
                        {"Partner-Oriented Vision"}
                    </Heading>
                    <Text size=TextSize::Large class="text-left">
                        {"ODP is an inclusive partnership open to OEMs, ODMs, silicon vendors, hardware developers, security researchers, and anyone committed to improving device software foundations."}
                    </Text>
                </div>
                <div class="flex flex-col items-start flex-1">
                    <Heading level=HeadingLevel::H3 class="text-left">
                        {"Get Involved!"}
                    </Heading>
                    <Text size=TextSize::Large class="text-left">
                        {"Explore our documentation, clone our public repositories, and contribute your expertise. Together, we can raise the standard for trusted devices."}
                    </Text>
                </div>
            </div>
        </Section>
    }
}
