# cabidl

**CABIDL Specification**
Version: 1.0

```yaml
kind: system
name: cabidl
```

A command-line tool for working with CABIDL architecture specification files. It reads CABIDL markdown documents, resolves `<!-- @include -->` directives into a single unified output, and validates that all YAML blocks conform to the CABIDL schemas and that boundary references between components are consistent.

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

The command-line interface exposed to the user. The `clap.yaml` is a declarative specification of the CLI contract, not a runtime configuration file. The CLI has two subcommands:

- **read** — Takes a CABIDL markdown file as input, resolves all `<!-- @include -->` directives recursively, and writes a single unified CABIDL document to stdout.
- **validate** — Takes a CABIDL markdown file as input, validates the document structure, YAML blocks, and boundary reference integrity. Produces no output on success; exits with a non-zero status and error messages on failure.

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

Abstraction over filesystem operations. The parser never reads files directly — it goes through this boundary so that the parsing logic can be tested with in-memory file systems and does not depend on real I/O.

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

The core parsing and validation contract. Provides the ability to parse a CABIDL markdown document into a structured representation, resolve include directives, and validate that all YAML blocks and cross-references are correct. The CLI component depends on this boundary to perform all meaningful work.

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

The entry point of the application. Parses command-line arguments using clap, dispatches to the appropriate subcommand handler, and formats output or errors for the terminal. It delegates all parsing and validation logic to the CabidlParser boundary — the CLI component itself contains no domain logic.

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

Implements the CABIDL parsing and validation logic. Responsibilities:

- Parse markdown into sections delimited by `---`
- Extract and parse YAML code blocks from each section
- Resolve `<!-- @include path -->` directives recursively, detecting circular includes
- Validate each YAML block against the appropriate schema (system, boundary, component) based on the `kind` field
- Validate that all boundary names referenced in component `boundaries.provides` and `boundaries.requires` have a corresponding boundary definition
- Report structured errors with file path and line number context

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

Implements the Filesystem boundary using the Rust standard library (`std::fs`). Reads file contents as UTF-8 strings and resolves relative paths against the directory of the file that contains the include directive.

---

**End of CABIDL specification**
