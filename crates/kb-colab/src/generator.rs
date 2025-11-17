use kb_core::generator::OutputGenerator;
/// Google Colab notebook generator implementation
///
/// Generates interactive Google Colab notebooks with mathematical examples
/// and explanations. Colab format is identical to Jupyter but with specific
/// metadata for Google Colab integration.
use kb_core::{Result, Schema};
use serde_json::{json, Value};

/// Google Colab notebook generator
pub struct ColabGenerator;

impl ColabGenerator {
    /// Create a new Colab generator
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Generate Colab notebook cells from schema
    fn generate_cells(&self, schema: &Schema) -> Vec<Value> {
        let mut cells = Vec::new();

        cells.push(json!({
            "cell_type": "markdown",
            "metadata": {
                "id": "header"
            },
            "source": [
                format!("# {}\n", schema.title),
                "\n",
                format!("{}\n", schema.description),
                "\n",
                "[![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)]",
                "(https://colab.research.google.com/github/mathhook/mathhook/blob/main/docs/colab/",
                format!("{}.ipynb)\n", schema.topic.replace('.', "_"))
            ]
        }));

        cells.push(json!({
            "cell_type": "code",
            "execution_count": null,
            "metadata": {
                "id": "setup"
            },
            "outputs": [],
            "source": [
                "# Install MathHook (if not already installed)\n",
                "!pip install mathhook\n",
                "\n",
                "# Import MathHook\n",
                "from mathhook import symbol, expr\n",
                format!("from mathhook.{} import *\n", schema.code_refs.python)
            ]
        }));

        if let Some(math_def) = &schema.mathematical_definition {
            cells.push(json!({
                "cell_type": "markdown",
                "metadata": {
                    "id": "math-def"
                },
                "source": [
                    "## Mathematical Definition\n",
                    "\n",
                    format!("$${}$$\n", math_def)
                ]
            }));
        }

        for (idx, example) in schema.examples.iter().enumerate() {
            cells.push(json!({
                "cell_type": "markdown",
                "metadata": {
                    "id": format!("example-{}-title", idx)
                },
                "source": [
                    format!("## Example {}: {}\n", idx + 1, example.title),
                    "\n",
                    format!("{}\n", example.explanation)
                ]
            }));

            let code_lines: Vec<String> = example
                .code
                .python
                .lines()
                .map(|l| format!("{}\n", l))
                .collect();

            cells.push(json!({
                "cell_type": "code",
                "execution_count": null,
                "metadata": {
                    "id": format!("example-{}-code", idx)
                },
                "outputs": [],
                "source": code_lines
            }));

            if let Some(output) = &example.expected_output {
                cells.push(json!({
                    "cell_type": "markdown",
                    "metadata": {
                        "id": format!("example-{}-output", idx)
                    },
                    "source": [
                        "**Expected Output:**\n",
                        "\n",
                        format!("```\n{}\n```\n", output)
                    ]
                }));
            }
        }

        if let Some(article) = &schema.article {
            if let Some(conclusion) = &article.conclusion {
                cells.push(json!({
                    "cell_type": "markdown",
                    "metadata": {
                        "id": "conclusion"
                    },
                    "source": [
                        "## Summary\n",
                        "\n",
                        format!("{}\n", conclusion.summary)
                    ]
                }));

                if !conclusion.exercises.is_empty() {
                    cells.push(json!({
                        "cell_type": "markdown",
                        "metadata": {
                            "id": "exercises"
                        },
                        "source": [
                            "## Try It Yourself\n",
                            "\n",
                            "Complete these exercises to test your understanding:\n"
                        ]
                    }));

                    for (idx, exercise) in conclusion.exercises.iter().enumerate() {
                        cells.push(json!({
                            "cell_type": "markdown",
                            "metadata": {
                                "id": format!("exercise-{}", idx)
                            },
                            "source": [
                                format!("### Exercise {}\n", idx + 1),
                                "\n",
                                format!("{}\n", exercise.prompt),
                                "\n",
                                format!("**Difficulty:** {:?}\n", exercise.difficulty)
                            ]
                        }));

                        cells.push(json!({
                            "cell_type": "code",
                            "execution_count": null,
                            "metadata": {
                                "id": format!("exercise-{}-solution", idx)
                            },
                            "outputs": [],
                            "source": [
                                "# Your solution here\n"
                            ]
                        }));
                    }
                }
            }
        }

        cells
    }
}

impl Default for ColabGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to initialize Colab generator")
    }
}

impl OutputGenerator for ColabGenerator {
    fn name(&self) -> &str {
        "colab"
    }

    fn file_extension(&self) -> &str {
        "colab.ipynb"
    }

    fn generate(&self, schema: &Schema) -> Result<String> {
        let cells = self.generate_cells(schema);

        let notebook = json!({
            "nbformat": 4,
            "nbformat_minor": 0,
            "metadata": {
                "colab": {
                    "name": format!("{}.ipynb", schema.topic.replace('.', "_")),
                    "provenance": [],
                    "collapsed_sections": [],
                    "toc_visible": true
                },
                "kernelspec": {
                    "name": "python3",
                    "display_name": "Python 3"
                },
                "language_info": {
                    "name": "python",
                    "version": "3.10.0"
                }
            },
            "cells": cells
        });

        Ok(serde_json::to_string_pretty(&notebook)?)
    }

    fn validate_output(&self, output: &str) -> Result<()> {
        let parsed: Value =
            serde_json::from_str(output).map_err(|e| kb_core::KbError::OutputValidationError {
                generator: "colab".to_string(),
                message: format!("Invalid JSON: {}", e),
            })?;

        if parsed.get("nbformat").is_none() {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "colab".to_string(),
                message: "Missing 'nbformat' field".to_string(),
            });
        }

        if parsed
            .get("metadata")
            .and_then(|m| m.get("colab"))
            .is_none()
        {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "colab".to_string(),
                message: "Missing 'metadata.colab' field".to_string(),
            });
        }

        if parsed.get("cells").is_none() {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "colab".to_string(),
                message: "Missing 'cells' array".to_string(),
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
        let generator = ColabGenerator::new();
        assert!(generator.is_ok());
    }

    #[test]
    fn test_generate() {
        let generator = ColabGenerator::new().unwrap();
        let schema = create_test_schema();
        let result = generator.generate(&schema);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("nbformat"));
        assert!(output.contains("colab"));
    }

    #[test]
    fn test_validate_output() {
        let generator = ColabGenerator::new().unwrap();
        let schema = create_test_schema();
        let output = generator.generate(&schema).unwrap();
        let validation = generator.validate_output(&output);
        assert!(validation.is_ok());
    }
}
