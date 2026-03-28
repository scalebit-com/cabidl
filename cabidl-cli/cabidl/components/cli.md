## Component: Cli

```yaml
kind: component
name: Cli
technology: Rust
boundaries:
  provides:
    - CliInterface
  requires:
    - Wiring
```

Entry point of the application. Parses command-line arguments, dispatches to the appropriate subcommand, and formats output or errors for the terminal. Contains no domain logic — obtains all domain boundaries from the Wiring composition root and delegates to them. The skill file (`skill.md` at the workspace root) is embedded into the binary at compile time via `include_str!` and passed to the AiProvider's `install_skill()` method when the `skill install` subcommand is invoked. For the `init` subcommand, the Cli delegates template listing to `Init::list_templates()`, scaffolding to `Init::scaffold()`, and provider-specific file creation to `AiProvider::init_project()`.

Implemented as the `cabidl` binary crate in `components/cli/`. Depends on `cabidl-wiring`, `cabidl-wiring-impl`, `cabidl-parser` (for resolve function), `cabidl-parser-impl` (for resolve function), `cabidl-filesystem` (for Filesystem trait used in file writing), `cabidl-filesystem-impl` (for RealFilesystem used in resolve), and `clap`.
