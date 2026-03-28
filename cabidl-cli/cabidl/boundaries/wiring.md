## Boundary: Wiring

```yaml
kind: boundary
name: Wiring
exposure: internal
specification:
  path: ./wiring_trait.rs
  typeDescription: Rust Traits
```

The dependency wiring contract. Provides access to all domain boundaries through a single composition root. This isolates the Cli component from concrete implementation types, enabling full testability via an in-memory wiring that injects test doubles. The trait is defined in `./wiring_trait.rs`.

Implemented as the `cabidl-wiring` crate in `boundaries/wiring/`. Contains only the trait — depends on all domain boundary crates for their trait types.
