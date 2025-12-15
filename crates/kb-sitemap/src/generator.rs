//! Sitemap XML generation from KB schemas
//!
//! Generates standard XML sitemaps following the sitemap.org protocol.

use crate::Result;
use chrono::{DateTime, Utc};
use kb_core::Schema;
use std::path::Path;

/// Configuration for sitemap generation
#[derive(Debug, Clone)]
pub struct SitemapConfig {
    /// Base URL for the site (e.g., "https://mathhook.org")
    pub base_url: String,

    /// URL path prefix for docs (e.g., "/docs")
    pub docs_prefix: String,

    /// Default priority for pages without SEO metadata (0.0 to 1.0)
    pub default_priority: f32,

    /// Default change frequency
    pub default_change_freq: String,

    /// Maximum URLs per sitemap file (50,000 is the limit)
    pub max_urls_per_sitemap: usize,

    /// Include static pages (homepage, docs index)
    pub include_static_pages: bool,
}

impl SitemapConfig {
    /// Create a new config with the given base URL
    pub fn new(base_url: impl Into<String>) -> Self {
        let mut url = base_url.into();
        // Remove trailing slash
        if url.ends_with('/') {
            url.pop();
        }
        Self {
            base_url: url,
            docs_prefix: "/docs".to_string(),
            default_priority: 0.5,
            default_change_freq: "monthly".to_string(),
            max_urls_per_sitemap: 50_000,
            include_static_pages: true,
        }
    }

    /// Set the docs URL prefix
    pub fn with_docs_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.docs_prefix = prefix.into();
        self
    }

    /// Set the default priority
    pub fn with_default_priority(mut self, priority: f32) -> Self {
        self.default_priority = priority.clamp(0.0, 1.0);
        self
    }

    /// Set the default change frequency
    pub fn with_default_change_freq(mut self, freq: impl Into<String>) -> Self {
        self.default_change_freq = freq.into();
        self
    }

    /// Disable static pages
    pub fn without_static_pages(mut self) -> Self {
        self.include_static_pages = false;
        self
    }
}

/// A single sitemap entry
#[derive(Debug, Clone)]
pub struct SitemapEntry {
    /// Full URL of the page
    pub loc: String,

    /// Last modification timestamp
    pub lastmod: Option<DateTime<Utc>>,

    /// Change frequency (always, hourly, daily, weekly, monthly, yearly, never)
    pub changefreq: Option<String>,

    /// Priority (0.0 to 1.0)
    pub priority: Option<f32>,
}

impl SitemapEntry {
    /// Create a new sitemap entry
    pub fn new(loc: impl Into<String>) -> Self {
        Self {
            loc: loc.into(),
            lastmod: None,
            changefreq: None,
            priority: None,
        }
    }

    /// Set the last modification time
    pub fn with_lastmod(mut self, lastmod: DateTime<Utc>) -> Self {
        self.lastmod = Some(lastmod);
        self
    }

    /// Set the change frequency
    pub fn with_changefreq(mut self, freq: impl Into<String>) -> Self {
        self.changefreq = Some(freq.into());
        self
    }

    /// Set the priority
    pub fn with_priority(mut self, priority: f32) -> Self {
        self.priority = Some(priority.clamp(0.0, 1.0));
        self
    }

    /// Convert to XML element
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("  <url>\n");
        xml.push_str(&format!("    <loc>{}</loc>\n", escape_xml(&self.loc)));

        if let Some(lastmod) = &self.lastmod {
            xml.push_str(&format!(
                "    <lastmod>{}</lastmod>\n",
                lastmod.format("%Y-%m-%d")
            ));
        }

        if let Some(changefreq) = &self.changefreq {
            xml.push_str(&format!("    <changefreq>{}</changefreq>\n", changefreq));
        }

        if let Some(priority) = &self.priority {
            xml.push_str(&format!("    <priority>{:.1}</priority>\n", priority));
        }

        xml.push_str("  </url>\n");
        xml
    }
}

/// Sitemap index for large sites (multiple sitemap files)
#[derive(Debug, Clone)]
pub struct SitemapIndex {
    /// Base URL for the site
    pub base_url: String,

