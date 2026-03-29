## Component: Graphviz

```yaml
kind: component
name: Graphviz
technology: Rust
boundaries:
  provides:
    - DiagramProvider
```

Implements the DiagramProvider boundary for Graphviz DOT format. Takes a System model and generates a DOT language string representing the architecture as a cluster-based graph. Returns `"graphviz"` from `diagram_type()`.

### Supported diagram sub-types

- **dark** (default) — Catppuccin Mocha dark mode color scheme. This is the current and default rendering.
- **light** — Light color scheme suitable for print and light-background contexts.

When `diagram_sub_type` is `None`, defaults to `"dark"`. Returns a `DiagramError` for unrecognized sub-types.

### Rendering rules

- **Boundaries as clusters** — Each boundary is rendered as a `subgraph cluster_*` container, not a standalone node. The cluster label is the boundary name.
  - External boundaries: bold red border (`#f38ba8` in dark), red label.
  - Internal boundaries: gray border (`#585b70` in dark), gray label.
- **Component placement** — A component is placed inside the cluster of its first `provides` boundary. If it provides additional boundaries, those are shown as explicit green "provides" edges to the other clusters. Components with no `provides` are free-floating outside all clusters.
- **Requires edges** — Solid blue (`#89b4fa` in dark) arrows from the requiring component to the target boundary's cluster border. Uses Graphviz `compound=true` with `lhead` on edges targeting an invisible anchor node inside each cluster.
- **Anchor nodes** — Each cluster contains an invisible anchor node (`_anchor:BoundaryName`) used as the edge endpoint so that `lhead` can route arrows to the cluster border.

Implemented as the `cabidl-graphviz` crate in `graphviz/`. Depends on `cabidl-diagram-provider` and `cabidl-parser`.
