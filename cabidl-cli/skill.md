---
name: cabidl
description: "How to use the cabidl CLI and follow a spec-first architecture development workflow with CABIDL. Use this skill whenever the user mentions cabidl, architecture specifications, boundary definitions, component architecture, cabidl.md files, or wants to validate/read/diagram an architecture spec. Also trigger when the user is working in a project that contains a cabidl/ directory or cabidl.md file and is making architectural changes, adding components or boundaries, or wants to understand the system structure. Also trigger when the user wants to install or manage cabidl skills for AI tool providers."
---

# CABIDL Development Workflow

CABIDL (Component Architecture Boundary and Interface Definition Language) is a spec-first architecture language. The `cabidl` CLI tool validates, reads, and generates diagrams from CABIDL specification files. The specification is the source of truth — implementation follows the spec, not the other way around.

## Finding the specification

Look for the CABIDL specification in this order:
1. `cabidl/cabidl.md` — a `cabidl/` subdirectory in the project root
2. `cabidl.md` — directly in the project root

If neither exists and the user wants to start using CABIDL, help them create one.

## CLI Reference

The `cabidl` tool has five subcommands:

### `cabidl read <file>`
Resolves all `<!-- @include ./path.md -->` directives and outputs a single unified CABIDL document to stdout. This is the primary way to understand the full architecture — it assembles all included files into one view.

Use `read` when you need to understand the complete system architecture, since the spec may be split across multiple files via includes.

```bash
cabidl read cabidl/cabidl.md
```

### `cabidl validate <file>`
Validates the document structure, YAML blocks, boundary reference integrity, and name uniqueness. Silent on success, reports errors with file and line numbers on failure.

Run `validate` after every spec change to catch structural errors early.

```bash
cabidl validate cabidl/cabidl.md
```

### `cabidl diagram <file> -o <output-file> [-f <format>] [-t <diagram-type>]`
Generates an architecture diagram from the spec. Supports Graphviz DOT and Mermaid formats.

- `-f/--format`: Output format — `graphviz` (default) or `mermaid`
- `-t/--diagram-type`: Sub-type within the format:
  - Graphviz: `dark` (default) or `light`
  - Mermaid: `c4` (default) or `class`

```bash
# Graphviz (default format, dark theme)
cabidl diagram cabidl/cabidl.md -o architecture.dot
dot -Tpng architecture.dot -o architecture.png

# Graphviz with light theme
cabidl diagram cabidl/cabidl.md -f graphviz -t light -o architecture.dot

# Mermaid C4 diagram (default mermaid sub-type)
cabidl diagram cabidl/cabidl.md -f mermaid -o architecture.mmd

# Mermaid class diagram
cabidl diagram cabidl/cabidl.md -f mermaid -t class -o architecture.mmd
```

### `cabidl skill install [-d <target-dir>]`
Installs the cabidl SKILL.md to an AI tool provider's skill directory. The skill file is embedded in the binary at compile time.

By default, installs to `~/.claude/skills/cabidl/SKILL.md` (Claude Code's global skill location). Use `-d`/`--target-dir` to override the base directory.

```bash
cabidl skill install                        # install to default location
cabidl skill install -d ./my-project        # install to ./my-project/.claude/skills/cabidl/SKILL.md
```

### `cabidl --version`
Shows the CLI version and the CABIDL specification version it implements.

## CABIDL Block Types

A CABIDL document is a Markdown file with embedded YAML blocks. Sections are separated by `---`. Prose between blocks provides context and rationale. There are three block types:

### System (exactly one per document)

```yaml
kind: system        # required, must be "system"
name: my-system     # required, the name of the system
```

The system description goes in the markdown prose after the YAML block. No other fields are allowed.

### Boundary (interface between components)

```yaml
kind: boundary              # required, must be "boundary"
name: UserApi               # required, unique within the document
exposure: external           # optional: "external" (user-facing) or "internal" (component-to-component)
specification:               # optional, points to the formal contract definition
  path: ./openapi.yaml       #   relative path to the spec file
  typeDescription: OpenAPI Schema  #   free-form description of the spec type
```

The `typeDescription` is intentionally free-form. Common values: "OpenAPI Schema", "Rust Traits", "gRPC Proto", "GraphQL SDL", "CLAP YAML", "JSON Schema", "TypeScript Types".

### Component (building block of the system)

```yaml
kind: component          # required, must be "component"
name: ApiServer          # required, unique within the document
technology: Rust         # optional, the tech stack (e.g. "Java + Spring Boot")
boundaries:              # optional
  provides:              #   boundaries this component exposes
    - UserApi
    - AdminApi
  requires:              #   boundaries this component depends on
    - Database
    - Cache
```

Every boundary name in `provides` and `requires` must match a defined boundary block in the document. No additional fields are allowed on any block type.

### Include directive

Documents can be split across files:

```
<!-- @include ./path/to/file.md -->
```

Includes are resolved recursively. Paths are relative to the containing file. Circular includes are an error.

## Spec-First Development Workflow

The core principle: **the CABIDL spec drives the implementation, not the other way around.** When the user wants to make architectural changes, follow this workflow:

### Phase 1: Update the specification

1. Run `cabidl read` to understand the full current architecture
2. Edit `cabidl.md` and/or any boundary specification files it references to reflect the desired changes — add new boundaries, components, or modify existing ones
3. Run `cabidl validate` after each change to catch errors early
4. Write meaningful prose in the markdown sections — this describes the intent and rationale behind the architecture, not just the structure

Wait for the user to confirm they are satisfied with the spec changes before moving to implementation. The user decides when the spec is ready.

### Phase 2: Implement from the specification

Once the user approves the spec:

1. Run `cabidl read` to get the full unified specification
2. Compare the spec against the current codebase to identify what needs to change:
   - New boundaries need contract files (traits, schemas, protos) at the paths specified in `specification.path`
   - New components need implementation modules/crates using the specified `technology`
   - Dependency relationships in code must match the `provides`/`requires` declarations
3. Implement the changes to align the code with the spec
4. Run `cabidl validate` one final time to confirm everything is consistent

### When to validate

Run `cabidl validate` after:
- Any edit to `cabidl.md` or included files
- Adding or removing boundaries or components
- Changing boundary references in component `provides`/`requires` lists
- Before committing spec changes

### When to use read

Run `cabidl read` when:
- You need to understand the full system architecture (includes are resolved)
- Starting implementation work after spec changes
- The spec uses `<!-- @include -->` directives and you need the complete picture
- The user asks about the system's structure or boundaries
