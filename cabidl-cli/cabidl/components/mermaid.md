## Component: Mermaid

```yaml
kind: component
name: Mermaid
technology: Rust
boundaries:
  provides:
    - DiagramProvider
```

Implements the DiagramProvider boundary for Mermaid diagram format. Takes a System model and an optional `diagram_sub_type` to select between C4 and Class diagram layouts. Returns `"mermaid"` from `diagram_type()`.

### Supported diagram sub-types

- **c4** (default) — Generates a Mermaid C4Context diagram. Boundaries are represented as `System_Boundary` containers. External boundaries use `System_Ext`. Components inside boundaries are rendered as `Component` nodes. Requires relationships are shown as `Rel` arrows.
- **class** — Generates a Mermaid class diagram. Boundaries are represented as classes with a `<<boundary>>` stereotype. Components are classes with a `<<component>>` stereotype. Provides relationships are shown as realization arrows (`--|>`). Requires relationships are shown as dependency arrows (`..>`).

When `diagram_sub_type` is `None`, defaults to `"c4"`. Returns a `DiagramError` for unrecognized sub-types.

Implemented as the `cabidl-mermaid` crate in `mermaid/`. Depends on `cabidl-diagram-provider` and `cabidl-parser`.
