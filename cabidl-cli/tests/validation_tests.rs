use cabidl::filesystem::InMemoryFilesystem;
use cabidl::parser::{parse, parse_content, resolve};
use cabidl::types::ValidationError;
use cabidl::validator::validate;
use std::path::Path;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_ok(content: &str) -> cabidl::types::CabidlDocument {
    parse_content(content, "test.md").expect("expected successful parse")
}

fn parse_err(content: &str) -> Vec<ValidationError> {
    parse_content(content, "test.md").expect_err("expected parse error")
}

fn parse_and_validate(content: &str) -> Vec<ValidationError> {
    let doc = parse_ok(content);
    validate(&doc, "test.md")
}

// ---------------------------------------------------------------------------
// Document structure tests
// ---------------------------------------------------------------------------

#[test]
fn test_empty_document() {
    let errors = parse_err("");
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("No system block found"));
}

#[test]
fn test_no_yaml_blocks() {
    let content = "# My System\n\nSome prose about the system.\n\n---\n\nMore prose.\n";
    let errors = parse_err(content);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("No system block found"));
}

#[test]
fn test_missing_system_block() {
    let content = "\
```yaml
kind: boundary
name: Api
```
";
    let errors = parse_err(content);
    assert!(errors.iter().any(|e| e.message.contains("No system block found")));
}

#[test]
fn test_multiple_system_blocks() {
    let content = "\
```yaml
kind: system
name: first
```

---

```yaml
kind: system
name: second
```
";
    let errors = parse_err(content);
    assert!(errors.iter().any(|e| e.message.contains("Multiple system blocks found")));
}

#[test]
fn test_valid_minimal_document() {
    let content = "\
```yaml
kind: system
name: minimal
```
";
    let doc = parse_ok(content);
    assert_eq!(doc.system.name, "minimal");
    assert!(doc.boundaries.is_empty());
    assert!(doc.components.is_empty());
}

#[test]
fn test_valid_full_document() {
    let content = "\
# my-system

```yaml
kind: system
name: my-system
```

A test system.

---

## Boundary: Api

```yaml
kind: boundary
name: Api
exposure: external
```

---

## Component: Server

```yaml
kind: component
name: Server
technology: Rust
boundaries:
  provides:
    - Api
```
";
    let doc = parse_ok(content);
    assert_eq!(doc.system.name, "my-system");
    assert_eq!(doc.boundaries.len(), 1);
    assert_eq!(doc.boundaries[0].name, "Api");
    assert_eq!(doc.components.len(), 1);
    assert_eq!(doc.components[0].name, "Server");

    // Validate should pass too
    let errors = validate(&doc, "test.md");
    assert!(errors.is_empty(), "expected no validation errors, got: {:?}", errors);
}

// ---------------------------------------------------------------------------
// YAML parsing tests
// ---------------------------------------------------------------------------

#[test]
fn test_invalid_yaml_syntax() {
    let content = "\
```yaml
kind: system
  bad indentation: [
```
";
    let errors = parse_err(content);
    assert!(errors.iter().any(|e| e.message.contains("Invalid YAML")));
}

#[test]
fn test_missing_kind_field() {
    let content = "\
```yaml
name: something
```
";
    let errors = parse_err(content);
    assert!(errors.iter().any(|e| e.message.contains("missing required 'kind' field")));
}

#[test]
fn test_unknown_block_kind() {
    let content = "\
```yaml
kind: foobar
name: something
```
";
    let errors = parse_err(content);
    assert!(errors.iter().any(|e| e.message.contains("Unknown block kind: 'foobar'")));
}

#[test]
fn test_missing_name_field_system() {
    let content = "\
```yaml
kind: system
```
";
    let errors = parse_err(content);
    assert!(errors.iter().any(|e| e.message.contains("Invalid system block")));
}

#[test]
fn test_missing_name_field_boundary() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: boundary
```
";
    let errors = parse_err(content);
    assert!(errors.iter().any(|e| e.message.contains("Invalid boundary block")));
}

#[test]
fn test_missing_name_field_component() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: component
```
";
    let errors = parse_err(content);
    assert!(errors.iter().any(|e| e.message.contains("Invalid component block")));
}

#[test]
fn test_unknown_fields_rejected() {
    let content = "\
```yaml
kind: system
name: test
extra_field: not_allowed
```
";
    let errors = parse_err(content);
    assert!(errors.iter().any(|e| e.message.contains("Invalid system block")));
}

#[test]
fn test_yaml_inside_non_yaml_fence_ignored() {
    // A yaml-looking block inside a ```rust fence should not be parsed
    let content = "\
```yaml
kind: system
name: test
```

---

```rust
kind: boundary
name: ShouldBeIgnored
```
";
    let doc = parse_ok(content);
    assert_eq!(doc.system.name, "test");
    assert!(doc.boundaries.is_empty());
}

// ---------------------------------------------------------------------------
// Boundary validation tests
// ---------------------------------------------------------------------------

#[test]
fn test_valid_exposure_external() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: boundary
name: Api
exposure: external
```
";
    let errors = parse_and_validate(content);
    assert!(errors.is_empty());
}

