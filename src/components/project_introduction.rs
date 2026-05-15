use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn ProjectIntroduction(
    #[prop(into)] project_title: String,
    #[prop(into)] project_summary: String,
    #[prop(into)] project_what: String,
    #[prop(into)] project_why: String,
    #[prop(into)] project_who: String,
    #[prop(into, optional)] small_image_url: String,
    #[prop(into, optional)] big_image_url: String,
) -> impl IntoView {
    view! {
        <section class="background_primary">
            <div class="flex flex-col md:flex-row gap-20">

                <div class="relative w-full max-w-[700px] aspect-square flex items-center justify-start flex-shrink-0">

                    <img
                        src=big_image_url
                        alt="Project Main"
                        class="w-full h-full object-cover block rounded-r-3xl"
                    />

                    <div class="absolute top-1/2 left-0 w-[90%] pl-2 md:pl-16 flex flex-col items-start -translate-y-1/2 z-[2] text-left">

                        <img
                            src=small_image_url
                            alt="Project Logo"
                            class="w-[48px] h-[48px] md:w-[102px] md:h-[102px] mb-4 md:mb-16 ml-0 object-contain"
                        />
                        <span class="h1 block !text-white mb-2.5 break-words text-left">
                            {project_title}
                        </span>
                        <span class="p1 block !text-white break-words text-left">
                            {project_summary}
                        </span>
                    </div>
                </div>
                <div class="flex flex-col items-start px-2 md:px-0 w-full">
                    <span class="mono block text-left">{"WHAT"}</span>
                    <span class="p2 block text-left" inner_html=project_what></span>
                    <span class="mono block text-left">{"WHY"}</span>
                    <span class="p2 block text-left">{project_why}</span>
                    <span class="mono block text-left">{"WHO"}</span>
                    <div class="link_mobile md:link block text-left">
                        <A href=project_who>{"Learn more about the team →"}</A>
                    </div>
                </div>
            </div>
        </section>
        <div class="background_primary px-2 md:px-0">
            <span
                class="p1"
                style="
                display: block;
                text-align: left;
                padding-left: 20px;
                padding-top: 100px;
                padding-bottom: 20px;
                "
            >
                Repository Diagram
            </span>
        </div>
    }
}
