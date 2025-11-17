use kb_core::generator::OutputGenerator;
/// mdBook generator implementation
///
/// Generates markdown files optimized for mdBook static site generation.
/// Output is reference-style with technical depth and cross-references.
use kb_core::{Result, Schema};
use tera::{Context, Tera};

/// mdBook markdown generator
pub struct MdBookGenerator {
    tera: Tera,
}

impl MdBookGenerator {
    /// Create a new mdBook generator
    pub fn new() -> Result<Self> {
        let mut tera = Tera::default();

        // Add embedded templates for mdBook style
        tera.add_raw_template("page", PAGE_TEMPLATE)?;
        tera.add_raw_template("section", SECTION_TEMPLATE)?;
        tera.add_raw_template("example", EXAMPLE_TEMPLATE)?;
        tera.add_raw_template("deep_dive", DEEP_DIVE_TEMPLATE)?;

        Ok(Self { tera })
    }

    /// Generate page content
    fn generate_page(&self, schema: &Schema) -> Result<String> {
        let mut context = Context::new();
        context.insert("title", &schema.title);
        context.insert("description", &schema.description);
        context.insert("topic", &schema.topic);

        // Add mathematical definition if present
        if let Some(math_def) = &schema.mathematical_definition {
            context.insert("math_definition", math_def);
        }

        // Add code references
        context.insert("rust_ref", &schema.code_refs.rust);
        context.insert("python_ref", &schema.code_refs.python);
        context.insert("nodejs_ref", &schema.code_refs.nodejs);

        // Check if article content exists
        if let Some(article) = &schema.article {
            context.insert("has_article", &true);
            context.insert("hook", &article.introduction.hook);

            // Add sections
            let sections: Vec<_> = article
                .sections
                .iter()
                .map(|s| {
                    let mut section_ctx = tera::Map::new();
                    section_ctx.insert("title".to_string(), tera::Value::String(s.title.clone()));
                    section_ctx.insert(
                        "content".to_string(),
                        tera::Value::String(s.content.clone()),
                    );
                    section_ctx
                })
                .collect();
            context.insert("sections", &sections);

            // Add mdBook-specific deep dives if available
            if let Some(variations) = &article.variations {
                if let Some(mdbook) = &variations.mdbook {
                    let deep_dives: Vec<_> = mdbook
                        .deep_dives
                        .iter()
                        .map(|d| {
                            let mut dive_ctx = tera::Map::new();
                            dive_ctx
                                .insert("title".to_string(), tera::Value::String(d.title.clone()));
                            dive_ctx.insert(
                                "content".to_string(),
                                tera::Value::String(d.content.clone()),
                            );
                            dive_ctx
                        })
                        .collect();
                    context.insert("deep_dives", &deep_dives);

                    if let Some(impl_notes) = &mdbook.implementation_notes {
                        context.insert("implementation_notes", impl_notes);
                    }

                    if let Some(complexity) = &mdbook.complexity_analysis {
                        context.insert("complexity_analysis", complexity);
                    }
                }
            }
        } else {
            context.insert("has_article", &false);
        }

        // Add examples
        let examples: Vec<_> = schema
            .examples
            .iter()
            .map(|e| {
                let mut ex_ctx = tera::Map::new();
                ex_ctx.insert("title".to_string(), tera::Value::String(e.title.clone()));
                ex_ctx.insert(
                    "explanation".to_string(),
                    tera::Value::String(e.explanation.clone()),
                );
                ex_ctx.insert("rust".to_string(), tera::Value::String(e.code.rust.clone()));
                ex_ctx.insert(
                    "python".to_string(),
                    tera::Value::String(e.code.python.clone()),
                );
                ex_ctx.insert(
                    "nodejs".to_string(),
                    tera::Value::String(e.code.nodejs.clone()),
                );
                if let Some(output) = &e.expected_output {
                    ex_ctx.insert("output".to_string(), tera::Value::String(output.clone()));
                }
                ex_ctx
            })
            .collect();
        context.insert("examples", &examples);

        // Add performance info if available
        if let Some(perf) = &schema.performance {
            context.insert("performance", &perf.complexity);
        }

        // Add related topics
        if !schema.related_topics.is_empty() {
            context.insert("related_topics", &schema.related_topics);
        }

        // Add SEO metadata if present
        if let Some(seo) = &schema.seo {
            if let Some(meta_desc) = &seo.meta_description {
                context.insert("seo_meta_description", meta_desc);
            }
            if !seo.keywords.is_empty() {
                let keywords_str = seo.keywords.join(", ");
                context.insert("seo_keywords", &keywords_str);
            }
            if let Some(canonical) = &seo.canonical_url {
                context.insert("seo_canonical_url", canonical);
            }
            if let Some(og_title) = &seo.og_title {
                context.insert("seo_og_title", og_title);
            }
            if let Some(og_desc) = &seo.og_description {
                context.insert("seo_og_description", og_desc);
            }
            if let Some(og_img) = &seo.og_image {
                context.insert("seo_og_image", og_img);
            }
            context.insert("seo_twitter_card", &seo.twitter_card);
            if let Some(schema_type) = &seo.schema_org_type {
                context.insert("seo_schema_org_type", schema_type);
            }
            context.insert("seo_language", &seo.language);
        }

        Ok(self.tera.render("page", &context)?)
    }
}

impl Default for MdBookGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to initialize mdBook generator")
    }
}

impl OutputGenerator for MdBookGenerator {
    fn name(&self) -> &str {
        "mdbook"
    }

    fn file_extension(&self) -> &str {
        "md"
    }

    fn generate(&self, schema: &Schema) -> Result<String> {
        self.generate_page(schema)
    }

