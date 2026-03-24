# cabidl

**CABIDL Specification**
Version: 2.0

```yaml
kind: system
name: cabidl
```

A command-line tool for working with CABIDL architecture specification files. It reads CABIDL markdown documents, resolves `<!-- @include -->` directives into a single unified output, and validates that all YAML blocks conform to the CABIDL schemas and that boundary references between components are consistent.

The system is implemented as a Rust Cargo workspace. Each boundary is its own crate containing trait definitions and types. Each component is its own crate containing the implementation. Workspace dependencies mirror the `provides`/`requires` relationships.

### Project Structure

```
cabidl-cli/
├── Cargo.toml                          # Workspace root
├── cabidl/                             # This CABIDL specification
│   ├── cabidl.md
│   ├── clap.yaml
│   ├── filesystem_trait.rs
│   └── parser_trait.rs
├── filesystem/                         # Filesystem boundary (cabidl-filesystem)
├── parser/                             # CabidlParser boundary (cabidl-parser)
├── filesystem-impl/                    # FilesystemImpl component (cabidl-filesystem-impl)
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

The command-line interface exposed to the user. Defined by `clap.yaml` and implemented directly inside the Cli component crate using clap's derive API. Two subcommands:

- **read** — Resolves all `<!-- @include -->` directives and writes a single unified CABIDL document to stdout.
- **validate** — Validates document structure, YAML blocks, and boundary reference integrity. Silent on success; errors to stderr with non-zero exit on failure.

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

Abstraction over filesystem operations so that the parser can be tested with in-memory file systems without real I/O. The trait contract is defined in `./filesystem_trait.rs`.

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
```

Entry point of the application. Parses command-line arguments, dispatches to the appropriate subcommand, and formats output or errors for the terminal. Contains no domain logic — delegates all parsing and validation to the CabidlParser boundary.

Implemented as the `cabidl` binary crate in `cli/`. Depends on `cabidl-parser`, `cabidl-parser-impl`, `cabidl-filesystem-impl`, and `clap`.

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

**End of CABIDL specification**
