use kb_core::generator::OutputGenerator;
/// LaTeX documentation generator implementation
///
/// Generates professional LaTeX documentation suitable for academic papers,
/// technical reports, and PDF generation. Uses article class with proper
/// mathematical typesetting.
use kb_core::{Result, Schema};
use tera::{Context, Tera};

/// LaTeX documentation generator
pub struct LatexGenerator {
    tera: Tera,
}

impl LatexGenerator {
    /// Create a new LaTeX generator
    pub fn new() -> Result<Self> {
        let mut tera = Tera::default();
        tera.add_raw_template("latex", LATEX_TEMPLATE)?;
        Ok(Self { tera })
    }

    /// Generate LaTeX document from schema
    fn generate_latex(&self, schema: &Schema) -> Result<String> {
        let mut context = Context::new();
        context.insert("title", &schema.title);
        context.insert("topic", &schema.topic);
        context.insert("description", &schema.description);

        if let Some(math_def) = &schema.mathematical_definition {
            context.insert("math_definition", math_def);
        }

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
                ex.insert(
                    "python_code".to_string(),
                    tera::Value::String(e.code.python.clone()),
                );
                if let Some(output) = &e.expected_output {
                    ex.insert("output".to_string(), tera::Value::String(output.clone()));
                }
                ex
            })
            .collect();
        context.insert("examples", &examples);

        let empty_sections: Vec<tera::Map<String, tera::Value>> = vec![];
        if let Some(article) = &schema.article {
            use kb_core::schema::Article;
            match article {
                Article::Simple(simple) => {
                    // For simple articles, use content as introduction
                    context.insert("introduction", &simple.content);
                    context.insert("sections", &empty_sections);
                }
                Article::Structured(structured) => {
                    context.insert("introduction", &structured.introduction.hook);

                    let sections: Vec<_> = structured
                        .sections
                        .iter()
                        .map(|s| {
                            let mut sec = tera::Map::new();
                            sec.insert("title".to_string(), tera::Value::String(s.title.clone()));
                            sec.insert(
                                "content".to_string(),
                                tera::Value::String(s.content.clone()),
                            );
                            sec
                        })
                        .collect();
                    context.insert("sections", &sections);

                    if let Some(conclusion) = &structured.conclusion {
                        context.insert("conclusion", &conclusion.summary);
                    }
                }
            }
        } else {
            context.insert("sections", &empty_sections);
        }

        Ok(self.tera.render("latex", &context)?)
    }
}

impl Default for LatexGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to initialize LaTeX generator")
    }
}

impl OutputGenerator for LatexGenerator {
    fn name(&self) -> &str {
        "latex"
    }

    fn file_extension(&self) -> &str {
        "tex"
    }

    fn generate(&self, schema: &Schema) -> Result<String> {
        self.generate_latex(schema)
    }

    fn validate_output(&self, output: &str) -> Result<()> {
        if output.is_empty() {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "latex".to_string(),
                message: "Output is empty".to_string(),
            });
        }

        if !output.contains(r"\documentclass") {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "latex".to_string(),
                message: "Missing \\documentclass directive".to_string(),
            });
        }

        if !output.contains(r"\begin{document}") || !output.contains(r"\end{document}") {
            return Err(kb_core::KbError::OutputValidationError {
                generator: "latex".to_string(),
                message: "Missing document environment".to_string(),
            });
        }

        Ok(())
    }
}

const LATEX_TEMPLATE: &str = r#"\documentclass[11pt,a4paper]{article}
\usepackage{amsmath}
\usepackage{amssymb}
\usepackage{listings}
\usepackage{xcolor}
\usepackage{hyperref}

\lstset{
    language=Python,
    basicstyle=\ttfamily\small,
    keywordstyle=\color{blue},
    commentstyle=\color{gray},
    stringstyle=\color{red},
    showstringspaces=false,
    breaklines=true,
    frame=single,
    numbers=left,
    numberstyle=\tiny\color{gray}
}

\title{ {{ title }} }
\author{MathHook CAS}
\date{\today}

\begin{document}

\maketitle

\begin{abstract}
{{ description }}
\end{abstract}

{% if math_definition %}
\section{Mathematical Definition}

\begin{equation}
{{ math_definition }}
\end{equation}
{% endif %}

{% if introduction %}
\section{Introduction}

{{ introduction }}
{% endif %}

{% for section in sections %}
\section{ {{ section.title }} }

{{ section.content }}
{% endfor %}

\section{Examples}

{% for example in examples %}
\subsection{ {{ example.title }} }

{{ example.explanation }}

\begin{lstlisting}
{{ example.python_code }}
\end{lstlisting}

{% if example.output %}
\textbf{Output:}

\begin{verbatim}
{{ example.output }}
\end{verbatim}
{% endif %}

{% endfor %}

{% if conclusion %}
\section{Conclusion}

{{ conclusion }}
{% endif %}

\end{document}
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
            code_refs: Some(CodeReferences {
                rust: "test::example".to_string(),
                python: "test.example".to_string(),
                nodejs: "test.example".to_string(),
            }),
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
        let generator = LatexGenerator::new();
        assert!(generator.is_ok());
    }

    #[test]
    fn test_generate() {
        let generator = LatexGenerator::new().unwrap();
        let schema = create_test_schema();
        let result = generator.generate(&schema);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains(r"\documentclass"));
        assert!(output.contains(r"\begin{document}"));
        assert!(output.contains(r"\end{document}"));
    }

    #[test]
    fn test_validate_output() {
        let generator = LatexGenerator::new().unwrap();
        let schema = create_test_schema();
        let output = generator.generate(&schema).unwrap();
        let validation = generator.validate_output(&output);
        assert!(validation.is_ok());
    }
}
