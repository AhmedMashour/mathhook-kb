use crate::notebook::Notebook;
use crate::templates::TemplateManager;
use kb_core::generator::OutputGenerator;
/// Jupyter notebook generator implementation
///
/// Implements the OutputGenerator trait to generate Jupyter notebooks from KB schemas.
use kb_core::{Result, Schema};
use std::collections::HashMap;

/// Jupyter notebook generator
pub struct JupyterGenerator {
    templates: TemplateManager,
}

impl JupyterGenerator {
    /// Create a new Jupyter generator
    pub fn new() -> Self {
        Self {
            templates: TemplateManager::new().expect("Failed to initialize templates"),
        }
    }

    /// Generate notebook from schema (internal implementation)
    fn generate_notebook(&self, schema: &Schema) -> Result<Notebook> {
        let mut notebook = Notebook::new();

        // Add header
        let header = self
            .templates
            .render_header(&schema.title, &schema.description)?;
        notebook.add_markdown(header);

        // If schema has article content, use rich article format
        if let Some(article) = &schema.article {
            use kb_core::schema::Article;
            match article {
                Article::Simple(simple) => {
                    // For simple articles, just add content
                    notebook.add_markdown(format!("## Overview\n\n{}", simple.content));
                }
                Article::Structured(structured) => {
                    // Add introduction section
                    let intro = self.templates.render_introduction(
                        &structured.introduction.hook,
                        &structured.introduction.learning_objectives,
                        &structured.introduction.prerequisites,
                        structured.introduction.estimated_time.as_ref(),
                    )?;
                    notebook.add_markdown(intro);

                    // Add Jupyter-specific tutorial intro if provided
                    if let Some(variations) = &structured.variations {
                        if let Some(jupyter) = &variations.jupyter {
                            if let Some(tutorial_intro) = &jupyter.tutorial_intro {
                                notebook.add_markdown(tutorial_intro.clone());
                            }
                        }
                    }

                    // Add sections
                    for section in &structured.sections {
                        let subsections: Vec<HashMap<String, String>> = section
                            .subsections
                            .iter()
                            .map(|subsection| {
                                let mut map = HashMap::new();
                                map.insert("title".to_string(), subsection.title.clone());
                                map.insert("content".to_string(), subsection.content.clone());
                                map
                            })
                            .collect();

                        let section_md = self.templates.render_section(
                            &section.title,
                            &section.content,
                            &subsections,
                        )?;
                        notebook.add_markdown(section_md);

                        // Add code examples from section
                        for example_ref in &section.code_examples {
                            // Find the example in schema.examples
                            if let Some(example) = schema.examples.iter().find(|e| e.title == *example_ref)
                            {
                                let example_md = self.templates.render_example(
                                    &example.title,
                                    &example.explanation,
                                    &example.code.python,
                                    example.expected_output.as_ref(),
                                )?;
                                notebook.add_markdown(example_md);

                                // Add interactive code cell
                                notebook.add_code(example.code.python.clone());
                            }
                        }
                    }

                    // Add interactive prompts if provided
                    if let Some(variations) = &structured.variations {
                        if let Some(jupyter) = &variations.jupyter {
                            for (idx, prompt) in jupyter.interactive_prompts.iter().enumerate() {
                                notebook.add_markdown(format!(
                                    "### 🤔 Interactive Checkpoint {}\n\n{}",
                                    idx + 1,
                                    prompt
                                ));
                                notebook.add_code("# Your answer here\n".to_string());
                            }
                        }
                    }

                    // Add sidebars
                    for sidebar in &structured.sidebars {
                        let sidebar_type_str = match sidebar.sidebar_type {
                            kb_core::schema::SidebarType::Tip => "tip",
                            kb_core::schema::SidebarType::Warning => "warning",
                            kb_core::schema::SidebarType::Note => "note",
                            kb_core::schema::SidebarType::Info => "info",
                            kb_core::schema::SidebarType::Performance => "performance",
                            kb_core::schema::SidebarType::BestPractice => "best_practice",
                        };

                        let sidebar_md = self.templates.render_sidebar(
                            sidebar_type_str,
                            &sidebar.title,
                            &sidebar.content,
                        )?;
                        notebook.add_markdown(sidebar_md);
                    }

                    // Add conclusion
                    if let Some(conclusion) = &structured.conclusion {
                        // Convert Resource to String for rendering
                        let further_reading: Vec<String> = conclusion
                            .further_reading
                            .iter()
                            .map(|r| {
                                if let Some(url) = &r.url {
                                    format!("[{}]({})", r.title, url)
                                } else {
                                    r.title.clone()
                                }
                            })
                            .collect();

                        let conclusion_md = self.templates.render_conclusion(
                            &conclusion.summary,
                            &conclusion.next_steps,
                            &further_reading,
                        )?;
                        notebook.add_markdown(conclusion_md);

                        // Add exercises
                        for (idx, exercise) in conclusion.exercises.iter().enumerate() {
                            let difficulty_str = match exercise.difficulty {
                                kb_core::schema::ExerciseDifficulty::Beginner => "Beginner",
                                kb_core::schema::ExerciseDifficulty::Intermediate => "Intermediate",
                                kb_core::schema::ExerciseDifficulty::Advanced => "Advanced",
                                kb_core::schema::ExerciseDifficulty::Expert => "Expert",
                            };

                            let exercise_md = self.templates.render_exercise(
                                idx + 1,
                                &exercise.prompt,
                                difficulty_str,
                                &exercise.prompt,
                                &exercise.hints,
                            )?;
                            notebook.add_markdown(exercise_md);
                        }
                    }
                }
            }
        } else {
            // Fallback: Simple format for schemas without articles

            // Add mathematical definition if present
            if let Some(math_def) = &schema.mathematical_definition {
                notebook.add_markdown(format!(
                    "## Mathematical Definition\n\n${}$\n\n---\n",
                    math_def
                ));
            }

            // Add code examples
            notebook.add_markdown("## Examples\n".to_string());
            for example in &schema.examples {
                let example_md = self.templates.render_example(
                    &example.title,
                    &example.explanation,
                    &example.code.python,
                    example.expected_output.as_ref(),
                )?;
                notebook.add_markdown(example_md);
                notebook.add_code(example.code.python.clone());
            }

            // Add use cases if present
            if !schema.use_cases.is_empty() {
                let mut use_cases_md = "## Use Cases\n\n".to_string();
                for use_case in &schema.use_cases {
                    use_cases_md.push_str(&format!("- {}\n", use_case));
                }
                notebook.add_markdown(use_cases_md);
            }

            // Add related topics if present
            if !schema.related_topics.is_empty() {
                let mut related_md = "## Related Topics\n\n".to_string();
                for topic in &schema.related_topics {
                    related_md.push_str(&format!("- {}\n", topic));
                }
                notebook.add_markdown(related_md);
            }
        }

        Ok(notebook)
    }
}

