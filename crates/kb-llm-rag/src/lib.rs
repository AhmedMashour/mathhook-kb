mod chunking;
/// LLM-RAG optimized markdown generator for MathHook KB
///
/// This module generates markdown documents optimized for LLM retrieval-augmented
/// generation (RAG) systems. The output is structured for efficient chunking,
/// embedding, and retrieval.
mod generator;

pub use chunking::{ChunkMetadata, ChunkStrategy};
pub use generator::LlmRagGenerator;