#[test]
fn test_valid_exposure_internal() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: boundary
name: Storage
exposure: internal
```
";
    let errors = parse_and_validate(content);
    assert!(errors.is_empty());
}

#[test]
fn test_invalid_exposure_value() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: boundary
name: Api
exposure: public
```
";
    let errors = parse_and_validate(content);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Invalid exposure value 'public'"));
    assert!(errors[0].message.contains("boundary 'Api'"));
}

#[test]
fn test_missing_exposure_is_valid() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: boundary
name: Api
```
";
    let errors = parse_and_validate(content);
    assert!(errors.is_empty());
}

#[test]
fn test_boundary_with_no_referencing_components() {
    // An orphan boundary should not cause errors
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: boundary
name: UnusedBoundary
exposure: internal
```

---

```yaml
kind: component
name: Standalone
```
";
    let errors = parse_and_validate(content);
    assert!(errors.is_empty());
}

// ---------------------------------------------------------------------------
// Component reference integrity tests
// ---------------------------------------------------------------------------

#[test]
fn test_component_provides_undefined_boundary() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: component
name: Server
boundaries:
  provides:
    - NonExistent
```
";
    let errors = parse_and_validate(content);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("provides undefined boundary 'NonExistent'"));
    assert!(errors[0].message.contains("Component 'Server'"));
}

#[test]
fn test_component_requires_undefined_boundary() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: component
name: Client
boundaries:
  requires:
    - MissingDep
```
";
    let errors = parse_and_validate(content);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("requires undefined boundary 'MissingDep'"));
    assert!(errors[0].message.contains("Component 'Client'"));
}

#[test]
fn test_component_with_empty_provides() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: component
name: Worker
boundaries:
  provides: []
```
";
    let errors = parse_and_validate(content);
    assert!(errors.is_empty());
}

#[test]
fn test_component_with_empty_requires() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: component
name: Worker
boundaries:
  requires: []
```
";
    let errors = parse_and_validate(content);
    assert!(errors.is_empty());
}

