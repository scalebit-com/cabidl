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
│   ├── boundaries/                    #   Boundary specifications (included by cabidl.md)
│   ├── components/                    #   Component specifications (included by cabidl.md)
│   ├── ai_provider_trait.rs
│   ├── diagram_provider_trait.rs
│   ├── diagram_trait.rs
│   ├── filesystem_trait.rs
│   ├── init_trait.rs
│   └── parser_trait.rs
├── boundaries/                         # Boundary trait crates
│   ├── ai-provider/                   #   AiProvider boundary (cabidl-ai-provider)
│   ├── diagram/                       #   Diagram boundary (cabidl-diagram)
│   ├── diagram-provider/              #   DiagramProvider boundary (cabidl-diagram-provider)
│   ├── filesystem/                    #   Filesystem boundary (cabidl-filesystem)
│   ├── init/                          #   Init boundary (cabidl-init)
│   └── parser/                        #   CabidlParser boundary (cabidl-parser)
└── components/                         # Implementation crates
    ├── cli/                           #   Cli component (cabidl binary)
    ├── claude-code/                   #   ClaudeCodeProvider component (cabidl-claude-code)
    ├── diagram-impl/                  #   Diagram component (cabidl-diagram-impl)
    ├── filesystem-impl/               #   FilesystemImpl component (cabidl-filesystem-impl)
    ├── graphviz/                      #   Graphviz component (cabidl-graphviz)
    ├── init-impl/                     #   InitImpl component (cabidl-init-impl)
    └── parser-impl/                   #   Parser component (cabidl-parser-impl)
```

---

<!-- @include ./boundaries/cli-interface.md -->

---

<!-- @include ./boundaries/filesystem.md -->

---

<!-- @include ./boundaries/parser.md -->

---

<!-- @include ./boundaries/diagram-provider.md -->

---

<!-- @include ./boundaries/diagram.md -->

---

<!-- @include ./boundaries/ai-provider.md -->

---

<!-- @include ./boundaries/init.md -->

---

<!-- @include ./boundaries/wiring.md -->

---

<!-- @include ./components/cli.md -->

---

<!-- @include ./components/parser.md -->

---

<!-- @include ./components/filesystem-impl.md -->

---

<!-- @include ./components/diagram.md -->

---

<!-- @include ./components/graphviz.md -->

---

<!-- @include ./components/claude-code.md -->

---

<!-- @include ./components/init-impl.md -->

---

<!-- @include ./components/wiring-impl.md -->

---

**End of CABIDL specification**
