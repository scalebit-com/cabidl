use std::path::{Path, PathBuf};

use cabidl_ai_provider::{AiProvider, AiProviderError};
use cabidl_filesystem::Filesystem;

pub struct ClaudeCodeProvider {
    fs: Box<dyn Filesystem>,
}

impl ClaudeCodeProvider {
    pub fn new(fs: Box<dyn Filesystem>) -> Self {
        Self { fs }
    }
}

impl AiProvider for ClaudeCodeProvider {
    fn provider_name(&self) -> &str {
        "claude-code"
    }

    fn install_skill(
        &self,
        target_dir: Option<&Path>,
        skill_content: &str,
    ) -> Result<(), AiProviderError> {
        let base = match target_dir {
            Some(dir) => dir.to_path_buf(),
            None => {
                let home = std::env::var("HOME").map_err(|_| AiProviderError {
                    message: "Could not determine home directory".to_string(),
                })?;
                PathBuf::from(home).join(".claude").join("skills")
            }
        };

        let skill_dir = base.join("cabidl");
        let skill_file = skill_dir.join("SKILL.md");

        self.fs.create_dir_all(&skill_dir).map_err(|e| AiProviderError {
            message: format!("Failed to create directory '{}': {}", skill_dir.display(), e),
        })?;

        self.fs
            .write_string(&skill_file, skill_content)
            .map_err(|e| AiProviderError {
                message: format!("Failed to write skill file '{}': {}", skill_file.display(), e),
            })?;

        Ok(())
    }

    fn init_project(
        &self,
        target_dir: &Path,
    ) -> Result<(), AiProviderError> {
        let claude_md = target_dir.join("CLAUDE.md");
        let content = "* The specification for how this software is built is defined in cabidl/cabidl.md and it use /cabidl skill to understand how to build a project with cabidl specifications and that skill will use the cabidl cli that you have installed.\n";
        self.fs
            .write_string(&claude_md, content)
            .map_err(|e| AiProviderError {
                message: format!("Failed to write '{}': {}", claude_md.display(), e),
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cabidl_ai_provider::AiProvider;
    use cabidl_filesystem_impl::InMemoryFilesystem;

    fn create_provider() -> Box<dyn AiProvider> {
        Box::new(ClaudeCodeProvider::new(Box::new(InMemoryFilesystem::new())))
    }

    #[test]
    fn provider_name_is_claude_code() {
        let provider = create_provider();
        assert_eq!(provider.provider_name(), "claude-code");
    }

    #[test]
    fn install_skill_writes_to_target_dir() {
        let fs = InMemoryFilesystem::new();
        let provider: Box<dyn AiProvider> = Box::new(ClaudeCodeProvider::new(Box::new(fs)));
        let result = provider.install_skill(
            Some(Path::new("/test/skills")),
            "# Test Skill\n",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn install_skill_uses_default_when_no_target_dir() {
        let fs = InMemoryFilesystem::new();
        let provider: Box<dyn AiProvider> = Box::new(ClaudeCodeProvider::new(Box::new(fs)));
        let result = provider.install_skill(None, "# Test Skill\n");
        assert!(result.is_ok());
    }

    #[test]
    fn init_project_creates_claude_md() {
        let fs = InMemoryFilesystem::new();
        let provider: Box<dyn AiProvider> = Box::new(ClaudeCodeProvider::new(Box::new(fs)));
        let result = provider.init_project(Path::new("/test/project"));
        assert!(result.is_ok());
    }
}
