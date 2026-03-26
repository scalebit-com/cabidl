use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use cabidl_diagram_provider::{DiagramError, DiagramProvider};
use cabidl_parser::System;

pub struct GraphvizProvider;

fn sanitize_id(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

impl DiagramProvider for GraphvizProvider {
    fn diagram_type(&self) -> &str {
        "graphviz"
    }

    fn generate(&self, system: &System) -> Result<String, DiagramError> {
        let mut dot = String::new();

        // --- Preparation: build assignment maps ---

        // Collect referenced boundary names
        let mut referenced_boundaries = HashSet::new();
        for component in &system.components {
            for b in &component.provides {
                referenced_boundaries.insert(b.name.as_str());
            }
            for b in &component.requires {
                referenced_boundaries.insert(b.name.as_str());
            }
        }

        // Assign each component to the cluster of its first provides boundary
        let mut component_cluster: HashMap<&str, &str> = HashMap::new();
        let mut cluster_components: HashMap<&str, Vec<usize>> = HashMap::new();

        for (i, component) in system.components.iter().enumerate() {
            if let Some(first_provides) = component.provides.first() {
                let boundary_name = first_provides.name.as_str();
                component_cluster.insert(component.name.as_str(), boundary_name);
                cluster_components
                    .entry(boundary_name)
                    .or_default()
                    .push(i);
            }
        }

        // --- DOT header ---

        writeln!(dot, "digraph \"{}\" {{", system.name).unwrap();
        writeln!(dot, "    bgcolor=\"#1e1e2e\"").unwrap();
        writeln!(dot, "    fontcolor=\"#cdd6f4\"").unwrap();
        writeln!(dot, "    fontname=\"Helvetica\"").unwrap();
        writeln!(dot, "    label=\"{}\"", system.name).unwrap();
        writeln!(dot, "    labelloc=t").unwrap();
        writeln!(dot, "    fontsize=20").unwrap();
        writeln!(dot, "    compound=true").unwrap();
        writeln!(dot).unwrap();

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

        // --- Emit cluster subgraphs for each referenced boundary ---

        for boundary in &system.boundaries {
            if !referenced_boundaries.contains(boundary.name.as_str()) {
                continue;
            }

            let is_external = boundary
                .exposure
                .as_deref()
                .is_some_and(|e| e == "external");

            let (border_color, font_color, style) = if is_external {
                ("#f38ba8", "#f38ba8", "rounded,bold")
            } else {
                ("#585b70", "#a6adc8", "rounded")
            };

            let cluster_id = sanitize_id(&boundary.name);

            writeln!(dot, "    subgraph cluster_{} {{", cluster_id).unwrap();
            writeln!(dot, "        label=\"{}\"", boundary.name).unwrap();
            writeln!(dot, "        style=\"{}\"", style).unwrap();
            writeln!(dot, "        color=\"{}\"", border_color).unwrap();
            writeln!(dot, "        fontcolor=\"{}\"", font_color).unwrap();
            writeln!(dot, "        fontname=\"Helvetica\"").unwrap();
            writeln!(dot, "        fontsize=14").unwrap();
            writeln!(dot).unwrap();

            // Invisible anchor node for edge targeting
            writeln!(
                dot,
                "        \"_anchor:{}\" [shape=point style=invis width=0 height=0]",
                boundary.name
            )
            .unwrap();

            // Emit component nodes assigned to this cluster
            if let Some(indices) = cluster_components.get(boundary.name.as_str()) {
                for &idx in indices {
                    let component = &system.components[idx];
                    let label = match &component.technology {
                        Some(tech) => format!("{}\\n[{}]", component.name, tech),
                        None => component.name.clone(),
                    };
                    writeln!(
                        dot,
                        "        \"component:{}\" [shape=box style=\"filled,rounded\" fillcolor=\"#313244\" fontcolor=\"#cdd6f4\" color=\"#585b70\" label=\"{}\"]",
                        component.name, label
                    )
                    .unwrap();
                }
            }

            writeln!(dot, "    }}").unwrap();
            writeln!(dot).unwrap();
        }

        // --- Emit free-floating components (no provides) ---

        for component in &system.components {
            if component.provides.is_empty() {
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
            }
        }

        // --- Emit edges ---

        for component in &system.components {
            // Extra provides edges (skip the first, which is the cluster assignment)
            for boundary in component.provides.iter().skip(1) {
                let cluster_id = sanitize_id(&boundary.name);
                writeln!(
                    dot,
                    "    \"component:{}\" -> \"_anchor:{}\" [color=\"#a6e3a1\" label=\"provides\" lhead=\"cluster_{}\"]",
                    component.name, boundary.name, cluster_id
                )
                .unwrap();
            }

            // Requires edges
            for boundary in &component.requires {
                let cluster_id = sanitize_id(&boundary.name);
                writeln!(
                    dot,
                    "    \"component:{}\" -> \"_anchor:{}\" [color=\"#89b4fa\" style=dashed label=\"requires\" lhead=\"cluster_{}\"]",
                    component.name, boundary.name, cluster_id
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
        assert!(result.contains("compound=true"));
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

        // Component should be inside the Api cluster
        assert!(result.contains("subgraph cluster_Api"));
        assert!(result.contains("\"_anchor:Api\""));
        assert!(result.contains("\"component:Server\""));
        assert!(result.contains("Server\\n[Rust]"));
        // No provides edge needed — component is inside its cluster
        assert!(!result.contains("provides"));
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

        // External boundaries should have red border and bold style
        assert!(result.contains("subgraph cluster_PublicApi"));
        assert!(result.contains("style=\"rounded,bold\""));
        assert!(result.contains("color=\"#f38ba8\""));
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

        assert!(result.contains("\"component:App\" -> \"_anchor:Database\""));
        assert!(result.contains("style=dashed"));
        assert!(result.contains("lhead=\"cluster_Database\""));
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

        // Cluster for the boundary and component node inside it — no collision
        assert!(result.contains("subgraph cluster_Diagram"));
        assert!(result.contains("\"component:Diagram\""));
    }

    #[test]
    fn generate_component_provides_multiple_boundaries() {
        let boundary_a = Arc::new(Boundary {
            name: "Auth".to_string(),
            exposure: Some("internal".to_string()),
            specification_path: None,
            specification_type: None,
            line: None,
        });
        let boundary_b = Arc::new(Boundary {
            name: "Sessions".to_string(),
            exposure: Some("internal".to_string()),
            specification_path: None,
            specification_type: None,
            line: None,
        });
        let component = Arc::new(Component {
            name: "AuthServer".to_string(),
            technology: Some("Rust".to_string()),
            provides: vec![Arc::clone(&boundary_a), Arc::clone(&boundary_b)],
            requires: vec![],
            line: None,
        });
        let system = System {
            name: "test".to_string(),
            boundaries: vec![boundary_a, boundary_b],
            components: vec![component],
            line: None,
        };

        let provider = GraphvizProvider;
        let result = provider.generate(&system).unwrap();

        // Component should be inside the first cluster (Auth)
        assert!(result.contains("subgraph cluster_Auth"));
        assert!(result.contains("subgraph cluster_Sessions"));
        // Explicit provides edge to the second boundary
        assert!(result.contains("\"component:AuthServer\" -> \"_anchor:Sessions\""));
        assert!(result.contains("lhead=\"cluster_Sessions\""));
        assert!(result.contains("provides"));
    }

    #[test]
    fn generate_requires_only_component_outside_clusters() {
        let boundary = Arc::new(Boundary {
            name: "Storage".to_string(),
            exposure: Some("internal".to_string()),
            specification_path: None,
            specification_type: None,
            line: None,
        });
        let provider_component = Arc::new(Component {
            name: "DiskStore".to_string(),
            technology: None,
            provides: vec![Arc::clone(&boundary)],
            requires: vec![],
            line: None,
        });
        let consumer_component = Arc::new(Component {
            name: "App".to_string(),
            technology: None,
            provides: vec![],
            requires: vec![Arc::clone(&boundary)],
            line: None,
        });
        let system = System {
            name: "test".to_string(),
            boundaries: vec![boundary],
            components: vec![provider_component, consumer_component],
            line: None,
        };

        let provider = GraphvizProvider;
        let result = provider.generate(&system).unwrap();

        // DiskStore should be inside the Storage cluster
        assert!(result.contains("subgraph cluster_Storage"));

        // App should be outside clusters (free-floating) with a requires edge
        assert!(result.contains("\"component:App\" -> \"_anchor:Storage\""));
        assert!(result.contains("lhead=\"cluster_Storage\""));

        // Verify App node is emitted outside the cluster by checking it appears
        // after the cluster closing brace
        let cluster_end = result.find("    }").unwrap();
        let app_node = result.find("\"component:App\" [shape=box").unwrap();
        assert!(app_node > cluster_end);
    }
}
