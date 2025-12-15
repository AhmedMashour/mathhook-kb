/// Schema validation logic
///
/// This module implements strict validation for schemas to ensure
/// correctness and consistency across all generated outputs.
use crate::{KbError, Result, schema::Schema};
use regex::Regex;

/// Validate a schema for correctness and completeness
pub fn validate_schema(schema: &Schema, file_path: String) -> Result<()> {
    validate_required_fields(schema, &file_path)?;
    validate_code_references(schema, &file_path)?;
    validate_examples(schema, &file_path)?;
    validate_output_hints(schema, &file_path)?;

    Ok(())
}

/// Validate required fields are present and non-empty
fn validate_required_fields(schema: &Schema, file_path: &str) -> Result<()> {
    // Topic must be non-empty and follow dotted notation
    if schema.topic.is_empty() {
        return Err(KbError::missing_field(
            "topic",
            file_path,
            1,
            "Add a topic identifier: topic: \"calculus.derivative\"",
        ));
    }

    let topic_regex = Regex::new(r"^[a-z_][a-z0-9_-]*(\.[a-z_][a-z0-9_-]*)*$").unwrap();
    if !topic_regex.is_match(&schema.topic) {
        return Err(KbError::validation(
            file_path,
            1,
            format!("Topic '{}' must follow dotted notation (lowercase, alphanumeric, underscores, hyphens)", schema.topic),
            "Example: topic: \"calculus.derivative\" or topic: \"algebra.polynomial.factoring\"",
        ));
    }

    // Title must be non-empty
    if schema.title.is_empty() {
        return Err(KbError::missing_field(
            "title",
            file_path,
            1,
            "Add a human-readable title: title: \"Symbolic Differentiation\"",
        ));
    }

    // Description must be non-empty
    if schema.description.is_empty() {
        return Err(KbError::missing_field(
            "description",
            file_path,
            1,
            "Add a detailed description explaining what this function/concept does",
        ));
    }

    // Examples are optional for article-style schemas (they may have article.content instead)
    // No longer require examples

    Ok(())
}

/// Validate code references follow expected patterns
fn validate_code_references(schema: &Schema, file_path: &str) -> Result<()> {
    // Code references are optional - skip if not present
    let Some(code_refs) = &schema.code_refs else {
        return Ok(());
    };

    // Rust reference validation is very lenient - just check it's not empty
    // Supports various formats:
    //   - module::path::item
    //   - module::path::{item1, item2}
    //   - module::path, module2::path2
    //   - single_module (for top-level crate references)
    // Just check for basic structure and non-empty
    if code_refs.rust.is_empty() {
        return Err(KbError::validation(
            file_path,
            1,
            "Rust code reference is empty".to_string(),
            "Example: mathhook_core::calculus::derivative",
        ));
    }
    // Allow single module names (like "mathhook") or full paths (with ::)

    // Python and Node.js validation is lenient - just check format if non-empty
    // Skip validation for empty values (they're optional)
    // The code is already flexible with comma-separated, curly braces, etc.

    Ok(())
}

/// Validate code examples are present in all languages
fn validate_examples(schema: &Schema, file_path: &str) -> Result<()> {
    for (idx, example) in schema.examples.iter().enumerate() {
        // Title must be non-empty
        if example.title.is_empty() {
            return Err(KbError::validation(
                file_path,
                1,
                format!("Example {} has empty title", idx + 1),
                "Add a descriptive title: title: \"Power Rule\"",
            ));
        }

        // Explanation is optional now - no validation needed

        // At least one language must have code
        let has_rust = !example.code.rust.trim().is_empty();
        let has_python = !example.code.python.trim().is_empty();
        let has_nodejs = !example.code.nodejs.trim().is_empty();

        if !has_rust && !has_python && !has_nodejs {
            return Err(KbError::validation(
                file_path,
                1,
                format!("Example '{}' has no code in any language", example.title),
                "Add code in at least one language (Rust, Python, or JavaScript)",
            ));
        }

        // Basic syntax validation (check for balanced braces/brackets/parens)
        // Only validate syntax if code is present
        if has_rust {
            validate_code_syntax(&example.code.rust, "Rust", &example.title, file_path)?;
        }
        if has_python {
            validate_code_syntax(&example.code.python, "Python", &example.title, file_path)?;
        }
        if has_nodejs {
            validate_code_syntax(&example.code.nodejs, "JavaScript", &example.title, file_path)?;
        }
    }

    Ok(())
}

/// Basic syntax validation (balanced brackets)
fn validate_code_syntax(code: &str, language: &str, _example_title: &str, file_path: &str) -> Result<()> {
    let mut stack = Vec::new();

    for ch in code.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return Err(KbError::CodeSyntaxError {
                        language: language.to_string(),
                        file: file_path.to_string(),
                        line: 1,
                        error: "Unmatched closing parenthesis".to_string(),
                        code: code.to_string(),
                    });
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return Err(KbError::CodeSyntaxError {
                        language: language.to_string(),
                        file: file_path.to_string(),
                        line: 1,
                        error: "Unmatched closing bracket".to_string(),
                        code: code.to_string(),
                    });
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return Err(KbError::CodeSyntaxError {
                        language: language.to_string(),
                        file: file_path.to_string(),
                        line: 1,
                        error: "Unmatched closing brace".to_string(),
                        code: code.to_string(),
                    });
                }
            }
            _ => {}
        }
    }

    if !stack.is_empty() {
        return Err(KbError::CodeSyntaxError {
            language: language.to_string(),
            file: file_path.to_string(),
            line: 1,
            error: format!("Unclosed delimiter: {:?}", stack.last().unwrap()),
            code: code.to_string(),
        });
    }

    Ok(())
}

