// Repository graph rendered with D3 v7. The Rust component
// `RepositoryGraph` (src/components/repo_view.rs) sets
// `window.__odpGraphData = { nodes, links }` and then loads this file
// once per session. Each call to `window.__odpRenderGraph()` clears
// the existing <svg> and (re-)renders the graph with whatever is in
// __odpGraphData, so that route changes between the three project
// pages get a fresh graph without re-injecting any <script>/<style>.

(function () {
    "use strict";

    const SPACING = 300;
    const BOX_HEIGHT = 60;
    const VERTICAL_GAP = 20;
    const ZOOM_MIN = 0.1;
    const ZOOM_MAX = 3;

    function render() {
        if (typeof d3 === "undefined") {
            // d3 is loaded as a sibling <script defer> in index.html;
            // on a very first paint it may not be parsed yet. Try
            // again on the next animation frame.
            requestAnimationFrame(render);
            return;
        }
        const data = window.__odpGraphData;
        if (!data) return;

        const svgEl = document.querySelector(".repository-graph svg");
        if (!svgEl) {
            // The host component was mounted but its <svg> isn't in the
            // DOM yet -- retry on next frame.
            requestAnimationFrame(render);
            return;
        }

        const nodes = data.nodes.map((d) => ({ ...d }));
        const links = data.links.map((d) => ({ ...d }));

        const width = window.innerWidth;
        const height = window.innerHeight;

        svgEl.innerHTML = "";

        const svg = d3.select(svgEl).attr("viewBox", [0, 0, width, height]);
        const zoomLayer = svg.append("g").attr("class", "zoom-layer");

        const grouped = {};
        nodes.forEach((d) => {
            d.fx = d.order * SPACING;
            (grouped[d.order] ||= []).push(d);
        });
        Object.values(grouped).forEach((group) => {
            const totalHeight = group.length * (BOX_HEIGHT + VERTICAL_GAP);
            const startY = (height - totalHeight) / 2;
            group.forEach((node, i) => {
                node.fy = startY + i * (BOX_HEIGHT + VERTICAL_GAP);
                node.initialFy = node.fy;
            });
        });

        const classifications = Array.from(
            new Set(nodes.map((d) => JSON.stringify({ classification: d.classification, order: d.order }))),
        )
            .map((s) => JSON.parse(s))
            .sort((a, b) => a.order - b.order);

        const headerGroup = zoomLayer.append("g").attr("class", "column-headers");
        classifications.forEach(({ classification, order }) => {
            const x = order * SPACING;
            headerGroup
                .append("line")
                .attr("x1", x)
                .attr("y1", 0)
                .attr("x2", x)
                .attr("y2", height)
                .attr("stroke", "#ccc")
                .attr("stroke-width", 2)
                .attr("stroke-dasharray", "4,4");
            headerGroup
                .append("text")
                .attr("x", x)
                .attr("y", 30)
                .attr("text-anchor", "middle")
                .attr("fill", "#666")
                .attr("font-size", "16px")
                .attr("font-weight", "bold")
                .text(classification);
        });

        const link = zoomLayer.append("g").selectAll("path").data(links).join("path").attr("class", "link");

        const simulation = d3
            .forceSimulation(nodes)
            .force(
                "link",
                d3
                    .forceLink(links)
                    .id((d) => d.id)
                    .distance(150),
            )
            .force("charge", d3.forceManyBody().strength(-300));

        const node = zoomLayer
            .append("g")
            .selectAll("g")
            .data(nodes)
            .join("g")
            .attr("class", "node")
            .call(makeDrag(simulation));

        node.append("text")
            .attr("text-anchor", "middle")
            .attr("dy", "0.35em")
            .text((d) => d.name);

        node.each(function (d) {
            const textEl = d3.select(this).select("text").node();
            const textWidth = textEl.getComputedTextLength();
            d.boxWidth = textWidth + 20;
            d._rect = d3
                .select(this)
                .insert("rect", "text")
                .attr("x", -d.boxWidth / 2)
                .attr("y", -20)
                .attr("width", d.boxWidth)
                .attr("height", 40)
                .attr("rx", 10)
                .attr("ry", 10)
                .attr("fill", "white")
                .attr("stroke", "#333")
                .attr("stroke-width", 1.5);
        });

        node.on("mouseover", function (_e, d) {
            d._rect.attr("fill", "#9BFABE");
        })
            .on("mouseout", function (_e, d) {
                d._rect.attr("fill", "white");
            })
            .on("click", function (_e, d) {
                window.open(d.url, "_blank");
            });

        let currentTransform = d3.zoomIdentity;

        const zoomBehavior = d3
            .zoom()
            .scaleExtent([ZOOM_MIN, ZOOM_MAX])
            .filter((event) => event.type !== "wheel")
            .on("zoom", (event) => {
                currentTransform = event.transform;
                zoomLayer.attr("transform", currentTransform);
            });

        svg.call(zoomBehavior);

        function applyZoom() {
            svg.transition().duration(300).call(zoomBehavior.transform, currentTransform);
        }

        function fitToScreen() {
            const xs = nodes.map((n) => n.x);
            const ys = nodes.map((n) => n.y);
            const minX = Math.min(...xs);
            const maxX = Math.max(...xs);
            const minY = Math.min(...ys);
            const maxY = Math.max(...ys);
            const boundsWidth = maxX - minX + 200;
            const boundsHeight = maxY - minY + 200;
            const scale = Math.min(width / boundsWidth, height / boundsHeight, ZOOM_MAX);
            const tx = width / 2 - (scale * (minX + maxX)) / 2;
            const ty = height / 2 - (scale * (minY + maxY)) / 2;
            currentTransform = d3.zoomIdentity.translate(tx, ty).scale(scale);
            return currentTransform;
        }

        let initialFitApplied = false;
        simulation.on("tick", () => {
            link.attr("d", (d) => {
                const dx = d.target.x - d.source.x;
                const dy = d.target.y - d.source.y;
                const dr = Math.sqrt(dx * dx + dy * dy) * 1.5;
                return `M${d.source.x},${d.source.y} A${dr},${dr} 0 0,1 ${d.target.x},${d.target.y}`;
            });
            node.attr("transform", (d) => `translate(${d.x},${d.y})`);
            if (!initialFitApplied && simulation.alpha() < 0.5) {
                initialFitApplied = true;
                currentTransform = fitToScreen();
                svg.call(zoomBehavior.transform, currentTransform);
            }
        });

        function makeDrag(sim) {
            return d3
                .drag()
                .on("start", (event) => {
                    if (!event.active) sim.alphaTarget(0.3).restart();
                })
                .on("drag", (event, d) => {
                    d.fy = event.y;
                })
                .on("end", (event, d) => {
                    if (!event.active) sim.alphaTarget(0);
                    d.fy = d.initialFy;
                });
        }

        d3.select("#zoom-in").on("click", () => {
            const newScale = Math.min(currentTransform.k + 0.1, ZOOM_MAX);
            currentTransform = d3.zoomIdentity.translate(currentTransform.x, currentTransform.y).scale(newScale);
            applyZoom();
        });
        d3.select("#zoom-out").on("click", () => {
            const newScale = Math.max(currentTransform.k - 0.1, ZOOM_MIN);
            currentTransform = d3.zoomIdentity.translate(currentTransform.x, currentTransform.y).scale(newScale);
            applyZoom();
        });
        d3.select("#zoom-fit").on("click", () => {
            currentTransform = fitToScreen();
            applyZoom();
        });
    }

    window.__odpRenderGraph = render;
})();