    fn validate_output(&self, output: &str) -> Result<()> {
        // Validate it's valid markdown
        if output.is_empty() {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "mdbook".to_string(),
                message: "Output is empty".to_string(),
            });
        }

        // Should have a title (# heading)
        if !output.contains("# ") {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "mdbook".to_string(),
                message: "Output must contain a title heading (#)".to_string(),
            });
        }

        Ok(())
    }
}

// Embedded templates for mdBook style

const PAGE_TEMPLATE: &str = r#"---
{% if seo_meta_description %}description: "{{ seo_meta_description }}"{% endif %}
{% if seo_keywords %}keywords: {{ seo_keywords }}{% endif %}
{% if seo_canonical_url %}canonical_url: "{{ seo_canonical_url }}"{% endif %}
{% if seo_og_title %}og:title: "{{ seo_og_title }}"{% endif %}
{% if seo_og_description %}og:description: "{{ seo_og_description }}"{% endif %}
{% if seo_og_image %}og:image: "{{ seo_og_image }}"{% endif %}
{% if seo_twitter_card %}twitter:card: "{{ seo_twitter_card }}"{% endif %}
{% if seo_schema_org_type %}schema_type: "{{ seo_schema_org_type }}"{% endif %}
{% if seo_language %}language: "{{ seo_language }}"{% endif %}
---

# {{ title }}

> **Topic**: `{{ topic }}`

{{ description }}

{% if math_definition %}
## Mathematical Definition

$${{ math_definition }}$$
{% endif %}

{% if has_article %}
{{ hook }}

{% for section in sections %}
## {{ section.title }}

{{ section.content }}
{% endfor %}

{% if deep_dives %}
---

## Deep Dives

{% for dive in deep_dives %}
### {{ dive.title }}

{{ dive.content }}
{% endfor %}
{% endif %}

{% if implementation_notes %}
---

## Implementation Notes

{{ implementation_notes }}
{% endif %}

{% if complexity_analysis %}
---

## Complexity Analysis

{{ complexity_analysis }}
{% endif %}

{% endif %}

## Examples

{% for example in examples %}
### {{ example.title }}

{{ example.explanation }}

<details>
<summary><b>Rust</b></summary>

```rust
{{ example.rust }}
```
</details>

<details>
<summary><b>Python</b></summary>

```python
{{ example.python }}
```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
{{ example.nodejs }}
```
</details>

{% if example.output %}
**Expected Output:**
```
{{ example.output }}
```
{% endif %}

{% endfor %}

{% if performance %}
## Performance

**Time Complexity**: {{ performance }}
{% endif %}

## API Reference

- **Rust**: `{{ rust_ref }}`
- **Python**: `{{ python_ref }}`
- **JavaScript**: `{{ nodejs_ref }}`

{% if related_topics %}
## See Also

{% for topic in related_topics %}
- [{{ topic }}](../{{ topic | replace(from=".", to="/") }}.md)
{% endfor %}
{% endif %}
"#;

const SECTION_TEMPLATE: &str = "";
const EXAMPLE_TEMPLATE: &str = "";
const DEEP_DIVE_TEMPLATE: &str = "";

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::schema::*;

    fn create_test_schema() -> Schema {
        Schema {
            topic: "test.example".to_string(),
            title: "Test Example".to_string(),
            description: "A test example for mdBook".to_string(),
            mathematical_definition: Some("f(x) = x^2".to_string()),
            code_refs: CodeReferences {
                rust: "test::example".to_string(),
                python: "test.example".to_string(),
                nodejs: "test.example".to_string(),
            },
            examples: vec![Example {
                title: "Power Rule".to_string(),
                explanation: "Demonstrates the power rule".to_string(),
                code: CodeSnippets {
                    rust: "let f = expr!(x ^ 2);".to_string(),
                    python: "f = x**2".to_string(),
                    nodejs: "const f = x**2;".to_string(),
                },
                expected_output: Some("2*x".to_string()),
            }],
            article: None,
            use_cases: vec![],
            related_topics: vec!["test.related".to_string()],
            performance: Some(Performance {
                complexity: "O(n)".to_string(),
                typical_time: None,
                benchmarks: std::collections::HashMap::new(),
            }),
            interactive_playground: None,
            outputs: OutputHints::default(),
            metadata: None,
            seo: None,
        }
    }

    #[test]
    fn test_generator_creation() {
        let generator = MdBookGenerator::new();
        assert!(generator.is_ok());

        let gen = generator.unwrap();
        assert_eq!(gen.name(), "mdbook");
        assert_eq!(gen.file_extension(), "md");
    }

    #[test]
    fn test_generate() {
        let generator = MdBookGenerator::new().unwrap();
        let schema = create_test_schema();
        let result = generator.generate(&schema);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("# Test Example"));
        assert!(output.contains("test.example"));
        assert!(output.contains("Power Rule"));
        assert!(output.contains("```rust"));
        assert!(output.contains("```python"));
        assert!(output.contains("```javascript"));
        assert!(output.contains("O(n)"));
    }

    #[test]
    fn test_validate_output() {
        let generator = MdBookGenerator::new().unwrap();
        let schema = create_test_schema();
        let output = generator.generate(&schema).unwrap();
        let validation = generator.validate_output(&output);
        assert!(validation.is_ok());
    }

    #[test]
    fn test_validate_empty_output() {
        let generator = MdBookGenerator::new().unwrap();
        let validation = generator.validate_output("");
        assert!(validation.is_err());
    }

    #[test]
    fn test_validate_no_title() {
        let generator = MdBookGenerator::new().unwrap();
        let validation = generator.validate_output("Some content without title");
        assert!(validation.is_err());
    }
}
