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
use odp::components::landing::{ClosingColumnsSection, HeroSection, ProjectsSection, ValuePropositionSection};
use odp::components::team_hero::TeamHero;
use odp::components::themed_icon::ThemedIcon;
use odp::components::ui::{
    ArrowLink, ArrowLinkSize, DocLinkItem, IconBlock, LabeledSection, TwoColumnIntro, ValuePropCard,
};
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

// ---------------------------------------------------------------------------
// t30 — <ArrowLink>
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn arrow_link_external_renders_target_blank_and_external_class() {
    let root = mount(|| {
        view! { <ArrowLink href="https://example.com" title="External" size=ArrowLinkSize::Large /> }
    });

    let wrapper = root.query_selector("div").unwrap().expect("wrapper div");
    let class = wrapper.get_attribute("class").unwrap();
    assert!(class.contains("external-link"), "wrapper class = {class}");
    assert!(!class.contains("internal-link"));

    let anchor = wrapper.query_selector("a").unwrap().expect("anchor");
    assert_eq!(anchor.get_attribute("href").as_deref(), Some("https://example.com"));
    assert_eq!(anchor.get_attribute("target").as_deref(), Some("_blank"));
    assert_eq!(anchor.get_attribute("rel").as_deref(), Some("noopener noreferrer"));

    // Arrow + label spans both present.
    let spans = anchor.get_elements_by_tag_name("span");
    assert_eq!(spans.length(), 2, "expected arrow + label spans");
}

#[wasm_bindgen_test]
fn arrow_link_internal_renders_router_anchor_without_target() {
    let root = mount(|| {
        view! {
            <Router>
                <ArrowLink href="/community" title="Community" />
            </Router>
        }
    });

    let wrapper = root.query_selector("div").unwrap().expect("wrapper div");
    let class = wrapper.get_attribute("class").unwrap();
    assert!(class.contains("internal-link"), "wrapper class = {class}");
    assert!(!class.contains("external-link"));

    let anchor = wrapper.query_selector("a").unwrap().expect("anchor");
    assert_eq!(anchor.get_attribute("href").as_deref(), Some("/community"));
    assert!(
        anchor.get_attribute("target").is_none(),
        "internal links must not open new tab"
    );
}

// ---------------------------------------------------------------------------
// t31 — <IconBlock>
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn icon_block_wraps_themed_icon_with_children() {
    let root = mount(|| {
        view! {
            <IconBlock name="lock" alt="Lock">
                <span class="probe-child">"hello"</span>
            </IconBlock>
        }
    });

    assert!(
        root.query_selector("picture").unwrap().is_some(),
        "IconBlock should render the ThemedIcon picture"
    );
    let child = root.query_selector(".probe-child").unwrap().expect("probe child");
    assert_eq!(child.text_content().as_deref(), Some("hello"));
}

// ---------------------------------------------------------------------------
// t32 — <ValuePropCard>
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn value_prop_card_contains_icon_title_and_body() {
    let root = mount(|| {
        view! { <ValuePropCard icon="lock" icon_alt="Lock" title="Probe Title" body="Probe body text." /> }
    });

    assert!(
        root.query_selector("picture").unwrap().is_some(),
        "value-prop card should render the icon"
    );
    let html = root.inner_html();
    assert!(html.contains("Probe Title"), "missing title in: {html}");
    assert!(html.contains("Probe body text."), "missing body in: {html}");
}

// ---------------------------------------------------------------------------
// t33 — landing page sections
// ---------------------------------------------------------------------------

fn assert_contains(root: &web_sys::Element, needle: &str) {
    let html = root.inner_html();
    assert!(html.contains(needle), "expected `{needle}` in section html");
}

#[wasm_bindgen_test]
fn hero_section_renders_headline() {
    let root = mount(|| view! { <HeroSection /> });
    assert_contains(&root, "An Open Collaboration for Secure, Modern Devices");
}

