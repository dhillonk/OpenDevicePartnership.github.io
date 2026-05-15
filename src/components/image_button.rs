use leptos::prelude::*;

/// Rounded image rendered as a link.
///
/// Sizing is purely declarative via Tailwind utilities passed in `class`.
/// The default `aspect-square max-w-[350px]` matches the landing-page
/// tiles; call sites override the cap when the surrounding layout calls
/// for a larger image. There is no per-instance `<style>` injection --
/// the image scales fluidly with the viewport up to the cap and the
/// aspect-ratio utility keeps the rendered shape square.
#[component]
pub fn ImageButton(
    #[prop(into)] href: String,
    #[prop(into)] img_src: String,
    #[prop(into, default = String::from("Button Image"))] alt: String,
    #[prop(into, default = String::from("aspect-square max-w-[350px]"))] class: String,
) -> impl IntoView {
    view! {
        <a href=href class=format!("inline-block w-full overflow-hidden rounded-3xl {}", class)>
            <img src=img_src alt=alt class="w-full h-full object-cover block" />
        </a>
    }
}
