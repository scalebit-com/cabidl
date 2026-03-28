## Boundary: Filesystem

```yaml
kind: boundary
name: Filesystem
exposure: internal
specification:
  path: ./filesystem_trait.rs
  typeDescription: Rust Traits
```

Abstraction over filesystem operations so that all components can be tested with in-memory file systems without real I/O. Covers read, write, and directory creation operations. The trait contract is defined in `./filesystem_trait.rs`.

Implemented as the `cabidl-filesystem` crate in `filesystem/`. Contains only the trait — no implementations, no external dependencies.