    /// List of sitemap URLs
    pub sitemaps: Vec<SitemapIndexEntry>,
}

/// Entry in a sitemap index
#[derive(Debug, Clone)]
pub struct SitemapIndexEntry {
    /// URL of the sitemap file
    pub loc: String,

    /// Last modification timestamp
    pub lastmod: Option<DateTime<Utc>>,
}

impl SitemapIndex {
    /// Create a new sitemap index
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            sitemaps: Vec::new(),
        }
    }

    /// Add a sitemap to the index
    pub fn add_sitemap(&mut self, filename: impl Into<String>, lastmod: Option<DateTime<Utc>>) {
        let filename = filename.into();
        let loc = if filename.starts_with("http") {
            filename
        } else {
            format!("{}/{}", self.base_url.trim_end_matches('/'), filename)
        };

        self.sitemaps.push(SitemapIndexEntry { loc, lastmod });
    }

    /// Generate the sitemap index XML
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml.push('\n');
        xml.push_str(r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
        xml.push('\n');

        for sitemap in &self.sitemaps {
            xml.push_str("  <sitemap>\n");
            xml.push_str(&format!("    <loc>{}</loc>\n", escape_xml(&sitemap.loc)));
            if let Some(lastmod) = &sitemap.lastmod {
                xml.push_str(&format!(
                    "    <lastmod>{}</lastmod>\n",
                    lastmod.format("%Y-%m-%d")
                ));
            }
            xml.push_str("  </sitemap>\n");
        }

        xml.push_str("</sitemapindex>\n");
        xml
    }
}

/// Sitemap generator
pub struct SitemapGenerator {
    config: SitemapConfig,
}

impl SitemapGenerator {
    /// Create a new sitemap generator
    pub fn new(config: SitemapConfig) -> Self {
        Self { config }
    }

    /// Generate sitemap XML from a list of schemas
    pub fn generate_from_schemas(&self, schemas: &[Schema]) -> Result<String> {
        let entries = self.schemas_to_entries(schemas);
        Ok(self.entries_to_xml(&entries))
    }

    /// Generate sitemap from a directory of schemas
    pub fn generate_from_directory(&self, dir: &Path) -> Result<String> {
        let schemas = Schema::load_from_directory(dir)?;
        self.generate_from_schemas(&schemas)
    }

    /// Convert schemas to sitemap entries
    pub fn schemas_to_entries(&self, schemas: &[Schema]) -> Vec<SitemapEntry> {
        let mut entries = Vec::new();

        // Add static pages if configured
        if self.config.include_static_pages {
            // Homepage
            entries.push(
                SitemapEntry::new(&self.config.base_url)
                    .with_priority(1.0)
                    .with_changefreq("weekly"),
            );

            // Docs index
            entries.push(
                SitemapEntry::new(format!(
                    "{}{}",
                    self.config.base_url, self.config.docs_prefix
                ))
                .with_priority(0.9)
                .with_changefreq("weekly"),
            );
        }

        // Add schema-based entries
        for schema in schemas {
            let entry = self.schema_to_entry(schema);
            entries.push(entry);
        }

        entries
    }

    /// Convert a single schema to a sitemap entry
    fn schema_to_entry(&self, schema: &Schema) -> SitemapEntry {
        // Build URL from topic (calculus.derivative -> /docs/calculus-derivative)
        let topic_slug = schema.topic.replace('.', "-");
        let url = format!(
            "{}{}/{}",
            self.config.base_url, self.config.docs_prefix, topic_slug
        );

        let mut entry = SitemapEntry::new(url);

        // Use SEO metadata if available
        if let Some(seo) = &schema.seo {
            entry.priority = Some(seo.priority);
            entry.changefreq = Some(seo.change_frequency.clone());
        } else {
            entry.priority = Some(self.config.default_priority);
            entry.changefreq = Some(self.config.default_change_freq.clone());
        }

        // Use metadata last_updated if available
        if let Some(metadata) = &schema.metadata {
            if let Some(last_updated) = &metadata.last_updated {
                // Try to parse as ISO date
                if let Ok(date) = chrono::NaiveDate::parse_from_str(last_updated, "%Y-%m-%d") {
                    let datetime = date
                        .and_hms_opt(0, 0, 0)
                        .and_then(|dt| dt.and_local_timezone(Utc).single());
                    entry.lastmod = datetime;
                }
            }
        }

        entry
    }

