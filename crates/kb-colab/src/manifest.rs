//! Colab manifest and GitHub URL generation
//!
//! Generates manifest files tracking all notebooks with their GitHub-based Colab URLs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Configuration for GitHub-based Colab URLs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColabConfig {
    /// GitHub username or organization
    pub github_user: String,
    /// GitHub repository name
    pub github_repo: String,
    /// Git branch (usually "main" or "master")
    pub github_branch: String,
    /// Path to notebooks directory in repo
    pub notebooks_path: String,
}

impl ColabConfig {
    /// Create a new config
    pub fn new(
        github_user: impl Into<String>,
        github_repo: impl Into<String>,
        github_branch: impl Into<String>,
        notebooks_path: impl Into<String>,
    ) -> Self {
        Self {
            github_user: github_user.into(),
            github_repo: github_repo.into(),
            github_branch: github_branch.into(),
            notebooks_path: notebooks_path.into(),
        }
    }

    /// Create config from environment variables
    ///
    /// - `COLAB_GITHUB_USER` (default: "AhmedMashour")
    /// - `COLAB_GITHUB_REPO` (default: "mathhook-kb")
    /// - `COLAB_GITHUB_BRANCH` (default: "master")
    /// - `COLAB_NOTEBOOKS_PATH` (default: "colab-notebooks")
    pub fn from_env() -> Self {
        Self {
            github_user: std::env::var("COLAB_GITHUB_USER")
                .unwrap_or_else(|_| "AhmedMashour".to_string()),
            github_repo: std::env::var("COLAB_GITHUB_REPO")
                .unwrap_or_else(|_| "mathhook-kb".to_string()),
            github_branch: std::env::var("COLAB_GITHUB_BRANCH")
                .unwrap_or_else(|_| "master".to_string()),
            notebooks_path: std::env::var("COLAB_NOTEBOOKS_PATH")
                .unwrap_or_else(|_| "colab-notebooks".to_string()),
        }
    }

    /// Generate Colab URL for a notebook hosted on GitHub
    ///
    /// # Arguments
    /// * `category` - Category folder (e.g., "calculus")
    /// * `filename` - Notebook filename (e.g., "derivative.colab.ipynb")
    ///
    /// # Returns
    /// Full Colab URL that opens the notebook from GitHub
    pub fn get_colab_url(&self, category: &str, filename: &str) -> String {
        format!(
            "https://colab.research.google.com/github/{}/{}/blob/{}/{}/{}/{}",
            self.github_user,
            self.github_repo,
            self.github_branch,
            self.notebooks_path,
            category,
            filename
        )
    }

    /// Generate raw GitHub URL for a notebook
    pub fn get_github_url(&self, category: &str, filename: &str) -> String {
        format!(
            "https://github.com/{}/{}/blob/{}/{}/{}/{}",
            self.github_user,
            self.github_repo,
            self.github_branch,
            self.notebooks_path,
            category,
            filename
        )
    }

    /// Get the local path where notebooks should be written
    pub fn get_local_path(&self, base_dir: &Path, category: &str, filename: &str) -> std::path::PathBuf {
        base_dir
            .join(&self.notebooks_path)
            .join(category)
            .join(filename)
    }
}

impl Default for ColabConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

/// Entry for a single notebook in the manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookEntry {
    /// Notebook filename
    pub filename: String,
    /// Topic from schema (e.g., "calculus.derivative")
    pub topic: String,
    /// Human-readable title
    pub title: String,
    /// Category/folder name
    pub category: String,
    /// Direct Colab URL (opens in Colab)
    pub colab_url: String,
    /// GitHub URL (view source)
    pub github_url: String,
}

/// Manifest tracking all generated Colab notebooks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColabManifest {
    /// Configuration used to generate URLs
    pub config: ColabConfig,
    /// Total number of notebooks
    pub total_notebooks: usize,
    /// Notebooks organized by category
    pub categories: HashMap<String, Vec<NotebookEntry>>,
}

impl ColabManifest {
    /// Create a new manifest with the given config
    pub fn new(config: ColabConfig) -> Self {
        Self {
            config,
            total_notebooks: 0,
            categories: HashMap::new(),
        }
    }

