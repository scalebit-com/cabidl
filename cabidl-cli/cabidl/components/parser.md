## Component: Parser

```yaml
kind: component
name: Parser
technology: Rust
boundaries:
  provides:
    - CabidlParser
  requires:
    - Filesystem
```

Implements all CABIDL parsing and validation logic:

- Resolves `<!-- @include -->` directives recursively with circular include detection
- Extracts YAML blocks from markdown, tracking line numbers for error reporting
- Builds the `System` graph model — parses YAML blocks into `Boundary` and `Component` instances, then resolves boundary name references into `Arc` links
- Provides a pure `parse_content()` function that takes a fully-resolved CABIDL string and returns a `System` model — this is the primary entry point for testing
- Validates YAML block structure, name uniqueness, boundary reference integrity, and exposure values
- Reports errors in compiler-style `file:line: message` format

Implemented as the `cabidl-parser-impl` crate in `parser-impl/`. Depends on `cabidl-parser`, `cabidl-filesystem`, `serde`, `serde_yaml`, and `regex`.
