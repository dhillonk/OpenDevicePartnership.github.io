use leptos::prelude::*;

#[component]
pub fn ProjectIntroduction(
    #[prop(into)] project_title: String,
    #[prop(into)] project_summary: String,
    #[prop(into)] project_what: String,
    #[prop(into)] project_why: String,
) -> impl IntoView {
    view! {
        <section
            class="background_primary"
            style="
                padding: 120px 120px 120px 0;
            "
        >
            <div class="flex flex-row gap-[80px]">
                {/* Left Column */}
                <div class="flex flex-col items-start" style="width: 1035px; height: 930px; position: relative; margin-left: 0; padding-left: 0;">
                    <div
                        style="
                            position: absolute;
                            top: 50%;
                            left: 50%;
                            transform: translate(-50%, -50%);
                            z-index: 10;
                            text-align: center;
                        "
                    >
                        <span
                            class="h1"
                            style="
                                display: block;
                                color: white;
                                text-shadow: 2px 2px 4px rgba(0,0,0,0.5);
                                margin-bottom: 10px;
                            "
                        >
                            {project_title}
                        </span>
                        <span
                            class="p1"
                            style="
                                display: block;
                                color: white;
                                text-shadow: 2px 2px 4px rgba(0,0,0,0.5);
                            "
                        >
                            {project_summary}
                        </span>
                    </div>
                    <picture style="width: 100%; height: 100%;">
                        <source srcset="/images/patina_header.svg" />
                        <img
                            src="/images/patina_header.svg"
                            alt="Patina Project"
                            class="icon"
                            style="
                                width: 100%;
                                height: 100%;
                                object-fit: contain;
                            "
                        />
                    </picture>
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
                        class="p1"
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
                        class="p1"
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
    }
}
