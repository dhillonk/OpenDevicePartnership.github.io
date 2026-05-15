use leptos::prelude::*;

#[component]
pub fn Partner(name: &'static str, url: &'static str, logo: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center justify-center w-full h-[80px] md:h-[100px]">
            <a href=url target="_blank" rel="noopener noreferrer">
                <img
                    src=logo
                    alt=name
                    class="w-full h-full max-w-[180px] max-h-[70px] md:max-w-[200px] md:max-h-[80px] object-contain"
                />
            </a>
        </div>
    }
}