#[test]
fn test_component_with_no_boundaries_block() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: component
name: Standalone
technology: Rust
```
";
    let errors = parse_and_validate(content);
    assert!(errors.is_empty());
}

// ---------------------------------------------------------------------------
// Include resolution tests (uses InMemoryFilesystem)
// ---------------------------------------------------------------------------

#[test]
fn test_basic_include_resolution() {
    let mut fs = InMemoryFilesystem::new();
    fs.add_file("/project/main.md", "\
# System

```yaml
kind: system
name: test
```

<!-- @include ./boundary.md -->");

    fs.add_file("/project/boundary.md", "\
---

```yaml
kind: boundary
name: Api
exposure: external
```");

    let result = resolve(&fs, Path::new("/project/main.md")).unwrap();
    assert!(result.contains("kind: system"));
    assert!(result.contains("kind: boundary"));
    assert!(result.contains("name: Api"));
}

#[test]
fn test_nested_include_resolution() {
    let mut fs = InMemoryFilesystem::new();
    fs.add_file("/project/main.md", "\
```yaml
kind: system
name: test
```

<!-- @include ./level1.md -->");

    fs.add_file("/project/level1.md", "\
---

```yaml
kind: boundary
name: Api
```

<!-- @include ./level2.md -->");

    fs.add_file("/project/level2.md", "\
---

```yaml
kind: component
name: Server
```");

    let result = resolve(&fs, Path::new("/project/main.md")).unwrap();
    assert!(result.contains("name: test"));
    assert!(result.contains("name: Api"));
    assert!(result.contains("name: Server"));
}

#[test]
fn test_circular_include_detection() {
    let mut fs = InMemoryFilesystem::new();
    fs.add_file("/project/a.md", "\
```yaml
kind: system
name: test
```

<!-- @include ./b.md -->");

    fs.add_file("/project/b.md", "<!-- @include ./a.md -->");

    let errors = resolve(&fs, Path::new("/project/a.md")).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("Circular include detected")));
}

#[test]
fn test_include_nonexistent_file() {
    let mut fs = InMemoryFilesystem::new();
    fs.add_file("/project/main.md", "\
```yaml
kind: system
name: test
```

<!-- @include ./missing.md -->");

    let errors = resolve(&fs, Path::new("/project/main.md")).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("Failed to read included file")));
}

#[test]
fn test_include_inside_code_fence_ignored() {
    let mut fs = InMemoryFilesystem::new();
    fs.add_file("/project/main.md", "\
```yaml
kind: system
name: test
```

```markdown
<!-- @include ./should_not_resolve.md -->
```");

    // Should succeed — the include inside the code fence is not processed
    let result = resolve(&fs, Path::new("/project/main.md")).unwrap();
    assert!(result.contains("@include ./should_not_resolve.md"));
}

// ---------------------------------------------------------------------------
// Include → parse → validate (full pipeline tests)
// ---------------------------------------------------------------------------

#[test]
fn test_include_produces_valid_parsed_document() {
    let mut fs = InMemoryFilesystem::new();
    fs.add_file("/project/main.md", "\
# test-system

```yaml
kind: system
name: test-system
```

A system split across files.

<!-- @include ./boundaries.md -->
<!-- @include ./components.md -->");

    fs.add_file("/project/boundaries.md", "\
---

## Boundary: Api

```yaml
kind: boundary
name: Api
exposure: external
```

---

## Boundary: Storage

```yaml
kind: boundary
name: Storage
exposure: internal
```");

    fs.add_file("/project/components.md", "\
---

## Component: Server

```yaml
kind: component
name: Server
technology: Rust
boundaries:
  provides:
    - Api
  requires:
    - Storage
```

---

## Component: Database

```yaml
kind: component
name: Database
boundaries:
  provides:
    - Storage
```");

    let doc = parse(&fs, Path::new("/project/main.md")).unwrap();
    assert_eq!(doc.system.name, "test-system");
    assert_eq!(doc.boundaries.len(), 2);
    assert_eq!(doc.components.len(), 2);

    let errors = validate(&doc, "/project/main.md");
    assert!(errors.is_empty(), "expected no validation errors, got: {:?}", errors);
}

#[test]
fn test_include_with_undefined_boundary_reports_error() {
    let mut fs = InMemoryFilesystem::new();
    fs.add_file("/project/main.md", "\
```yaml
kind: system
name: test
```

<!-- @include ./comp.md -->");

    fs.add_file("/project/comp.md", "\
---

```yaml
kind: component
name: Broken
boundaries:
  requires:
    - NoBoundaryDefined
```");

    let doc = parse(&fs, Path::new("/project/main.md")).unwrap();
    let errors = validate(&doc, "/project/main.md");
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("requires undefined boundary 'NoBoundaryDefined'"));
}

#[test]
fn test_nested_include_produces_valid_document() {
    let mut fs = InMemoryFilesystem::new();
    fs.add_file("/project/main.md", "\
```yaml
kind: system
name: nested-test
```

<!-- @include ./level1.md -->");

    fs.add_file("/project/level1.md", "\
---

```yaml
kind: boundary
name: Api
exposure: external
```

<!-- @include ./level2.md -->");

    fs.add_file("/project/level2.md", "\
---

```yaml
kind: component
name: Server
boundaries:
  provides:
    - Api
