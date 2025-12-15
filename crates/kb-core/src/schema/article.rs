/// Article content types for rich documentation narratives
///
/// This module defines structures for comprehensive article-style documentation
/// that can be rendered differently across output formats (tutorial vs reference vs marketing).
use serde::{Deserialize, Serialize};

/// Complete article structure - supports both simple content and structured format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Article {
    /// Simple markdown content (most common)
    Simple(SimpleArticle),
    /// Structured article with introduction, sections, conclusion (boxed to reduce enum size)
    Structured(Box<StructuredArticle>),
}

/// Simple article with just markdown content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimpleArticle {
    /// Markdown content
    pub content: String,
}

/// Structured article with introduction, body sections, and conclusion
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructuredArticle {
    /// Article introduction (sets context, explains importance)
    pub introduction: Introduction,

    /// Main article sections (concept explanations, deep dives)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<Section>,

    /// Conclusion (summary, next steps, further reading)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<Conclusion>,

    /// Sidebar content (tips, warnings, notes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sidebars: Vec<Sidebar>,

    /// Format-specific variations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variations: Option<ArticleVariations>,
}

impl Article {
    /// Get the content as markdown string (works for both variants)
    pub fn to_markdown(&self) -> String {
        match self {
            Article::Simple(s) => s.content.clone(),
            Article::Structured(boxed_s) => {
                let s = boxed_s.as_ref();
                let mut md = String::new();
                md.push_str(&s.introduction.hook);
                md.push_str("\n\n");
                for section in &s.sections {
                    md.push_str(&format!("## {}\n\n{}\n\n", section.title, section.content));
                }
                if let Some(conclusion) = &s.conclusion {
                    md.push_str(&format!("## Conclusion\n\n{}\n", conclusion.summary));
                }
                md
            }
        }
    }

    /// Get the structured article if this is the Structured variant
    pub fn as_structured(&self) -> Option<&StructuredArticle> {
        match self {
            Article::Structured(s) => Some(s),
            Article::Simple(_) => None,
        }
    }

    /// Get the simple article if this is the Simple variant
    pub fn as_simple(&self) -> Option<&SimpleArticle> {
        match self {
            Article::Simple(s) => Some(s),
            Article::Structured(_) => None,
        }
    }
}

/// Introduction section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Introduction {
    /// Hook/opening paragraph (engaging, sets tone)
    pub hook: String,

    /// What you'll learn (bullet points)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub learning_objectives: Vec<String>,

    /// Prerequisites (what reader should know first)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prerequisites: Vec<String>,

    /// Estimated reading/working time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time: Option<String>,
}

/// Main content section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Section {
    /// Section title
    pub title: String,

    /// Section content (markdown-formatted narrative)
    pub content: String,

    /// Subsections (for hierarchical content)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subsections: Vec<SubSection>,

    /// Code examples specific to this section
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_examples: Vec<String>,

    /// Mathematical content (LaTeX)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub math_content: Option<String>,
}

/// Subsection for hierarchical content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubSection {
    /// Subsection title
    pub title: String,

    /// Subsection content
    pub content: String,
}

/// Conclusion section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conclusion {
    /// Summary of key takeaways
    pub summary: String,

    /// Next steps (what to explore next)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub next_steps: Vec<String>,

    /// Further reading/resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub further_reading: Vec<Resource>,

    /// Practice exercises
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exercises: Vec<Exercise>,
}

/// External resource reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Resource {
    /// Resource title
    pub title: String,

    /// Resource URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Resource description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Practice exercise
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Exercise {
    /// Exercise title/prompt
    pub prompt: String,

    /// Difficulty level
    pub difficulty: ExerciseDifficulty,

    /// Hints (optional)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hints: Vec<String>,

    /// Solution (optional, can be hidden)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution: Option<String>,
}

/// Exercise difficulty levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExerciseDifficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Sidebar content (tips, warnings, notes)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sidebar {
    /// Sidebar type
    #[serde(rename = "type")]
    pub sidebar_type: SidebarType,

    /// Sidebar title
    pub title: String,

    /// Sidebar content
    pub content: String,
}

/// Sidebar types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SidebarType {
    /// Helpful tip
    Tip,

    /// Warning about common mistakes
    Warning,

    /// Important note
    Note,

    /// Additional information
    Info,

    /// Performance consideration
    Performance,

    /// Best practice
    BestPractice,
}

/// Format-specific article variations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArticleVariations {
    /// Jupyter-specific content (tutorial style)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jupyter: Option<JupyterArticle>,

    /// mdBook-specific content (reference style)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdbook: Option<MdBookArticle>,

    /// Vue site-specific content (marketing style)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vue_site: Option<VueSiteArticle>,
}

/// Jupyter-specific article content (conversational, tutorial)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JupyterArticle {
    /// Tutorial-style introduction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tutorial_intro: Option<String>,

    /// Interactive prompts throughout
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interactive_prompts: Vec<String>,

    /// Checkpoint questions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checkpoint_questions: Vec<String>,
}

/// mdBook-specific article content (technical reference)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MdBookArticle {
    /// Technical deep dive sections
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deep_dives: Vec<DeepDive>,

    /// Implementation details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation_notes: Option<String>,

    /// Algorithm complexity analysis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complexity_analysis: Option<String>,
}

/// Technical deep dive
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeepDive {
    /// Deep dive title
    pub title: String,

    /// Deep dive content
    pub content: String,
}

/// Vue site-specific article content (marketing/engagement)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VueSiteArticle {
    /// Marketing hook (engaging opening)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_hook: Option<String>,

    /// Use case stories
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_case_stories: Vec<UseCaseStory>,

    /// Call-to-action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_to_action: Option<String>,
}

/// Use case story (real-world application)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UseCaseStory {
    /// Story title
    pub title: String,

    /// The problem
    pub problem: String,

    /// The solution using mathhook
    pub solution: String,

    /// The outcome
    pub outcome: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_deserialization() {
        let yaml = r#"
introduction:
  hook: "Let's explore symbolic differentiation!"
  learning_objectives:
    - "Understand derivatives"
    - "Use mathhook for calculus"
  estimated_time: "15 minutes"

sections:
  - title: "What is a Derivative?"
    content: "A derivative measures the rate of change..."

sidebars:
  - type: tip
    title: "Pro Tip"
    content: "Always simplify before differentiating"
"#;

        let article: Article = serde_yaml::from_str(yaml).expect("Failed to deserialize");
        let structured = article.as_structured().expect("Expected structured article");
        assert_eq!(
            structured.introduction.hook,
            "Let's explore symbolic differentiation!"
        );
        assert_eq!(structured.sections.len(), 1);
        assert_eq!(structured.sidebars.len(), 1);
    }
}
