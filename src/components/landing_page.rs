use crate::components::image_button::ImageButton;
use crate::components::section::{Section, Surface};
use crate::components::themed_icon::ThemedIcon;
use crate::components::ui::{Heading, HeadingLevel, Text, TextSize};
use leptos::prelude::*;

#[component]
pub fn LandingPage() -> impl IntoView {
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
        <Section surface=Surface::Secondary class="py-20">
            <div>
                <Heading level=HeadingLevel::H2 class="text-left">
                    {"Value Proposition"}
                </Heading>
                <div class="flex flex-col md:flex-row gap-16">
                    <div class="flex flex-col items-start w-full md:flex-1">
                        <ThemedIcon name="lock" alt="Security Icon" class="icon" />
                        <Heading level=HeadingLevel::H3 class="break-words w-full text-left">
                            {"Enhanced Security"}
                        </Heading>
                        <Text size=TextSize::Large class="break-words w-full text-left">
                            {"Security threats continue to evolve. ODP takes a proactive approach: reducing attack surfaces, using secure hardware features, leveraging the memory-safe Rust language, and designing every component with security-first principles."}
                        </Text>
                    </div>
                    <div class="flex flex-col items-start w-full md:flex-1">
                        <ThemedIcon name="checkcircle" alt="Interoperability Icon" class="icon" />
                        <Heading level=HeadingLevel::H3 class="break-words w-full text-left">
                            {"Standardization"}
                        </Heading>
                        <Text size=TextSize::Large class="break-words w-full text-left">
                            {"Many device firmware components are 'invisible plumbing' - necessary but costly to build and maintain. ODP's standards-based approach simplifies this infrastructure, maximizing reuse across devices, architectures (x86 and ARM), and generations."}
                        </Text>
                    </div>
                    <div class="flex flex-col items-start w-full md:flex-1">
                        <ThemedIcon name="fastforward" alt="Innovation Icon" class="icon" />
                        <Heading level=HeadingLevel::H3 class="break-words w-full text-left">
                            {"Accelerated Development"}
                        </Heading>
                        <Text size=TextSize::Large class="break-words w-full text-left">
                            {"Open collaboration means sharing solutions, reducing duplicated work, and speeding up the development of high-quality products."}
                        </Text>
                    </div>
                </div>
            </div>
        </Section>

        // ODP Projects Section
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

        // Boot Firmware Buttons Section
        <Section class="pb-32">
            <div class="flex flex-col md:flex-row gap-16 justify-start">
                <ImageButton
                    href="/boot-firmware"
                    img_src="/images/patina.png"
                    alt="Boot Firmware"
                />
                <ImageButton
                    href="/embedded-controller"
                    img_src="/images/ec.png"
                    alt="Embedded Controller"
                />
                <ImageButton
                    href="/windows-ec-services"
                    img_src="/images/ec_services.png"
                    alt="EC Services"
                />
            </div>
        </Section>

        // Two Columns Section
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
