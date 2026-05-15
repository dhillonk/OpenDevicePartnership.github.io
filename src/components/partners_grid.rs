use leptos::prelude::*;

use crate::components::partner::Partner;
use crate::components::section::Section;
use crate::data::partners::PARTNERS;

#[component]
pub fn PartnersGrid() -> impl IntoView {
    view! {
        <Section class="py-20">
            <div class="mb-16">
                <span class="h1 break-words w-full block text-left">{"Our Partners"}</span>
            </div>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-10 md:gap-16 items-center justify-items-center">
                <For
                    each=|| PARTNERS.iter()
                    key=|partner| partner.name
                    children=|partner| {
                        view! { <Partner name=partner.name url=partner.url logo=partner.logo /> }
                    }
                />
            </div>
        </Section>
    }
}
