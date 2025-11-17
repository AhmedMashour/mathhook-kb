/// Chunking strategies and metadata for LLM-RAG systems
///
/// This module provides different strategies for splitting documentation into
/// chunks optimized for vector embedding and retrieval.
use serde::{Deserialize, Serialize};

/// Strategy for chunking documentation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ChunkStrategy {
    /// Chunk by individual examples (one example per chunk)
    #[default]
    ByExample,

    /// Chunk by sections (one section per chunk)
    BySection,

    /// Fixed-size chunking with overlap
    FixedSize { size: usize, overlap: usize },
}

/// Metadata for a documentation chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    /// Unique identifier for this chunk
    pub id: String,

    /// Topic this chunk belongs to
    pub topic: String,

    /// Section or example title
    pub title: String,

    /// Keywords for retrieval
    pub keywords: Vec<String>,

    /// Programming languages featured in this chunk
    pub languages: Vec<String>,

    /// Chunk index in the document
    pub chunk_index: usize,

    /// Total chunks in the document
    pub total_chunks: usize,

    /// Embedding priority (high/medium/low)
    pub priority: String,
}

impl ChunkMetadata {
    /// Create metadata for a chunk
    pub fn new(topic: String, title: String, chunk_index: usize, total_chunks: usize) -> Self {
        let id = format!("{}::{}", topic.replace('.', "_"), chunk_index);

        Self {
            id,
            topic,
            title,
            keywords: Vec::new(),
            languages: vec![
                "rust".to_string(),
                "python".to_string(),
                "javascript".to_string(),
            ],
            chunk_index,
            total_chunks,
            priority: "medium".to_string(),
        }
    }

    /// Add keywords to this chunk
    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }

    /// Set priority for this chunk
    pub fn with_priority(mut self, priority: String) -> Self {
        self.priority = priority;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_strategy_default() {
        assert_eq!(ChunkStrategy::default(), ChunkStrategy::ByExample);
    }

    #[test]
    fn test_chunk_metadata_creation() {
        let metadata = ChunkMetadata::new(
            "calculus.derivative".to_string(),
            "Power Rule".to_string(),
            0,
            5,
        );

        assert_eq!(metadata.topic, "calculus.derivative");
        assert_eq!(metadata.title, "Power Rule");
        assert_eq!(metadata.chunk_index, 0);
        assert_eq!(metadata.total_chunks, 5);
        assert_eq!(metadata.id, "calculus_derivative::0");
    }

    #[test]
    fn test_chunk_metadata_with_keywords() {
        let metadata = ChunkMetadata::new(
            "calculus.derivative".to_string(),
            "Power Rule".to_string(),
            0,
            5,
        )
        .with_keywords(vec!["power".to_string(), "derivative".to_string()]);

        assert_eq!(metadata.keywords.len(), 2);
        assert!(metadata.keywords.contains(&"power".to_string()));
    }

    #[test]
    fn test_chunk_metadata_with_priority() {
        let metadata = ChunkMetadata::new(
            "calculus.derivative".to_string(),
            "Power Rule".to_string(),
            0,
            5,
        )
        .with_priority("high".to_string());

        assert_eq!(metadata.priority, "high");
    }
}
