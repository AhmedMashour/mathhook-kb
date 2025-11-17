/// Jupyter notebook data structures
///
/// Represents the internal structure of Jupyter notebooks (.ipynb format).
/// Reference: https://nbformat.readthedocs.io/en/latest/format_description.html
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// A Jupyter notebook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notebook {
    /// Notebook format version (e.g., "4.5")
    pub nbformat: u8,
    pub nbformat_minor: u8,

    /// Notebook metadata
    pub metadata: NotebookMetadata,

    /// List of cells
    pub cells: Vec<Cell>,
}

impl Notebook {
    /// Create a new empty notebook with default settings
    pub fn new() -> Self {
        Self {
            nbformat: 4,
            nbformat_minor: 5,
            metadata: NotebookMetadata::default(),
            cells: Vec::new(),
        }
    }

    /// Add a cell to the notebook
    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    /// Add a markdown cell with the given content
    pub fn add_markdown(&mut self, content: String) {
        self.add_cell(Cell::markdown(content));
    }

    /// Add a code cell with the given Python code
    pub fn add_code(&mut self, code: String) {
        self.add_cell(Cell::code(code));
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
}

/// Notebook-level metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotebookMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernelspec: Option<KernelSpec>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_info: Option<LanguageInfo>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Kernel specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelSpec {
    pub display_name: String,
    pub language: String,
    pub name: String,
}

impl Default for KernelSpec {
    fn default() -> Self {
        Self {
            display_name: "Python 3".to_string(),
            language: "python".to_string(),
            name: "python3".to_string(),
        }
    }
}

/// Language information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageInfo {
    pub name: String,
    pub version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Default for LanguageInfo {
    fn default() -> Self {
        Self {
            name: "python".to_string(),
            version: "3.10.0".to_string(),
            mimetype: Some("text/x-python".to_string()),
            file_extension: Some(".py".to_string()),
            extra: HashMap::new(),
        }
    }
}

/// A notebook cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub cell_type: CellType,
    pub metadata: CellMetadata,
    pub source: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Value>>,

    // execution_count should always be serialized for code cells (as null if not executed)
    pub execution_count: Option<Value>,
}

impl Cell {
    /// Create a markdown cell
    pub fn markdown(content: String) -> Self {
        Self {
            cell_type: CellType::Markdown,
            metadata: CellMetadata::default(),
            source: content.lines().map(|s| s.to_string() + "\n").collect(),
            outputs: None,
            execution_count: None,
        }
    }

    /// Create a code cell
    pub fn code(code: String) -> Self {
        Self {
            cell_type: CellType::Code,
            metadata: CellMetadata::default(),
            source: code.lines().map(|s| s.to_string() + "\n").collect(),
            outputs: Some(Vec::new()),
            execution_count: None,
        }
    }

    /// Create a raw cell
    pub fn raw(content: String) -> Self {
        Self {
            cell_type: CellType::Raw,
            metadata: CellMetadata::default(),
            source: content.lines().map(|s| s.to_string() + "\n").collect(),
            outputs: None,
            execution_count: None,
        }
    }
}

/// Cell type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CellType {
    Code,
    Markdown,
    Raw,
}

/// Cell-level metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CellMetadata {
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_notebook() {
        let notebook = Notebook::new();
        assert_eq!(notebook.nbformat, 4);
        assert_eq!(notebook.nbformat_minor, 5);
        assert_eq!(notebook.cells.len(), 0);
    }

    #[test]
    fn test_add_markdown_cell() {
        let mut notebook = Notebook::new();
        notebook.add_markdown("# Hello World".to_string());
        assert_eq!(notebook.cells.len(), 1);
        assert_eq!(notebook.cells[0].cell_type, CellType::Markdown);
    }

    #[test]
    fn test_add_code_cell() {
        let mut notebook = Notebook::new();
        notebook.add_code("print('Hello')".to_string());
        assert_eq!(notebook.cells.len(), 1);
        assert_eq!(notebook.cells[0].cell_type, CellType::Code);
    }

    #[test]
    fn test_notebook_to_json() {
        let mut notebook = Notebook::new();
        notebook.add_markdown("# Test".to_string());
        let json = notebook.to_json().expect("Failed to serialize");
        assert!(json.contains("nbformat"));
        assert!(json.contains("cells"));
    }

    #[test]
    fn test_cell_source_lines() {
        let cell = Cell::markdown("Line 1\nLine 2\nLine 3".to_string());
        assert_eq!(cell.source.len(), 3);
        assert_eq!(cell.source[0], "Line 1\n");
        assert_eq!(cell.source[1], "Line 2\n");
        assert_eq!(cell.source[2], "Line 3\n");
    }
}
