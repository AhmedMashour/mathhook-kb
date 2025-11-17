use std::collections::HashMap;
/// Tera templates for Jupyter notebook generation
///
/// This module provides template rendering for converting KB schemas into
/// Jupyter notebook markdown and code cells.
use tera::{Context, Tera};

/// Template manager for Jupyter notebooks
pub struct TemplateManager {
    tera: Tera,
}

impl TemplateManager {
    /// Create a new template manager with embedded templates
    pub fn new() -> Result<Self, tera::Error> {
        let mut tera = Tera::default();

        // Add embedded templates
        tera.add_raw_template("header", HEADER_TEMPLATE)?;
        tera.add_raw_template("introduction", INTRODUCTION_TEMPLATE)?;
        tera.add_raw_template("section", SECTION_TEMPLATE)?;
        tera.add_raw_template("example", EXAMPLE_TEMPLATE)?;
        tera.add_raw_template("sidebar", SIDEBAR_TEMPLATE)?;
        tera.add_raw_template("conclusion", CONCLUSION_TEMPLATE)?;
        tera.add_raw_template("exercise", EXERCISE_TEMPLATE)?;

        Ok(Self { tera })
    }

    /// Render a template with the given context
    pub fn render(&self, template_name: &str, context: &Context) -> Result<String, tera::Error> {
        self.tera.render(template_name, context)
    }

    /// Render header section
    pub fn render_header(&self, title: &str, description: &str) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("title", title);
        context.insert("description", description);
        self.render("header", &context)
    }

    /// Render introduction section (for articles)
    pub fn render_introduction(
        &self,
        hook: &str,
        objectives: &[String],
        prerequisites: &[String],
        time: Option<&String>,
    ) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("hook", hook);
        context.insert("objectives", objectives);
        context.insert("prerequisites", prerequisites);
        context.insert("time", &time);
        self.render("introduction", &context)
    }

    /// Render a content section
    pub fn render_section(
        &self,
        title: &str,
        content: &str,
        subsections: &[HashMap<String, String>],
    ) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("title", title);
        context.insert("content", content);
        context.insert("subsections", subsections);
        self.render("section", &context)
    }

    /// Render a code example
    pub fn render_example(
        &self,
        title: &str,
        explanation: &str,
        code: &str,
        output: Option<&String>,
    ) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("title", title);
        context.insert("explanation", explanation);
        context.insert("code", code);
        context.insert("output", &output);
        self.render("example", &context)
    }

    /// Render a sidebar
    pub fn render_sidebar(
        &self,
        sidebar_type: &str,
        title: &str,
        content: &str,
    ) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("sidebar_type", sidebar_type);
        context.insert("title", title);
        context.insert("content", content);
        self.render("sidebar", &context)
    }

    /// Render conclusion section
    pub fn render_conclusion(
        &self,
        summary: &str,
        next_steps: &[String],
        further_reading: &[String],
    ) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("summary", summary);
        context.insert("next_steps", next_steps);
        context.insert("further_reading", further_reading);
        self.render("conclusion", &context)
    }

    /// Render an exercise
    pub fn render_exercise(
        &self,
        number: usize,
        title: &str,
        difficulty: &str,
        problem: &str,
        hints: &[String],
    ) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("number", &number);
        context.insert("title", title);
        context.insert("difficulty", difficulty);
        context.insert("problem", problem);
        context.insert("hints", hints);
        self.render("exercise", &context)
    }
}

impl Default for TemplateManager {
    fn default() -> Self {
        Self::new().expect("Failed to initialize template manager")
    }
}

// Embedded template strings

const HEADER_TEMPLATE: &str = r#"# {{ title }}

{{ description }}

---
"#;

const INTRODUCTION_TEMPLATE: &str = r#"## Introduction

{{ hook }}

### 🎯 Learning Objectives

{% for objective in objectives -%}
- {{ objective }}
{% endfor %}

### 📋 Prerequisites

{% for prerequisite in prerequisites -%}
- {{ prerequisite }}
{% endfor %}

{% if time -%}
**⏱️ Estimated Time:** {{ time }}
{% endif -%}

---
"#;

