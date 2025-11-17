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

    let topic_regex = Regex::new(r"^[a-z_][a-z0-9_]*(\.[a-z_][a-z0-9_]*)*$").unwrap();
    if !topic_regex.is_match(&schema.topic) {
        return Err(KbError::validation(
            file_path,
            1,
            format!("Topic '{}' must follow dotted notation (lowercase, alphanumeric, underscores)", schema.topic),
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

    // Must have at least one example
    if schema.examples.is_empty() {
        return Err(KbError::validation(
            file_path,
            1,
            "Schema must have at least one code example",
            "Add an examples section with code in Rust, Python, and JavaScript",
        ));
    }

    Ok(())
}

/// Validate code references follow expected patterns
fn validate_code_references(schema: &Schema, file_path: &str) -> Result<()> {
    // Rust reference should follow module::path format
    let rust_regex = Regex::new(r"^[a-z_][a-z0-9_]*(::([a-z_][a-z0-9_]*))*$").unwrap();
    if !rust_regex.is_match(&schema.code_refs.rust) {
        return Err(KbError::validation(
            file_path,
            1,
            format!(
                "Rust code reference '{}' should follow module::path format",
                schema.code_refs.rust
            ),
            "Example: mathhook_core::calculus::derivative",
        ));
    }

    // Python reference should follow module.path format
    let python_regex = Regex::new(r"^[a-z_][a-z0-9_]*(\.[a-z_][a-z0-9_]*)*$").unwrap();
    if !python_regex.is_match(&schema.code_refs.python) {
        return Err(KbError::validation(
            file_path,
            1,
            format!(
                "Python code reference '{}' should follow module.path format",
                schema.code_refs.python
            ),
            "Example: mathhook.calculus.derivative",
        ));
    }

    // Node.js reference should follow module.path format
    if !python_regex.is_match(&schema.code_refs.nodejs) {
        return Err(KbError::validation(
            file_path,
            1,
            format!(
                "Node.js code reference '{}' should follow module.path format",
                schema.code_refs.nodejs
            ),
            "Example: mathhook.calculus.derivative",
        ));
    }

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

        // Explanation must be non-empty
        if example.explanation.is_empty() {
            return Err(KbError::validation(
                file_path,
                1,
                format!("Example '{}' has empty explanation", example.title),
                "Add an explanation of what this example demonstrates",
            ));
        }

        // All code snippets must be non-empty
        if example.code.rust.trim().is_empty() {
            return Err(KbError::validation(
                file_path,
                1,
                format!("Example '{}' has empty Rust code", example.title),
                "Add Rust code demonstrating the usage",
            ));
        }

        if example.code.python.trim().is_empty() {
            return Err(KbError::validation(
                file_path,
                1,
                format!("Example '{}' has empty Python code", example.title),
                "Add Python code demonstrating the usage",
            ));
        }

        if example.code.nodejs.trim().is_empty() {
            return Err(KbError::validation(
                file_path,
                1,
                format!("Example '{}' has empty JavaScript code", example.title),
                "Add JavaScript code demonstrating the usage",
            ));
        }

        // Basic syntax validation (check for balanced braces/brackets/parens)
        validate_code_syntax(&example.code.rust, "Rust", &example.title, file_path)?;
        validate_code_syntax(&example.code.python, "Python", &example.title, file_path)?;
        validate_code_syntax(&example.code.nodejs, "JavaScript", &example.title, file_path)?;
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
        let valid_strategies = ["by_example", "by_section", "fixed_size"];
        if !valid_strategies.contains(&rag_hints.chunk_strategy.as_str()) {
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
            code_refs: CodeReferences {
                rust: "mathhook_core::calculus::derivative".to_string(),
                python: "mathhook.calculus.derivative".to_string(),
                nodejs: "mathhook.calculus.derivative".to_string(),
            },
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
    fn test_missing_topic() {
        let schema = Schema {
            topic: "".to_string(),
            title: "Test".to_string(),
            description: "Test description".to_string(),
            mathematical_definition: None,
            code_refs: CodeReferences {
                rust: "test::func".to_string(),
                python: "test.func".to_string(),
                nodejs: "test.func".to_string(),
            },
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
            code_refs: CodeReferences {
                rust: "test::func".to_string(),
                python: "test.func".to_string(),
                nodejs: "test.func".to_string(),
            },
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
