# cabidl
Component Architecture Boundary and Interface Definition Language

---

# What is CABIDL?

**CABIDL** stands for **Component Architecture Boundary and Interface Definition Language**.

CABIDL is a textual software architecture language that describes a system in terms of:

- **Components** — the building blocks of your software, what they provide and require
- **Boundaries** — the interfaces and contracts between components, both internal and external
- **Relationships** — how components connect through shared boundaries

Everything lives in a single Markdown file (`cabidl.md`) with embedded YAML blocks — human-readable, machine-parseable, and AI-friendly.

A CABIDL file answers:

- What are the major components in this system?
- What boundaries exist between them?
- Which boundaries are external (user-facing) and which are internal?
- What does each component provide and depend on?
- What technology backs each component?
- Where is the formal specification for each boundary contract?

> **In short:**
> CABIDL is architecture-as-code. Write your system's structure in a markdown file, and it becomes a single source of truth that humans, tools, and AI can all read.

---

# The Format

A CABIDL document is a Markdown file with three types of YAML blocks:

```yaml
kind: system
name: my-system
```

```yaml
kind: boundary
name: Api
exposure: external
specification:
  path: ./openapi.yaml
  typeDescription: OpenAPI Schema
```

```yaml
kind: component
name: Server
technology: Rust
boundaries:
  provides:
    - Api
  requires:
    - Database
```

Sections are separated by `---`. Markdown prose between blocks provides context and rationale. Boundary `specification` fields point to formal contract definitions (OpenAPI schemas, Rust traits, gRPC protos, etc.).

Documents can be split across files using `<!-- @include ./path.md -->` directives.

The full format is defined in [specification.md](specification.md).

---

# Why CABIDL?

**Architecture as a first-class artifact.** Instead of relying on disconnected diagrams, wikis, and tribal knowledge, CABIDL provides a single textual representation that lives in version control alongside your code.

**AI-friendly by design.** The format is designed to be consumed and interpreted by AI. An AI can read a `cabidl.md`, understand the full system architecture, and implement or restructure a codebase from it. Free-form fields like `typeDescription` allow natural language descriptions that both humans and AI understand.

**Tooling-ready.** The structured YAML blocks enable validation, diagram generation, dependency analysis, and conformance checks. The `cabidl` CLI tool validates documents and resolves includes.

---

# When to Use CABIDL

CABIDL is valuable when:

- **Your system has multiple components** — services, modules, adapters, APIs — that interact in nontrivial ways
- **Interfaces matter** — you need exact contracts between components
- **Boundaries matter** — external APIs, trusted boundaries, third-party integration zones
- **Architecture should live in version control** — diffable, reviewable, alongside code
- **A shared source of truth is required** — for multi-team environments
- **You want to generate artifacts** — diagrams, dependency maps, validation, scaffolding

CABIDL is particularly strong for service-oriented systems, modular monoliths, microservice platforms, plugin architectures, and systems requiring strict boundary governance.

---

# When Not to Use CABIDL

CABIDL may not be necessary if your system is small, short-lived, or structurally trivial. It is not a replacement for detailed code, protocol schemas, deployment manifests, or runtime observability. Its purpose is to define architecture, not to cover all engineering details.

---

# The cabidl CLI

The `cabidl` command-line tool works with CABIDL documents:

```
cabidl validate cabidl.md    # Validate structure and references (silent on success)
cabidl read cabidl.md        # Resolve includes, output unified document
```

The tool validates YAML block structure, boundary reference integrity, name uniqueness, and reports errors with file and line number context. See [cabidl-cli/](cabidl-cli/) for the implementation.

### Install

**Homebrew (macOS / Linux):**

```bash
brew install scalebit-com/tools/cabidl
```

Or tap first, then install:

```bash
brew tap scalebit-com/tools
brew install cabidl
```

**Verify:**

```bash
cabidl --version
```

---

# Learn More

- [specification.md](specification.md) — The full CABIDL format specification
- [cabidl-cli/cabidl/cabidl.md](cabidl-cli/cabidl/cabidl.md) — The `cabidl` CLI tool's own architecture, written in CABIDL
- [examples/simple-ls/cabidl.md](examples/simple-ls/cabidl.md) — A minimal example describing a simple `ls` command
