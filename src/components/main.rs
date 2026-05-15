use crate::components::themed_icon::ThemedIcon;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Main() -> impl IntoView {
    view! {
        <main class="background_primary">
            <div class="mx-auto flex flex-col lg:flex-row items-start justify-between w-full px-2 sm:px-4">
                <div class="pl-0 lg:pl-32 flex flex-col gap-6 w-full lg:w-auto">
                    <h1 class="h1 py-4 w-full max-w-full text-left break-words">
                        "Building the Future of Trusted System Software Together"
                    </h1>
                    <p class="p1 w-full max-w-full text-left break-words">
                        "Leading technology partners creating secure, reusable, and reliable firmware for modern client devices."
                    </p>
                </div>

                <div class="flex flex-col w-full lg:w-auto mt-4 lg:mt-0">
                    <div
                        style="border: none; text-decoration: none;"
                        class="flex background_secondary w-full lg:w-[478px] h-[120px] lg:h-[176px] items-center justify-center px-4 lg:px-16"
                    >
                        <A href="/getting-started">
                            <div class="flex flex-row items-center justify-center gap-4 w-full max-w-full">
                                <span class="h3">"Getting started"</span>
                                <span class="h3">"→"</span>
                            </div>
                        </A>
                    </div>

                    <div
                        style="border: none; text-decoration: none;"
                        class="flex background_tertiary w-full lg:w-[478px] h-[120px] lg:h-[176px] items-center justify-center px-4 lg:px-16"
                    >
                        <A href="/projects">
                            <div class="flex flex-row items-center justify-center gap-4 w-full max-w-full">
                                <span class="h3">"Projects"</span>
                                <span class="h3">"→"</span>
                            </div>
                        </A>
                    </div>
                </div>
            </div>

            <div class="flex flex-col pt-10 px-2 sm:px-4 md:pl-[117px] w-full">
                <div class="flex flex-col lg:flex-row items-start w-full gap-4">
                    <div class="flex flex-col items-start w-full lg:flex-1 mr-0 lg:mr-16 mb-6 lg:mb-0">
                        <ThemedIcon
                            name="video"
                            alt="Video Icon"
                            style="
                            width: 150px;
                            height: 150px;
                            padding: 0;
                            object-fit: contain;
                            display: block;
                            margin-bottom: 16px;
                            "
                        />
                        <span class="h2 block w-full max-w-full text-left break-words">
                            "Welcome"
                        </span>
                        <div style="height: 10px;"></div>
                        <span class="p1 block w-full max-w-full text-left break-words">
                            "Learn how ODP projects help build secure, modern devices"
                        </span>
                    </div>
                    <div
                        class="w-full lg:flex-[2] aspect-video rounded-lg overflow-hidden"
                        style="max-width:100vw;"
                    >
                        <iframe
                            class="w-full h-full"
                            style="border-radius: 10px; display: block;"
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
