## Component: FilesystemImpl

```yaml
kind: component
name: FilesystemImpl
technology: Rust
boundaries:
  provides:
    - Filesystem
```

The filesystem component implements the `Filesystem` boundary using the Rust standard library (`std::fs`). It reads directory entries and returns file metadata. It has no external dependencies beyond the OS.
