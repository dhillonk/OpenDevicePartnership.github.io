use crate::components::section::Section;
use crate::components::team_grid::TeamGrid;
use crate::components::themed_icon::ThemedIcon;
use crate::components::ui::{Heading, HeadingLevel, Text, TextSize};
use crate::data::teams::steering_committee;
use leptos::prelude::*;

#[component]
pub fn CommunityTeams() -> impl IntoView {
    let steering_committee = steering_committee();

    view! {
        <Section class="py-6">
            <div class="flex flex-col md:flex-row gap-16 mb-20">
                <div class="flex flex-col items-start w-full md:flex-1">
                    <Heading level=HeadingLevel::H1 class="break-words w-full text-left">
                        {"How ODP is built by its community"}
                    </Heading>
                </div>
                <div class="flex flex-col justify-start w-full md:flex-1">
                    <Text size=TextSize::Large class="break-words w-full text-left">

                        {"The Open Device Partnership (ODP) is a collaborative open-source initiative designed to promote cooperative innovation in firmware development through contribution and transparency."}
                        <br />
                        <br />
                        {"To support this vision, ODP has adopted a lightweight yet formal governance model that clarifies how decisions are made, how contributions are recognized, and how the community remains focused on shared goals."}
                        <br />
                        <br />
                        {"Like most open-source communities, the ODP governance model consists of a group of core maintainers that are responsible for guiding the technical direction of the project known as the \"Technical Steering Committee\" (TSC). Members are typically industry-experienced contributors nominated by the community members or its sponsors."}
                        <br />
                        {"The model also provides for certain \"Working Groups\", which are task-focused teams that handle the development or specification within a particular area of concern (for example, Patina, EC Services)."}
                        <br />
                        {"Decisions are made via public discussion followed by majority vote among TSC members."}
                        <br />
                        {"All official actions, road maps, and meeting notes are transparently published, and all project materials are hosted openly on GitHub."}
                        <br />
                        <br />
                        {"For more information, see the complete governance policies at the "}
                        <a
                            href="https://github.com/OpenDevicePartnership/governance/blob/main/README.md"
                            target="_blank"
                            class="underline"
                        >
                            {"ODP Governance Repository"}
                        </a>
                        {", or join the discussion on our public chat on "}
                        <a
                            href="https://opendevicepartnership.zulipchat.com/"
                            target="_blank"
                            class="underline"
                        >
                            {"Zulip"}
                        </a>
                        {"."}
                    </Text>
                </div>
            </div>
            <div class="flex flex-col items-start mb-20">
                <ThemedIcon name="teams" alt="Teams" class="icon" />
                <Heading level=HeadingLevel::H2 class="break-words w-full text-left">
                    {"Steering Committee"}
                </Heading>
            </div>
            <TeamGrid members=steering_committee />
            <div class="flex flex-col items-start mb-20">
                <Heading level=HeadingLevel::H2 class="break-words w-full text-left">
                    {"Teams"}
                </Heading>
            </div>
            <div class="flex flex-col md:flex-row items-stretch justify-center gap-16 md:gap-24 lg:gap-[175px]">
                <TeamCard
                    title="Boot Firmware (Patina)"
                    description="Developing and managing development of a new modern UEFI"
                    href="/team-patina"
                />
                <TeamCard
                    title="Secure Embedded Controller"
                    description="Developing and managing secure EC internals"
                    href="/team-ec"
                />
                <TeamCard
                    title="Unified EC Services"
                    description="Designing and managing implementation of a unified EC Services interface"
                    href="team-ec-services"
                />
            </div>
        </Section>
    }
}

#[component]
fn TeamCard(title: &'static str, description: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-start h-full w-full md:w-80 min-h-[350px]">
            <Heading level=HeadingLevel::H3 class="break-words w-full text-left">
                {title}
            </Heading>
            <Text size=TextSize::Large class="break-words w-full text-left">
                {description}
            </Text>
            <div class="flex-1"></div>
            <a href=href class="odp-btn odp-btn-text mt-auto no-underline">
                {"Members + Contacts"}
            </a>
        </div>
    }
}
