//! Landing page section: "ODP Projects" intro paragraph + the three
//! project image-button tiles.

use crate::components::image_button::ImageButton;
use crate::components::section::Section;
use crate::components::ui::{Heading, HeadingLevel, Text, TextSize};
use leptos::prelude::*;

#[component]
pub fn ProjectsSection() -> impl IntoView {
    view! {
        <Section class="py-32">
            <div style="max-width: 960px;">
                <Heading level=HeadingLevel::H2 class="text-left">
                    {"ODP Projects"}
                </Heading>
                <Text size=TextSize::Large class="text-left max-w-full">
                    {"While ODP's first projects focus on boot firmware and embedded controller software, the partnership welcomes new ideas aligned with our core goals: security, efficiency, and broad reusability."}
                    <br />
                    <br />
                </Text>
            </div>
        </Section>
        <Section class="pb-32">
            <div class="flex flex-col md:flex-row gap-16 justify-start">
                <ImageButton
                    href="/boot-firmware"
                    img_src="/images/patina.webp"
                    alt="Boot Firmware"
                />
                <ImageButton
                    href="/embedded-controller"
                    img_src="/images/ec.webp"
                    alt="Embedded Controller"
                />
                <ImageButton
                    href="/windows-ec-services"
                    img_src="/images/ec_services.webp"
                    alt="EC Services"
                />
            </div>
        </Section>
    }
}
