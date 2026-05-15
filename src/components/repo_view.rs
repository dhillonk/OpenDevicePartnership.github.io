//! Renders the repository dependency graph using D3 v7.
//!
//! All heavy lifting (force simulation, zoom controls, drag handlers,
//! styles) lives in two static assets that Trunk copies next to the
//! Wasm bundle and `index.html` loads as deferred scripts:
//!
//!  * `public/repo_graph.js`  -- defines `window.__odpRenderGraph()`.
//!  * `style/repo_graph.css`  -- the graph styles.
//!
//! Because both scripts are `<script defer>` they finish executing
//! before the Wasm bundle boots, so on every component mount we can
//! just publish the per-page payload as `window.__odpGraphData` and
//! call `__odpRenderGraph()`. `repo_graph.js` itself handles the rare
//! race where its `<svg>` host hasn't mounted yet by retrying on the
//! next animation frame.
//!
//! ## Why no `<script>` injection from Rust?
//!
//! Earlier versions appended d3 + `repo_graph.js` from this Effect on
//! the first mount. That introduced a load-order race: on the first
//! navigation to a project page the scripts had not finished
//! downloading by the time the data was published, so the graph
//! never appeared until the user reloaded the page (and the scripts
//! were served from cache).

use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys;

#[component]
pub fn RepositoryGraph(#[prop(into)] nodes: String, #[prop(into)] links: String) -> impl IntoView {
    Effect::new(move |_| {
        publish_graph_data(&nodes, &links);
        request_render();
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

/// Parses the per-page node/link JSON via the browser's native
/// `JSON.parse` and stores the result on `window.__odpGraphData`.
/// Silently no-ops if either string is not valid JSON.
fn publish_graph_data(nodes_json: &str, links_json: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let payload = js_sys::Object::new();
    if let Ok(nodes) = js_sys::JSON::parse(nodes_json) {
        let _ = js_sys::Reflect::set(&payload, &JsValue::from_str("nodes"), &nodes);
    }
    if let Ok(links) = js_sys::JSON::parse(links_json) {
        let _ = js_sys::Reflect::set(&payload, &JsValue::from_str("links"), &links);
    }
    let _ = js_sys::Reflect::set(&window, &JsValue::from_str("__odpGraphData"), &payload);
}

/// Calls `window.__odpRenderGraph()` if it is defined. The script
/// that defines it is injected via `<script defer>` in `index.html`,
/// so by the time the Wasm bundle mounts a `RepositoryGraph` it has
/// already executed.
fn request_render() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(render) = js_sys::Reflect::get(&window, &JsValue::from_str("__odpRenderGraph")) else {
        return;
    };
    if let Some(func) = render.dyn_ref::<js_sys::Function>() {
        let _ = func.call0(&JsValue::UNDEFINED);
    }
}
