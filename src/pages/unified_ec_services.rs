use crate::components::documentation_training::DocumentationTraining;
use crate::components::page_layout::PageLayout;
use crate::components::project_introduction::ProjectIntroduction;
use crate::components::repo_view::RepositoryGraph;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn WindowsEcServices() -> impl IntoView {
    let project_title = "Unified Embedded Controller Interface";
    let project_summary = "";
    let project_what = r#"The Unified Windows EC Service interface defines runtime coordination between firmware components using async message-passing. 
    Each service manages a domain — like power, battery, or host communication — and exposes a structured protocol. 
    Components register with services and receive commands for events, capabilities, and state changes. 
    This model enables loosely coupled subsystems, observability, and test injection — without sacrificing platform coherence. Learn more about EC Services on our <a href="https://github.com/OpenDevicePartnership/haf-ec-service" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">GitHub repository</a> and check out the <a href="https://opendevicepartnership.github.io/documentation/guide/intro/concepts/EC_Services.html" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">Documentation</a> page."#;
    let project_why = "Without a common interface, EC firmware becomes tangled and brittle. 
    Unified EC Services -- designed for Windows Platforms -- bring structure and predictability by defining how components interact at runtime. 
    With async protocols and policy-aware life cycles, they support clean separation of concerns and cross-subsystem coordination. 
    Whether debugging power flows or integrating a new device, these services provide the glue, guardrails, and visibility you need.";

    let nodes_data = r#"[{"id": 0, "name": "ec-test-app", "url": "https://github.com/OpenDevicePartnership/ec-test-app", "classification": "app & driver", "order": 1}, {"id": 1, "name": "haf-ec-service", "url": "https://github.com/OpenDevicePartnership/haf-ec-service", "classification": "secure partition", "order": 2}, {"id": 2, "name": "ffa", "url": "https://github.com/OpenDevicePartnership/ffa", "classification": "ff-a", "order": 3}]"#;
    let links_data = r#"[{"source": 0, "target": 1}, {"source": 1, "target": 2}]"#;

    view! {
        <PageLayout scrollable_x=true>
            <ProjectIntroduction project_title=project_title project_summary=project_summary project_what=project_what project_why=project_why
                project_who="/team-ec-services"
                big_image_url="/images/ECServicesBackground.png"
                small_image_url="/images/dark/ProjectIcon_ES_Patina_DarkMode.svg" />
            <RepositoryGraph nodes=nodes_data links=links_data/>
            <DocumentationTraining />
        </PageLayout>
    }
}
