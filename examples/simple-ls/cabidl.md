# simple-ls

**CABIDL Specification**
Version: 1.0

```yaml
kind: system
name: simple-ls
```

A minimal reimplementation of the Unix `ls` command. Lists directory contents with an optional long format flag.

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

The command-line interface boundary. Defines the arguments and flags the user can pass to the `simple-ls` binary. The `clap.yaml` is a declarative specification of the CLI interface, not a runtime configuration file — it describes the contract that the implementation must satisfy.

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

The filesystem abstraction boundary. Defines the contract for reading directory entries and file metadata. This boundary exists so that the CLI logic is decoupled from direct filesystem calls, making the system testable and extensible.

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
    - Filesystem
```

The CLI component is the entry point of the application. It parses command-line arguments (via CLAP), calls the filesystem component to list directory contents, and formats the output. When the `--long` flag is present it prints file size, permissions, and modification time.

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

The filesystem component implements the `Filesystem` boundary using the Rust standard library (`std::fs`). It reads directory entries and returns file metadata. It has no external dependencies beyond the OS.

---

**End of CABIDL specification**
