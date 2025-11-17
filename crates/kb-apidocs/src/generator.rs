use kb_core::generator::OutputGenerator;
/// API documentation generator implementation
///
/// Generates API reference documentation in OpenAPI 3.0 format with code examples
/// and interactive playground support.
use kb_core::{Result, Schema};
use serde_json::json;

/// API documentation generator
pub struct ApiDocsGenerator;

impl ApiDocsGenerator {
    /// Create a new API docs generator
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Generate OpenAPI specification
    fn generate_openapi(&self, schema: &Schema) -> Result<String> {
        let topic_path = schema.topic.replace('.', "/");
        let category = self.extract_category(&schema.topic);

        let spec = json!({
            "openapi": "3.0.0",
            "info": {
                "title": schema.title,
                "description": schema.description,
                "version": "1.0.0"
            },
            "servers": [
                {
                    "url": "https://api.mathhook.org/v1",
                    "description": "Production server"
                }
            ],
            "paths": {
                format!("/{}", topic_path): {
                    "post": {
                        "summary": schema.title.clone(),
                        "description": schema.description.clone(),
                        "requestBody": {
                            "required": true,
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/Request"
                                    },
                                    "examples": self.generate_request_examples(schema)
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "Successful computation",
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": "#/components/schemas/Response"
                                        },
                                        "examples": self.generate_response_examples(schema)
                                    }
                                }
                            },
                            "400": {
                                "description": "Invalid input",
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": "#/components/schemas/Error"
                                        }
                                    }
                                }
                            }
                        },
                        "tags": [category.clone()]
                    }
                }
            },
            "components": {
                "schemas": {
                    "Request": {
                        "type": "object",
                        "required": ["expression"],
                        "properties": {
                            "expression": {
                                "type": "string",
                                "description": "Mathematical expression to evaluate"
                            },
                            "variables": {
                                "type": "object",
                                "description": "Variable values for evaluation"
                            }
                        }
                    },
                    "Response": {
                        "type": "object",
                        "properties": {
                            "result": {
                                "type": "string",
                                "description": "Computed result"
                            },
                            "latex": {
                                "type": "string",
                                "description": "LaTeX representation"
                            },
                            "steps": {
                                "type": "array",
                                "description": "Step-by-step solution",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "description": { "type": "string" },
                                        "expression": { "type": "string" }
                                    }
                                }
                            }
                        }
                    },
                    "Error": {
                        "type": "object",
                        "properties": {
                            "error": {
                                "type": "string",
                                "description": "Error message"
                            },
                            "code": {
                                "type": "string",
                                "description": "Error code"
                            }
                        }
                    }
                }
            },
            "tags": [
                {
                    "name": category,
                    "description": format!("{} operations", category)
                }
            ]
        });

        Ok(serde_json::to_string_pretty(&spec)?)
    }

    fn generate_request_examples(&self, schema: &Schema) -> serde_json::Value {
        let mut examples = serde_json::Map::new();

        for (idx, example) in schema.examples.iter().enumerate() {
            let example_name = example.title.replace(' ', "_").to_lowercase();
            examples.insert(
                example_name,
                json!({
                    "summary": example.title,
                    "description": example.explanation,
                    "value": {
                        "expression": self.extract_expression_from_code(&example.code.python)
                    }
                }),
            );

            if idx >= 2 {
                break;
            }
        }

        json!(examples)
    }

    fn generate_response_examples(&self, schema: &Schema) -> serde_json::Value {
        let mut examples = serde_json::Map::new();

        for (idx, example) in schema.examples.iter().enumerate() {
            if let Some(output) = &example.expected_output {
                let example_name = example.title.replace(' ', "_").to_lowercase();
                examples.insert(
                    example_name,
                    json!({
                        "summary": example.title,
                        "value": {
                            "result": output,
                            "latex": format!("DOLLAR{}DOLLAR", output).replace("DOLLAR", "$")
                        }
                    }),
                );
            }

            if idx >= 2 {
                break;
            }
        }

        json!(examples)
    }

    fn extract_category(&self, topic: &str) -> String {
        topic.split('.').next().unwrap_or(topic).to_string()
    }

    fn extract_expression_from_code(&self, code: &str) -> String {
        for line in code.lines() {
            if line.contains("=") && !line.trim().starts_with('#') {
                if let Some(expr) = line.split('=').nth(1) {
                    return expr.trim().to_string();
                }
            }
        }
        "x**2".to_string()
    }
}

impl Default for ApiDocsGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to initialize API docs generator")
    }
}

impl OutputGenerator for ApiDocsGenerator {
    fn name(&self) -> &str {
        "api-docs"
    }

    fn file_extension(&self) -> &str {
        "json"
    }

    fn generate(&self, schema: &Schema) -> Result<String> {
        self.generate_openapi(schema)
    }

    fn validate_output(&self, output: &str) -> Result<()> {
        let parsed: serde_json::Value =
            serde_json::from_str(output).map_err(|e| kb_core::KbError::OutputValidationError {
                generator: "api-docs".to_string(),
                message: format!("Invalid JSON: {}", e),
            })?;

        if parsed.get("openapi").is_none() {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "api-docs".to_string(),
                message: "Missing 'openapi' field".to_string(),
            });
        }

        if parsed.get("paths").is_none() {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "api-docs".to_string(),
                message: "Missing 'paths' field".to_string(),
            });
        }

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
            code_refs: CodeReferences {
                rust: "test::example".to_string(),
                python: "test.example".to_string(),
                nodejs: "test.example".to_string(),
            },
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
        let generator = ApiDocsGenerator::new();
        assert!(generator.is_ok());
    }

    #[test]
    fn test_generate() {
        let generator = ApiDocsGenerator::new().unwrap();
        let schema = create_test_schema();
        let result = generator.generate(&schema);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("openapi"));
        assert!(output.contains("paths"));
    }

    #[test]
    fn test_validate_output() {
        let generator = ApiDocsGenerator::new().unwrap();
        let schema = create_test_schema();
        let output = generator.generate(&schema).unwrap();
        let validation = generator.validate_output(&output);
        assert!(validation.is_ok());
    }
}
