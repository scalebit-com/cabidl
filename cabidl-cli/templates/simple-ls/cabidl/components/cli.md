## Component: Cli

```yaml
kind: component
name: Cli
technology: Rust
boundaries:
  provides:
    - CliInterface
  requires:
    - Filesystem
```

The CLI component is the entry point of the application. It parses command-line arguments (via CLAP), calls the filesystem component to list directory contents, and formats the output. When the `--long` flag is present it prints file size, permissions, and modification time.
