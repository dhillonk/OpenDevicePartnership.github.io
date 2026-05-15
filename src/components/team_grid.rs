use crate::components::ui::{Heading, HeadingLevel, Text, TextSize};
use leptos::html::*;
use leptos::prelude::*;

#[derive(Clone)]
pub struct TeamMember {
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub role: &'static str,
    pub github_username: &'static str,
    pub github_url: &'static str,
    pub image_url: &'static str,
}

#[component]
pub fn TeamGrid(#[prop(into)] members: Vec<TeamMember>) -> impl IntoView {
    view! {
        <div class="background_primary">
            <div class="grid-container px-4 sm:px-8 md:px-16 lg:px-32 pb-32">
                {members
                    .into_iter()
                    .map(|member| {
                        let full_name = format!("{} {}", member.first_name, member.last_name);
                        view! {
                            <div>
                                <img
                                    class="member-image"
                                    src=member.image_url
                                    alt="Profile Picture"
                                />
                                <Heading level=HeadingLevel::H3>{full_name}</Heading>
                                <Text size=TextSize::Large>{member.role}</Text>
                                <Text size=TextSize::Large>
                                    {"GitHub: "}
                                    <a class="link" href=member.github_url target="_blank">
                                        {member.github_username}
                                    </a>
                                </Text>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
