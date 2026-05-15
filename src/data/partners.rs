//! Partner organisations rendered on the home page.

pub struct PartnerInfo {
    pub name: &'static str,
    pub url: &'static str,
    pub logo: &'static str,
}

pub const PARTNERS: &[PartnerInfo] = &[
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
