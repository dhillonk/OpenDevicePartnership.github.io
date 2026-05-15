use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::ui::AnnouncementCard;
use crate::data::announcements::{index_of, ANNOUNCEMENTS};
use leptos::prelude::Effect;
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};

// Welcome Patina Press Release
fn patina_press_release() -> impl IntoView {
    view! {
        <div>
            <p class="mb-4">
                <strong>"October 7, 2025 – Redmond, WA"</strong>
                " – The "
                <strong>"Open Device Partnership (ODP)"</strong>
                " is announcing "
                <strong>"Patina"</strong>
                ", a new open-source firmware project is public, and details will be shared at the upcoming "
                <a
                    href="https://uefi.org/events/uefi-2025-developers-conference-and-plugfest"
                    class="underline hover:text-blue-600 transition-colors duration-150"
                >
                    "UEFI 2025 Developer Conference & Plugfest"
                </a>
                ", October 7–10 in Sunnyvale, California. Patina is a Rust-based, UEFI-compatible firmware designed for memory safety and to address long standing challenges in the PC firmware ecosystem. It reimagines system firmware development to meet the evolving needs of modern hardware, software development lifecycles, supply chains, and industry collaboration. Patina joins a growing portfolio of ODP projects aimed at building a secure, modern foundation for device enablement. To learn more about Patina, please visit the project page and documentation here: "
                <a
                    href="https://opendevicepartnership.github.io/patina"
                    class="underline hover:text-blue-600 transition-colors duration-150"
                >
                    "Patina Documentation"
                </a>
                "."
            </p>

            <p class="mb-4">
                "ODP is an industry-wide, open-source initiative focused on advancing "
                <strong>"security, fundamentals, and standardization"</strong>
                " in device software. The partnership's work addresses long-standing challenges in firmware and system design by leveraging "
                <strong>"memory-safe programming languages like Rust"</strong> " and "
                <strong>"hardware-rooted security features"</strong>
                " and doing so based on standards that will work across a partner's entire device product line. This approach reduces exposure to common vulnerabilities while providing manufacturers with a sustainable, consistent foundation that lowers engineering costs across product lines."
            </p>

            <p class="mb-4">
                "In addition to Patina, ODP is currently focused on three other major projects:"
            </p>
            <ul class="list-disc pl-12 mb-4">
                <li class="mb-2">
                    <strong>"Secure EC firmware"</strong>
                    ", providing a modern, security-focused embedded controller implementation designed to eliminate classes of bugs prevalent in legacy EC codebases."
                </li>
                <li class="mb-2">
                    <strong>"Unified OS-EC service interface"</strong>
                    ", ensuring that operating systems can interact with embedded controllers in a consistent, well-defined way across devices."
                </li>
                <li class="mb-2">
                    <strong>"MPTF"</strong>
                    ", extending recent advancements in the Windows power-thermal framework to meet partner needs."
                </li>
            </ul>
            <p class="mb-4">
                "Together, these efforts give hardware makers the ability to standardize firmware and device software across their entire portfolios—improving reliability, accelerating time-to-market, and reducing redundant engineering work."
            </p>

            <p class="mb-4">
                "The Open Device Partnership is about building trust at the foundation. By focusing on memory safety, hardware-rooted security, and clear standards, we're making it easier for partners to deliver secure, consistent solutions while also reducing long-term development costs."
            </p>

            <p class="mb-4">
                "With Patina's public launch and the ongoing progress of ODP's other projects, the initiative is now seeking broader industry participation. ODP invites partners, contributors, and stakeholders to join in shaping the future of secure, open device enablement."
            </p>

            <p class="mb-4">
                "👉 Learn more and get involved at "
                <a
                    href="https://opendevicepartnership.org/"
                    class="underline hover:text-blue-600 transition-colors duration-150"
                >
                    "opendevicepartnership.org"
                </a>"."
            </p>
        </div>
    }
}

/// Map an announcement slug to its rendered detail content.
fn render_content(slug: &str) -> AnyView {
    match slug {
        "welcome-patina-announcement" => patina_press_release().into_any(),
        _ => view! { <p>"Content not found"</p> }.into_any(),
    }
}

/// Parse the `id=` query parameter (without depending on a URL parser).
/// Returns the slug substring up to the next `&`, if any.
fn slug_from_query(search: &str) -> Option<&str> {
    let id_start = search.find("id=")?;
    let rest = &search[id_start + 3..];
    Some(match rest.find('&') {
        Some(end) => &rest[..end],
        None => rest,
    })
}

#[component]
pub fn AnnouncementsPage() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();

    let (selected, set_selected) = signal(0);

    // Sync the selected index from the `?id=slug` query param.
    {
        let location = location.clone();
        Effect::new(move |_| {
            let search = location.search.get();
            if let Some(slug) = slug_from_query(&search) {
                if let Some(idx) = index_of(slug) {
                    set_selected.set(idx);
                }
            }
        });
    }

    view! {
        <div class="flex flex-col w-full min-h-screen background_quaternary">
            <Header background_class="background_quaternary" />
            <div class="h1 px-4 md:px-10 pt-4 md:pt-20 pb-4 md:pb-20">Announcements</div>
            <div class="flex flex-col lg:flex-row w-full flex-1 relative">
                <div class="w-full lg:w-[450px] xl:w-[500px] min-h-[200px] lg:min-h-[600px] xl:min-h-[700px] overflow-y-auto background_tertiary z-10 p-2 md:p-4 lg:p-6 mb-4 lg:mb-0">
                    <ul class="space-y-2 md:space-y-4">
                        {ANNOUNCEMENTS
                            .iter()
                            .enumerate()
                            .map(|(i, a)| {
                                let navigate = navigate.clone();
                                let slug = a.slug;
                                view! {
                                    <li>
                                        <button
                                            class="link_mobile md:link w-full text-left p-2 md:p-3"
                                            on:click=move |_| {
                                                set_selected.set(i);
                                                navigate(
                                                    &format!("/announcements?id={}", slug),
                                                    Default::default(),
                                                );
                                            }
                                        >
                                            {a.link_label}
                                        </button>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </ul>
                </div>
                <div class="flex-1 min-h-[400px] lg:min-h-[600px] xl:min-h-[700px] background_primary rounded-tl-[20px] md:rounded-tl-[30px] lg:rounded-tl-[50px] -ml-0 lg:-ml-16 z-20 overflow-y-auto p-4 md:p-6 lg:p-10">
                    {move || {
                        let idx = selected.get();
                        if let Some(a) = ANNOUNCEMENTS.get(idx) {
                            view! {
                                <AnnouncementCard title=a
                                    .title
                                    .to_string()>{render_content(a.slug)}</AnnouncementCard>
                            }
                                .into_any()
                        } else {
                            view! {
                                <AnnouncementCard title="No announcement selected".to_string()>
                                    <p>{""}</p>
                                </AnnouncementCard>
                            }
                                .into_any()
                        }
                    }}
                </div>
            </div>
            <Footer />
        </div>
    }
}
