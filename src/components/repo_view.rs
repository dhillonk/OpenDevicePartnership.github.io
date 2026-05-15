//! Renders the repository dependency graph using D3 v7.
//!
//! Heavy lifting (the D3 force simulation, zoom controls, drag handlers,
//! style sheet) lives in two static assets that Trunk copies next to the
//! Wasm bundle:
//!
//!  * `public/repo_graph.js`   -- the D3 rendering code, exposed as
//!    `window.__odpRenderGraph()`.
//!  * `style/repo_graph.css`   -- linked once from `index.html`.
//!
//! On mount this component publishes the per-page node/link payload as
//! `window.__odpGraphData` and asks the JS module to (re-)render. If
//! the JS / D3 hasn't loaded yet, `<script>` tags for both are appended
//! exactly once and the loader chains the first render. Subsequent
//! mounts (clicking through the three project pages) reuse the already
//! loaded code and just call `__odpRenderGraph()`.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

const D3_SRC: &str = "https://d3js.org/d3.v7.min.js";
const D3_SCRIPT_ID: &str = "d3-cdn";
const GRAPH_SCRIPT_ID: &str = "odp-repo-graph-script";

#[component]
pub fn RepositoryGraph(#[prop(into)] nodes: String, #[prop(into)] links: String) -> impl IntoView {
    Effect::new(move |_| {
        publish_graph_data(&nodes, &links);
        ensure_graph_loaded();
    });

    view! {
        <div class="repository-graph">
            <div id="zoom-controls">
                <button id="zoom-in">"+"</button>
                <button id="zoom-out">"−"</button>
                <button id="zoom-fit">"⛶"</button>
            </div>
            <svg width="100%" height="100%" style="position:absolute;"></svg>
        </div>
    }
}

/// Pushes the per-page node/link JSON onto `window.__odpGraphData` so
/// `repo_graph.js` can pick it up. Done via an inline `<script>` because
/// constructing the JS objects from Rust would require pulling in a
/// JSON-to-JsValue dependency just for this one call site.
fn publish_graph_data(nodes_json: &str, links_json: &str) {
    let document = window().and_then(|w| w.document()).expect("document");
    let head = document.head().expect("head");
    let script = document.create_element("script").expect("script");
    script.set_inner_html(&format!(
        "window.__odpGraphData = {{ nodes: {nodes_json}, links: {links_json} }};\
         if (typeof window.__odpRenderGraph === 'function') window.__odpRenderGraph();"
    ));
    head.append_child(&script).ok();
    // The inline script does its work on parse; remove it so it doesn't
    // accumulate one element per route change.
    script.remove();
}

/// Appends the d3 CDN `<script>` and the local `repo_graph.js` exactly
/// once. After the first mount both tags stay in `<head>`; on later
/// mounts this is a no-op and `repo_graph.js` is already executing
/// `window.__odpRenderGraph` from `publish_graph_data`'s inline script.
fn ensure_graph_loaded() {
    let document = window().and_then(|w| w.document()).expect("document");
    let head = document.head().expect("head");

    if document.get_element_by_id(GRAPH_SCRIPT_ID).is_some() {
        return;
    }

    let load_graph_script = move || {
        let document = window().and_then(|w| w.document()).expect("document");
        let head = document.head().expect("head");
        if document.get_element_by_id(GRAPH_SCRIPT_ID).is_some() {
            return;
        }
        let el = document
            .create_element("script")
            .expect("script")
            .dyn_into::<web_sys::HtmlScriptElement>()
            .expect("HtmlScriptElement");
        el.set_id(GRAPH_SCRIPT_ID);
        el.set_src("/repo_graph.js");
        head.append_child(&el).ok();
    };

    if document.get_element_by_id(D3_SCRIPT_ID).is_none() {
        let d3 = document
            .create_element("script")
            .expect("script")
            .dyn_into::<web_sys::HtmlScriptElement>()
            .expect("HtmlScriptElement");
        d3.set_id(D3_SCRIPT_ID);
        d3.set_src(D3_SRC);
        let onload = wasm_bindgen::closure::Closure::once_into_js(load_graph_script);
        d3.set_onload(Some(onload.as_ref().unchecked_ref()));
        head.append_child(&d3).ok();
    } else {
        load_graph_script();
    }
}
