/// Schema types for MathHook Knowledge Base
///
/// This module defines the structure of documentation schemas following
/// the Hybrid Option C approach: Core content (required) + Output hints (optional)
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod validation;
mod article;

pub use validation::validate_schema;
pub use article::*;

/// Main schema structure for a documentation topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Schema {
    /// Unique topic identifier (e.g., "calculus.derivative")
    pub topic: String,

    /// Human-readable title
    pub title: String,

    /// Detailed description of the function/concept
    pub description: String,

    /// Mathematical definition (LaTeX)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mathematical_definition: Option<String>,

    /// Code references across languages
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_refs: Option<CodeReferences>,

    /// Code examples demonstrating usage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<Example>,

    /// Article content (rich narrative, tutorial, explanations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article: Option<Article>,

    /// Use cases and applications (for quick reference)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_cases: Vec<String>,

    /// Related topics
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_topics: Vec<String>,

    /// Performance characteristics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<Performance>,

    /// Interactive playground configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive_playground: Option<PlaygroundConfig>,

    /// Output-specific hints (optional customization)
    #[serde(default, skip_serializing_if = "OutputHints::is_default")]
    pub outputs: OutputHints,

    /// Schema metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// SEO optimization data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo: Option<SeoMetadata>,
}

/// Code references for different language bindings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeReferences {
    /// Rust function path (e.g., "mathhook_core::calculus::derivative")
    pub rust: String,

    /// Python function path (e.g., "mathhook.calculus.derivative") - optional
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub python: String,

    /// Node.js function path (e.g., "mathhook.calculus.derivative") - optional
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub nodejs: String,
}

/// A code example with multi-language support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Example {
    /// Example title (e.g., "Power Rule")
    pub title: String,

    /// Explanation of what this example demonstrates (optional)
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub explanation: String,

    /// Code snippets in different languages
    pub code: CodeSnippets,

    /// Expected output (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_output: Option<String>,
}

/// Code snippets in supported languages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeSnippets {
    /// Rust code (optional for non-Rust bindings)
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub rust: String,

    /// Python code (optional, defaults to empty)
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub python: String,

    /// JavaScript/Node.js code (optional, defaults to empty)
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub nodejs: String,
}

/// Performance characteristics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Performance {
    /// Time complexity (e.g., "O(n)")
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub complexity: String,

    /// Typical execution time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typical_time: Option<String>,

    /// Benchmark data - accepts any format and converts to map
    #[serde(default, deserialize_with = "deserialize_benchmarks", skip_serializing_if = "HashMap::is_empty")]
    pub benchmarks: HashMap<String, String>,
}

/// Deserialize benchmarks flexibly - accepts map, sequence, or anything else
fn deserialize_benchmarks<'de, D>(deserializer: D) -> std::result::Result<HashMap<String, String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;

    match value {
        Value::Object(map) => {
            let mut result = HashMap::new();
            for (k, v) in map {
                let v_str = match v {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    other => other.to_string(),
                };
                result.insert(k, v_str);
            }
            Ok(result)
        }
        // Any other format (array, string, number, bool, null) returns empty map
        _ => Ok(HashMap::new()),
    }
}

/// Interactive playground configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlaygroundConfig {
    /// Enable playground
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Default expression to show
    pub default_expression: String,

    /// Default variable
    pub default_variable: String,
}

/// Output-specific hints for customization
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct OutputHints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jupyter: Option<JupyterHints>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdbook: Option<MdBookHints>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vue_site: Option<VueSiteHints>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_docs: Option<ApiDocsHints>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_rag: Option<LlmRagHints>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub colab: Option<ColabHints>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub latex: Option<LatexHints>,
}

impl OutputHints {
    fn is_default(&self) -> bool {
        self.jupyter.is_none()
            && self.mdbook.is_none()
            && self.vue_site.is_none()
            && self.api_docs.is_none()
            && self.llm_rag.is_none()
            && self.colab.is_none()
            && self.latex.is_none()
    }
}

/// Jupyter notebook specific hints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JupyterHints {
    #[serde(default)]
    pub include_interactive_plots: bool,

    #[serde(default)]
    pub include_performance_section: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_intro: Option<String>,
}

/// mdBook specific hints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MdBookHints {
    #[serde(default)]
    pub include_mathematical_proof: bool,

    #[serde(default)]
    pub include_implementation_details: bool,

    #[serde(default = "default_true")]
    pub runnable_code: bool,
}

/// Vue SSR site specific hints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VueSiteHints {
    #[serde(default = "default_true")]
    pub include_live_demo: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub seo_keywords: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_image: Option<String>,
}

/// API documentation specific hints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiDocsHints {
    #[serde(default = "default_true")]
    pub include_playground: bool,

    #[serde(default = "default_true")]
    pub show_all_languages: bool,
}

/// LLM RAG specific hints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LlmRagHints {
    /// Chunking strategy ("by_example", "by_section", "fixed_size")
    #[serde(default = "default_chunk_strategy")]
    pub chunk_strategy: String,

    /// Maximum chunk size in tokens
    #[serde(default = "default_chunk_size")]
    pub max_chunk_size: usize,

    /// Embedding priority ("high", "medium", "low")
    #[serde(default = "default_priority")]
    pub embedding_priority: String,
}

