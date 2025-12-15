//! Google Colab notebook generator implementation
//!
//! Generates interactive Google Colab notebooks with mathematical examples
//! and explanations. Notebooks are designed to be hosted on GitHub and
//! opened via Colab's native GitHub integration.

use crate::manifest::ColabConfig;
use kb_core::generator::OutputGenerator;
use kb_core::{Result, Schema};
use serde_json::{json, Value};

/// Google Colab notebook generator
pub struct ColabGenerator {
    config: ColabConfig,
}

impl ColabGenerator {
    /// Create a new Colab generator with default config
    pub fn new() -> Result<Self> {
        Ok(Self {
            config: ColabConfig::default(),
        })
    }

    /// Create a new Colab generator with custom config
    pub fn with_config(config: ColabConfig) -> Self {
        Self { config }
    }

    /// Get the config
    pub fn config(&self) -> &ColabConfig {
        &self.config
    }

    /// Generate Colab notebook cells from schema
    fn generate_cells(&self, schema: &Schema) -> Vec<Value> {
        let mut cells = Vec::new();

        // Extract category from topic for URL generation
        let category = schema.topic.split('.').next().unwrap_or("misc");
        let filename = format!("{}.colab.ipynb", schema.topic.replace('.', "-"));
        let colab_url = self.config.get_colab_url(category, &filename);

        // Header cell with Colab badge
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
                format!("[![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)]({})\n", colab_url)
            ]
        }));

        // Setup cell
        cells.push(json!({
            "cell_type": "code",
            "execution_count": null,
            "metadata": {
                "id": "setup"
            },
            "outputs": [],
            "source": self.generate_setup_source(schema)
        }));

        // Mathematical definition
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

        // Examples
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

        // Article content
        if let Some(article) = &schema.article {
            self.add_article_cells(&mut cells, article);
        }

        // Footer
        cells.push(json!({
            "cell_type": "markdown",
            "metadata": {
                "id": "footer"
            },
            "source": [
                "---\n",
                "\n",
                "**MathHook** - Symbolic Power · Educational Clarity · Native Speed\n",
                "\n",
                format!("[View on GitHub]({}) | ", self.config.get_github_url(category, &filename)),
                "[MathHook Documentation](https://mathhook.dev)\n"
            ]
        }));

        cells
    }

    fn generate_setup_source(&self, schema: &Schema) -> Vec<String> {
        let mut source = vec![
            "# Install MathHook (if not already installed)\n".to_string(),
            "!pip install -q mathhook\n".to_string(),
            "\n".to_string(),
            "# Import MathHook\n".to_string(),
            "from mathhook import symbol, expr\n".to_string(),
        ];

        if let Some(code_refs) = &schema.code_refs {
            // Extract module path from full reference
            let module = code_refs.python.rsplit('.').skip(1).collect::<Vec<_>>();
            if !module.is_empty() {
                let module_path = module.into_iter().rev().collect::<Vec<_>>().join(".");
                source.push(format!("from {} import *\n", module_path));
            }
        }

        source
    }

    fn add_article_cells(&self, cells: &mut Vec<Value>, article: &kb_core::schema::Article) {
        use kb_core::schema::Article;

        match article {
            Article::Simple(simple) => {
                cells.push(json!({
                    "cell_type": "markdown",
                    "metadata": {
                        "id": "article-content"
                    },
                    "source": [
                        "## Content\n",
                        "\n",
                        format!("{}\n", simple.content)
                    ]
                }));
            }
            Article::Structured(structured) => {
                // Add sections
                for (idx, section) in structured.sections.iter().enumerate() {
                    cells.push(json!({
                        "cell_type": "markdown",
                        "metadata": {
                            "id": format!("section-{}", idx)
                        },
                        "source": [
                            format!("## {}\n", section.title),
                            "\n",
                            format!("{}\n", section.content)
                        ]
                    }));
                }

                // Conclusion and exercises
                if let Some(conclusion) = &structured.conclusion {
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
        }
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
                    "name": format!("{} - MathHook", schema.title),
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
            topic: "calculus.derivative".to_string(),
            title: "Symbolic Differentiation".to_string(),
            description: "Compute derivatives symbolically".to_string(),
            mathematical_definition: Some("\\frac{d}{dx} f(x)".to_string()),
            code_refs: Some(CodeReferences {
                rust: "mathhook::calculus::derivative".to_string(),
                python: "mathhook.calculus.derivative".to_string(),
                nodejs: "mathhook.calculus.derivative".to_string(),
            }),
            examples: vec![Example {
                title: "Power Rule".to_string(),
                explanation: "Derivative of x^n is n*x^(n-1)".to_string(),
                code: CodeSnippets {
                    rust: "let f = expr!(x ^ 2);".to_string(),
                    python: "f = x**2\ndf = diff(f, x)".to_string(),
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
        assert!(output.contains("colab-badge.svg"));
    }

    #[test]
    fn test_colab_url_in_output() {
        let config = ColabConfig::new("TestUser", "test-repo", "main", "notebooks");
        let generator = ColabGenerator::with_config(config);
        let schema = create_test_schema();

        let output = generator.generate(&schema).unwrap();

        assert!(output.contains("colab.research.google.com/github/TestUser/test-repo"));
        assert!(output.contains("notebooks/calculus/"));
    }

    #[test]
    fn test_validate_output() {
        let generator = ColabGenerator::new().unwrap();
        let schema = create_test_schema();
        let output = generator.generate(&schema).unwrap();
        let validation = generator.validate_output(&output);
        assert!(validation.is_ok());
    }

    #[test]
    fn test_output_filename() {
        let generator = ColabGenerator::new().unwrap();
        let schema = create_test_schema();
        let filename = generator.get_output_filename(&schema);
        assert_eq!(filename, "calculus-derivative.colab.ipynb");
    }
}
