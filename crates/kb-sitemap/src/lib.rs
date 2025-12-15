//! # MathHook KB Sitemap Generator
//!
//! Generates XML sitemaps from KB schemas and submits them to search engines.
//!
//! ## Features
//!
//! - Generate sitemap.xml from schema files
//! - Support for sitemap index (for large sites)
//! - Automatic submission to Google and Bing via ping API
//! - SEO metadata integration (priority, change_frequency)
//!
//! ## Example
//!
//! ```rust,ignore
//! use kb_sitemap::{SitemapGenerator, SitemapConfig, SearchEnginePinger};
//! use kb_core::Schema;
//!
//! let schemas = Schema::load_from_directory(Path::new("schemas/"))?;
//! let config = SitemapConfig::new("https://mathhook.org");
//! let generator = SitemapGenerator::new(config);
//!
//! // Generate sitemap
//! let sitemap = generator.generate_from_schemas(&schemas)?;
//! std::fs::write("sitemap.xml", sitemap)?;
//!
//! // Submit to search engines
//! let pinger = SearchEnginePinger::new();
//! pinger.ping_all("https://mathhook.org/sitemap.xml")?;
//! ```

mod generator;
mod pinger;

pub use generator::{SitemapConfig, SitemapEntry, SitemapGenerator, SitemapIndex};
pub use pinger::{
    generate_robots_txt_snippet, verify_robots_txt, BingQuota, BingSubmitter, IndexNowSubmitter,
    PingResult, PingerConfig, SearchEngine, SearchEnginePinger,
};

use thiserror::Error;

/// Sitemap generation and submission errors
#[derive(Error, Debug)]
pub enum SitemapError {
    #[error("Failed to generate sitemap: {0}")]
    GenerationError(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("HTTP request failed: {0}")]
    HttpError(String),

    #[error("Schema error: {0}")]
    SchemaError(#[from] kb_core::KbError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SitemapError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sitemap_config_creation() {
        let config = SitemapConfig::new("https://mathhook.org");
        assert_eq!(config.base_url, "https://mathhook.org");
    }
}
