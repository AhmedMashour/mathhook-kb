/// MathHook Knowledge Base - Core library
///
/// This crate provides the foundational schema types and validation logic
/// for the MathHook Knowledge Base engine.
pub mod schema;
pub mod parser;
pub mod generator;
pub mod error;

pub use schema::Schema;
pub use error::{CrossLanguageInconsistencyError, KbError, Result};
