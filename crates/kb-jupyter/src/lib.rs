/// Jupyter notebook generator for MathHook KB
///
/// This module generates interactive Jupyter notebooks from KB schemas, with special
/// support for article-style content featuring tutorial-focused narratives.
mod generator;
mod notebook;
mod templates;

pub use generator::JupyterGenerator;
pub use notebook::{Cell, CellType, Notebook};
