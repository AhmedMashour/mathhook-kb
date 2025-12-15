/// Generator trait for output formats
///
/// This module defines the trait that all output generators must implement,
/// enabling extensibility for new formats.
use crate::{Result, Schema};
use std::path::Path;

/// Trait for generating documentation output from schemas
pub trait OutputGenerator: Send + Sync {
    /// Name of this generator (e.g., "jupyter", "mdbook")
    fn name(&self) -> &str;

    /// File extension for generated files (e.g., "ipynb", "md")
    fn file_extension(&self) -> &str;

    /// Generate output from a schema
    ///
    /// # Arguments
    ///
    /// * `schema` - The validated schema to generate from
    ///
    /// # Returns
    ///
    /// Returns the generated content as a string
    fn generate(&self, schema: &Schema) -> Result<String>;

    /// Validate generated output
    ///
    /// This should check that the output is syntactically valid for its format.
    /// For example, Jupyter generator should validate JSON structure.
    ///
    /// # Arguments
    ///
    /// * `output` - The generated content to validate
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if valid, or an error describing the validation failure
    fn validate_output(&self, output: &str) -> Result<()>;

    /// Generate and save output to a file
    ///
    /// This is a convenience method that generates output and writes it to disk.
    ///
    /// # Arguments
    ///
    /// * `schema` - The schema to generate from
    /// * `output_path` - Path to write the output file
    fn generate_to_file(&self, schema: &Schema, output_path: &Path) -> Result<()> {
        let output = self.generate(schema)?;
        self.validate_output(&output)?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(output_path, output)?;
        Ok(())
    }

    /// Get the default output filename for a schema
    ///
    /// # Arguments
    ///
    /// * `schema` - The schema to generate a filename for
    ///
    /// # Returns
    ///
    /// Returns a filename like "calculus-derivative.ipynb"
    fn get_output_filename(&self, schema: &Schema) -> String {
        let topic_slug = schema.topic.replace('.', "-");
        format!("{}.{}", topic_slug, self.file_extension())
    }

    /// Check if this generator supports a specific output hint
    ///
    /// This allows generators to conditionally enable features based on schema hints.
    ///
    /// # Arguments
    ///
    /// * `schema` - The schema to check
    ///
    /// # Returns
    ///
    /// Returns true if the generator has specific hints in the schema
    fn has_output_hints(&self, schema: &Schema) -> bool {
        match self.name() {
            "jupyter" => schema.outputs.jupyter.is_some(),
            "mdbook" => schema.outputs.mdbook.is_some(),
            "vue" => schema.outputs.vue_site.is_some(),
            "api-docs" => schema.outputs.api_docs.is_some(),
            "llm-rag" => schema.outputs.llm_rag.is_some(),
            "colab" => schema.outputs.colab.is_some(),
            "latex" => schema.outputs.latex.is_some(),
            _ => false,
        }
    }
}

/// Registry of output generators
pub struct GeneratorRegistry {
    generators: Vec<Box<dyn OutputGenerator>>,
}

impl GeneratorRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            generators: Vec::new(),
        }
    }

    /// Register a new generator
    pub fn register<G: OutputGenerator + 'static>(&mut self, generator: G) {
        self.generators.push(Box::new(generator));
    }

    /// Get a generator by name
    pub fn get(&self, name: &str) -> Option<&dyn OutputGenerator> {
        self.generators
            .iter()
            .find(|g| g.name() == name)
            .map(|g| &**g)
    }

    /// Get all registered generators
    pub fn all(&self) -> &[Box<dyn OutputGenerator>] {
        &self.generators
    }

    /// Generate all formats for a schema
    pub fn generate_all(&self, schema: &Schema, output_dir: &Path) -> Result<Vec<String>> {
        let mut generated_files = Vec::new();

        for generator in &self.generators {
            let filename = generator.get_output_filename(schema);
            let output_path = output_dir.join(&filename);
            generator.generate_to_file(schema, &output_path)?;
            generated_files.push(filename);
        }

        Ok(generated_files)
    }
}

impl Default for GeneratorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::*;

    struct MockGenerator;

    impl OutputGenerator for MockGenerator {
        fn name(&self) -> &str {
            "mock"
        }

        fn file_extension(&self) -> &str {
            "txt"
        }

        fn generate(&self, schema: &Schema) -> Result<String> {
            Ok(format!("Generated: {}", schema.title))
        }

        fn validate_output(&self, _output: &str) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_generator_filename() {
        let schema = Schema {
            topic: "calculus.derivative".to_string(),
            title: "Test".to_string(),
            description: "Test".to_string(),
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

        let gen = MockGenerator;
        let filename = gen.get_output_filename(&schema);
        assert_eq!(filename, "calculus-derivative.txt");
    }

    #[test]
    fn test_generator_registry() {
        let mut registry = GeneratorRegistry::new();
        registry.register(MockGenerator);

        assert!(registry.get("mock").is_some());
        assert!(registry.get("nonexistent").is_none());
        assert_eq!(registry.all().len(), 1);
    }
}
