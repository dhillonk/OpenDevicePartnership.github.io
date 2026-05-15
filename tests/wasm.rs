//! Browser-side integration tests.
//!
//! These run under `wasm-pack test --headless --chrome` (or `--firefox`)
//! and exercise pieces of the design system that need a real DOM:
//! the `<picture>` shape of `ThemedIcon`, the back-button anchor in
//! `<TeamHero>`, and the `aria-expanded` toggle on the mobile nav.
//!
//! Pure-logic tests (graph JSON parsing, class-string composition)
//! live as `#[cfg(test)] mod tests` blocks next to their owning
//! source so they can run on the host with plain `cargo test`.
//!
//! For tests that need router context (`<A>` from `leptos_router`),
//! we wrap the component in a tiny `<Router>` so the routing context
//! is provided.

#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use leptos_router::components::Router;
use odp::components::header::Header;
use odp::components::team_hero::TeamHero;
use odp::components::themed_icon::ThemedIcon;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::*;
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

fn document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

/// Yield to the event loop so queued Leptos effects (reactive
/// attribute re-renders) flush before we assert on the DOM.
async fn next_tick() {
    let promise = js_sys::Promise::resolve(&JsValue::NULL);
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
}

/// Mount `view` into a freshly-created `<div>` appended to the body
/// and return a handle to that container so the test can query it.
/// Each call gets its own container so tests don't see each other's
/// DOM.
fn mount<F, V>(view: F) -> web_sys::Element
where
    F: FnOnce() -> V + 'static,
    V: IntoView + 'static,
{
    let doc = document();
    let container = doc.create_element("div").unwrap();
    doc.body().unwrap().append_child(&container).unwrap();
    let container_clone = container.clone();
    leptos::mount::mount_to(container_clone.unchecked_into::<HtmlElement>(), view).forget();
    container
}

#[wasm_bindgen_test]
fn themed_icon_renders_picture_with_dark_source_and_light_img() {
    let root = mount(|| {
        view! { <ThemedIcon name="lock" alt="lock icon" /> }
    });

    let picture = root.query_selector("picture").unwrap().expect("picture");
    let source = picture.query_selector("source").unwrap().expect("source");
    let img = picture.query_selector("img").unwrap().expect("img");

    assert!(source.get_attribute("srcset").unwrap().contains("/images/dark/lock"));
    assert!(img.get_attribute("src").unwrap().contains("/images/light/lock"));
    assert_eq!(img.get_attribute("alt").as_deref(), Some("lock icon"));
}

#[wasm_bindgen_test]
fn team_hero_renders_back_button_anchor() {
    let root = mount(|| {
        view! { <TeamHero team_name="Patina team" description="d" /> }
    });

    let anchors = root.query_selector("a[href='javascript:history.back()']").unwrap();
    assert!(anchors.is_some(), "expected at least one back-button anchor");
}

#[wasm_bindgen_test]
async fn header_mobile_menu_toggles_aria_expanded() {
    let root = mount(|| {
        view! {
            <Router>
                <Header />
            </Router>
        }
    });

    let button = root
        .query_selector("button[aria-controls='primary-mobile-nav']")
        .unwrap()
        .expect("hamburger button")
        .unchecked_into::<HtmlElement>();

    next_tick().await;
    assert_eq!(button.get_attribute("aria-expanded").as_deref(), Some("false"));
    button.click();
    next_tick().await;
    assert_eq!(button.get_attribute("aria-expanded").as_deref(), Some("true"));
    button.click();
    next_tick().await;
    assert_eq!(button.get_attribute("aria-expanded").as_deref(), Some("false"));
}

#[wasm_bindgen_test]
fn header_logo_is_wrapped_in_home_anchor() {
    let root = mount(|| {
        view! {
            <Router>
                <Header />
            </Router>
        }
    });

    let logo_anchor = root
        .query_selector("a[aria-label='Open Device Partnership home']")
        .unwrap()
        .expect("logo anchor");

    assert_eq!(logo_anchor.get_attribute("href").as_deref(), Some("/"));
    assert!(
        logo_anchor.query_selector("picture").unwrap().is_some(),
        "logo anchor should wrap the themed-icon picture"
    );
}
