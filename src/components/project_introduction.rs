use leptos::prelude::*;

#[component]
pub fn ProjectIntroduction(
    #[prop(into)] project_title: String,
    #[prop(into)] project_summary: String,
    #[prop(into)] project_what: String,
    #[prop(into)] project_why: String,
    #[prop(into, optional)] small_image_url: String,
    #[prop(into, optional)] big_image_url: String,
) -> impl IntoView {
    view! {
        <section class="background_primary">
            <div class="flex flex-row gap-[80px]">
                {/* Left Column: Big Picture with Overlayed Text and Small Image */}
                <div
                    class="relative"
                    style="
                        width: 1035px;
                        height: 930px;
                        margin-left: 0;
                        padding-left: 0;
                        flex-shrink: 0;
                        position: relative;
                        display: flex;
                        align-items: center;
                        justify-content: flex-start;
                    "
                >
                    {/* Big Picture */}
                    <img
                        src={big_image_url}
                        alt="Project Main"
                        style="
                            width: 1035px;
                            height: 930px;
                            object-fit: cover;
                            display: block;
                            border-top-right-radius: 100px;
                            border-bottom-right-radius: 100px;
                        "
                    />
                    {/* Overlayed Text and Small Picture */}
                    <div
                        style="
                            position: absolute;
                            top: 50%;
                            left: 0;
                            transform: translateY(-50%);
                            z-index: 2;
                            text-align: left;
                            width: 90%;
                            padding-left: 60px;
                            display: flex;
                            flex-direction: column;
                            align-items: flex-start;
                        "
                    >
                        {/* Small Picture aligned with project title and 60px above */}
                        <img
                            src={small_image_url}
                            alt="Project Logo"
                            style="
                                width: 102px;
                                height: 102px;
                                object-fit: contain;
                                margin-bottom: 60px;
                                margin-left: 0;
                            "
                        />
                        <span
                            class="h1"
                            style="
                                display: block;
                                color: white;
                                margin-bottom: 10px;
                                word-break: break-word;
                                text-align: left;
                            "
                        >
                            {project_title}
                        </span>
                        <span
                            class="p1"
                            style="
                                display: block;
                                color: white;
                                word-break: break-word;
                                text-align: left;
                            "
                        >
                            {project_summary}
                        </span>
                    </div>
                </div>
                {/* Right Column */}
                <div class="flex flex-col items-start" style="width: 600px;">
                    {/* WHAT label */}
                    <span
                        class="mono"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"WHAT"}
                    </span>
                    {/* WHAT description */}
                    <span
                        class="p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {project_what}
                    </span>
                    {/* WHY label */}
                    <span
                        class="mono"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"WHY"}
                    </span>
                    {/* WHY description */}
                    <span
                        class="p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {project_why}
                    </span>
                </div>
            </div>
        </section>
        <div class="background_primary">
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
