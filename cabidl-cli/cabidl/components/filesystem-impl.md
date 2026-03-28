## Component: FilesystemImpl

```yaml
kind: component
name: FilesystemImpl
technology: Rust
boundaries:
  provides:
    - Filesystem
```

Implements the Filesystem boundary. Provides a real implementation using `std::fs` and an in-memory implementation for testing.

Implemented as the `cabidl-filesystem-impl` crate in `filesystem-impl/`. Depends on `cabidl-filesystem`.