const SECTION_TEMPLATE: &str = r#"## {{ title }}

{{ content }}

{% for subsection in subsections -%}
### {{ subsection.title }}

{{ subsection.content }}

{% endfor -%}
"#;

const EXAMPLE_TEMPLATE: &str = r#"### Example: {{ title }}

{{ explanation }}

**Code:**
```python
{{ code }}
```

{% if output -%}
**Expected Output:**
```
{{ output }}
```
{% endif -%}
"#;

const SIDEBAR_TEMPLATE: &str = r#"{% if sidebar_type == "tip" -%}
> 💡 **Tip:** {{ title }}
>
> {{ content }}
{% elif sidebar_type == "warning" -%}
> ⚠️ **Warning:** {{ title }}
>
> {{ content }}
{% elif sidebar_type == "performance" -%}
> ⚡ **Performance:** {{ title }}
>
> {{ content }}
{% elif sidebar_type == "best_practice" -%}
> ✅ **Best Practice:** {{ title }}
>
> {{ content }}
{% else -%}
> 📝 **Note:** {{ title }}
>
> {{ content }}
{% endif -%}
"#;

const CONCLUSION_TEMPLATE: &str = r#"## 🎓 Conclusion

{{ summary }}

### 🚀 Next Steps

{% for step in next_steps -%}
- {{ step }}
{% endfor %}

### 📚 Further Reading

{% for reading in further_reading -%}
- {{ reading }}
{% endfor %}
"#;

const EXERCISE_TEMPLATE: &str = r#"### Exercise {{ number }}: {{ title }}

**Difficulty:** {{ difficulty }}

**Problem:**

{{ problem }}

{% if hints -%}
**Hints:**
{% for hint in hints -%}
- {{ hint }}
{% endfor %}
{% endif -%}

**Your Solution:**
```python
# Write your solution here

```
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_manager_creation() {
        let manager = TemplateManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_render_header() {
        let manager = TemplateManager::new().unwrap();
        let result = manager.render_header("Test Title", "Test description");
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("Test Title"));
        assert!(rendered.contains("Test description"));
    }

    #[test]
    fn test_render_introduction() {
        let manager = TemplateManager::new().unwrap();
        let objectives = vec!["Learn X".to_string(), "Understand Y".to_string()];
        let prerequisites = vec!["Know Z".to_string()];
        let time = Some("30 minutes".to_string());

        let result = manager.render_introduction(
            "This is a hook",
            &objectives,
            &prerequisites,
            time.as_ref(),
        );
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("This is a hook"));
        assert!(rendered.contains("Learn X"));
        assert!(rendered.contains("30 minutes"));
    }

    #[test]
    fn test_render_example() {
        let manager = TemplateManager::new().unwrap();
        let result = manager.render_example(
            "Power Rule",
            "This demonstrates the power rule",
            "x**2",
            Some(&"2*x".to_string()),
        );
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("Power Rule"));
        assert!(rendered.contains("x**2"));
        assert!(rendered.contains("2*x"));
    }

    #[test]
    fn test_render_sidebar_tip() {
        let manager = TemplateManager::new().unwrap();
        let result = manager.render_sidebar("tip", "Quick Tip", "Always validate input");
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("💡"));
        assert!(rendered.contains("Quick Tip"));
    }

    #[test]
    fn test_render_conclusion() {
        let manager = TemplateManager::new().unwrap();
        let next_steps = vec!["Step 1".to_string(), "Step 2".to_string()];
        let further_reading = vec!["Book A".to_string()];

        let result = manager.render_conclusion("We learned a lot", &next_steps, &further_reading);
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("We learned a lot"));
        assert!(rendered.contains("Step 1"));
        assert!(rendered.contains("Book A"));
    }

    #[test]
    fn test_render_exercise() {
        let manager = TemplateManager::new().unwrap();
        let hints = vec!["Hint 1".to_string(), "Hint 2".to_string()];

        let result =
            manager.render_exercise(1, "First Exercise", "Medium", "Solve this problem", &hints);
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("Exercise 1"));
        assert!(rendered.contains("Medium"));
        assert!(rendered.contains("Hint 1"));
    }
}
