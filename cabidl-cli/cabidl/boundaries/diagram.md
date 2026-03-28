## Boundary: Diagram

```yaml
kind: boundary
name: Diagram
exposure: internal
specification:
  path: ./diagram_trait.rs
  typeDescription: Rust Traits
```

The diagram orchestration contract. Takes a parsed System model and a diagram type string. Selects the appropriate DiagramProvider and returns the generated content as a string. The caller is responsible for writing the output to a file. The trait is defined in `./diagram_trait.rs`.

Implemented as the `cabidl-diagram` crate in `diagram/`. Contains only the trait — no implementations. Depends on `cabidl-parser` (for the System type) and `cabidl-diagram-provider` (for the DiagramError type).
