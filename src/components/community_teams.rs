use crate::components::team_grid::{TeamGrid, TeamMember};
use crate::components::themed_icon::ThemedIcon;
use leptos::prelude::*;

fn create_steering_committee() -> Vec<TeamMember> {
    vec![
        TeamMember {
            first_name: "Karan",
            last_name: "Dhillon",
            role: "Member",
            github_username: "dhillonk",
            github_url: "https://github.com/dhillonk",
            image_url: "https://github.com/dhillonk.png?size=200",
        },
        TeamMember {
            first_name: "Jerry",
            last_name: "Xie",
            role: "Member",
            github_username: "jerrysxie",
            github_url: "https://github.com/jerrysxie",
            image_url: "https://github.com/jerrysxie.png?size=200",
        },
        TeamMember {
            first_name: "Michael",
            last_name: "Kubacki",
            role: "Member",
            github_username: "makubacki",
            github_url: "https://github.com/makubacki",
            image_url: "https://github.com/makubacki.png?size=200",
        },
    ]
}

#[component]
pub fn CommunityTeams() -> impl IntoView {
    let steering_committee = create_steering_committee();

    view! {
        <section class="background_primary px-4 py-6 md:px-32">
            <div class="flex flex-col md:flex-row gap-16 mb-20">
                <div class="flex flex-col items-start w-full md:flex-1">
                    <span class="h1 break-words w-full block text-left">
                        {"How ODP is built by its community"}
                    </span>
                </div>
                <div class="flex flex-col justify-start w-full md:flex-1">
                    <span class="p2 break-words w-full block text-left">

                        {"The Open Device Partnership (ODP) is a collaborative open-source initiative designed to promote cooperative innovation in firmware development through contribution and transparency."}
                        <br /> <br />
                        {"To support this vision, ODP has adopted a lightweight yet formal governance model that clarifies how decisions are made, how contributions are recognized, and how the community remains focused on shared goals."}
                        <br /> <br />
                        {"Like most open-source communities, the ODP governance model consists of a group of core maintainers that are responsible for guiding the technical direction of the project known as the \"Technical Steering Committee\" (TSC). Members are typically industry-experienced contributors nominated by the community members or its sponsors."}
                        <br />
                        {"The model also provides for certain \"Working Groups\", which are task-focused teams that handle the development or specification within a particular area of concern (for example, Patina, EC Services)."}
                        <br />
                        {"Decisions are made via public discussion followed by majority vote among TSC members."}
                        <br />
                        {"All official actions, road maps, and meeting notes are transparently published, and all project materials are hosted openly on GitHub."}
                        <br /> <br />
                        {"For more information, see the complete governance policies at the "}
                        <a
                            href="https://github.com/OpenDevicePartnership/governance/blob/main/README.md"
                            target="_blank"
                            style="text-decoration: underline;"
                        >
                            {"ODP Governance Repository"}
                        </a> {", or join the discussion on our public chat on "}
                        <a
                            href="https://opendevicepartnership.zulipchat.com/"
                            target="_blank"
                            style="text-decoration: underline;"
                        >
                            {"Zulip"}
                        </a> {"."}
                    </span>
                </div>
            </div>
            <div class="flex flex-col items-left mb-20">
                <ThemedIcon name="teams" alt="Teams" class="icon" />
                <span
                    class="h2 break-words w-full"
                    style="display: flex; justify-content: left; align-items: left;"
                >
                    {"Steering Committee"}
                </span>
            </div>
            <TeamGrid members=steering_committee />
            <div class="flex flex-col items-left mb-20">
                <span
                    class="h2 break-words w-full"
                    style="display: flex; justify-content: left; align-items: left;"
                >
                    {"Teams"}
                </span>
            </div>
            <div class="flex flex-col md:flex-row items-stretch justify-center" style="gap: 175px;">
                <div
                    class="flex flex-col items-start h-full"
                    style="width: 320px; min-height: 350px; justify-content: flex-start;"
                >
                    <span class="h3 break-words w-full block text-left">
                        {"Boot Firmware (Patina)"}
                    </span>
                    <span class="p2 break-words w-full block text-left">
                        {"Developing and managing development of a new modern UEFI"}
                    </span>
                    <div style="flex: 1 1 auto;"></div>
                    <a
                        href="/team-patina"
                        class="odp-btn odp-btn-text"
                        style="
                        margin-top: auto;
                        text-decoration: none;
                        "
                    >
                        {"Members + Contacts"}
                    </a>
                </div>
                <div
                    class="flex flex-col items-start h-full"
                    style="width: 320px; min-height: 350px; justify-content: flex-start;"
                >
                    <span class="h3 break-words w-full block text-left">
                        {"Secure Embedded Controller"}
                    </span>
                    <span class="p2 break-words w-full block text-left">
                        {"Developing and managing secure EC internals"}
                    </span>
                    <div style="flex: 1 1 auto;"></div>
                    <a
                        href="/team-ec"
                        class="odp-btn odp-btn-text"
                        style="
                        margin-top: auto;
                        text-decoration: none;
                        "
                    >
                        {"Members + Contacts"}
                    </a>
                </div>
                <div
                    class="flex flex-col items-start h-full"
                    style="width: 320px; min-height: 350px; justify-content: flex-start;"
                >
                    <span class="h3 break-words w-full block text-left">
                        {"Unified EC Services"}
                    </span>
                    <span class="p2 break-words w-full block text-left">
                        {"Designing and managing implementation of a unified EC Services interface"}
                    </span>
                    <div style="flex: 1 1 auto;"></div>
                    <a
                        href="team-ec-services"
                        class="odp-btn odp-btn-text"
                        style="
                        margin-top: auto;
                        text-decoration: none;
                        "
                    >
                        {"Members + Contacts"}
                    </a>
                </div>
            </div>
        </section>
    }
}