/// Google Colab specific hints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColabHints {
    #[serde(default)]
    pub include_gpu_example: bool,

    #[serde(default = "default_install_command")]
    pub install_command: String,
}

/// LaTeX specific hints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LatexHints {
    #[serde(default = "default_document_class")]
    pub document_class: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_packages: Vec<String>,
}

/// Schema metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    /// Schema version
    #[serde(default = "default_schema_version")]
    pub schema_version: String,

    /// Author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Last updated date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,

    /// Tags for categorization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// SEO metadata for search engine optimization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeoMetadata {
    /// Primary SEO keywords (most important, 3-5 keywords)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,

    /// Secondary keywords (related terms, 5-10 keywords)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secondary_keywords: Vec<String>,

    /// Meta description (150-160 characters, shown in search results)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_description: Option<String>,

    /// Canonical URL (prevents duplicate content issues)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_url: Option<String>,

    /// Open Graph title (social media sharing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_title: Option<String>,

    /// Open Graph description (social media sharing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_description: Option<String>,

    /// Open Graph image URL (social media preview image)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_image: Option<String>,

    /// Twitter card type ("summary", "summary_large_image")
    #[serde(default = "default_twitter_card")]
    pub twitter_card: String,

    /// Structured data type (Schema.org type, e.g., "TechArticle", "HowTo")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_org_type: Option<String>,

    /// Page priority for sitemap (0.0 to 1.0)
    #[serde(default = "default_priority_value")]
    pub priority: f32,

    /// Update frequency for sitemap ("always", "hourly", "daily", "weekly", "monthly", "yearly", "never")
    #[serde(default = "default_change_frequency")]
    pub change_frequency: String,

    /// Language code (e.g., "en", "en-US")
    #[serde(default = "default_language")]
    pub language: String,

    /// Alternative language versions (for multilingual sites)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub alternate_languages: HashMap<String, String>,
}

// Default value functions
fn default_true() -> bool {
    true
}

fn default_chunk_strategy() -> String {
    "by_example".to_string()
}

fn default_chunk_size() -> usize {
    512
}

fn default_priority() -> String {
    "medium".to_string()
}

fn default_install_command() -> String {
    "!pip install mathhook".to_string()
}

fn default_document_class() -> String {
    "article".to_string()
}

fn default_schema_version() -> String {
    "1.0".to_string()
}

fn default_twitter_card() -> String {
    "summary".to_string()
}

fn default_priority_value() -> f32 {
    0.5
}

fn default_change_frequency() -> String {
    "monthly".to_string()
}

fn default_language() -> String {
    "en".to_string()
}

impl Schema {
    /// Load a schema from a YAML file
    pub fn load_from_file(path: &std::path::Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let schema: Schema = serde_yaml::from_str(&content)?;
        validate_schema(&schema, path.to_string_lossy().to_string())?;
        Ok(schema)
    }

    /// Load multiple schemas from a directory
    pub fn load_from_directory(path: &std::path::Path) -> crate::Result<Vec<Self>> {
        let mut schemas = Vec::new();

        for entry in walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "yaml" || ext == "yml"))
        {
            schemas.push(Self::load_from_file(entry.path())?);
        }

        Ok(schemas)
    }

    /// Save schema to YAML file
    pub fn save_to_file(&self, path: &std::path::Path) -> crate::Result<()> {
        let yaml = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_deserialization() {
        let yaml = r#"
topic: "calculus.derivative"
title: "Symbolic Differentiation"
description: "Computes the derivative of an expression"

code_refs:
  rust: "mathhook_core::calculus::derivative"
  python: "mathhook.calculus.derivative"
  nodejs: "mathhook.calculus.derivative"

examples:
  - title: "Power Rule"
    explanation: "Derivative of x^n"
    code:
      rust: "let f = expr!(x ^ 3);"
      python: "f = expr('x^3')"
      nodejs: "const f = expr('x^3');"
"#;

        let schema: Schema = serde_yaml::from_str(yaml).expect("Failed to deserialize");
        assert_eq!(schema.topic, "calculus.derivative");
        assert_eq!(schema.title, "Symbolic Differentiation");
        assert_eq!(schema.examples.len(), 1);
        assert_eq!(schema.examples[0].title, "Power Rule");
    }

    #[test]
    fn test_schema_with_output_hints() {
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

outputs:
  jupyter:
    include_interactive_plots: true
  llm_rag:
    chunk_strategy: "by_section"
    max_chunk_size: 1024
"#;

        let schema: Schema = serde_yaml::from_str(yaml).expect("Failed to deserialize");
        assert!(schema.outputs.jupyter.is_some());
        assert!(
            schema
                .outputs
                .jupyter
                .as_ref()
                .unwrap()
                .include_interactive_plots
        );
        assert!(schema.outputs.llm_rag.is_some());
        assert_eq!(
            schema.outputs.llm_rag.as_ref().unwrap().max_chunk_size,
            1024
        );
    }
}
