use crate::components::themed_icon::ThemedIcon;
use leptos::ev;
use leptos::prelude::RwSignal;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header(#[prop(optional, default = "header_background")] background_class: &'static str) -> impl IntoView {
    let menu_open = RwSignal::new(false);
    let close_menu = move || menu_open.set(false);

    // ESC closes the mobile menu when it is open.
    window_event_listener(ev::keydown, move |e| {
        if e.key() == "Escape" && menu_open.get_untracked() {
            menu_open.set(false);
        }
    });

    view! {
        <header class=format!(
            "w-full h-[80px] lg:h-[160px] px-4 sm:px-8 md:px-16 {} flex items-center justify-between z-50 m-0 p-0 relative",
            background_class,
        )>
            <div class="flex items-center space-x-6 flex-shrink-0">
                <ThemedIcon
                    name="odplogo"
                    alt="ODP Logo"
                    class="w-[120px] h-[41px] sm:w-[140px] sm:h-[48px] lg:w-[180px] lg:h-[62px] object-contain"
                />
            </div>

            <button
                class="lg:hidden flex flex-col justify-center items-center w-10 h-10 p-2 focus:outline-none"
                aria-label=move || if menu_open.get() { "Close menu" } else { "Open menu" }
                aria-expanded=move || if menu_open.get() { "true" } else { "false" }
                aria-controls="primary-mobile-nav"
                on:click=move |_| menu_open.update(|v| *v = !*v)
            >
                <span class="block w-6 h-0.5 bg-black dark:bg-white mb-1"></span>
                <span class="block w-6 h-0.5 bg-black dark:bg-white mb-1"></span>
                <span class="block w-6 h-0.5 bg-black dark:bg-white"></span>
            </button>

            <nav class="hidden lg:flex [column-gap:25px]" aria-label="Primary">
                <NavButton href="/getting-started" label="Getting Started" />
                <NavButton href="/projects" label="Projects" />
                <ExternalNavButton
                    href="https://opendevicepartnership.github.io/documentation/"
                    label="Library"
                />
                <NavButton href="/community" label="Community" />
                <NavButton href="/home" label="Home" />
            </nav>

            // Backdrop: catches clicks outside the open mobile menu and dismisses it.
            <div
                class="fixed inset-0 z-40 lg:hidden"
                style=move || { if menu_open.get() { "display: block;" } else { "display: none;" } }
                on:click=move |_| close_menu()
                aria-hidden="true"
            ></div>

            <nav
                id="primary-mobile-nav"
                aria-label="Primary"
                class="absolute right-0 top-full w-[80vw] max-w-xs background_primary flex-col items-end px-4 py-4 space-y-2 shadow-lg lg:hidden transition-all duration-200 z-50"
                style=move || if menu_open.get() { "display: flex;" } else { "display: none;" }
            >
                <NavButton
                    href="/getting-started"
                    label="Getting Started"
                    mobile=true
                    on_navigate=close_menu
                />
                <NavButton href="/projects" label="Projects" mobile=true on_navigate=close_menu />
                <ExternalNavButton
                    href="https://opendevicepartnership.github.io/documentation/"
                    label="Library"
                    mobile=true
                    on_navigate=close_menu
                />
                <NavButton href="/community" label="Community" mobile=true on_navigate=close_menu />
                <NavButton href="/home" label="Home" mobile=true on_navigate=close_menu />
            </nav>
        </header>
    }
}

#[component]
fn NavButton(
    href: &'static str,
    label: &'static str,
    #[prop(optional)] mobile: bool,
    #[prop(optional, into)] on_navigate: Option<Callback<()>>,
) -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let is_active = move || location.pathname.get().starts_with(href);

    view! {
        <A
            href=href
            class:odp-header-btn=true
            class:odp-header-btn-text=true
            class:odp-header-btn-active=is_active
            class:odp-header-btn-active-text=is_active
            class:w-full=mobile
            attr:aria-current=move || if is_active() { Some("page") } else { None }
            on:click=move |_| {
                if let Some(cb) = on_navigate {
                    cb.run(());
                }
            }
        >
            {label}
        </A>
    }
}

#[component]
fn ExternalNavButton(
    href: &'static str,
    label: &'static str,
    #[prop(optional)] mobile: bool,
    #[prop(optional, into)] on_navigate: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <a
            href=href
            class=format!(
                "odp-header-btn odp-header-btn-text{}",
                if mobile { " w-full" } else { "" },
            )
            target="_blank"
            rel="noopener noreferrer"
            on:click=move |_| {
                if let Some(cb) = on_navigate {
                    cb.run(());
                }
            }
        >
            {label}
        </a>
    }
}
