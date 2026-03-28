# cabidl

**CABIDL Specification**
Version: 2.0

```yaml
kind: system
name: cabidl
```

A command-line tool for working with CABIDL architecture specification files. It reads CABIDL markdown documents, resolves `<!-- @include -->` directives into a single unified output, validates that all YAML blocks conform to the CABIDL schemas and that boundary references between components are consistent, generates architecture diagrams in various formats, installs AI tool provider skills for spec-first development workflows, and scaffolds new CABIDL projects from embedded templates.

The system is implemented as a Rust Cargo workspace. Each boundary is its own crate containing trait definitions and types. Each component is its own crate containing the implementation. Workspace dependencies mirror the `provides`/`requires` relationships.

### Project Structure

```
cabidl-cli/
├── Cargo.toml                          # Workspace root
├── skill.md                            # CABIDL skill file (embedded into the binary)
├── templates/                          # Project templates (compressed and embedded at build time)
│   └── <template-name>/               # Each template has a template.yaml + project files
│       ├── template.yaml              #   Metadata: name, language, description
│       └── cabidl/                    #   Template content (copied to target dir on init)
├── cabidl/                             # This CABIDL specification
│   ├── cabidl.md
│   ├── clap.yaml
│   ├── ai_provider_trait.rs
│   ├── diagram_provider_trait.rs
│   ├── diagram_trait.rs
│   ├── filesystem_trait.rs
│   ├── init_trait.rs
│   └── parser_trait.rs
├── ai-provider/                        # AiProvider boundary (cabidl-ai-provider)
├── diagram/                            # Diagram boundary (cabidl-diagram)
├── diagram-provider/                   # DiagramProvider boundary (cabidl-diagram-provider)
├── filesystem/                         # Filesystem boundary (cabidl-filesystem)
├── init/                               # Init boundary (cabidl-init)
├── parser/                             # CabidlParser boundary (cabidl-parser)
├── claude-code/                        # ClaudeCode component (cabidl-claude-code)
├── diagram-impl/                       # Diagram component (cabidl-diagram-impl)
├── filesystem-impl/                    # FilesystemImpl component (cabidl-filesystem-impl)
├── graphviz/                           # Graphviz component (cabidl-graphviz)
├── init-impl/                          # InitImpl component (cabidl-init-impl)
├── parser-impl/                        # Parser component (cabidl-parser-impl)
├── cli/                                # Cli component (cabidl binary)
└── tests/
    └── validation_tests.rs
```

---

## Boundary: CliInterface

```yaml
kind: boundary
name: CliInterface
exposure: external
specification:
  path: ./clap.yaml
  typeDescription: CLAP YAML
```

The command-line interface exposed to the user. Defined by `clap.yaml` and implemented directly inside the Cli component crate using clap's derive API.

Global options:

- **--version** / **-V** — Prints version information. The short form (`-V`) prints the CLI version number. The long form (`--version`) prints the CLI version and the supported CABIDL specification version (e.g. `cabidl 0.1.1 (spec 1.1)`). The CLI version is embedded at compile time from `CARGO_PKG_VERSION`; the spec version is a compile-time constant matching `specification.md`.

Five subcommands:

- **read** — Resolves all `<!-- @include -->` directives and writes a single unified CABIDL document to stdout.
- **validate** — Validates document structure, YAML blocks, and boundary reference integrity. Silent on success; errors to stderr with non-zero exit on failure.
- **diagram** — Parses the CABIDL document, generates an architecture diagram in the requested format (`-t/--type`, default `graphviz`), and writes the result to the specified output file (`-o/--output-file`).
- **skill install** — Installs the embedded cabidl skill file to an AI tool provider's skill directory. Accepts an optional `--target-dir` (`-d`) to override the default installation path. The skill file content (`skill.md`) is embedded into the binary at compile time.
- **init** — Scaffolds a new CABIDL project from an embedded template. Accepts `-d/--dir` (target directory, defaults to current directory), `-p/--provider` (AI tool provider, defaults to `claude-code`), and `-t/--template` (template name). When `-t` is omitted, lists all available templates in a table sorted by language then name, showing language, name, and description columns. When `-t` is given, decompresses the embedded templates archive, copies the selected template's contents (excluding `template.yaml`) to the target directory, creates provider-specific files (e.g. `CLAUDE.md` for claude-code), and cleans up the temp directory.

