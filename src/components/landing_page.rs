use crate::components::image_button::ImageButton;
use leptos::prelude::*;

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <div
            class="background_primary"
            style="
                width: auto;
                height: auto;
                padding: 120px;
            "
        >
            <div class="flex flex-row gap-[80px] items-start">
                {/* Left Column: h1 */}
                <div class="flex flex-col items-start flex-shrink-0" style="width: 800px">
                    <span
                        class="h1"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"An Open Collaboration for Secure, Modern Devices"}
                    </span>
                </div>
                {/* Right Column: Text */}
                <div class="flex flex-col items-start flex-shrink min-w-0" style="width: 650px;">
                    <span
                        class="p1"
                        style="
                            display: block;
                            text-align: left;
                            word-break: break-word;
                            overflow-wrap: break-word;
                        "
                    >
                        {"The Open Device Partnership (ODP) is a global initiative to make it easier for developers and device makers to build secure, efficient, and reliable client devices for cross-platform needs and certified environments."}
                        <br /><br />
                        {"By combining open standards with collaborative development practices, ODP reduces complexity, improves security, and accelerates innovation across diverse silicon and hardware platforms."}
                    </span>
                </div>
            </div>
        </div>

        // Value Proposition Section
        <section
            class="background_secondary"
            style="
                padding: 80px 120px;
            "
        >
            <div>
                <h2
                    class="h2"
                    style="
                        text-align: left;
                    "
                >
                    {"Value Proposition"}
                </h2>
                <div class="flex flex-row gap-[60px]">
                    {/* Column 1 */}
                    <div class="flex flex-col items-start" style="width: 400px;">
                        <picture>
                            <source srcset="/images/dark/lock.svg" media="(prefers-color-scheme: dark)" />
                            <img
                                src="/images/light/lock.svg"
                                alt="Security Icon"
                                class="icon"
                            />
                        </picture>
                        <span
                            class="h3"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Enhanced Security"}
                        </span>
                        <span
                            class="p2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Security threats continue to evolve. ODP takes a proactive approach: reducing attack surfaces, using secure hardware features, leveraging the memory-safe Rust language, and designing every component with security-first principles."}
                        </span>
                    </div>
                    {/* Column 2 */}
                    <div class="flex flex-col items-start" style="width: 400px;">
                        <picture>
                            <source srcset="/images/dark/checkcircle.svg" media="(prefers-color-scheme: dark)" />
                            <img
                                src="/images/light/checkcircle.svg"
                                alt="Interoperability Icon"
                                class="icon"
                            />
                        </picture>
                        <span
                            class="h3"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Standardization"}
                        </span>
                        <span
                            class="p2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {r#"Many device firmware components are "invisible plumbing" — necessary but costly to build and maintain. ODP's standards-based approach simplifies this infrastructure, maximizing reuse across devices, architectures (x86 and ARM), and generations."#}
                        </span>
                    </div>
                    {/* Column 3 */}
                    <div class="flex flex-col items-start" style="width: 400px;">
                        <picture>
                            <source srcset="/images/dark/fastforward.svg" media="(prefers-color-scheme: dark)" />
                            <img
                                src="/images/light/fastforward.svg"
                                alt="Innovation Icon"
                                class="icon"
                            />
                        </picture>
                        <span
                            class="h3"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Accelerated Development"}
                        </span>
                        <span
                            class="p2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Open collaboration means sharing solutions, reducing duplicated work, and speeding up the development of high-quality products."}
                        </span>
                    </div>
                </div>
            </div>
        </section>

        // ODP Projects Section
        <section
            class="background_primary"
            style="
                padding: 120px 0;
            "
        >
            <div style="max-width: 960px; margin-left: 120px;">
                <h2
                    class="h2"
                    style="
                        text-align: left;
                    "
                >
                    {"ODP Projects"}
                </h2>
                <p
                    class="p2"
                    style="
                        text-align: left;
                        max-width: 100%;
                    "
                >
                    {"While ODP’s first projects focus on boot firmware and embedded controller software, the partnership welcomes new ideas aligned with our core goals: security, efficiency, and broad reusability."}
                    <br /><br />
                </p>
            </div>
        </section>

        // Boot Firmware Buttons Section
        <section
            class="background_primary"
            style="
                padding: 0 120px 120px 120px;
            "
        >
            <div class="flex flex-row gap-[60px] justify-start">
                <ImageButton href="/boot-firmware" img_src="/images/patina.png" alt="Boot Firmware" />
                <ImageButton href="/embedded-controller" img_src="/images/ec.png" alt="Embedded Controller" />
                <ImageButton href="/windows-ec-services" img_src="/images/ec_services.png" alt="EC Services" />
            </div>
        </section>

        // Two Columns Section
        <section
            class="background_primary"
            style="
                padding: 80px 120px;
            "
        >
            <div class="flex flex-row gap-[60px]">
                {/* Column 1 */}
                <div class="flex flex-col items-start" style="flex: 1;">
                    <span
                        class="h3"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Partner-Oriented Vision"}
                    </span>
                    <span
                        class="p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"ODP is an inclusive partnership open to OEMs, ODMs, silicon vendors, hardware developers, security researchers, and anyone committed to improving device software foundations."}
                    </span>
                </div>
                {/* Column 2 */}
                <div class="flex flex-col items-start" style="flex: 1;">
                    <span
                        class="h3"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Get Involved!"}
                    </span>
                    <span
                        class="p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Explore our documentation, clone our public repositories, and contribute your expertise. Together, we can raise the standard for trusted devices."}
                    </span>
                </div>
            </div>
        </section>
    }
}
