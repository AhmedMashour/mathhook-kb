//! Search engine sitemap submission
//!
//! Modern approaches to notify search engines about sitemaps:
//!
//! ## Recommended Methods (2024+)
//!
//! 1. **robots.txt** - Add `Sitemap: URL` directive (best, always works)
//! 2. **IndexNow API** - Instant indexing for Bing, Yandex, Seznam
//! 3. **Google Search Console** - Manual submission via web UI or API
//!
//! ## Legacy Methods (Deprecated)
//!
//! - Google ping API was deprecated in June 2023
//! - Bing ping API was deprecated in late 2023
//!
//! ## Usage
//!
//! ```rust,ignore
//! use kb_sitemap::{SearchEnginePinger, IndexNowSubmitter};
//!
//! // Check if robots.txt is configured correctly
//! let pinger = SearchEnginePinger::new();
//! pinger.verify_robots_txt("https://mathhook.org")?;
//!
//! // Use IndexNow for instant indexing (Bing, Yandex)
//! let submitter = IndexNowSubmitter::new("your-api-key");
//! submitter.submit_url("https://mathhook.org/docs/derivative")?;
//! ```

use crate::{Result, SitemapError};

/// Search engines that support sitemap ping
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchEngine {
    /// Google Search
    Google,
    /// Bing (also powers Yahoo, DuckDuckGo)
    Bing,
    /// IndexNow protocol (Bing, Yandex, Seznam)
    IndexNow,
}

impl SearchEngine {
    /// Get the ping URL for this search engine
    pub fn ping_url(&self, sitemap_url: &str) -> String {
        let encoded = urlencoding::encode(sitemap_url);
        match self {
            SearchEngine::Google => {
                format!("https://www.google.com/ping?sitemap={}", encoded)
            }
            SearchEngine::Bing => {
                format!("https://www.bing.com/ping?sitemap={}", encoded)
            }
            SearchEngine::IndexNow => {
                // IndexNow requires an API key, we'll use Bing's endpoint
                format!("https://www.bing.com/indexnow?url={}", encoded)
            }
        }
    }

    /// Human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            SearchEngine::Google => "Google",
            SearchEngine::Bing => "Bing",
            SearchEngine::IndexNow => "IndexNow",
        }
    }
}

/// Result of a ping operation
#[derive(Debug, Clone)]
pub struct PingResult {
    /// Search engine that was pinged
    pub engine: SearchEngine,

    /// HTTP status code
    pub status_code: u16,

    /// Whether the ping was successful (2xx status)
    pub success: bool,

    /// Response message or error
    pub message: String,
}

impl PingResult {
    fn success(engine: SearchEngine, status_code: u16, message: impl Into<String>) -> Self {
        Self {
            engine,
            status_code,
            success: true,
            message: message.into(),
        }
    }

    fn failure(engine: SearchEngine, status_code: u16, message: impl Into<String>) -> Self {
        Self {
            engine,
            status_code,
            success: false,
            message: message.into(),
        }
    }
}

/// Configuration for the pinger
#[derive(Debug, Clone)]
pub struct PingerConfig {
    /// Timeout for HTTP requests in seconds
    pub timeout_secs: u64,

    /// User agent string
    pub user_agent: String,

    /// Which engines to ping
    pub engines: Vec<SearchEngine>,

    /// IndexNow API key (optional)
    pub indexnow_key: Option<String>,

    /// Verbose output
    pub verbose: bool,
}

impl Default for PingerConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            user_agent: format!(
                "MathHook-KB-Sitemap/{} (https://mathhook.org)",
                env!("CARGO_PKG_VERSION")
            ),
            engines: vec![SearchEngine::Google, SearchEngine::Bing],
            indexnow_key: None,
            verbose: false,
        }
    }
}

impl PingerConfig {
    /// Create a new config with custom settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set timeout
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Enable verbose output
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// Set engines to ping
    pub fn with_engines(mut self, engines: Vec<SearchEngine>) -> Self {
        self.engines = engines;
        self
    }

    /// Add IndexNow API key
    pub fn with_indexnow_key(mut self, key: impl Into<String>) -> Self {
        self.indexnow_key = Some(key.into());
        if !self.engines.contains(&SearchEngine::IndexNow) {
            self.engines.push(SearchEngine::IndexNow);
        }
        self
    }
}

/// Pings search engines with sitemap URLs
pub struct SearchEnginePinger {
    config: PingerConfig,
}

impl SearchEnginePinger {
    /// Create a new pinger with default configuration
    pub fn new() -> Self {
        Self {
            config: PingerConfig::default(),
        }
    }

