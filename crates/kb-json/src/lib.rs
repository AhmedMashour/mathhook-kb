//! JSON Schema Data Generator
//!
//! Generates JSON files containing schema data for the Vue documentation site.
//! This outputs the schema structure as-is for the frontend to consume.

use kb_core::generator::OutputGenerator;
use kb_core::{Result, Schema};

/// JSON schema data generator
pub struct JsonGenerator;

impl JsonGenerator {
    /// Create a new JSON generator
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputGenerator for JsonGenerator {
    fn name(&self) -> &str {
        "json"
    }

    fn file_extension(&self) -> &str {
        "json"
    }

    fn generate(&self, schema: &Schema) -> Result<String> {
        // Serialize the schema directly to JSON
        let json = serde_json::to_string_pretty(schema)?;
        Ok(json)
    }

    fn validate_output(&self, output: &str) -> Result<()> {
        if output.is_empty() {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "json".to_string(),
                message: "Output is empty".to_string(),
            });
        }

        // Validate it's valid JSON
        serde_json::from_str::<serde_json::Value>(output).map_err(|e| {
            kb_core::KbError::OutputValidationError {
                generator: "json".to_string(),
                message: format!("Invalid JSON: {}", e),
            }
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::schema::*;

    fn create_test_schema() -> Schema {
        Schema {
            topic: "test.example".to_string(),
            title: "Test Example".to_string(),
            description: "A test example".to_string(),
            mathematical_definition: Some("f(x) = x^2".to_string()),
            code_refs: Some(CodeReferences {
                rust: "test::example".to_string(),
                python: "test.example".to_string(),
                nodejs: "test.example".to_string(),
            }),
            examples: vec![Example {
                title: "Power Rule".to_string(),
                explanation: "Test explanation".to_string(),
                code: CodeSnippets {
                    rust: "let f = expr!(x ^ 2);".to_string(),
                    python: "f = x**2".to_string(),
                    nodejs: "const f = x**2;".to_string(),
                },
                expected_output: Some("2*x".to_string()),
            }],
            article: None,
            use_cases: vec![],
            related_topics: vec![],
            performance: None,
            interactive_playground: None,
            outputs: OutputHints::default(),
            metadata: None,
            seo: None,
        }
    }

    #[test]
    fn test_generator_creation() {
        let _ = JsonGenerator::new();
    }

    #[test]
    fn test_generate() {
        let generator = JsonGenerator::new();
        let schema = create_test_schema();
        let result = generator.generate(&schema);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("\"topic\": \"test.example\""));
        assert!(output.contains("\"title\": \"Test Example\""));
    }

    #[test]
    fn test_validate_output() {
        let generator = JsonGenerator::new();
        let schema = create_test_schema();
        let output = generator.generate(&schema).unwrap();
        let validation = generator.validate_output(&output);
        assert!(validation.is_ok());
    }
}
