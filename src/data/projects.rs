//! Per-project marketing copy (title, summary, "what", "why") and the
//! associated repository graph data.
//!
//! Project copy stays in Rust so we can reference it as `&'static str`
//! in components. Graph data lives in `data/graphs/*.json` and is
//! pulled in via `include_str!` -- no JSON parsing in the wasm
//! bundle, the strings are passed straight to the D3 renderer.

pub struct ProjectCopy {
    pub title: &'static str,
    pub summary: &'static str,
    pub what: &'static str,
    pub why: &'static str,
    pub team_route: &'static str,
    pub big_image_url: &'static str,
    pub small_image_url: &'static str,
    pub nodes_json: &'static str,
    pub links_json: &'static str,
}

pub const PATINA: ProjectCopy = ProjectCopy {
    title: "Patina",
    summary: "Modern Boot Firmware",
    what: r#"Patina is a UEFI compatible firmware interface written in the Rust language with a focus on memory safety and composition. For Patina, we re-evaluated the good and the bad from today's UEFI boot firmware and used this opportunity to embrace new language capabilities, software architecture, programming paradigms, and industry supported tooling. Patina isn't designed to replace everything necessary for system boot but instead to provide a sustainable path forward with high return on investment. Learn more about Patina on our <a href="https://github.com/OpenDevicePartnership/patina" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">GitHub repository</a> and check out the <a href="https://opendevicepartnership.github.io/patina/" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">Documentation</a> page."#,
    why: "A lot has changed in the last quarter century. UEFI boot firmware has scaled remarkably well, seamlessly ushering in new generations of hardware for PCs, but as active maintainers of UEFI we know the systemic problems that can’t be addressed without significant change. We understand the challenges of supporting a vast, diverse ecosystem of hardware devices as unique as each user.  We see the nuance in our industry's partnerships and the supply chains critical for their success.  For this reason, we started the Patina project to build a future and a coalition ready for the next set of challenges.",
    team_route: "/team-patina",
    big_image_url: "/images/PatinaBackground.png",
    small_image_url: "/images/dark/ProjectIcon_P_Patina_DarkMode.svg",
    nodes_json: include_str!("../../data/graphs/patina_nodes.json"),
    links_json: include_str!("../../data/graphs/patina_links.json"),
};

pub const EMBEDDED_CONTROLLER: ProjectCopy = ProjectCopy {
    title: "Secure Embedded Controller",
    summary: "A hardened firmware platform for modern embedded controllers",
    what: r#"The ODP Secure EC stack is a Rust-based firmware platform for modern embedded controllers, supporting both discrete and integrated ECs.

It provides modular subsystems for power sequencing, thermal policy, event routing, and more. 
Components are defined by traits, composed into devices, and managed by a shared runtime that drives platform behavior.
Built for portability and testability, it supports both std and no-std builds and integrates cleanly with real-time runtimes like Embassy. Learn more about Secure EC on our <a href="https://github.com/OpenDevicePartnership/embedded-services" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">GitHub repository</a> and check out the <a href="https://opendevicepartnership.github.io/documentation/tracks/embedded_controller/track_overview.html" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">Documentation</a> page."#,
    why: "Embedded Controllers do more than ever — yet many EC stacks are stuck in the past.
The ODP EC firmware rethinks the EC as a secure, modular orchestrator for power, telemetry, and system policy. With clearly scoped components and Rust’s safety guarantees, it helps you move faster, catch bugs earlier, and support diverse platforms with confidence.
It’s a modern foundation for building reliable, adaptable EC firmware — not just patching legacy code.",
    team_route: "/team-ec",
    big_image_url: "/images/ECBackground.png",
    small_image_url: "/images/dark/ProjectIcon_EC_Patina_DarkMode.svg",
    nodes_json: include_str!("../../data/graphs/ec_nodes.json"),
    links_json: include_str!("../../data/graphs/ec_links.json"),
};

pub const EC_SERVICES: ProjectCopy = ProjectCopy {
    title: "Unified Embedded Controller Interface",
    summary: "",
    what: r#"The Unified Windows EC Service interface defines runtime coordination between firmware components using async message-passing. 
    Each service manages a domain — like power, battery, or host communication — and exposes a structured protocol. 
    Components register with services and receive commands for events, capabilities, and state changes. 
    This model enables loosely coupled subsystems, observability, and test injection — without sacrificing platform coherence. Learn more about EC Services on our <a href="https://github.com/OpenDevicePartnership/haf-ec-service" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">GitHub repository</a> and check out the <a href="https://opendevicepartnership.github.io/documentation/guide/intro/concepts/EC_Services.html" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">Documentation</a> page."#,
    why: "Without a common interface, EC firmware becomes tangled and brittle. 
    Unified EC Services -- designed for Windows Platforms -- bring structure and predictability by defining how components interact at runtime. 
    With async protocols and policy-aware life cycles, they support clean separation of concerns and cross-subsystem coordination. 
    Whether debugging power flows or integrating a new device, these services provide the glue, guardrails, and visibility you need.",
    team_route: "/team-ec-services",
    big_image_url: "/images/ECServicesBackground.png",
    small_image_url: "/images/dark/ProjectIcon_ES_Patina_DarkMode.svg",
    nodes_json: include_str!("../../data/graphs/ec_services_nodes.json"),
    links_json: include_str!("../../data/graphs/ec_services_links.json"),
};