    /// Add a notebook entry to the manifest
    pub fn add(&mut self, topic: &str, title: &str, filename: &str) {
        // Extract category from topic (first part before '.')
        let category = topic
            .split('.')
            .next()
            .unwrap_or("misc")
            .to_string();

        let entry = NotebookEntry {
            filename: filename.to_string(),
            topic: topic.to_string(),
            title: title.to_string(),
            category: category.clone(),
            colab_url: self.config.get_colab_url(&category, filename),
            github_url: self.config.get_github_url(&category, filename),
        };

        self.categories
            .entry(category)
            .or_default()
            .push(entry);
        self.total_notebooks += 1;
    }

    /// Get all categories sorted alphabetically
    pub fn sorted_categories(&self) -> Vec<&String> {
        let mut cats: Vec<_> = self.categories.keys().collect();
        cats.sort();
        cats
    }

    /// Save manifest to a JSON file
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)
    }

    /// Load manifest from a JSON file
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Generate a README.md for the notebooks directory
    pub fn generate_readme(&self) -> String {
        let mut readme = String::new();

        readme.push_str("# MathHook Colab Notebooks\n\n");
        readme.push_str("Interactive Google Colab notebooks for learning MathHook symbolic mathematics.\n\n");
        readme.push_str("Click any **Open in Colab** badge to launch the notebook instantly - no setup required!\n\n");
        readme.push_str("## Categories\n\n");

        for category in self.sorted_categories() {
            let cap_category = capitalize(category);
            let notebooks = &self.categories[category];

            readme.push_str(&format!("### {}\n\n", cap_category));
            readme.push_str("| Topic | Open in Colab |\n");
            readme.push_str("|-------|---------------|\n");

            for nb in notebooks {
                readme.push_str(&format!(
                    "| {} | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)]({}) |\n",
                    nb.title,
                    nb.colab_url
                ));
            }
            readme.push('\n');
        }

        readme.push_str("---\n\n");
        readme.push_str("*Auto-generated from MathHook Knowledge Base schemas*\n");

        readme
    }

    /// Generate category README files
    pub fn generate_category_readme(&self, category: &str) -> Option<String> {
        let notebooks = self.categories.get(category)?;
        let cap_category = capitalize(category);

        let mut readme = String::new();

        readme.push_str(&format!("# {} Notebooks\n\n", cap_category));
        readme.push_str(&format!("MathHook {} tutorials for Google Colab.\n\n", cap_category));
        readme.push_str("## Notebooks\n\n");
        readme.push_str("| Topic | Description | Open in Colab |\n");
        readme.push_str("|-------|-------------|---------------|\n");

        for nb in notebooks {
            readme.push_str(&format!(
                "| {} | {} | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)]({}) |\n",
                nb.title,
                nb.topic,
                nb.colab_url
            ));
        }

        Some(readme)
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colab_url_generation() {
        let config = ColabConfig::new("AhmedMashour", "mathhook", "main", "colab-notebooks");

        let url = config.get_colab_url("calculus", "derivative.colab.ipynb");

        assert_eq!(
            url,
            "https://colab.research.google.com/github/AhmedMashour/mathhook/blob/main/colab-notebooks/calculus/derivative.colab.ipynb"
        );
    }

    #[test]
    fn test_github_url_generation() {
        let config = ColabConfig::new("AhmedMashour", "mathhook", "main", "colab-notebooks");

        let url = config.get_github_url("algebra", "simplify.colab.ipynb");

        assert_eq!(
            url,
            "https://github.com/AhmedMashour/mathhook/blob/main/colab-notebooks/algebra/simplify.colab.ipynb"
        );
    }

    #[test]
    fn test_manifest_add() {
        let config = ColabConfig::default();
        let mut manifest = ColabManifest::new(config);

        manifest.add("calculus.derivative", "Symbolic Differentiation", "calculus-derivative.colab.ipynb");
        manifest.add("calculus.integral", "Symbolic Integration", "calculus-integral.colab.ipynb");
        manifest.add("algebra.simplify", "Expression Simplification", "algebra-simplify.colab.ipynb");

        assert_eq!(manifest.total_notebooks, 3);
        assert_eq!(manifest.categories.len(), 2);
        assert_eq!(manifest.categories["calculus"].len(), 2);
        assert_eq!(manifest.categories["algebra"].len(), 1);
    }

    #[test]
    fn test_readme_generation() {
        let config = ColabConfig::new("test", "repo", "main", "notebooks");
        let mut manifest = ColabManifest::new(config);

        manifest.add("calculus.derivative", "Derivatives", "calc-deriv.ipynb");

        let readme = manifest.generate_readme();

        assert!(readme.contains("# MathHook Colab Notebooks"));
        assert!(readme.contains("### Calculus"));
        assert!(readme.contains("colab-badge.svg"));
    }
}
