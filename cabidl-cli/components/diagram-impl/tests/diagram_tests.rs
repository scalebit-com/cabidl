use std::sync::Arc;

use cabidl_diagram::Diagram;
use cabidl_diagram_impl::DiagramImpl;
use cabidl_graphviz::GraphvizProvider;
use cabidl_parser::{Boundary, Component, System};

fn test_system() -> System {
    let boundary = Arc::new(Boundary {
        name: "Api".to_string(),
        exposure: Some("external".to_string()),
        specification_path: None,
        specification_type: None,
        line: None,
    });
    let component = Arc::new(Component {
        name: "Server".to_string(),
        technology: Some("Rust".to_string()),
        provides: vec![Arc::clone(&boundary)],
        requires: vec![],
        line: None,
    });
    System {
        name: "test-system".to_string(),
        boundaries: vec![boundary],
        components: vec![component],
        line: None,
    }
}

#[test]
fn generate_with_known_type_returns_content() {
    let diagram = DiagramImpl::new(vec![Box::new(GraphvizProvider)]);
    let result = diagram.generate(&test_system(), "graphviz");
    assert!(result.is_ok());
    let content = result.unwrap();
    assert!(content.contains("digraph"));
}

#[test]
fn generate_with_unknown_type_returns_error() {
    let diagram = DiagramImpl::new(vec![Box::new(GraphvizProvider)]);
    let result = diagram.generate(&test_system(), "mermaid");
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.message.contains("Unknown diagram type: 'mermaid'"));
}
