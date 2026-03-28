## Component: WiringImpl

```yaml
kind: component
name: WiringImpl
technology: Rust
boundaries:
  provides:
    - Wiring
  requires:
    - CabidlParser
    - Diagram
    - DiagramProvider
    - AiProvider
    - Init
    - Filesystem
```

Implements the Wiring boundary. Constructs and owns all concrete component instances — the parser, diagram orchestrator, AI provider, and project initializer — wired together with a real filesystem. Provides the composition root for the application.

Implemented as the `cabidl-wiring-impl` crate in `components/wiring-impl/`. Depends on `cabidl-wiring`, `cabidl-parser`, `cabidl-parser-impl`, `cabidl-filesystem`, `cabidl-filesystem-impl`, `cabidl-diagram`, `cabidl-diagram-impl`, `cabidl-diagram-provider`, `cabidl-graphviz`, `cabidl-ai-provider`, `cabidl-claude-code`, `cabidl-init`, and `cabidl-init-impl`.