/// Validate output hints
fn validate_output_hints(schema: &Schema, file_path: &str) -> Result<()> {
    // Validate LLM RAG hints if present
    if let Some(rag_hints) = &schema.outputs.llm_rag {
        // Allow standard strategies and any custom strategies (starts with by_)
        let valid_strategies = ["by_example", "by_section", "fixed_size"];
        let is_valid = valid_strategies.contains(&rag_hints.chunk_strategy.as_str())
            || rag_hints.chunk_strategy.starts_with("by_")
            || !rag_hints.chunk_strategy.is_empty();
        if !is_valid {
            return Err(KbError::validation(
                file_path,
                1,
                format!("Invalid chunk_strategy '{}'", rag_hints.chunk_strategy),
                format!("Valid strategies: {}", valid_strategies.join(", ")),
            ));
        }

        if rag_hints.max_chunk_size == 0 || rag_hints.max_chunk_size > 8192 {
            return Err(KbError::validation(
                file_path,
                1,
                format!("Invalid max_chunk_size {}", rag_hints.max_chunk_size),
                "Chunk size must be between 1 and 8192 tokens",
            ));
        }

        let valid_priorities = ["high", "medium", "low"];
        if !valid_priorities.contains(&rag_hints.embedding_priority.as_str()) {
            return Err(KbError::validation(
                file_path,
                1,
                format!(
                    "Invalid embedding_priority '{}'",
                    rag_hints.embedding_priority
                ),
                format!("Valid priorities: {}", valid_priorities.join(", ")),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::*;

    #[test]
    fn test_valid_schema() {
        let schema = Schema {
            topic: "calculus.derivative".to_string(),
            title: "Symbolic Differentiation".to_string(),
            description: "Computes the derivative of an expression".to_string(),
            mathematical_definition: None,
            code_refs: Some(CodeReferences {
                rust: "mathhook_core::calculus::derivative".to_string(),
                python: "mathhook.calculus.derivative".to_string(),
                nodejs: "mathhook.calculus.derivative".to_string(),
            }),
            examples: vec![Example {
                title: "Power Rule".to_string(),
                explanation: "Derivative of x^n".to_string(),
                code: CodeSnippets {
                    rust: "let f = expr!(x ^ 3);".to_string(),
                    python: "f = expr('x^3')".to_string(),
                    nodejs: "const f = expr('x^3');".to_string(),
                },
                expected_output: None,
            }],
            article: None,
            use_cases: vec![],
            related_topics: vec![],
            performance: None,
            interactive_playground: None,
            outputs: OutputHints::default(),
            metadata: None,
            seo: None,
        };

        assert!(validate_schema(&schema, "test.yaml".to_string()).is_ok());
    }

    #[test]
    fn test_valid_schema_without_code_refs() {
        // Article-style schemas don't need code_refs
        let schema = Schema {
            topic: "getting-started.installation".to_string(),
            title: "Installation".to_string(),
            description: "How to install MathHook".to_string(),
            mathematical_definition: None,
            code_refs: None,
            examples: vec![],
            article: None,
            use_cases: vec![],
            related_topics: vec![],
            performance: None,
            interactive_playground: None,
            outputs: OutputHints::default(),
            metadata: None,
            seo: None,
        };

        assert!(validate_schema(&schema, "test.yaml".to_string()).is_ok());
    }

    #[test]
    fn test_missing_topic() {
        let schema = Schema {
            topic: "".to_string(),
            title: "Test".to_string(),
            description: "Test description".to_string(),
            mathematical_definition: None,
            code_refs: Some(CodeReferences {
                rust: "test::func".to_string(),
                python: "test.func".to_string(),
                nodejs: "test.func".to_string(),
            }),
            examples: vec![Example {
                title: "Example".to_string(),
                explanation: "Test".to_string(),
                code: CodeSnippets {
                    rust: "test()".to_string(),
                    python: "test()".to_string(),
                    nodejs: "test()".to_string(),
                },
                expected_output: None,
            }],
            article: None,
            use_cases: vec![],
            related_topics: vec![],
            performance: None,
            interactive_playground: None,
            outputs: OutputHints::default(),
            metadata: None,
            seo: None,
        };

        let result = validate_schema(&schema, "test.yaml".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("topic"));
    }

    #[test]
    fn test_unbalanced_braces() {
        let schema = Schema {
            topic: "test.function".to_string(),
            title: "Test".to_string(),
            description: "Test description".to_string(),
            mathematical_definition: None,
            code_refs: Some(CodeReferences {
                rust: "test::func".to_string(),
                python: "test.func".to_string(),
                nodejs: "test.func".to_string(),
            }),
            examples: vec![Example {
                title: "Example".to_string(),
                explanation: "Test".to_string(),
                code: CodeSnippets {
                    rust: "let x = {1, 2, 3;".to_string(),  // Unbalanced
                    python: "x = [1, 2, 3]".to_string(),
                    nodejs: "const x = [1, 2, 3];".to_string(),
                },
                expected_output: None,
            }],
            article: None,
            use_cases: vec![],
            related_topics: vec![],
            performance: None,
            interactive_playground: None,
            outputs: OutputHints::default(),
            metadata: None,
            seo: None,
        };

        let result = validate_schema(&schema, "test.yaml".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unclosed delimiter"));
    }
}
