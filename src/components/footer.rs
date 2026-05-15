use crate::components::themed_icon::ThemedIcon;
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full px-2 md:px-16 py-2 md:py-8 background_secondary">
            <div class="flex flex-col md:flex-row items-center justify-between w-full gap-1 md:gap-0">
                <div class="flex items-center w-full md:w-auto justify-center md:justify-start">
                    <ThemedIcon
                        name="odplogo"
                        alt="Logo"
                        class="w-[70px] h-[24px] md:w-[114px] md:h-[40px] object-contain"
                    />
                    <p class="ml-2 md:ml-[30px] leading-[18px] md:leading-[26px] p_mobile md:p">
                        {"© 2025 Open Device Partnership"}
                    </p>
                </div>

                <div class="flex flex-row items-center justify-center w-full md:w-auto">
                    <a
                        href="https://github.com/OpenDevicePartnership"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="flex items-center justify-center px-1 md:px-[16px] py-1 md:py-[24px]"
                    >
                        <ThemedIcon
                            name="github"
                            alt="GitHub"
                            class="w-5 h-5 md:w-auto md:h-auto"
                        />
                    </a>
                    <a
                        href="https://opendevicepartnership.zulipchat.com"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="flex items-center justify-center px-1 md:px-[16px] py-1 md:py-[24px]"
                    >
                        <ThemedIcon name="zulip" alt="Zulip" class="w-5 h-5 md:w-auto md:h-auto" />
                    </a>

                    <a
                        href="https://discord.gg/a8cEfTDQN4"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="flex items-center justify-center px-1 md:px-[16px] py-1 md:py-[24px]"
                    >
                        <ThemedIcon
                            name="discord"
                            alt="Discord"
                            class="w-5 h-5 md:w-auto md:h-auto"
                        />
                    </a>
                </div>
            </div>
        </footer>
    }
}
