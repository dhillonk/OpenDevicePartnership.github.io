use crate::components::themed_icon::ThemedIcon;
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
        <section class="flex flex-col md:flex-row items-start background_primary w-full overflow-x-hidden px-4 py-8 md:py-16 md:px-32">
            <div class="flex flex-col items-start w-full" style="align-items: flex-start;">
                <ThemedIcon
                    name="documentation"
                    alt="Documentation Icon"
                    class="w-[80px] h-[80px] md:w-[150px] md:h-[150px] object-contain mb-4"
                    style="display: block; margin-bottom: 16px;"
                />
                <span class="h2_mobile md:h2 text-left break-words block">"Documentation"</span>
                <div style="height: 10px;"></div>
                <span class="p1_mobile md:p1 text-left break-words block">
                    "Start developing with ODP"
                </span>
            </div>
            <div class="hidden md:block" style="width: 200px;"></div>
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
                                            <div
                                                class="link_mobile md:link_large internal-link"
                                                style="text-decoration: none;"
                                            >
                                                <A href=link.href>
                                                    <span style="text-decoration: none;">{"→ "}</span>
                                                    <span style="text-decoration: underline;">
                                                        {link.title}
                                                    </span>
                                                </A>
                                            </div>
                                        }
                                    }
                                >
                                    <div
                                        class="link_mobile md:link_large external-link"
                                        style="text-decoration: none;"
                                    >
                                        <a
                                            href=link.href
                                            target="_blank"
                                            style="text-decoration: none;"
                                        >
                                            <span style="text-decoration: none;">{"→ "}</span>
                                            <span style="text-decoration: underline;">
                                                {link.title}
                                            </span>
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
