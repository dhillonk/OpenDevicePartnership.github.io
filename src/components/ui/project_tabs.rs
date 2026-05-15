//! `<ProjectTabs>` -- horizontal tab strip for cross-project
//! navigation.
//!
//! Once a visitor is on `/boot-firmware`, the only way to reach
//! `/embedded-controller` or `/windows-ec-services` was to back out
//! to the projects index. This molecule renders a row of router
//! links to all three project pages, with the current page styled
//! as the active tab and marked with `aria-current="page"` for
//! assistive tech.
//!
//! Place near the top of each project page, immediately below the
//! `<ProjectIntroduction>` hero.

use crate::data::projects::{ProjectCopy, EC_SERVICES, EMBEDDED_CONTROLLER, PATINA};
use leptos::prelude::*;
use leptos_router::components::A;

/// Identifier for the active project tab.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProjectId {
    Patina,
    EmbeddedController,
    EcServices,
}

impl ProjectId {
    fn copy(self) -> &'static ProjectCopy {
        match self {
            ProjectId::Patina => &PATINA,
            ProjectId::EmbeddedController => &EMBEDDED_CONTROLLER,
            ProjectId::EcServices => &EC_SERVICES,
        }
    }
}

const ALL_PROJECTS: &[ProjectId] = &[ProjectId::Patina, ProjectId::EmbeddedController, ProjectId::EcServices];

/// Cross-project tab strip.
#[component]
pub fn ProjectTabs(active: ProjectId) -> impl IntoView {
    view! {
        <nav
            aria-label="Other ODP projects"
            class="background_secondary w-full px-4 sm:px-8 md:px-16 lg:px-32 py-4 md:py-6"
        >
            <div class="flex flex-col md:flex-row md:items-center gap-2 md:gap-6">
                <span class="mono text-left text-sm uppercase opacity-70">{"Other projects:"}</span>
                <ul class="flex flex-row flex-wrap gap-3 md:gap-6">
                    {ALL_PROJECTS
                        .iter()
                        .map(|id| {
                            let id = *id;
                            let project = id.copy();
                            let is_active = id == active;
                            view! { <ProjectTab project=project is_active=is_active /> }
                        })
                        .collect_view()}
                </ul>
            </div>
        </nav>
    }
}

#[component]
fn ProjectTab(project: &'static ProjectCopy, is_active: bool) -> impl IntoView {
    let class = tab_class(is_active);
    let aria_current = if is_active { Some("page") } else { None };
    view! {
        <li>
            <A href=project.route attr:class=class attr:aria-current=aria_current>
                {project.short_label}
            </A>
        </li>
    }
}

fn tab_class(active: bool) -> &'static str {
    if active {
        "link_mobile md:link font-bold underline aria-current-page"
    } else {
        "link_mobile md:link no-underline hover:underline"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tab_class_distinguishes_active() {
        assert!(tab_class(true).contains("font-bold"));
        assert!(tab_class(true).contains("underline"));
        assert!(!tab_class(false).contains("font-bold"));
    }

    #[test]
    fn project_id_copy_round_trips() {
        assert_eq!(ProjectId::Patina.copy().route, "/boot-firmware");
        assert_eq!(ProjectId::EmbeddedController.copy().route, "/embedded-controller");
        assert_eq!(ProjectId::EcServices.copy().route, "/windows-ec-services");
    }

    #[test]
    fn all_projects_covers_every_variant() {
        assert_eq!(ALL_PROJECTS.len(), 3);
    }
}
