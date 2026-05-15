//! Landing page section: "Value Proposition" -- three icon cards.

use crate::components::section::{Section, Surface};
use crate::components::ui::{Heading, HeadingLevel, ValuePropCard};
use leptos::prelude::*;

#[component]
pub fn ValuePropositionSection() -> impl IntoView {
    view! {
        <Section surface=Surface::Secondary class="py-20">
            <div>
                <Heading level=HeadingLevel::H2 class="text-left">
                    {"Value Proposition"}
                </Heading>
                <div class="flex flex-col md:flex-row gap-16">
                    <ValuePropCard
                        icon="lock"
                        icon_alt="Security Icon"
                        title="Enhanced Security"
                        body="Security threats continue to evolve. ODP takes a proactive approach: reducing attack surfaces, using secure hardware features, leveraging the memory-safe Rust language, and designing every component with security-first principles."
                    />
                    <ValuePropCard
                        icon="checkcircle"
                        icon_alt="Interoperability Icon"
                        title="Standardization"
                        body="Many device firmware components are 'invisible plumbing' - necessary but costly to build and maintain. ODP's standards-based approach simplifies this infrastructure, maximizing reuse across devices, architectures (x86 and ARM), and generations."
                    />
                    <ValuePropCard
                        icon="fastforward"
                        icon_alt="Innovation Icon"
                        title="Accelerated Development"
                        body="Open collaboration means sharing solutions, reducing duplicated work, and speeding up the development of high-quality products."
                    />
                </div>
            </div>
        </Section>
    }
}
