use crate::chunking::{ChunkMetadata, ChunkStrategy};
use kb_core::generator::OutputGenerator;
/// LLM-RAG generator implementation
///
/// Generates markdown optimized for vector embedding and retrieval by LLMs.
use kb_core::{Result, Schema};

/// LLM-RAG optimized markdown generator
pub struct LlmRagGenerator {
    chunk_strategy: ChunkStrategy,
    #[allow(dead_code)]
    max_chunk_size: usize,
    embedding_priority: String,
}

impl LlmRagGenerator {
    /// Create a new LLM-RAG generator with default settings
    pub fn new() -> Self {
        Self {
            chunk_strategy: ChunkStrategy::ByExample,
            max_chunk_size: 512,
            embedding_priority: "medium".to_string(),
        }
    }

    /// Create generator from schema hints
    pub fn from_schema(schema: &Schema) -> Self {
        if let Some(hints) = &schema.outputs.llm_rag {
            let strategy = match hints.chunk_strategy.as_str() {
                "by_section" => ChunkStrategy::BySection,
                "fixed_size" => ChunkStrategy::FixedSize {
                    size: hints.max_chunk_size,
                    overlap: hints.max_chunk_size / 4,
                },
                _ => ChunkStrategy::ByExample,
            };

            Self {
                chunk_strategy: strategy,
                max_chunk_size: hints.max_chunk_size,
                embedding_priority: hints.embedding_priority.clone(),
            }
        } else {
            Self::new()
        }
    }

    /// Generate a single chunk of markdown
    fn generate_chunk(&self, metadata: &ChunkMetadata, content: String, schema: &Schema) -> String {
        let mut output = String::new();

        // Add metadata header (YAML frontmatter for RAG systems)
        output.push_str("---\n");
        output.push_str(&format!("chunk_id: {}\n", metadata.id));
        output.push_str(&format!("topic: {}\n", metadata.topic));
        output.push_str(&format!("title: {}\n", metadata.title));
        output.push_str(&format!("priority: {}\n", metadata.priority));
        if !metadata.keywords.is_empty() {
            output.push_str("keywords:\n");
            for keyword in &metadata.keywords {
                output.push_str(&format!("  - {}\n", keyword));
            }
        }
        output.push_str(&format!("languages: [{}]\n", metadata.languages.join(", ")));
        output.push_str(&format!(
            "chunk: {}/{}\n",
            metadata.chunk_index + 1,
            metadata.total_chunks
        ));

        // Add SEO metadata to chunk frontmatter for better retrieval
        if let Some(seo) = &schema.seo {
            if let Some(meta_desc) = &seo.meta_description {
                output.push_str(&format!("description: \"{}\"\n", meta_desc));
            }
            if !seo.keywords.is_empty() {
                output.push_str("seo_keywords:\n");
                for keyword in &seo.keywords {
                    output.push_str(&format!("  - {}\n", keyword));
                }
            }
            if let Some(canonical) = &seo.canonical_url {
                output.push_str(&format!("canonical_url: \"{}\"\n", canonical));
            }
        }

        output.push_str("---\n\n");

        // Add content
        output.push_str(&content);
        output.push_str("\n\n");

        output
    }

    /// Generate by example strategy
    fn generate_by_example(&self, schema: &Schema) -> Result<String> {
        let mut output = String::new();
        let total_chunks = schema.examples.len();

        // Add schema header
        output.push_str(&format!("# {}\n\n", schema.title));
        output.push_str(&format!("{}\n\n", schema.description));

        // Generate chunk for each example
        for (idx, example) in schema.examples.iter().enumerate() {
            let keywords = vec![
                schema
                    .topic
                    .split('.')
                    .next_back()
                    .unwrap_or(&schema.topic)
                    .to_string(),
                example.title.to_lowercase(),
            ];

            let metadata = ChunkMetadata::new(
                schema.topic.clone(),
                example.title.clone(),
                idx,
                total_chunks,
            )
            .with_keywords(keywords)
            .with_priority(self.embedding_priority.clone());

            let mut chunk_content = String::new();
            chunk_content.push_str(&format!("## {}\n\n", example.title));
            chunk_content.push_str(&format!("{}\n\n", example.explanation));

            // Add code in all languages
            chunk_content.push_str("### Rust\n\n");
            chunk_content.push_str(&format!("```rust\n{}\n```\n\n", example.code.rust));

            chunk_content.push_str("### Python\n\n");
            chunk_content.push_str(&format!("```python\n{}\n```\n\n", example.code.python));

            chunk_content.push_str("### JavaScript\n\n");
            chunk_content.push_str(&format!("```javascript\n{}\n```\n\n", example.code.nodejs));

            if let Some(expected_output) = &example.expected_output {
                chunk_content.push_str("### Expected Output\n\n");
                chunk_content.push_str(&format!("```\n{}\n```\n\n", expected_output));
            }

            output.push_str(&self.generate_chunk(&metadata, chunk_content, schema));
        }

        Ok(output)
    }

