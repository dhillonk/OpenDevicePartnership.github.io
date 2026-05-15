use leptos::prelude::*;

use crate::components::partner::Partner;

struct PartnerInfo {
    name: &'static str,
    url: &'static str,
    logo: &'static str,
}

const PARTNERS: &[PartnerInfo] = &[
    PartnerInfo {
        name: "Microsoft Surface",
        url: "https://microsoft.com/surface",
        logo: "/images/partners/microsoft_surface.svg",
    },
    PartnerInfo {
        name: "NXP",
        url: "https://nxp.com",
        logo: "/images/partners/nxp.svg",
    },
    PartnerInfo {
        name: "Tweede Golf",
        url: "https://tweedegolf.nl/en",
        logo: "/images/partners/tweede_golf.svg",
    },
    PartnerInfo {
        name: "CIX",
        url: "https://en.cixtech.com",
        logo: "/images/partners/cix.svg",
    },
];

#[component]
pub fn PartnersGrid() -> impl IntoView {
    view! {
        <section class="background_primary px-4 md:px-[120px] py-[80px]">
            <div class="mb-[60px]">
                <span class="h1_mobile md:h1 break-words w-full block text-left">
                    {"Our Partners"}
                </span>
            </div>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-[40px] md:gap-[60px] items-center justify-items-center">
                <For
                    each=|| PARTNERS.iter()
                    key=|partner| partner.name
                    children=|partner| {
                        view! { <Partner name=partner.name url=partner.url logo=partner.logo /> }
                    }
                />
            </div>
        </section>
    }
}