---

## Boundary: Filesystem

```yaml
kind: boundary
name: Filesystem
exposure: internal
specification:
  path: ./filesystem_trait.rs
  typeDescription: Rust Traits
```

Abstraction over filesystem operations so that the parser and diagram components can be tested with in-memory file systems without real I/O. Covers read, write, and directory creation operations. The trait contract is defined in `./filesystem_trait.rs`.

Implemented as the `cabidl-filesystem` crate in `filesystem/`. Contains only the trait — no implementations, no external dependencies.

---

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

---

## Boundary: DiagramProvider

```yaml
kind: boundary
name: DiagramProvider
exposure: internal
specification:
  path: ./diagram_provider_trait.rs
  typeDescription: Rust Traits
```

The diagram provider contract. Each implementer generates diagram content in a specific output format from a parsed System model. The trait and the `DiagramError` type are defined in `./diagram_provider_trait.rs`.

A provider identifies itself via a `diagram_type()` method that returns a string (e.g. `"graphviz"`). The `generate()` method takes a `&System` and returns the diagram content as a `String`, or a `DiagramError` on failure.

Implemented as the `cabidl-diagram-provider` crate in `diagram-provider/`. Contains only the trait and error type — no implementations, no external dependencies beyond `cabidl-parser`.

---

## Boundary: Diagram

```yaml
kind: boundary
name: Diagram
exposure: internal
specification:
  path: ./diagram_trait.rs
  typeDescription: Rust Traits
```

The diagram orchestration contract. Takes a parsed System model, a diagram type string, and an output file path. Selects the appropriate DiagramProvider, generates the content, and writes it to the file. The trait is defined in `./diagram_trait.rs`.

Implemented as the `cabidl-diagram` crate in `diagram/`. Contains only the trait — no implementations. Depends on `cabidl-parser` (for the System type) and `cabidl-diagram-provider` (for the DiagramError type).

---

## Boundary: AiProvider

```yaml
kind: boundary
name: AiProvider
exposure: internal
specification:
  path: ./ai_provider_trait.rs
  typeDescription: Rust Traits
```

The AI tool provider contract. Abstracts operations for installing skill files into AI-powered development tools. Each implementer handles the provider-specific directory structure and conventions. The trait and the `AiProviderError` type are defined in `./ai_provider_trait.rs`.

A provider identifies itself via a `provider_name()` method that returns a string (e.g. `"claude-code"`). The `install_skill()` method takes an optional target directory path and the skill content as a string. When the target directory is `None`, the implementation uses the provider's default location. The `init_project()` method creates provider-specific project files (e.g. `CLAUDE.md` for Claude Code) in the target directory during project initialization.

Implemented as the `cabidl-ai-provider` crate in `ai-provider/`. Contains only the trait and error type — no implementations, no external dependencies.

---

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

---

## Component: Cli

```yaml
kind: component
name: Cli
technology: Rust
boundaries:
  provides:
    - CliInterface
  requires:
    - CabidlParser
    - Diagram
    - AiProvider
    - Init
```

Entry point of the application. Parses command-line arguments, dispatches to the appropriate subcommand, and formats output or errors for the terminal. Contains no domain logic — delegates parsing and validation to the CabidlParser boundary, diagram generation to the Diagram boundary, skill installation to the AiProvider boundary, and project initialization to the Init boundary. The skill file (`skill.md` at the workspace root) is embedded into the binary at compile time via `include_str!("../skill.md")` and passed to the AiProvider's `install_skill()` method when the `skill install` subcommand is invoked. For the `init` subcommand, the Cli delegates template listing to `Init::list_templates()`, scaffolding to `Init::scaffold()`, and provider-specific file creation to `AiProvider::init_project()`.

Implemented as the `cabidl` binary crate in `cli/`. Depends on `cabidl-parser`, `cabidl-parser-impl`, `cabidl-filesystem-impl`, `cabidl-diagram`, `cabidl-diagram-impl`, `cabidl-ai-provider`, `cabidl-claude-code`, `cabidl-init`, `cabidl-init-impl`, and `clap`.

