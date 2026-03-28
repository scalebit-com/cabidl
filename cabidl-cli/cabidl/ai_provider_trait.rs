use std::fmt;
use std::path::Path;

/// An error that occurs during AI provider operations.
#[derive(Debug)]
pub struct AiProviderError {
    pub message: String,
}

impl fmt::Display for AiProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// The AiProvider boundary trait.
///
/// Abstracts AI tool provider operations such as installing skill files.
/// Each implementer handles the provider-specific directory structure and
/// conventions (e.g. Claude Code uses `.claude/skills/<name>/SKILL.md`).
pub trait AiProvider {
    /// Returns the provider name (e.g. "claude-code").
    fn provider_name(&self) -> &str;

    /// Install a skill file to the provider's skill directory.
    ///
    /// - `target_dir`: Optional override for the base directory. When `None`,
    ///   the implementation uses the provider's default location (e.g.
    ///   `~/.claude/skills/` for Claude Code).
    /// - `skill_content`: The skill file content to install.
    fn install_skill(
        &self,
        target_dir: Option<&Path>,
        skill_content: &str,
    ) -> Result<(), AiProviderError>;
}
