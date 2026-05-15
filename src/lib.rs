use crate::components::site_shell::{SiteShell, SiteShellScrollable};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod components;
mod data;
mod pages;

// Top-Level pages
use crate::pages::announcements::AnnouncementsPage;
use crate::pages::boot_firmware::BootFirmware;
use crate::pages::community::Community;
use crate::pages::embedded_controller::EmbeddedController;
use crate::pages::getting_started::GettingStarted;
use crate::pages::home::Home;
use crate::pages::projects::Projects;
use crate::pages::team_ec::TeamEC;
use crate::pages::team_ec_services::TeamECServices;
use crate::pages::team_patina::TeamPatina;
use crate::pages::unified_ec_services::WindowsEcServices;

/// An app router which renders the homepage
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
        <Stylesheet id="leptos" href="/style/output.css" />

        // sets the document title
        <Title text="Open Device Partnership" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <div class="min-h-screen w-full">
            <Router base="/">
                <Routes fallback=|| view! { NotFound }>
                    // Pages whose content always fits the viewport horizontally
                    // share the standard chrome with `overflow-x: hidden`.
                    <ParentRoute path=path!("") view=SiteShell>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/community") view=Community />
                        <Route path=path!("/team-ec") view=TeamEC />
                        <Route path=path!("/team-ec-services") view=TeamECServices />
                        <Route path=path!("/team-patina") view=TeamPatina />
                    </ParentRoute>

                    // Pages that may overflow horizontally (the project pages
                    // embed the wide repository graph SVG, the projects index
                    // and the getting-started page have wide hero images) use
                    // a chrome variant with `overflow-x: auto`.
                    <ParentRoute path=path!("") view=SiteShellScrollable>
                        <Route path=path!("/getting-started") view=GettingStarted />
                        <Route path=path!("/boot-firmware") view=BootFirmware />
                        <Route path=path!("/embedded-controller") view=EmbeddedController />
                        <Route path=path!("/windows-ec-services") view=WindowsEcServices />
                        <Route path=path!("/projects") view=Projects />
                    </ParentRoute>

                    // Announcements brings its own chrome (different
                    // background colour and a custom Header variant) so it
                    // stays a top-level route.
                    <Route path=path!("/announcements") view=AnnouncementsPage />
                </Routes>
            </Router>
        </div>
    }
}