    /// Convert entries to XML
    fn entries_to_xml(&self, entries: &[SitemapEntry]) -> String {
        let mut xml = String::new();
        xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml.push('\n');
        xml.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
        xml.push('\n');

        for entry in entries {
            xml.push_str(&entry.to_xml());
        }

        xml.push_str("</urlset>\n");
        xml
    }

    /// Generate multiple sitemaps if needed (for large sites)
    /// Returns (sitemap_files, sitemap_index)
    pub fn generate_split_sitemaps(
        &self,
        schemas: &[Schema],
    ) -> Result<(Vec<(String, String)>, Option<String>)> {
        let entries = self.schemas_to_entries(schemas);

        if entries.len() <= self.config.max_urls_per_sitemap {
            // Single sitemap is enough
            return Ok((vec![("sitemap.xml".to_string(), self.entries_to_xml(&entries))], None));
        }

        // Split into multiple sitemaps
        let mut sitemaps = Vec::new();
        let mut sitemap_index = SitemapIndex::new(&self.config.base_url);
        let now = Utc::now();

        for (i, chunk) in entries.chunks(self.config.max_urls_per_sitemap).enumerate() {
            let filename = format!("sitemap-{}.xml", i + 1);
            let xml = self.entries_to_xml(chunk);
            sitemaps.push((filename.clone(), xml));
            sitemap_index.add_sitemap(filename, Some(now));
        }

        Ok((sitemaps, Some(sitemap_index.to_xml())))
    }

    /// Write sitemap(s) to the output directory
    pub fn write_to_directory(&self, schemas: &[Schema], output_dir: &Path) -> Result<Vec<String>> {
        std::fs::create_dir_all(output_dir)?;

        let (sitemaps, index) = self.generate_split_sitemaps(schemas)?;
        let mut written_files = Vec::new();

        for (filename, content) in sitemaps {
            let path = output_dir.join(&filename);
            std::fs::write(&path, content)?;
            written_files.push(filename);
        }

        if let Some(index_xml) = index {
            let index_path = output_dir.join("sitemap-index.xml");
            std::fs::write(&index_path, index_xml)?;
            written_files.push("sitemap-index.xml".to_string());
        }

        Ok(written_files)
    }
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sitemap_entry_to_xml() {
        let entry = SitemapEntry::new("https://example.com/page")
            .with_priority(0.8)
            .with_changefreq("weekly");

        let xml = entry.to_xml();
        assert!(xml.contains("<loc>https://example.com/page</loc>"));
        assert!(xml.contains("<priority>0.8</priority>"));
        assert!(xml.contains("<changefreq>weekly</changefreq>"));
    }

    #[test]
    fn test_sitemap_config_builder() {
        let config = SitemapConfig::new("https://mathhook.org/")
            .with_docs_prefix("/documentation")
            .with_default_priority(0.7);

        assert_eq!(config.base_url, "https://mathhook.org");
        assert_eq!(config.docs_prefix, "/documentation");
        assert_eq!(config.default_priority, 0.7);
    }

    #[test]
    fn test_xml_escaping() {
        let escaped = escape_xml("<test & \"quoted\" 'single'>");
        assert_eq!(escaped, "&lt;test &amp; &quot;quoted&quot; &apos;single&apos;&gt;");
    }

    #[test]
    fn test_sitemap_index_to_xml() {
        let mut index = SitemapIndex::new("https://example.com");
        index.add_sitemap("sitemap-1.xml", None);
        index.add_sitemap("sitemap-2.xml", None);

        let xml = index.to_xml();
        assert!(xml.contains("<sitemapindex"));
        assert!(xml.contains("https://example.com/sitemap-1.xml"));
        assert!(xml.contains("https://example.com/sitemap-2.xml"));
    }
}
