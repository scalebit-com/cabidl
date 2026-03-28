## Component: Graphviz

```yaml
kind: component
name: Graphviz
technology: Rust
boundaries:
  provides:
    - DiagramProvider
```

Implements the DiagramProvider boundary for Graphviz DOT format. Takes a System model and generates a DOT language string representing the architecture as a cluster-based graph. Uses a Catppuccin Mocha dark mode color scheme.

### Rendering rules

- **Boundaries as clusters** — Each boundary is rendered as a `subgraph cluster_*` container, not a standalone node. The cluster label is the boundary name.
  - External boundaries: bold red border (`#f38ba8`), red label.
  - Internal boundaries: gray border (`#585b70`), gray label.
- **Component placement** — A component is placed inside the cluster of its first `provides` boundary. If it provides additional boundaries, those are shown as explicit green "provides" edges to the other clusters. Components with no `provides` are free-floating outside all clusters.
- **Requires edges** — Solid blue (`#89b4fa`) arrows from the requiring component to the target boundary's cluster border. Uses Graphviz `compound=true` with `lhead` on edges targeting an invisible anchor node inside each cluster.
- **Anchor nodes** — Each cluster contains an invisible anchor node (`_anchor:BoundaryName`) used as the edge endpoint so that `lhead` can route arrows to the cluster border.

Implemented as the `cabidl-graphviz` crate in `graphviz/`. Depends on `cabidl-diagram-provider` and `cabidl-parser`.
