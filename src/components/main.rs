use crate::components::themed_icon::ThemedIcon;
use crate::components::ui::{Heading, HeadingLevel, Text, TextSize};
use leptos::prelude::*;

#[component]
pub fn Main() -> impl IntoView {
    view! {
        <main class="background_primary">
            <div class="mx-auto flex flex-col items-start w-full px-2 sm:px-4">
                <div class="pl-0 lg:pl-32 flex flex-col gap-6 w-full">
                    <h1 class="h1 py-4 w-full max-w-full text-left break-words">
                        "Building the Future of Trusted System Software Together"
                    </h1>
                    <p class="p1 w-full max-w-full text-left break-words">
                        "Leading technology partners creating secure, reusable, and reliable firmware for modern client devices."
                    </p>
                </div>
            </div>

            <div class="flex flex-col pt-10 px-2 sm:px-4 md:pl-[117px] w-full">
                <div class="flex flex-col lg:flex-row items-start w-full gap-4">
                    <div class="flex flex-col items-start w-full lg:flex-1 mr-0 lg:mr-16 mb-6 lg:mb-0">
                        <ThemedIcon
                            name="video"
                            alt="Video Icon"
                            class="w-[150px] h-[150px] object-contain block mb-4"
                        />
                        <Heading
                            level=HeadingLevel::H2
                            class="w-full max-w-full text-left break-words"
                        >
                            "Welcome"
                        </Heading>
                        <div class="h-2.5"></div>
                        <Text size=TextSize::Lead class="w-full max-w-full text-left break-words">
                            "Learn how ODP projects help build secure, modern devices"
                        </Text>
                    </div>
                    <div class="w-full lg:flex-[2] aspect-video rounded-lg overflow-hidden max-w-screen">
                        <iframe
                            class="w-full h-full block rounded-[10px]"
                            src="https://www.youtube.com/embed/FMlPxYSY1LM?rel=0"
                            title="YouTube Video of the Open Device Partnership"
                            frameborder="0"
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                            allowfullscreen
                        ></iframe>
                    </div>
                </div>
            </div>
        </main>
    }
}
