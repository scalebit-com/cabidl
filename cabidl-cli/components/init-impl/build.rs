use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let templates_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../templates");

    println!("cargo:rerun-if-changed=../../templates");

    // Scan template.yaml files and generate index
    let mut entries = Vec::new();
    if let Ok(dirs) = fs::read_dir(&templates_dir) {
        for dir_entry in dirs.flatten() {
            let template_yaml = dir_entry.path().join("template.yaml");
            if template_yaml.exists() {
                let content = fs::read_to_string(&template_yaml).unwrap();
                let name = extract_yaml_field(&content, "name").unwrap_or_default();
                let language = extract_yaml_field(&content, "language")
                    .or_else(|| extract_yaml_field(&content, "laguage"))
                    .unwrap_or_default();
                let description = extract_yaml_field(&content, "description").unwrap_or_default();
                entries.push((name, language, description));
            }
        }
    }

    // Generate template_index.rs
    let index_path = Path::new(&out_dir).join("template_index.rs");
    let mut index_file = fs::File::create(&index_path).unwrap();
    writeln!(index_file, "fn template_index() -> Vec<cabidl_init::TemplateEntry> {{").unwrap();
    writeln!(index_file, "    vec![").unwrap();
    for (name, language, description) in &entries {
        writeln!(
            index_file,
            "        cabidl_init::TemplateEntry {{ name: \"{}\".to_string(), language: \"{}\".to_string(), description: \"{}\".to_string() }},",
            escape_str(name),
            escape_str(language),
            escape_str(description),
        )
        .unwrap();
    }
    writeln!(index_file, "    ]").unwrap();
    writeln!(index_file, "}}").unwrap();

    // Compress templates directory into tar.gz
    let archive_path = Path::new(&out_dir).join("templates.tar.gz");
    let archive_file = fs::File::create(&archive_path).unwrap();
    let encoder = flate2::write::GzEncoder::new(archive_file, flate2::Compression::default());
    let mut tar_builder = tar::Builder::new(encoder);

    if templates_dir.exists() {
        for dir_entry in fs::read_dir(&templates_dir).unwrap().flatten() {
            if dir_entry.path().is_dir() {
                let dir_name = dir_entry.file_name();
                tar_builder
                    .append_dir_all(dir_name.to_str().unwrap(), dir_entry.path())
                    .unwrap();
            }
        }
    }

    tar_builder.finish().unwrap();
}

fn extract_yaml_field(content: &str, field: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(&format!("{}:", field)) {
            return Some(rest.trim().trim_matches('"').trim_matches('\'').to_string());
        }
    }
    None
}

fn escape_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
