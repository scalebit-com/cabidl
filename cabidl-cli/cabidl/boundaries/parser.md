## Boundary: CabidlParser

```yaml
kind: boundary
name: CabidlParser
exposure: internal
specification:
  path: ./parser_trait.rs
  typeDescription: Rust Traits
```

The core parsing and validation contract. The trait and all domain model types are defined in `./parser_trait.rs`.

The parser returns a `System` — a graph model where components hold `Arc` references to the boundaries they provide and require. This makes the architecture directly navigable without string lookups. The model mirrors the CABIDL specification structure: a `System` contains `Boundary` and `Component` instances, and components reference boundaries through `Arc`, reflecting the `provides`/`requires` relationships. This model is the foundation for validation, the `read` command output, and future tools like diagram generation.

Implemented as the `cabidl-parser` crate in `parser/`. Contains the `CabidlParser` trait, the domain model (`System`, `Boundary`, `Component`, `ValidationError`), and no external dependencies. Each type lives in its own module file.
