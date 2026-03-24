# CABIDL Specification

**Component Architecture Boundary and Interface Definition Language**
Version: 1.1

---

## Document Format

A CABIDL document is a **Markdown file** named `cabidl.md` with **embedded YAML code blocks** that carry structured, machine-readable data. The Markdown prose provides context, rationale, and human-readable descriptions. The YAML blocks provide the structured architecture data.

Sections are delimited by `---` (triple dashes on their own line). A `---` line inside a fenced code block is not a section delimiter — only `---` at the top level of the Markdown document separates sections. Each section typically follows this structure:

1. A Markdown heading (e.g. `## Boundary: UserAPI`)
2. A fenced YAML code block (using the `` ```yaml `` language tag) containing structured data
3. Optional prose describing the section in detail

Markdown headings and any text before a YAML block within a section are for human readability only. A processor must ignore them — the `kind` and `name` fields inside the YAML block are the authoritative identity of each block. Headings may repeat or paraphrase the YAML name for convenience, but discrepancies between a heading and the YAML `name` field are not an error — the YAML value takes precedence.

YAML blocks must use the `` ```yaml `` language tag on the opening fence. Blocks with other language tags (e.g. `` ```yml ``, `` ```rust ``) are not treated as CABIDL blocks.

No additional properties beyond those defined for each block type are allowed in YAML blocks. A conforming processor must reject YAML blocks that contain unknown fields.

---

## Block Types

Every YAML block contains a `kind` field that identifies its type. There are three block types:

### System

Declares the system being described. There must be exactly one system block per CABIDL document (including any included files). A document with zero system blocks or more than one system block is invalid.

The system description is written in the Markdown prose following the YAML block.

| Field  | Type   | Required | Description              |
|--------|--------|----------|--------------------------|
| `kind` | string | yes      | Must be `"system"`       |
| `name` | string | yes      | The name of the system   |

Schema: `cabidl-system.schema.json`

### Boundary

Defines an architectural boundary — an interface or contract between components. Boundaries represent the seams in a system where one component meets another. The `specification` property points to a formal definition of the contract.

Boundary names must be unique within a document. Two boundary blocks with the same `name` value is an error.

| Field                        | Type   | Required | Description                                                   |
|------------------------------|--------|----------|---------------------------------------------------------------|
| `kind`                       | string | yes      | Must be `"boundary"`                                          |
| `name`                       | string | yes      | The name of the boundary                                      |
| `exposure`                   | string | no       | `"external"` (user/world-facing) or `"internal"` (component-to-component) |
| `specification.path`         | string | no       | Relative path to the file or directory containing the spec    |
| `specification.typeDescription` | string | no    | Description of the specification type (e.g. "OpenAPI Schema") |

Schema: `cabidl-boundary.schema.json`

If the `exposure` field is present, its value must be either `"external"` or `"internal"`. Any other value is an error.

The `typeDescription` is intentionally free-form. Examples include: "OpenAPI Schema", "CLAP YAML", "Rust Traits", "gRPC Proto", "GraphQL SDL", "JSON Schema", "TypeScript Types".

### Component

Defines a building block of the system. A component provides and/or requires boundaries, connecting it to other components through those contracts.

Component names must be unique within a document. Two component blocks with the same `name` value is an error.

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

### Include Processing Rules

- The include directive must appear on its own line. Include directives embedded in inline text or within backtick code spans are not processed.
- Include directives inside fenced code blocks (`` ``` ``) are not processed. This allows documentation to show include syntax as examples without triggering resolution.
- Include resolution is recursive — an included file may itself contain include directives, which are resolved relative to that file's location.
- Circular includes are an error. If file A includes file B which includes file A (directly or transitively), a processor must detect this and report an error rather than entering an infinite loop.
- If the referenced file does not exist or cannot be read, a processor must report an error.

---

## Validation Rules

A conforming CABIDL processor must enforce the following rules when validating a document. A valid document satisfies all of these rules. Errors should be reported with the file path and line number of the offending block, following the format `file:line: message`, similar to compiler diagnostics.

### Structure

1. There must be exactly one system block in the document (after include resolution). Zero or more than one is an error.
2. Every YAML block must contain a `kind` field with a recognized value (`"system"`, `"boundary"`, or `"component"`).
3. Every YAML block must contain all fields marked as required for its block type.
4. YAML blocks must not contain fields beyond those defined for their block type.

### Uniqueness

5. Boundary names must be unique. Two boundaries with the same `name` is an error.
6. Component names must be unique. Two components with the same `name` is an error.

### Referential Integrity

7. Every boundary name listed in a component's `boundaries.provides` must correspond to a defined boundary block in the document.
8. Every boundary name listed in a component's `boundaries.requires` must correspond to a defined boundary block in the document.

### Values

9. If the `exposure` field is present on a boundary, its value must be `"external"` or `"internal"`.

---

## Design Principles

- **Markdown is the core.** The prose carries meaning. The YAML provides structure for tooling.
- **Single source of truth.** One CABIDL document (or a set of included files) describes the entire system architecture. The YAML blocks are the machine-readable source of truth — Markdown headings and prose are for human consumption only.
- **AI-friendly.** The format is designed to be consumed and interpreted by both humans and AI. Field values like `typeDescription` are intentionally free-form to allow natural language descriptions.
- **Minimal structure.** Only three block types. Complexity lives in the prose and the referenced specifications, not in the CABIDL format itself.

---
