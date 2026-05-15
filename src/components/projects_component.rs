use crate::components::image_button::ImageButton;
use leptos::prelude::*;

#[component]
pub fn ProjectsComponent() -> impl IntoView {
    view! {
        <section class="background_primary px-6 py-8 md:px-32 md:py-32">
            <div class="flex flex-col md:flex-row gap-10 md:gap-20">
                <div class="flex flex-col items-start w-full md:w-[700px]">
                    <span class="h1 block text-left">{"System Firmware Domains"}</span>
                    <span class="h2 block text-left">
                        {"Reusable foundations for secure, high-quality device platforms"}
                    </span>
                </div>
                <div class="flex flex-col items-start w-full md:w-[600px] mt-8 md:mt-0">
                    <span class="mono block text-left">{"WHAT"}</span>
                    <span class="p1 block text-left">
                        {"ODP supports development across three core areas of system firmware. Each domain is designed for modularity, security, and long-term reuse across hardware platforms."}
                    </span>
                    <span class="mono block text-left">{"WHY"}</span>
                    <span class="p1 block text-left">
                        {"Modern computing devices need modern solutions that prioritize memory safety and security from the language on up."}
                    </span>
                </div>
            </div>
        </section>

        // Projects Details Section
        <section class="background_primary px-6 py-8 md:px-32 md:py-32">
            <div class="flex flex-col gap-16">

                <div class="flex flex-col md:flex-row gap-8 md:gap-16 items-center">
                    <div class="w-full max-w-full md:max-w-[600px] md:w-[600px] md:h-[518px]">
                        <ImageButton
                            href="/boot-firmware"
                            img_src="/images/patina.png"
                            alt="Boot Firmware"
                            width=600
                            height=518
                            mobile_width=Some(320)
                            mobile_height=Some(250)
                        />
                    </div>
                    <div
                        class="flex flex-col items-start w-full md:w-auto mt-8 md:mt-0"
                        style="flex: 1;"
                    >
                        <span class="h2 block text-left">{"Patina (Boot Firmware)"}</span>
                        <span class="p1 block text-left">{"Rethink your boot firmware"}</span>
                        <span class="p2 block text-left">
                            {"Patina provides a UEFI compatible firmware interface written in the Rust language with a focus on memory safety and composition.  Compatible with current UEFI device drivers and loaders but with a focus on the future."}
                        </span>
                        <div class="flex flex-col gap-[8px]">
                            <a
                                href="https://opendevicepartnership.github.io/patina"
                                class="link"
                                style="text-decoration: none;"
                            >
                                <span style="text-decoration: none;">{"→ "}</span>
                                <span style="text-decoration: underline;">
                                    {"Read the Patina Documentation"}
                                </span>
                            </a>
                            <a
                                href="https://github.com/opendevicepartnership/patina"
                                class="link"
                                style="text-decoration: none;"
                            >
                                <span style="text-decoration: none;">{"→ "}</span>
                                <span style="text-decoration: underline;">
                                    {"View Patina Source Code on GitHub"}
                                </span>
                            </a>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col md:flex-row gap-8 md:gap-16 items-center">
                    <div class="w-full max-w-full md:max-w-[600px] md:w-[600px] md:h-[518px]">
                        <ImageButton
                            href="/embedded-controller"
                            img_src="/images/ec.png"
                            alt="Embedded Controller"
                            width=600
                            height=518
                            mobile_width=Some(320)
                            mobile_height=Some(250)
                        />
                    </div>
                    <div
                        class="flex flex-col items-start w-full md:w-auto mt-8 md:mt-0"
                        style="flex: 1;"
                    >
                        <span class="h2 block text-left">{"Secure Embedded Controller"}</span>
                        <span class="p1 block text-left">
                            {"A Secure end-to-end Rust-based EC implementation"}
                        </span>
                        <span class="p2 block text-left">
                            {"ODP EC provides a modern embedded controller firmware written in Rust. Designed for safety and composability, it serves as a foundational layer for secure device management."}
                        </span>
                        <div class="flex flex-col gap-[8px]">
                            <a
                                href="https://opendevicepartnership.github.io/documentation/tracks/embedded_controller/track_overview.html"
                                class="link"
                                style="text-decoration: none;"
                            >
                                <span style="text-decoration: none;">{"→ "}</span>
                                <span style="text-decoration: underline;">
                                    {"Read the Secure EC Documentation"}
                                </span>
                            </a>
                            <a
                                href="https://github.com/OpenDevicePartnership/embedded-services"
                                class="link"
                                style="text-decoration: none;"
                            >
                                <span style="text-decoration: none;">{"→ "}</span>
                                <span style="text-decoration: underline;">
                                    {"View Secure EC Source Code on GitHub"}
                                </span>
                            </a>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col md:flex-row gap-8 md:gap-16 items-center">
                    <div class="w-full max-w-full md:max-w-[600px] md:w-[600px] md:h-[518px]">
                        <ImageButton
                            href="/windows-ec-services"
                            img_src="/images/ec_services.png"
                            alt="EC Services"
                            width=600
                            height=518
                            mobile_width=Some(320)
                            mobile_height=Some(250)
                        />
                    </div>
                    <div
                        class="flex flex-col items-start w-full md:w-auto mt-8 md:mt-0"
                        style="flex: 1;"
                    >
                        <span class="h2 block text-left">
                            {"Unified Embedded Controller Services"}
                        </span>
                        <span class="p1 block text-left">
                            {"A standard and secure cross-architecture EC services implementation"}
                        </span>
                        <span class="p2 block text-left">
                            {"ODP EC Services provides a modern EC services firmware written in Rust. Designed for safety and composability, it serves as a foundational layer for secure EC services on Windows platforms."}
                        </span>
                        <div class="flex flex-col gap-[8px]">
                            <a
                                href="https://opendevicepartnership.github.io/documentation/guide/intro/concepts/EC_Services.html"
                                class="link"
                                style="text-decoration: none;"
                            >
                                <span style="text-decoration: none;">{"→ "}</span>
                                <span style="text-decoration: underline;">
                                    {"Read the EC Services Documentation"}
                                </span>
                            </a>
                            <a
                                href="https://github.com/OpenDevicePartnership/haf-ec-service"
                                class="link"
                                style="text-decoration: none;"
                            >
                                <span style="text-decoration: none;">{"→ "}</span>
                                <span style="text-decoration: underline;">
                                    {"View EC Services Source Code on GitHub"}
                                </span>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