impl Default for JupyterGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputGenerator for JupyterGenerator {
    fn name(&self) -> &str {
        "jupyter"
    }

    fn file_extension(&self) -> &str {
        "ipynb"
    }

    fn generate(&self, schema: &Schema) -> Result<String> {
        let notebook = self.generate_notebook(schema)?;
        let json = notebook.to_json()?;
        Ok(json)
    }

    fn validate_output(&self, output: &str) -> Result<()> {
        // Validate JSON structure
        let parsed: serde_json::Value = serde_json::from_str(output)?;

        // Check required fields
        if parsed.get("nbformat").is_none() {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "jupyter".to_string(),
                message: "Missing 'nbformat' field".to_string(),
            });
        }

        if parsed.get("cells").is_none() {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "jupyter".to_string(),
                message: "Missing 'cells' field".to_string(),
            });
        }

        // Validate cells array
        if let Some(cells) = parsed.get("cells").and_then(|c| c.as_array()) {
            for (idx, cell) in cells.iter().enumerate() {
                if cell.get("cell_type").is_none() {
                    return Err(kb_core::KbError::OutputValidationError {
                        generator: "jupyter".to_string(),
                        message: format!("Cell {} missing 'cell_type' field", idx),
                    });
                }
                if cell.get("source").is_none() {
                    return Err(kb_core::KbError::OutputValidationError {
                        generator: "jupyter".to_string(),
                        message: format!("Cell {} missing 'source' field", idx),
                    });
                }
            }
        } else {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "jupyter".to_string(),
                message: "'cells' is not an array".to_string(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::schema::*;

    fn create_simple_schema() -> Schema {
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
                title: "Simple Example".to_string(),
                explanation: "This is a test".to_string(),
                code: CodeSnippets {
                    rust: "let x = 2;".to_string(),
                    python: "x = 2".to_string(),
                    nodejs: "const x = 2;".to_string(),
                },
                expected_output: Some("2".to_string()),
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
        let generator = JupyterGenerator::new();
        assert_eq!(generator.name(), "jupyter");
        assert_eq!(generator.file_extension(), "ipynb");
    }

    #[test]
    fn test_generate_simple_schema() {
        let generator = JupyterGenerator::new();
        let schema = create_simple_schema();
        let result = generator.generate(&schema);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("nbformat"));
        assert!(output.contains("cells"));
        assert!(output.contains("Test Example"));
    }

    #[test]
    fn test_validate_output() {
        let generator = JupyterGenerator::new();
        let schema = create_simple_schema();
        let output = generator.generate(&schema).unwrap();
        let validation = generator.validate_output(&output);
        assert!(validation.is_ok());
    }

    #[test]
    fn test_validate_invalid_output() {
        let generator = JupyterGenerator::new();
        let invalid_json = r#"{"invalid": "json"}"#;
        let validation = generator.validate_output(invalid_json);
        assert!(validation.is_err());
    }

    #[test]
    fn test_generate_with_multiple_examples() {
        let generator = JupyterGenerator::new();
        let mut schema = create_simple_schema();
        schema.examples.push(Example {
            title: "Second Example".to_string(),
            explanation: "Another test".to_string(),
            code: CodeSnippets {
                rust: "let y = 3;".to_string(),
                python: "y = 3".to_string(),
                nodejs: "const y = 3;".to_string(),
            },
            expected_output: None,
        });

        let result = generator.generate(&schema);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("Simple Example"));
        assert!(output.contains("Second Example"));
    }
}
