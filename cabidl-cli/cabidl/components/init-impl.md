## Component: InitImpl

```yaml
kind: component
name: InitImpl
technology: Rust
boundaries:
  provides:
    - Init
  requires:
    - Filesystem
```

Implements the Init boundary. Handles template listing and project scaffolding from embedded templates.

### Build-time template embedding

At build time (via `build.rs`):

1. Scans `cabidl-cli/templates/*/template.yaml` files and parses each to extract `name`, `language`, and `description` fields.
2. Generates a Rust source file with a `const TEMPLATE_INDEX` array of `TemplateEntry` values, enabling `list_templates()` to return the index without decompression.
3. Compresses the entire `cabidl-cli/templates/` directory into a tar.gz archive.
4. Embeds the archive into the binary via `include_bytes!`.

### Runtime behavior

- `list_templates()` — Returns the compile-time `TEMPLATE_INDEX`. No I/O or decompression needed.
- `scaffold(template_name, target_dir)` — Validates the template name exists in the index. Decompresses the embedded tar.gz archive in memory and streams the selected template's file entries (excluding `template.yaml`) directly to `target_dir` via the Filesystem boundary. No temporary directories are created — all file operations go through the Filesystem trait, making the component fully testable with an in-memory filesystem.

Implemented as the `cabidl-init-impl` crate in `init-impl/`. Depends on `cabidl-init`, `cabidl-filesystem`, `flate2`, and `tar`.
