## Boundary: CliInterface

```yaml
kind: boundary
name: CliInterface
exposure: external
specification:
  path: ./boundaries/clap.yaml
  typeDescription: CLAP YAML
```

The command-line interface boundary. Defines the arguments and flags the user can pass to the `simple-ls` binary. The `clap.yaml` is a declarative specification of the CLI interface, not a runtime configuration file — it describes the contract that the implementation must satisfy.