---

## Component: Parser

```yaml
kind: component
name: Parser
technology: Rust
boundaries:
  provides:
    - CabidlParser
  requires:
    - Filesystem
```

Implements all CABIDL parsing and validation logic:

- Resolves `<!-- @include -->` directives recursively with circular include detection
- Extracts YAML blocks from markdown, tracking line numbers for error reporting
- Builds the `System` graph model — parses YAML blocks into `Boundary` and `Component` instances, then resolves boundary name references into `Arc` links
- Provides a pure `parse_content()` function that takes a fully-resolved CABIDL string and returns a `System` model — this is the primary entry point for testing
- Validates YAML block structure, name uniqueness, boundary reference integrity, and exposure values
- Reports errors in compiler-style `file:line: message` format

Implemented as the `cabidl-parser-impl` crate in `parser-impl/`. Depends on `cabidl-parser`, `cabidl-filesystem`, `serde`, `serde_yaml`, and `regex`.

---

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

---

## Component: Diagram

```yaml
kind: component
name: Diagram
technology: Rust
boundaries:
  provides:
    - Diagram
  requires:
    - DiagramProvider
    - Filesystem
```

Implements the Diagram boundary. Holds a registry of DiagramProvider implementations. On invocation, selects the provider matching the requested diagram type, calls it to generate content, and writes the result to the output file via the Filesystem boundary.

Implemented as the `cabidl-diagram-impl` crate in `diagram-impl/`. Depends on `cabidl-diagram`, `cabidl-diagram-provider`, and `cabidl-filesystem`.

---

## Component: Graphviz

```yaml
kind: component
name: Graphviz
technology: Rust
boundaries:
  provides:
    - DiagramProvider
```

Implements the DiagramProvider boundary for Graphviz DOT format. Takes a System model and generates a DOT language string representing the architecture as a cluster-based graph. Uses a Catppuccin Mocha dark mode color scheme.

### Rendering rules

- **Boundaries as clusters** — Each boundary is rendered as a `subgraph cluster_*` container, not a standalone node. The cluster label is the boundary name.
  - External boundaries: bold red border (`#f38ba8`), red label.
  - Internal boundaries: gray border (`#585b70`), gray label.
- **Component placement** — A component is placed inside the cluster of its first `provides` boundary. If it provides additional boundaries, those are shown as explicit green "provides" edges to the other clusters. Components with no `provides` are free-floating outside all clusters.
- **Requires edges** — Dashed blue (`#89b4fa`) arrows from the requiring component to the target boundary's cluster border. Uses Graphviz `compound=true` with `lhead` on edges targeting an invisible anchor node inside each cluster.
- **Anchor nodes** — Each cluster contains an invisible anchor node (`_anchor:BoundaryName`) used as the edge endpoint so that `lhead` can route arrows to the cluster border.

Implemented as the `cabidl-graphviz` crate in `graphviz/`. Depends on `cabidl-diagram-provider` and `cabidl-parser`.

---

## Component: ClaudeCode

```yaml
kind: component
name: ClaudeCode
technology: Rust
boundaries:
  provides:
    - AiProvider
  requires:
    - Filesystem
```

Implements the AiProvider boundary for Claude Code. Handles the Claude Code skill folder structure: skills are installed to `<target_dir>/cabidl/SKILL.md`. When no target directory is provided, defaults to `~/.claude/skills/`.

The `provider_name()` returns `"claude-code"`. The `install_skill()` method creates the necessary directory structure and writes the skill content to the correct path. The `init_project()` method creates a `CLAUDE.md` file in the target directory via the Filesystem boundary with the following content:

```
* The specification for how this software is built is defined in cabidl/cabidl.md and it use /cabidl skill to understand how to build a project with cabidl specifications and that skill will use the cabidl cli that you have installed.
```

Implemented as the `cabidl-claude-code` crate in `claude-code/`. Depends on `cabidl-ai-provider` and `cabidl-filesystem`.

---

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

---

**End of CABIDL specification**
