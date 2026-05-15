use crate::components::themed_icon::ThemedIcon;
use crate::components::ui::{Heading, HeadingLevel, Text, TextSize};
use leptos::prelude::*;
use leptos_router::components::A;

#[derive(Clone)]
pub struct DocLink {
    pub href: &'static str,
    pub title: &'static str,
    pub external: bool,
}

/// Canonical "Documentation" link list shown at the bottom of every
/// content page. Pages typically render `<DocumentationTraining />`
/// with no `links` prop so they pick up this default; pass an explicit
/// `links=...` only when a page needs a different set.
pub const DEFAULT_DOC_LINKS: &[DocLink] = &[
    DocLink {
        href: "https://opendevicepartnership.github.io/documentation/guide/why/why.html",
        title: "Why ODP?",
        external: true,
    },
    DocLink {
        href: "https://opendevicepartnership.github.io/documentation/guide/intro/getting_started.html",
        title: "Getting Started with ODP",
        external: true,
    },
    DocLink {
        href: "https://opendevicepartnership.github.io/documentation/guide/intro/welcome.html",
        title: "Tutorials",
        external: true,
    },
    DocLink {
        href: "https://opendevicepartnership.github.io/documentation/guide/specs/specifications.html",
        title: "Specifications",
        external: true,
    },
    DocLink {
        href: "/community",
        title: "Contributing to ODP",
        external: false,
    },
];

#[component]
pub fn DocumentationTraining(#[prop(default = DEFAULT_DOC_LINKS.to_vec())] links: Vec<DocLink>) -> impl IntoView {
    view! {
        <section class="flex flex-col md:flex-row items-start background_primary w-full overflow-x-hidden px-4 py-8 md:py-16 md:px-16 lg:px-32">
            <div class="flex flex-col items-start w-full">
                <ThemedIcon
                    name="documentation"
                    alt="Documentation Icon"
                    class="w-[80px] h-[80px] md:w-[150px] md:h-[150px] object-contain mb-4 block"
                />
                <Heading level=HeadingLevel::H2 class="text-left break-words">
                    "Documentation"
                </Heading>
                <div class="h-2.5"></div>
                <Text size=TextSize::Lead class="text-left break-words">
                    "Start developing with ODP"
                </Text>
            </div>
            <div class="hidden md:block w-[200px]"></div>
            <ul class="flex flex-col pt-4 w-full max-w-full break-words md:pt-4 md:w-[760px] md:max-w-[760px]">
                {links
                    .into_iter()
                    .map(|link| {
                        view! {
                            <li>
                                <Show
                                    when=move || link.external
                                    fallback=move || {
                                        view! {
                                            <div class="link_mobile md:link_large internal-link no-underline">
                                                <A href=link.href>
                                                    <span class="no-underline">{"→ "}</span>
                                                    <span class="underline">{link.title}</span>
                                                </A>
                                            </div>
                                        }
                                    }
                                >
                                    <div class="link_mobile md:link_large external-link no-underline">
                                        <a href=link.href target="_blank" class="no-underline">
                                            <span class="no-underline">{"→ "}</span>
                                            <span class="underline">{link.title}</span>
                                        </a>
                                    </div>
                                </Show>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </section>
    }
}
