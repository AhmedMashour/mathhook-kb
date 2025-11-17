/// Schema parser module
///
/// Handles loading and parsing of YAML schema files
use crate::{Result, Schema};
use std::path::Path;

/// Load a single schema from a YAML file
pub fn load_schema(path: &Path) -> Result<Schema> {
    Schema::load_from_file(path)
}

/// Load all schemas from a directory (recursive)
pub fn load_schemas_from_directory(path: &Path) -> Result<Vec<Schema>> {
    Schema::load_from_directory(path)
}

/// Save a schema to a YAML file
pub fn save_schema(schema: &Schema, path: &Path) -> Result<()> {
    schema.save_to_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_schema() {
        let yaml = r#"
topic: "test.function"
title: "Test Function"
description: "A test function"

code_refs:
  rust: "test::function"
  python: "test.function"
  nodejs: "test.function"

examples:
  - title: "Example"
    explanation: "An example"
    code:
      rust: "test()"
      python: "test()"
      nodejs: "test()"
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();

        let schema = load_schema(file.path()).expect("Failed to load schema");
        assert_eq!(schema.topic, "test.function");
        assert_eq!(schema.title, "Test Function");
    }
}
