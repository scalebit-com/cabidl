## Boundary: Filesystem

```yaml
kind: boundary
name: Filesystem
exposure: internal
specification:
  path: ./boundaries/filesystem_trait.rs
  typeDescription: Rust Traits
```

The filesystem abstraction boundary. Defines the contract for reading directory entries and file metadata. This boundary exists so that the CLI logic is decoupled from direct filesystem calls, making the system testable and extensible.
