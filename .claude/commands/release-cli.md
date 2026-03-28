# Release CLI

Walk through the cabidl CLI release process step by step. This is a guided workflow — pause for user input between steps because releases often include files beyond the version bump.

## Steps

### 1. Show current state

Read `cabidl-cli/version.txt` to display the current version. Run `git status` and `git log --oneline -5` so the user can see what's staged, unstaged, and recent history.

Report all of this to the user before proceeding.

### 2. Ask for the new version

Ask the user what the new version should be (semver format: MAJOR.MINOR.PATCH). Suggest the next patch version as a default.

### 3. Bump version files

Update these two files with the new version:
- `cabidl-cli/version.txt` — write the new version string
- `cabidl-cli/Cargo.toml` — update the `version = "..."` line under `[workspace.package]`

If the spec version in `cabidl-cli/cli/src/main.rs` (the `LONG_VERSION` const) needs updating, do that too and mention it.

Run `cargo check --manifest-path cabidl-cli/Cargo.toml` to verify the workspace compiles with the new version.

### 4. Stage and commit

Show `git status` so the user can see all changes. Ask if there are additional files to include in the release commit.

Stage the version files and any other files the user specifies. Create a commit with the message `Bump version to X.Y.Z`.

### 5. Tag

Create an annotated git tag: `git tag vX.Y.Z`

### 6. Push

Before pushing, show the user what will be pushed (the commit and tag). Ask for confirmation.

Then push the commit and tag:
```
git push
git push origin vX.Y.Z
```

Report that the release workflow has been triggered and link to the GitHub Actions page if the remote URL is available.