#[wasm_bindgen_test]
fn value_proposition_section_renders_three_cards() {
    let root = mount(|| view! { <ValuePropositionSection /> });
    assert_contains(&root, "Value Proposition");
    assert_contains(&root, "Enhanced Security");
    assert_contains(&root, "Standardization");
    assert_contains(&root, "Accelerated Development");
    let pictures = root.get_elements_by_tag_name("picture");
    assert_eq!(pictures.length(), 3, "expected 3 value-prop card icons");
}

#[wasm_bindgen_test]
fn projects_section_renders_intro_and_three_image_buttons() {
    let root = mount(|| {
        view! {
            <Router>
                <ProjectsSection />
            </Router>
        }
    });
    assert_contains(&root, "ODP Projects");
    let imgs = root.get_elements_by_tag_name("img");
    assert!(
        imgs.length() >= 3,
        "expected >=3 project tile images, got {}",
        imgs.length()
    );
}

#[wasm_bindgen_test]
fn closing_columns_section_renders_both_columns() {
    let root = mount(|| view! { <ClosingColumnsSection /> });
    assert_contains(&root, "Partner-Oriented Vision");
    assert_contains(&root, "Get Involved!");
}

// ---------------------------------------------------------------------------
// t34 — <TwoColumnIntro> + <LabeledSection>
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn two_column_intro_places_left_and_right_in_separate_columns() {
    let root = mount(|| {
        view! {
            <TwoColumnIntro
                left=|| view! { <span class="probe-left">"L"</span> }
                right=|| view! { <span class="probe-right">"R"</span> }
            />
        }
    });

    let row = root
        .query_selector(".flex.flex-col.md\\:flex-row")
        .unwrap()
        .expect("row");
    let cols = row.children();
    assert_eq!(cols.length(), 2, "expected exactly two columns");

    let left_col = cols.item(0).unwrap();
    let right_col = cols.item(1).unwrap();
    assert!(
        left_col.query_selector(".probe-left").unwrap().is_some(),
        "left slot must render in first column"
    );
    assert!(
        right_col.query_selector(".probe-right").unwrap().is_some(),
        "right slot must render in second column"
    );
    // Right column gets the mobile-only top margin so the stacked
    // mobile layout has breathing room.
    let right_class = right_col.get_attribute("class").unwrap();
    assert!(right_class.contains("mt-8"), "right column class = {right_class}");
}

#[wasm_bindgen_test]
fn labeled_section_renders_uppercase_label_then_children() {
    let root = mount(|| {
        view! {
            <LabeledSection label="WHAT">
                <p class="probe-body">"body text"</p>
            </LabeledSection>
        }
    });

    let html = root.inner_html();
    let label_pos = html.find("WHAT").expect("label present");
    let body_pos = html.find("body text").expect("body present");
    assert!(label_pos < body_pos, "label must precede body in source order");
    // The label is wrapped in a Mono span (the `mono` brand class).
    assert!(html.contains("WHAT"), "literal WHAT should be in DOM");
}

// ---------------------------------------------------------------------------
// t35 — <DocLinkItem>
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn doc_link_item_external_renders_li_with_arrow_link() {
    let root = mount(|| {
        view! {
            <ul>
                <DocLinkItem href="https://example.com/x" title="External Doc" external=true />
            </ul>
        }
    });

    let li = root.query_selector("li").unwrap().expect("li");
    let anchor = li.query_selector("a").unwrap().expect("anchor inside li");
    assert_eq!(anchor.get_attribute("href").as_deref(), Some("https://example.com/x"));
    assert_eq!(anchor.get_attribute("target").as_deref(), Some("_blank"));
    let html = li.inner_html();
    assert!(html.contains("External Doc"), "title missing: {html}");
}

#[wasm_bindgen_test]
fn doc_link_item_internal_uses_router_anchor() {
    let root = mount(|| {
        view! {
            <Router>
                <ul>
                    <DocLinkItem href="/community" title="Community" external=false />
                </ul>
            </Router>
        }
    });

    let li = root.query_selector("li").unwrap().expect("li");
    let anchor = li.query_selector("a").unwrap().expect("anchor");
    assert_eq!(anchor.get_attribute("href").as_deref(), Some("/community"));
    assert!(
        anchor.get_attribute("target").is_none(),
        "internal link must not open new tab"
    );
}
