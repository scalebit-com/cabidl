# CLAUDE.md

## General
- This project is for a system architecture specification format called CABIDL
- CABIDL stands for Component Architecture Boundary and Interface Definition Language
- The repo contains the specification `specification.md` and `*.json` json schema files in the root that are schemas for the yaml inside any CABIDL specification

## CABIDL CLI
- The architecture spec for the CLI is at `cabidl-cli/cabidl/cabidl.md`
- When developing compile `cabidl-cli` and use `cabidl-cli/target/debug/cabidl` as `cabidl`
- Usually `cabidl`is installed as a cli already but during development we will use the latest debug build
- Use `cabidl read cabidl/cabidl.md` to get the full resolved spec (includes are expanded) in folder `cabidl-cli`
- Use `cabidl validate cabidl/cabidl.md` after any spec change in folder `cabidl-cli`
- **Spec first**: update the spec before writing implementation code, validate, then implement
- Rust workspace in `cabidl-cli/` — boundaries in `boundaries/`, components in `components/`
- All file I/O goes through the `Filesystem` boundary — never use `std::fs` directly in components
- Tests use `InMemoryFilesystem` via `Box<dyn Trait>` — test against boundary traits, not concrete types
- `WiringImpl` is the composition root — `Cli` only depends on `Wiring`
- Build: `cd cabidl-cli && cargo build --release`

## Testing
- Test: `cd cabidl-cli && cargo test`
- Release: use `/release-cli` skill
- An code should try to use pure functions as much as possible to make it testable
- Unit tests should not use I/O at all, either test pure functions or use in memory implementations of traits
