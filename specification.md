# CABIDL Specification

**Component Architecture Boundary and Interface Definition Language**
Version: 1.0

---

## Document Format

A CABIDL document is a **Markdown file** (`.md`) with **embedded YAML code blocks** that carry structured, machine-readable data. The Markdown prose provides context, rationale, and human-readable descriptions. The YAML blocks provide the structured architecture data.

Sections are delimited by `---` (triple dashes on their own line). Each section typically follows this structure:

1. A Markdown heading (e.g. `## Boundary: UserAPI`)
2. A fenced YAML code block containing structured data
3. Optional prose describing the section in detail

---

## Block Types

Every YAML block contains a `kind` field that identifies its type. There are three block types:

### System

Declares the system being described. There should be exactly one system block per CABIDL document. The system description is written in the Markdown prose following the YAML block.

| Field  | Type   | Required | Description              |
|--------|--------|----------|--------------------------|
| `kind` | string | yes      | Must be `"system"`       |
| `name` | string | yes      | The name of the system   |

Schema: `cabidl-system.schema.json`

### Boundary

Defines an architectural boundary — an interface or contract between components. Boundaries represent the seams in a system where one component meets another. The `specification` property points to a formal definition of the contract.

| Field                        | Type   | Required | Description                                                   |
|------------------------------|--------|----------|---------------------------------------------------------------|
| `kind`                       | string | yes      | Must be `"boundary"`                                          |
| `name`                       | string | yes      | The name of the boundary                                      |
| `exposure`                   | string | no       | `"external"` (user/world-facing) or `"internal"` (component-to-component) |
| `specification.path`         | string | no       | Relative path to the file or directory containing the spec    |
| `specification.typeDescription` | string | no    | Description of the specification type (e.g. "OpenAPI Schema") |

Schema: `cabidl-boundary.schema.json`

The `typeDescription` is intentionally free-form. Examples include: "OpenAPI Schema", "CLAP YAML", "Rust Traits", "gRPC Proto", "GraphQL SDL", "JSON Schema", "TypeScript Types".

### Component

Defines a building block of the system. A component provides and/or requires boundaries, connecting it to other components through those contracts.

| Field                  | Type     | Required | Description                                           |
|------------------------|----------|----------|-------------------------------------------------------|
| `kind`                 | string   | yes      | Must be `"component"`                                 |
| `name`                 | string   | yes      | The name of the component                             |
| `technology`           | string   | no       | The technology stack (e.g. "Rust", "Java + Spring")   |
| `boundaries.provides`  | string[] | no       | Boundary names this component exposes                 |
| `boundaries.requires`  | string[] | no       | Boundary names this component depends on              |

Schema: `cabidl-component.schema.json`

---

## Include Directive

A CABIDL document can be split across multiple Markdown files using the include directive:

```
<!-- @include ./path/to/file.md -->
```

When a processor encounters this directive, it replaces it with the full contents of the referenced file. Paths are relative to the file containing the directive.

This uses HTML comment syntax so that the directive is invisible when the Markdown is rendered normally. It works similarly to C's `#include` — the included file's content is inlined at the point of inclusion.

---

## Design Principles

- **Markdown is the core.** The prose carries meaning. The YAML provides structure for tooling.
- **Single source of truth.** One CABIDL document (or a set of included files) describes the entire system architecture.
- **AI-friendly.** The format is designed to be consumed and interpreted by both humans and AI. Field values like `typeDescription` are intentionally free-form to allow natural language descriptions.
- **Minimal structure.** Only three block types. Complexity lives in the prose and the referenced specifications, not in the CABIDL format itself.

---
