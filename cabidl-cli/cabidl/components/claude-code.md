## Component: ClaudeCodeProvider

```yaml
kind: component
name: ClaudeCodeProvider
technology: Rust
boundaries:
  provides:
    - AiProvider
  requires:
    - Filesystem
```

Implements the AiProvider boundary for Claude Code. Handles the Claude Code skill folder structure: skills are installed to `<target_dir>/cabidl/SKILL.md`. When no target directory is provided, defaults to `~/.claude/skills/`.

The `provider_name()` returns `"claude-code"`. The `install_skill()` method creates the necessary directory structure and writes the skill content to the correct path. The `init_project()` method creates a `CLAUDE.md` file in the target directory via the Filesystem boundary with the following content:

```
* The specification for how this software is built is defined in cabidl/cabidl.md and it use /cabidl skill to understand how to build a project with cabidl specifications and that skill will use the cabidl cli that you have installed.
```

Implemented as the `cabidl-claude-code` crate in `claude-code/`. Depends on `cabidl-ai-provider` and `cabidl-filesystem`.
