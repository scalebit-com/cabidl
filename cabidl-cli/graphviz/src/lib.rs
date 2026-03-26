use std::collections::HashSet;
use std::fmt::Write;

use cabidl_diagram_provider::{DiagramError, DiagramProvider};
use cabidl_parser::System;

pub struct GraphvizProvider;

impl DiagramProvider for GraphvizProvider {
    fn diagram_type(&self) -> &str {
        "graphviz"
    }

    fn generate(&self, system: &System) -> Result<String, DiagramError> {
        let mut dot = String::new();

        writeln!(dot, "digraph \"{}\" {{", system.name).unwrap();
        writeln!(dot, "    bgcolor=\"#1e1e2e\"").unwrap();
        writeln!(dot, "    fontcolor=\"#cdd6f4\"").unwrap();
        writeln!(dot, "    fontname=\"Helvetica\"").unwrap();
        writeln!(dot, "    label=\"{}\"", system.name).unwrap();
        writeln!(dot, "    labelloc=t").unwrap();
        writeln!(dot, "    fontsize=20").unwrap();
        writeln!(dot).unwrap();

        // Default node styling
        writeln!(
            dot,
            "    node [fontname=\"Helvetica\" fontsize=12 fontcolor=\"#cdd6f4\"]"
        )
        .unwrap();
        writeln!(
            dot,
            "    edge [fontname=\"Helvetica\" fontsize=10 fontcolor=\"#a6adc8\"]"
        )
        .unwrap();
        writeln!(dot).unwrap();

        // Collect boundary names referenced by components to avoid orphan boundary nodes
        let mut referenced_boundaries = HashSet::new();
        for component in &system.components {
            for b in &component.provides {
                referenced_boundaries.insert(&b.name);
            }
            for b in &component.requires {
                referenced_boundaries.insert(&b.name);
            }
        }

        // Emit boundary nodes (prefixed IDs to avoid collision with component names)
        for boundary in &system.boundaries {
            if !referenced_boundaries.contains(&boundary.name) {
                continue;
            }

            let is_external = boundary
                .exposure
                .as_deref()
                .is_some_and(|e| e == "external");

            if is_external {
                writeln!(
                    dot,
                    "    \"boundary:{}\" [shape=ellipse style=\"filled,bold\" fillcolor=\"#1e1e2e\" fontcolor=\"#f38ba8\" color=\"#f38ba8\" label=\"{}\"]",
                    boundary.name, boundary.name
                )
                .unwrap();
            } else {
                writeln!(
                    dot,
                    "    \"boundary:{}\" [shape=ellipse style=filled fillcolor=\"#1e1e2e\" fontcolor=\"#a6adc8\" color=\"#585b70\" label=\"{}\"]",
                    boundary.name, boundary.name
                )
                .unwrap();
            }
        }
        writeln!(dot).unwrap();

        // Emit component nodes and edges (prefixed IDs)
        for component in &system.components {
            let label = match &component.technology {
                Some(tech) => format!("{}\\n[{}]", component.name, tech),
                None => component.name.clone(),
            };

            writeln!(
                dot,
                "    \"component:{}\" [shape=box style=\"filled,rounded\" fillcolor=\"#313244\" fontcolor=\"#cdd6f4\" color=\"#585b70\" label=\"{}\"]",
                component.name, label
            )
            .unwrap();

            for boundary in &component.provides {
                writeln!(
                    dot,
                    "    \"component:{}\" -> \"boundary:{}\" [color=\"#a6e3a1\" label=\"provides\"]",
                    component.name, boundary.name
                )
                .unwrap();
            }

            for boundary in &component.requires {
                writeln!(
                    dot,
                    "    \"component:{}\" -> \"boundary:{}\" [color=\"#89b4fa\" style=dashed label=\"requires\"]",
                    component.name, boundary.name
                )
                .unwrap();
            }
        }

        writeln!(dot, "}}").unwrap();

        Ok(dot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cabidl_parser::{Boundary, Component};
    use std::sync::Arc;

    #[test]
    fn diagram_type_returns_graphviz() {
        let provider = GraphvizProvider;
        assert_eq!(provider.diagram_type(), "graphviz");
    }

    #[test]
    fn generate_minimal_system() {
        let system = System {
            name: "test".to_string(),
            boundaries: vec![],
            components: vec![],
            line: None,
        };
        let provider = GraphvizProvider;
        let result = provider.generate(&system).unwrap();
        assert!(result.contains("digraph \"test\""));
        assert!(result.contains("bgcolor=\"#1e1e2e\""));
    }

    #[test]
    fn generate_with_components_and_boundaries() {
        let boundary = Arc::new(Boundary {
            name: "Api".to_string(),
            exposure: Some("internal".to_string()),
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
        let system = System {
            name: "test".to_string(),
            boundaries: vec![boundary],
            components: vec![component],
            line: None,
        };

        let provider = GraphvizProvider;
        let result = provider.generate(&system).unwrap();

        assert!(result.contains("\"component:Server\""));
        assert!(result.contains("Server\\n[Rust]"));
        assert!(result.contains("\"boundary:Api\""));
        assert!(result.contains("\"component:Server\" -> \"boundary:Api\""));
        assert!(result.contains("provides"));
    }

    #[test]
    fn generate_external_boundary_distinguished() {
        let boundary = Arc::new(Boundary {
            name: "PublicApi".to_string(),
            exposure: Some("external".to_string()),
            specification_path: None,
            specification_type: None,
            line: None,
        });
        let component = Arc::new(Component {
            name: "Gateway".to_string(),
            technology: None,
            provides: vec![Arc::clone(&boundary)],
            requires: vec![],
            line: None,
        });
        let system = System {
            name: "test".to_string(),
            boundaries: vec![boundary],
            components: vec![component],
            line: None,
        };

        let provider = GraphvizProvider;
        let result = provider.generate(&system).unwrap();

        // External boundaries should have red-tinted styling
        assert!(result.contains("\"boundary:PublicApi\" [shape=ellipse style=\"filled,bold\""));
        assert!(result.contains("#f38ba8"));
    }

    #[test]
    fn generate_requires_edges_are_dashed() {
        let boundary = Arc::new(Boundary {
            name: "Database".to_string(),
            exposure: Some("internal".to_string()),
            specification_path: None,
            specification_type: None,
            line: None,
        });
        let component = Arc::new(Component {
            name: "App".to_string(),
            technology: None,
            provides: vec![],
            requires: vec![Arc::clone(&boundary)],
            line: None,
        });
        let system = System {
            name: "test".to_string(),
            boundaries: vec![boundary],
            components: vec![component],
            line: None,
        };

        let provider = GraphvizProvider;
        let result = provider.generate(&system).unwrap();

        assert!(result.contains("\"component:App\" -> \"boundary:Database\""));
        assert!(result.contains("style=dashed"));
        assert!(result.contains("requires"));
    }

    #[test]
    fn generate_same_name_boundary_and_component_no_collision() {
        let boundary = Arc::new(Boundary {
            name: "Diagram".to_string(),
            exposure: Some("internal".to_string()),
            specification_path: None,
            specification_type: None,
            line: None,
        });
        let component = Arc::new(Component {
            name: "Diagram".to_string(),
            technology: Some("Rust".to_string()),
            provides: vec![Arc::clone(&boundary)],
            requires: vec![],
            line: None,
        });
        let system = System {
            name: "test".to_string(),
            boundaries: vec![boundary],
            components: vec![component],
            line: None,
        };

        let provider = GraphvizProvider;
        let result = provider.generate(&system).unwrap();

        // Both should exist as distinct nodes
        assert!(result.contains("\"boundary:Diagram\""));
        assert!(result.contains("\"component:Diagram\""));
        assert!(result.contains("\"component:Diagram\" -> \"boundary:Diagram\""));
    }
}
