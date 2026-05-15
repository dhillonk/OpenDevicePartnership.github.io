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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partners_list_well_formed() {
        assert!(!PARTNERS.is_empty(), "PARTNERS must not be empty");
        for partner in PARTNERS {
            assert!(!partner.name.is_empty(), "partner name must not be empty");
            assert!(
                partner.url.starts_with("https://"),
                "partner.url must be https: {:?}",
                partner.url
            );
            assert!(
                partner.logo.starts_with("/images/partners/"),
                "partner.logo must live under /images/partners/: {:?}",
                partner.logo
            );
        }
    }
}
