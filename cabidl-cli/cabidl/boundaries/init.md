## Boundary: Init

```yaml
kind: boundary
name: Init
exposure: internal
specification:
  path: ./init_trait.rs
  typeDescription: Rust Traits
```

The project initialization contract. Handles listing available templates and scaffolding new CABIDL projects from embedded templates. The trait, `InitError`, and `TemplateEntry` types are defined in `./init_trait.rs`.

The `list_templates()` method returns all available templates from a compile-time index — no decompression is needed since the index is generated at build time from `template.yaml` files in `cabidl-cli/templates/`. The `scaffold()` method decompresses the embedded templates archive in memory and streams the selected template's file contents (excluding `template.yaml`) directly to the target directory via the Filesystem boundary. No temporary directories are used — all file operations go through the Filesystem trait for testability.

Implemented as the `cabidl-init` crate in `init/`. Contains only the trait, error type, and template entry struct — no implementations, no external dependencies.