```");

    let doc = parse(&fs, Path::new("/project/main.md")).unwrap();
    assert_eq!(doc.system.name, "nested-test");
    assert_eq!(doc.boundaries.len(), 1);
    assert_eq!(doc.boundaries[0].name, "Api");
    assert_eq!(doc.components.len(), 1);
    assert_eq!(doc.components[0].name, "Server");
    assert_eq!(doc.components[0].provides, vec!["Api"]);

    let errors = validate(&doc, "/project/main.md");
    assert!(errors.is_empty());
}

#[test]
fn test_include_missing_system_block_in_included_file() {
    // System block is in an included file — should still be found
    let mut fs = InMemoryFilesystem::new();
    fs.add_file("/project/main.md", "<!-- @include ./system.md -->");
    fs.add_file("/project/system.md", "\
```yaml
kind: system
name: from-include
```");

    let doc = parse(&fs, Path::new("/project/main.md")).unwrap();
    assert_eq!(doc.system.name, "from-include");
}

#[test]
fn test_include_no_system_block_anywhere() {
    let mut fs = InMemoryFilesystem::new();
    fs.add_file("/project/main.md", "\
# No system here

<!-- @include ./boundary.md -->");

    fs.add_file("/project/boundary.md", "\
```yaml
kind: boundary
name: Api
```");

    let errors = parse(&fs, Path::new("/project/main.md")).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("No system block found")));
}

// ---------------------------------------------------------------------------
// Error quality tests
// ---------------------------------------------------------------------------

#[test]
fn test_error_line_numbers_point_to_correct_block() {
    // First block is valid (line 1), second block is invalid (line 8)
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: foobar
name: bad
```
";
    let errors = parse_err(content);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].line, Some(8), "error should point to line 8 where the bad yaml block starts");
    assert!(errors[0].message.contains("Unknown block kind: 'foobar'"));
}

#[test]
fn test_multiple_errors_in_one_document() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: boundary
name: Api
exposure: bogus
```

---

```yaml
kind: component
name: Server
boundaries:
  provides:
    - NonExistent
```
";
    let doc = parse_ok(content);
    let errors = validate(&doc, "test.md");
    assert_eq!(errors.len(), 2, "expected 2 validation errors, got: {:?}", errors);
    assert!(errors.iter().any(|e| e.message.contains("Invalid exposure")));
    assert!(errors.iter().any(|e| e.message.contains("provides undefined boundary")));
}

#[test]
fn test_error_contains_file_path() {
    let errors = parse_content("", "my/architecture.md")
        .expect_err("expected error");
    assert_eq!(errors[0].file, "my/architecture.md");
}

#[test]
fn test_validator_errors_include_line_numbers() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: boundary
name: Api
exposure: invalid_value
```
";
    let doc = parse_ok(content);
    let errors = validate(&doc, "test.md");
    assert_eq!(errors.len(), 1);
    // The boundary block starts at line 8, so the validator error should carry that line
    assert_eq!(errors[0].line, Some(8));
}

#[test]
fn test_validator_component_error_has_line_number() {
    let content = "\
```yaml
kind: system
name: test
```

---

```yaml
kind: component
name: Broken
boundaries:
  requires:
    - DoesNotExist
```
";
    let doc = parse_ok(content);
    let errors = validate(&doc, "test.md");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].line, Some(8));
    assert!(errors[0].message.contains("requires undefined boundary"));
}

// ---------------------------------------------------------------------------
// Error display format tests
// ---------------------------------------------------------------------------

#[test]
fn test_error_display_format_with_line() {
    let err = ValidationError {
        message: "something went wrong".to_string(),
        file: "arch.md".to_string(),
        line: Some(42),
    };
    assert_eq!(format!("{}", err), "arch.md:42: something went wrong");
}

#[test]
fn test_error_display_format_without_line() {
    let err = ValidationError {
        message: "document-level error".to_string(),
        file: "arch.md".to_string(),
        line: None,
    };
    assert_eq!(format!("{}", err), "arch.md: document-level error");
}