    /// Create a pinger with custom configuration
    pub fn with_config(config: PingerConfig) -> Self {
        Self { config }
    }

    /// Ping a single search engine with the sitemap URL
    pub fn ping(&self, engine: SearchEngine, sitemap_url: &str) -> PingResult {
        let ping_url = engine.ping_url(sitemap_url);

        if self.config.verbose {
            eprintln!("Pinging {}: {}", engine.name(), ping_url);
        }

        let agent = ureq::AgentBuilder::new()
            .timeout(std::time::Duration::from_secs(self.config.timeout_secs))
            .user_agent(&self.config.user_agent)
            .build();

        match agent.get(&ping_url).call() {
            Ok(response) => {
                let status = response.status();
                if status >= 200 && status < 300 {
                    PingResult::success(engine, status, format!("Successfully pinged {}", engine.name()))
                } else {
                    PingResult::failure(
                        engine,
                        status,
                        format!("Unexpected status {} from {}", status, engine.name()),
                    )
                }
            }
            Err(ureq::Error::Status(code, response)) => {
                let body = response.into_string().unwrap_or_default();
                PingResult::failure(
                    engine,
                    code,
                    format!("HTTP {} from {}: {}", code, engine.name(), body),
                )
            }
            Err(ureq::Error::Transport(transport)) => PingResult::failure(
                engine,
                0,
                format!("Transport error for {}: {}", engine.name(), transport),
            ),
        }
    }

    /// Ping all configured search engines
    pub fn ping_all(&self, sitemap_url: &str) -> Vec<PingResult> {
        self.config
            .engines
            .iter()
            .map(|engine| self.ping(*engine, sitemap_url))
            .collect()
    }

    /// Ping all engines and return only if at least one succeeded
    pub fn ping_all_require_one(&self, sitemap_url: &str) -> Result<Vec<PingResult>> {
        let results = self.ping_all(sitemap_url);

        if results.iter().any(|r| r.success) {
            Ok(results)
        } else {
            Err(SitemapError::HttpError(
                "All search engine pings failed".to_string(),
            ))
        }
    }

    /// Submit multiple sitemap URLs (e.g., for sitemap index)
    pub fn ping_multiple(&self, sitemap_urls: &[String]) -> Vec<(String, Vec<PingResult>)> {
        sitemap_urls
            .iter()
            .map(|url| (url.clone(), self.ping_all(url)))
            .collect()
    }
}

impl Default for SearchEnginePinger {
    fn default() -> Self {
        Self::new()
    }
}

/// Submit a sitemap and print results
pub fn submit_sitemap_with_output(sitemap_url: &str, verbose: bool) -> Result<()> {
    let config = PingerConfig::default();
    let config = if verbose {
        config.verbose()
    } else {
        config
    };

    let pinger = SearchEnginePinger::with_config(config);
    let results = pinger.ping_all(sitemap_url);

    let mut any_success = false;
    for result in &results {
        if result.success {
            println!("  {} {}: {}", "✓", result.engine.name(), result.message);
            any_success = true;
        } else {
            println!("  {} {}: {}", "✗", result.engine.name(), result.message);
        }
    }

    if any_success {
        Ok(())
    } else {
        Err(SitemapError::HttpError(
            "All search engine pings failed".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_url_generation() {
        let sitemap = "https://mathhook.org/sitemap.xml";

        let google_url = SearchEngine::Google.ping_url(sitemap);
        assert!(google_url.contains("google.com/ping"));
        assert!(google_url.contains("sitemap="));

        let bing_url = SearchEngine::Bing.ping_url(sitemap);
        assert!(bing_url.contains("bing.com/ping"));
    }

    #[test]
    fn test_url_encoding() {
        let sitemap = "https://example.com/sitemap.xml?v=1&test=2";
        let url = SearchEngine::Google.ping_url(sitemap);
        assert!(url.contains("%3F")); // ? encoded
        assert!(url.contains("%26")); // & encoded
    }

    #[test]
    fn test_default_engines() {
        let config = PingerConfig::default();
        assert!(config.engines.contains(&SearchEngine::Google));
        assert!(config.engines.contains(&SearchEngine::Bing));
    }

    #[test]
    fn test_config_builder() {
        let config = PingerConfig::new()
            .with_timeout(60)
            .verbose()
            .with_engines(vec![SearchEngine::Google]);

        assert_eq!(config.timeout_secs, 60);
        assert!(config.verbose);
        assert_eq!(config.engines.len(), 1);
    }
}
