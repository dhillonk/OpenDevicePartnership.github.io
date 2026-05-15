use crate::components::page_layout::PageLayout;
use crate::components::team_grid::{TeamGrid, TeamMember};
use crate::components::themed_icon::ThemedIcon;

use leptos::prelude::*;

fn create_team() -> Vec<TeamMember> {
    vec![
        TeamMember {
            first_name: "Jerry",
            last_name: "Xie",
            role: "Team leader",
            github_username: "jerrysxie",
            github_url: "https://github.com/jerrysxie",
            image_url: "https://github.com/jerrysxie.png?size=200",
        },
        TeamMember {
            first_name: "Felipe",
            last_name: "Balbi",
            role: "",
            github_username: "felipebalbi",
            github_url: "https://github.com/felipebalbi",
            image_url: "https://github.com/felipebalbi.png?size=200",
        },
        TeamMember {
            first_name: "Robert",
            last_name: "Zieba",
            role: "",
            github_username: "RobertZ2011",
            github_url: "https://github.com/RobertZ2011",
            image_url: "https://github.com/RobertZ2011.png?size=200",
        },
        TeamMember {
            first_name: "Matteo",
            last_name: "Tullo",
            role: "",
            github_username: "tullom",
            github_url: "https://github.com/tullom",
            image_url: "https://github.com/tullom.png?size=200",
        },
        TeamMember {
            first_name: "Kurtis",
            last_name: "Dinelle",
            role: "",
            github_username: "kurtjd",
            github_url: "https://github.com/kurtjd",
            image_url: "https://github.com/kurtjd.png?size=200",
        },
        TeamMember {
            first_name: "Jimi",
            last_name: "Huard",
            role: "",
            github_username: "JamesHuard",
            github_url: "https://github.com/JamesHuard",
            image_url: "https://github.com/JamesHuard.png?size=200",
        },
        TeamMember {
            first_name: "Adam",
            last_name: "Sasine",
            role: "",
            github_username: "asasine",
            github_url: "https://github.com/asasine",
            image_url: "https://github.com/asasine.png?size=200",
        },
        TeamMember {
            first_name: "Billy",
            last_name: "Price",
            role: "",
            github_username: "williampMSFT",
            github_url: "https://github.com/williampMSFT",
            image_url: "https://github.com/williampMSFT.png?size=200",
        },
    ]
}

#[component]
pub fn TeamEC() -> impl IntoView {
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
                    <div class="flex flex-col items-start w-full md:w-[700px]">
                        <span class="h1 block text-left">{"Meet the team"}</span>
                    </div>
                    <div class="flex flex-col items-start w-full md:w-[600px]">
                        <span class="mono block text-left">{"Secure EC team"}</span>
                        <span class="p1_mobile md:p1 block text-left">
                            {"Developing and managing secure EC internals"}
                        </span>
                    </div>
                </div>
            </div>
            <TeamGrid members=team />
        </PageLayout>
    }
}
