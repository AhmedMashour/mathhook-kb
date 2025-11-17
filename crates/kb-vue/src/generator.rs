use kb_core::generator::OutputGenerator;
/// Vue SSR site generator implementation
///
/// Generates Vue.js Single File Components (.vue) for interactive documentation
/// with live demos, syntax highlighting, and responsive design.
use kb_core::{Result, Schema};
use tera::{Context, Tera};

/// Vue SSR site generator
pub struct VueGenerator {
    tera: Tera,
}

impl VueGenerator {
    /// Create a new Vue generator
    pub fn new() -> Result<Self> {
        let mut tera = Tera::default();
        tera.add_raw_template("page", VUE_COMPONENT_TEMPLATE)?;
        Ok(Self { tera })
    }

    /// Generate Vue component content
    fn generate_component(&self, schema: &Schema) -> Result<String> {
        let mut context = Context::new();
        context.insert("title", &schema.title);
        context.insert("description", &schema.description);
        context.insert("topic", &schema.topic);

        // Add mathematical definition
        if let Some(math_def) = &schema.mathematical_definition {
            context.insert("math_definition", math_def);
        }

        // Add examples
        let examples: Vec<_> = schema
            .examples
            .iter()
            .map(|e| {
                let mut ex = tera::Map::new();
                ex.insert("title".to_string(), tera::Value::String(e.title.clone()));
                ex.insert(
                    "explanation".to_string(),
                    tera::Value::String(e.explanation.clone()),
                );
                ex.insert("rust".to_string(), tera::Value::String(e.code.rust.clone()));
                ex.insert(
                    "python".to_string(),
                    tera::Value::String(e.code.python.clone()),
                );
                ex.insert(
                    "nodejs".to_string(),
                    tera::Value::String(e.code.nodejs.clone()),
                );
                if let Some(output) = &e.expected_output {
                    ex.insert("output".to_string(), tera::Value::String(output.clone()));
                }
                ex
            })
            .collect();
        context.insert("examples", &examples);

        // Add SEO metadata
        let empty_keywords: Vec<String> = vec![];
        if let Some(seo) = &schema.seo {
            context.insert("keywords", &seo.keywords);
            if let Some(desc) = &seo.meta_description {
                context.insert("meta_description", desc);
            }
            if let Some(og_img) = &seo.og_image {
                context.insert("og_image", og_img);
            }
        } else {
            context.insert("keywords", &empty_keywords);
        }

        Ok(self.tera.render("page", &context)?)
    }
}

impl Default for VueGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to initialize Vue generator")
    }
}

impl OutputGenerator for VueGenerator {
    fn name(&self) -> &str {
        "vue"
    }

    fn file_extension(&self) -> &str {
        "vue"
    }

    fn generate(&self, schema: &Schema) -> Result<String> {
        self.generate_component(schema)
    }

    fn validate_output(&self, output: &str) -> Result<()> {
        if output.is_empty() {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "vue".to_string(),
                message: "Output is empty".to_string(),
            });
        }

        // Should have Vue component structure
        if !output.contains("<template>") || !output.contains("<script") {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "vue".to_string(),
                message: "Output must contain Vue component structure (<template> and <script>)"
                    .to_string(),
            });
        }

        Ok(())
    }
}

const VUE_COMPONENT_TEMPLATE: &str = r#"<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>{{ title }}</h1>
      <p class="description">{{ description }}</p>
    </header>

    {% if math_definition %}
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">{{ math_definition }}</div>
    </section>
    {% endif %}

    <section class="examples">
      <h2>Interactive Examples</h2>
      {% for example in examples %}
      <div class="example-card">
        <h3>{{ example.title }}</h3>
        <p>{{ example.explanation }}</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">{{ example.rust }}</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">{{ example.python }}</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">{{ example.nodejs }}</code></pre>
        </div>

        {% if example.output %}
        <div class="output">
          <strong>Output:</strong>
          <pre><code>{{ example.output }}</code></pre>
        </div>
        {% endif %}
      </div>
      {% endfor %}
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const activeTab = ref('python')

// SEO metadata
const metaDescription = "{{ meta_description | default(value=description) }}"
const keywords = {{ keywords | json_encode }}

// Define page metadata
definePageMeta({
  title: '{{ title }}',
  description: metaDescription,
  keywords: keywords.join(', '),
  {% if og_image %}ogImage: '{{ og_image }}',{% endif %}
})
</script>

<style scoped>
.doc-page {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}

.doc-header h1 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.description {
  font-size: 1.2rem;
  color: #666;
}

.math-definition {
  background: #f5f5f5;
  padding: 1.5rem;
  border-radius: 8px;
  margin: 2rem 0;
}

.example-card {
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 1.5rem;
  margin-bottom: 2rem;
}

.code-tabs {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.code-tabs button {
  padding: 0.5rem 1rem;
  border: none;
  background: #eee;
  cursor: pointer;
  border-radius: 4px;
}

.code-tabs button:hover {
  background: #ddd;
}

.code-block {
  background: #1e1e1e;
  color: #d4d4d4;
  padding: 1rem;
  border-radius: 4px;
  overflow-x: auto;
}

.output {
  margin-top: 1rem;
  padding: 1rem;
  background: #f9f9f9;
  border-left: 4px solid #42b883;
}
</style>
"#;

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
        let generator = VueGenerator::new();
        assert!(generator.is_ok());
    }

    #[test]
    fn test_generate() {
        let generator = VueGenerator::new().unwrap();
        let schema = create_test_schema();
        let result = generator.generate(&schema);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("<template>"));
        assert!(output.contains("<script"));
        assert!(output.contains("Test Example"));
    }

    #[test]
    fn test_validate_output() {
        let generator = VueGenerator::new().unwrap();
        let schema = create_test_schema();
        let output = generator.generate(&schema).unwrap();
        println!("Generated Vue output:\n{}", output);
        let validation = generator.validate_output(&output);
        if validation.is_err() {
            eprintln!("Validation error: {:?}", validation);
        }
        assert!(validation.is_ok());
    }
}
