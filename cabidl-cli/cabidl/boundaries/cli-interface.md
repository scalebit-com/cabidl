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

- **--version** / **-V** — Prints version information. The short form (`-V`) prints the CLI version number. The long form (`--version`) prints the CLI version and the supported CABIDL specification version (e.g. `cabidl 0.1.1 (spec 1.1)`). The CLI version is embedded at compile time from `CARGO_PKG_VERSION`; the spec version is a compile-time constant defined in `main.rs`.

Five subcommands:

- **read** — Resolves all `<!-- @include -->` directives and writes a single unified CABIDL document to stdout.
- **validate** — Validates document structure, YAML blocks, and boundary reference integrity. Silent on success; errors to stderr with non-zero exit on failure.
- **diagram** — Parses the CABIDL document, generates an architecture diagram in the requested format (`-f/--format`, default `graphviz`, also accepts `mermaid`), and writes the result to the specified output file (`-o/--output-file`). An optional `-t/--diagram-type` flag selects the diagram sub-type within the chosen format: `dark` (default) or `light` for graphviz, `c4` (default) or `class` for mermaid. When `-t` is omitted, each format uses its own default sub-type.
- **skill install** — Installs the embedded cabidl skill file to an AI tool provider's skill directory. Accepts an optional `--target-dir` (`-d`) to override the default installation path. The skill file content (`skill.md`) is embedded into the binary at compile time.
- **init** — Scaffolds a new CABIDL project from an embedded template. Accepts `-d/--dir` (target directory, defaults to current directory), `-p/--provider` (AI tool provider, defaults to `claude-code`), and `-t/--template` (template name). When `-t` is omitted, lists all available templates in a table sorted by language then name, showing language, name, and description columns. When `-t` is given, decompresses the embedded templates archive, copies the selected template's contents (excluding `template.yaml`) to the target directory, and creates provider-specific files (e.g. `CLAUDE.md` for claude-code).
