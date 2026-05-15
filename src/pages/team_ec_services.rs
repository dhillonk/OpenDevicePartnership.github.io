use crate::components::page_layout::PageLayout;
use crate::components::team_grid::{TeamGrid, TeamMember};
use crate::components::themed_icon::ThemedIcon;

use leptos::prelude::*;

fn create_team() -> Vec<TeamMember> {
    vec![
        TeamMember {
            first_name: "Phil",
            last_name: "Weber",
            role: "Team leader",
            github_username: "philgweber",
            github_url: "https://github.com/philgweber",
            image_url: "https://github.com/philgweber.png?size=200",
        },
        TeamMember {
            first_name: "Dylan",
            last_name: "Knutson",
            role: "Team leader",
            github_username: "dymk",
            github_url: "https://github.com/dymk",
            image_url: "https://github.com/dymk.png?size=200",
        },
    ]
}

#[component]
pub fn TeamECServices() -> impl IntoView {
    let team = create_team();

    view! {
        <PageLayout>
            <div class="background_primary px-2 md:px-32 py-4 md:py-32" style="position: relative;">

                <div class="block md:hidden mb-4">
                    <a href="javascript:history.back()" class="block m-0 p-0">
                        <ThemedIcon
                            name="backbutton"
                            alt="Back"
                            style="margin: 0; padding: 0; display: block;"
                        />
                    </a>
                </div>
                <div class="hidden md:block" style="position: absolute; left: 0; top: 0;">
                    <a href="javascript:history.back()" class="block m-0 p-0">
                        <ThemedIcon
                            name="backbutton"
                            alt="Back"
                            style="margin: 0; padding: 0; display: block;"
                        />
                    </a>
                </div>
                <div class="flex flex-col md:flex-row gap-20 items-start">
                    <div class="flex flex-col items-start w-full md:flex-1">
                        <span class="h1 block text-left">{"Meet the team"}</span>
                    </div>
                    <div class="flex flex-col items-start w-full md:flex-1">
                        <span class="mono block text-left">{"Unified EC services team"}</span>
                        <span class="p1 block text-left">
                            {"Designing and managing implementation of a unified EC Services interface"}
                        </span>
                    </div>
                </div>
            </div>
            <TeamGrid members=team />
        </PageLayout>
    }
}
