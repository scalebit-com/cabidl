## Boundary: DiagramProvider

```yaml
kind: boundary
name: DiagramProvider
exposure: internal
specification:
  path: ./diagram_provider_trait.rs
  typeDescription: Rust Traits
```

The diagram provider contract. Each implementer generates diagram content in a specific output format from a parsed System model. The trait and the `DiagramError` type are defined in `./diagram_provider_trait.rs`.

A provider identifies itself via a `diagram_type()` method that returns a string (e.g. `"graphviz"`, `"mermaid"`). The `generate()` method takes a `&System` and an optional `diagram_sub_type` string, and returns the diagram content as a `String`, or a `DiagramError` on failure. The `diagram_sub_type` allows providers that support multiple diagram layouts or themes (e.g. Graphviz's dark/light themes, Mermaid's C4/Class layouts) to select the appropriate variant. Each provider defines its own valid sub-types and default.

Implemented as the `cabidl-diagram-provider` crate in `diagram-provider/`. Contains only the trait and error type — no implementations, no external dependencies beyond `cabidl-parser`.
