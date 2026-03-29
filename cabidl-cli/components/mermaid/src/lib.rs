use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use cabidl_diagram_provider::{DiagramError, DiagramProvider};
use cabidl_parser::System;

pub struct MermaidProvider;

fn sanitize_id(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

impl MermaidProvider {
    fn generate_c4(&self, system: &System) -> Result<String, DiagramError> {
        let mut out = String::new();

        writeln!(out, "C4Context").unwrap();
        writeln!(out, "    title {}", system.name).unwrap();
        writeln!(out).unwrap();

        // Build component-to-boundary assignment (same logic as graphviz)
        let mut component_cluster: HashMap<&str, &str> = HashMap::new();
        let mut cluster_components: HashMap<&str, Vec<usize>> = HashMap::new();
        let mut referenced_boundaries = HashSet::new();

        for (i, component) in system.components.iter().enumerate() {
            for b in &component.provides {
                referenced_boundaries.insert(b.name.as_str());
            }
            for b in &component.requires {
                referenced_boundaries.insert(b.name.as_str());
            }
            if let Some(first_provides) = component.provides.first() {
                let boundary_name = first_provides.name.as_str();
                component_cluster.insert(component.name.as_str(), boundary_name);
                cluster_components
                    .entry(boundary_name)
                    .or_default()
                    .push(i);
            }
        }

        // Emit boundaries with their components
        for boundary in &system.boundaries {
            if !referenced_boundaries.contains(boundary.name.as_str()) {
                continue;
            }

            let bid = sanitize_id(&boundary.name);
            let is_external = boundary
                .exposure
                .as_deref()
                .is_some_and(|e| e == "external");

            if is_external {
                writeln!(out, "    Boundary({}, \"{}\") {{", bid, boundary.name).unwrap();

                if let Some(indices) = cluster_components.get(boundary.name.as_str()) {
                    for &idx in indices {
                        let component = &system.components[idx];
                        let cid = sanitize_id(&component.name);
                        let tech = component.technology.as_deref().unwrap_or("");
                        writeln!(out, "        Component_Ext({}, \"{}\", \"{}\")", cid, component.name, tech).unwrap();
                    }
                }

                writeln!(out, "    }}").unwrap();
            } else {
                writeln!(out, "    Boundary({}, \"{}\") {{", bid, boundary.name).unwrap();

                if let Some(indices) = cluster_components.get(boundary.name.as_str()) {
                    for &idx in indices {
                        let component = &system.components[idx];
                        let cid = sanitize_id(&component.name);
                        let tech = component.technology.as_deref().unwrap_or("");
                        writeln!(out, "        Component({}, \"{}\", \"{}\")", cid, component.name, tech).unwrap();
                    }
                }

                writeln!(out, "    }}").unwrap();
            }
            writeln!(out).unwrap();
        }

        // Free-floating components (no provides)
        for component in &system.components {
            if component.provides.is_empty() {
                let cid = sanitize_id(&component.name);
                let tech = component.technology.as_deref().unwrap_or("");
                writeln!(out, "    Component({}, \"{}\", \"{}\")", cid, component.name, tech).unwrap();
            }
        }

        // Requires relationships
        for component in &system.components {
            let cid = sanitize_id(&component.name);
            for boundary in &component.requires {
                let bid = sanitize_id(&boundary.name);
                writeln!(out, "    Rel({}, {}, \"requires\")", cid, bid).unwrap();
            }
        }

        Ok(out)
    }

    fn generate_class(&self, system: &System) -> Result<String, DiagramError> {
        let mut out = String::new();

        writeln!(out, "classDiagram").unwrap();

        // Collect referenced boundaries
        let mut referenced_boundaries = HashSet::new();
        for component in &system.components {
            for b in &component.provides {
                referenced_boundaries.insert(b.name.as_str());
            }
            for b in &component.requires {
                referenced_boundaries.insert(b.name.as_str());
            }
        }

        // Emit boundary classes
        for boundary in &system.boundaries {
            if !referenced_boundaries.contains(boundary.name.as_str()) {
                continue;
            }
            let bid = sanitize_id(&boundary.name);
            writeln!(out, "    class {} {{", bid).unwrap();
            writeln!(out, "        <<boundary>>").unwrap();
            writeln!(out, "    }}").unwrap();
        }

        // Emit component classes
        for component in &system.components {
            let cid = sanitize_id(&component.name);
            writeln!(out, "    class {} {{", cid).unwrap();
            writeln!(out, "        <<component>>").unwrap();
            if let Some(tech) = &component.technology {
                writeln!(out, "        {}", tech).unwrap();
            }
            writeln!(out, "    }}").unwrap();
        }

        // Provides relationships (realization)
        for component in &system.components {
            let cid = sanitize_id(&component.name);
            for boundary in &component.provides {
                let bid = sanitize_id(&boundary.name);
                writeln!(out, "    {} --|> {}", cid, bid).unwrap();
            }
        }

        // Requires relationships (dependency)
        for component in &system.components {
            let cid = sanitize_id(&component.name);
            for boundary in &component.requires {
                let bid = sanitize_id(&boundary.name);
                writeln!(out, "    {} ..> {}", cid, bid).unwrap();
            }
        }

        Ok(out)
    }
}

impl DiagramProvider for MermaidProvider {
    fn diagram_type(&self) -> &str {
        "mermaid"
    }

    fn generate(&self, system: &System, diagram_sub_type: Option<&str>) -> Result<String, DiagramError> {
        match diagram_sub_type {
            None | Some("c4") => self.generate_c4(system),
            Some("class") => self.generate_class(system),
            Some(other) => Err(DiagramError {
                message: format!("Unknown mermaid diagram sub-type: '{}'. Valid values: c4, class", other),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cabidl_parser::{Boundary, Component};
    use std::sync::Arc;

    #[test]
    fn diagram_type_returns_mermaid() {
        let provider = MermaidProvider;
        assert_eq!(provider.diagram_type(), "mermaid");
    }

    #[test]
    fn generate_c4_minimal_system() {
        let system = System {
            name: "test".to_string(),
            boundaries: vec![],
            components: vec![],
            line: None,
        };
        let provider = MermaidProvider;
        let result = provider.generate(&system, None).unwrap();
        assert!(result.contains("C4Context"));
        assert!(result.contains("title test"));
    }

    #[test]
    fn generate_c4_is_default() {
        let system = System {
            name: "test".to_string(),
            boundaries: vec![],
            components: vec![],
            line: None,
        };
        let provider = MermaidProvider;
        let default = provider.generate(&system, None).unwrap();
        let explicit_c4 = provider.generate(&system, Some("c4")).unwrap();
        assert_eq!(default, explicit_c4);
    }

    #[test]
    fn generate_c4_with_boundaries_and_components() {
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

        let provider = MermaidProvider;
        let result = provider.generate(&system, Some("c4")).unwrap();
        assert!(result.contains("Boundary(Api, \"Api\")"));
        assert!(result.contains("Component(Server, \"Server\", \"Rust\")"));
    }

    #[test]
    fn generate_c4_external_boundary() {
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

        let provider = MermaidProvider;
        let result = provider.generate(&system, Some("c4")).unwrap();
        assert!(result.contains("Boundary(PublicApi, \"PublicApi\")"));
        assert!(result.contains("Component_Ext(Gateway, \"Gateway\","));
    }

    #[test]
    fn generate_c4_requires_relationships() {
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

        let provider = MermaidProvider;
        let result = provider.generate(&system, Some("c4")).unwrap();
        assert!(result.contains("Rel(App, Database, \"requires\")"));
    }

    #[test]
    fn generate_class_diagram() {
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

        let provider = MermaidProvider;
        let result = provider.generate(&system, Some("class")).unwrap();
        assert!(result.contains("classDiagram"));
        assert!(result.contains("<<boundary>>"));
        assert!(result.contains("<<component>>"));
        assert!(result.contains("Server --|> Api"));
    }

    #[test]
    fn generate_class_requires_dependency() {
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

        let provider = MermaidProvider;
        let result = provider.generate(&system, Some("class")).unwrap();
        assert!(result.contains("App ..> Database"));
    }

    #[test]
    fn generate_unknown_sub_type_returns_error() {
        let system = System {
            name: "test".to_string(),
            boundaries: vec![],
            components: vec![],
            line: None,
        };
        let provider = MermaidProvider;
        let result = provider.generate(&system, Some("sequence"));
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Unknown mermaid diagram sub-type"));
    }
}
