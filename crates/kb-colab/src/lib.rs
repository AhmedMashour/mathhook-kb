//! Google Colab notebook generator with GitHub integration
//!
//! Generates Colab-optimized notebooks that are hosted on GitHub.
//! Colab natively supports opening notebooks from GitHub URLs.
//!
//! # Usage
//!
//! ```rust,ignore
//! use kb_colab::{ColabGenerator, ColabConfig, ColabManifest};
//! use kb_core::generator::OutputGenerator;
//!
//! // Generate notebook
//! let generator = ColabGenerator::new()?;
//! let notebook = generator.generate(&schema)?;
//!
//! // Generate Colab URL for GitHub-hosted notebook
//! let config = ColabConfig::default();
//! let url = config.get_colab_url("calculus", "derivative.colab.ipynb");
//! // -> https://colab.research.google.com/github/AhmedMashour/mathhook/blob/main/colab-notebooks/calculus/derivative.colab.ipynb
//! ```

pub mod generator;
mod manifest;

pub use generator::ColabGenerator;
pub use manifest::{ColabConfig, ColabManifest, NotebookEntry};
