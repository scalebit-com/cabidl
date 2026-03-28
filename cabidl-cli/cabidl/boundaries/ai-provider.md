## Boundary: AiProvider

```yaml
kind: boundary
name: AiProvider
exposure: internal
specification:
  path: ./ai_provider_trait.rs
  typeDescription: Rust Traits
```

The AI tool provider contract. Abstracts operations for installing skill files into AI-powered development tools. Each implementer handles the provider-specific directory structure and conventions. The trait and the `AiProviderError` type are defined in `./ai_provider_trait.rs`.

A provider identifies itself via a `provider_name()` method that returns a string (e.g. `"claude-code"`). The `install_skill()` method takes an optional target directory path and the skill content as a string. When the target directory is `None`, the implementation uses the provider's default location. The `init_project()` method creates provider-specific project files (e.g. `CLAUDE.md` for Claude Code) in the target directory during project initialization.

Implemented as the `cabidl-ai-provider` crate in `ai-provider/`. Contains only the trait and error type — no implementations, no external dependencies.