    /// Generate by section strategy (for articles)
    fn generate_by_section(&self, schema: &Schema) -> Result<String> {
        let mut output = String::new();

        // Add schema header
        output.push_str(&format!("# {}\n\n", schema.title));
        output.push_str(&format!("{}\n\n", schema.description));

        if let Some(article) = &schema.article {
            let total_chunks = article.sections.len();

            // Introduction as first chunk
            let intro_metadata = ChunkMetadata::new(
                schema.topic.clone(),
                "Introduction".to_string(),
                0,
                total_chunks + 1,
            )
            .with_priority("high".to_string());

            let mut intro_content = String::new();
            intro_content.push_str("## Introduction\n\n");
            intro_content.push_str(&format!("{}\n\n", article.introduction.hook));

            output.push_str(&self.generate_chunk(&intro_metadata, intro_content, schema));

            // Each section as a chunk
            for (idx, section) in article.sections.iter().enumerate() {
                let metadata = ChunkMetadata::new(
                    schema.topic.clone(),
                    section.title.clone(),
                    idx + 1,
                    total_chunks + 1,
                )
                .with_priority(self.embedding_priority.clone());

                let mut section_content = String::new();
                section_content.push_str(&format!("## {}\n\n", section.title));
                section_content.push_str(&format!("{}\n\n", section.content));

                output.push_str(&self.generate_chunk(&metadata, section_content, schema));
            }
        } else {
            // Fallback to by_example if no article
            return self.generate_by_example(schema);
        }

        Ok(output)
    }
}

impl Default for LlmRagGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputGenerator for LlmRagGenerator {
    fn name(&self) -> &str {
        "llm-rag"
    }

    fn file_extension(&self) -> &str {
        "md"
    }

    fn generate(&self, schema: &Schema) -> Result<String> {
        match &self.chunk_strategy {
            ChunkStrategy::ByExample => self.generate_by_example(schema),
            ChunkStrategy::BySection => self.generate_by_section(schema),
            ChunkStrategy::FixedSize { .. } => {
                // For now, fallback to by_example for fixed_size
                // Could implement fixed-size chunking later
                self.generate_by_example(schema)
            }
        }
    }

    fn validate_output(&self, output: &str) -> Result<()> {
        // Validate YAML frontmatter exists (after header section)
        if !output.contains("---\n") {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "llm-rag".to_string(),
                message: "Output must contain YAML frontmatter (---)".to_string(),
            });
        }

        // Validate chunks have metadata
        if !output.contains("chunk_id:") {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "llm-rag".to_string(),
                message: "Output must contain chunk_id metadata".to_string(),
            });
        }

        // Validate essential metadata fields
        if !output.contains("topic:") {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "llm-rag".to_string(),
                message: "Output must contain topic metadata".to_string(),
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
            mathematical_definition: None,
            code_refs: CodeReferences {
                rust: "test::example".to_string(),
                python: "test.example".to_string(),
                nodejs: "test.example".to_string(),
            },
            examples: vec![Example {
                title: "Basic Example".to_string(),
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
        let generator = LlmRagGenerator::new();
        assert_eq!(generator.name(), "llm-rag");
        assert_eq!(generator.file_extension(), "md");
    }

    #[test]
    fn test_generate_by_example() {
        let generator = LlmRagGenerator::new();
        let schema = create_test_schema();
        let result = generator.generate(&schema);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("---\n"));
        assert!(output.contains("chunk_id:"));
        assert!(output.contains("Test Example"));
        assert!(output.contains("Basic Example"));
        assert!(output.contains("```rust"));
        assert!(output.contains("```python"));
        assert!(output.contains("```javascript"));
    }

    #[test]
    fn test_validate_output() {
        let generator = LlmRagGenerator::new();
        let schema = create_test_schema();
        let output = generator.generate(&schema).unwrap();

        // Debug: print output to see what's generated
        println!("Generated output:\n{}", output);

        let validation = generator.validate_output(&output);
        if validation.is_err() {
            println!("Validation error: {:?}", validation.as_ref().unwrap_err());
        }
        assert!(validation.is_ok());
    }

    #[test]
    fn test_validate_invalid_output() {
        let generator = LlmRagGenerator::new();
        let invalid = "No frontmatter here";
        let validation = generator.validate_output(invalid);
        assert!(validation.is_err());
    }

    #[test]
    fn test_from_schema_with_hints() {
        let mut schema = create_test_schema();
        schema.outputs.llm_rag = Some(LlmRagHints {
            chunk_strategy: "by_section".to_string(),
            max_chunk_size: 1024,
            embedding_priority: "high".to_string(),
        });

        let generator = LlmRagGenerator::from_schema(&schema);
        assert_eq!(generator.embedding_priority, "high");
        assert_eq!(generator.max_chunk_size, 1024);
    }
}
