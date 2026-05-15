use crate::components::page_layout::PageLayout;
use crate::components::team_grid::TeamGrid;
use crate::components::themed_icon::ThemedIcon;
use crate::data::teams::ec_team;

use leptos::prelude::*;

#[component]
pub fn TeamEC() -> impl IntoView {
    let team = ec_team();

    view! {
        <PageLayout>
            <div
                class="background_primary px-4 sm:px-8 md:px-16 lg:px-32 py-4 md:py-32"
                style="position: relative;"
            >

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
                        <span class="mono block text-left">{"Secure EC team"}</span>
                        <span class="p1 block text-left">
                            {"Developing and managing secure EC internals"}
                        </span>
                    </div>
                </div>
            </div>
            <TeamGrid members=team />
        </PageLayout>
    }
}
