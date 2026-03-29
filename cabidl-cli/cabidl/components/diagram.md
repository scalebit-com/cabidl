## Component: Diagram

```yaml
kind: component
name: Diagram
technology: Rust
boundaries:
  provides:
    - Diagram
  requires:
    - DiagramProvider
```

Implements the Diagram boundary. Holds a registry of DiagramProvider implementations. On invocation, selects the provider matching the requested diagram type, passes the diagram sub-type through to the provider, and returns the result as a string.

Implemented as the `cabidl-diagram-impl` crate in `diagram-impl/`. Depends on `cabidl-diagram` and `cabidl-diagram-provider`.
