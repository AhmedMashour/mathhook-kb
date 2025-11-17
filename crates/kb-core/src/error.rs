use thiserror::Error;

/// Custom result type for KB operations
pub type Result<T> = std::result::Result<T, KbError>;

/// Errors that can occur in the Knowledge Base engine
#[derive(Error, Debug)]
pub enum KbError {
    #[error("Schema validation failed in '{file}':\n  Line {line}: {message}\n\n  Suggestion: {suggestion}")]
    ValidationError {
        file: String,
        line: usize,
        message: String,
        suggestion: String,
    },

    #[error("Missing required field '{field}' in schema '{file}' at line {line}\n\n  Suggestion: {suggestion}")]
    MissingField {
        field: String,
        file: String,
        line: usize,
        suggestion: String,
    },

    #[error("Invalid code reference '{reference}' in '{file}' at line {line}\n\n  Function not found in mathhook codebase.\n  Did you mean '{suggestion}'?")]
    InvalidCodeRef {
        reference: String,
        file: String,
        line: usize,
        suggestion: String,
    },

    #[error("Syntax error in {language} code example in '{file}' at line {line}:\n  {error}\n\n  Code:\n{code}")]
    CodeSyntaxError {
        language: String,
        file: String,
        line: usize,
        error: String,
        code: String,
    },

    #[error("Cross-language consistency error in '{file}' example '{example}':\n  {message}\n\n  Rust output: {rust_output}\n  Python output: {python_output}\n  JavaScript output: {js_output}")]
    CrossLanguageInconsistency {
        file: String,
        example: String,
        message: String,
        rust_output: String,
        python_output: String,
        js_output: String,
    },

    #[error("Generated {format} output is invalid:\n  {error}\n\n  Output file: {file}")]
    InvalidOutput {
        format: String,
        file: String,
        error: String,
    },

    #[error("Output validation failed for {generator} generator:\n  {message}")]
    OutputValidationError {
        generator: String,
        message: String,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Template error: {0}")]
    Template(String),

    #[error("Tera template error: {0}")]
    Tera(#[from] tera::Error),

    #[error("{0}")]
    Other(String),
}

impl KbError {
    /// Create a validation error with context
    pub fn validation(file: impl Into<String>, line: usize, message: impl Into<String>, suggestion: impl Into<String>) -> Self {
        Self::ValidationError {
            file: file.into(),
            line,
            message: message.into(),
            suggestion: suggestion.into(),
        }
    }

    /// Create a missing field error
    pub fn missing_field(field: impl Into<String>, file: impl Into<String>, line: usize, suggestion: impl Into<String>) -> Self {
        Self::MissingField {
            field: field.into(),
            file: file.into(),
            line,
            suggestion: suggestion.into(),
        }
    }

    /// Create an invalid code reference error
    pub fn invalid_code_ref(reference: impl Into<String>, file: impl Into<String>, line: usize, suggestion: impl Into<String>) -> Self {
        Self::InvalidCodeRef {
            reference: reference.into(),
            file: file.into(),
            line,
            suggestion: suggestion.into(),
        }
    }
}
