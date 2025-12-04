use reqwest::blocking::Client;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Cached content from a URL fetch
#[derive(Clone, Debug)]
pub struct CachedContent {
    pub content: String,
    #[allow(dead_code)]
    pub fetch_time: SystemTime,
}

/// Result of URL resolution
#[derive(Debug)]
pub enum UrlResolution {
    /// Direct raw content URL
    RawUrl(String),
    /// Repository root - need to fetch README
    RepoRoot { user: String, repo: String },
    /// Not a GitHub URL (or not recognized)
    NotGitHub,
}

/// Error types for fetching
#[derive(Debug)]
pub enum FetchError {
    Network(String),
    NotFound,
    InvalidUrl,
    Timeout,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::Network(msg) => write!(f, "Network error: {}", msg),
            FetchError::NotFound => write!(f, "File not found (404)"),
            FetchError::InvalidUrl => write!(f, "Invalid URL format"),
            FetchError::Timeout => write!(f, "Request timed out"),
        }
    }
}

/// GitHub URL fetcher with session cache
pub struct GitHubFetcher {
    client: Client,
    cache: HashMap<String, CachedContent>,
}

impl Default for GitHubFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl GitHubFetcher {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("barkdocs/0.1")
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            client,
            cache: HashMap::new(),
        }
    }

    /// Resolve a URL to determine how to fetch it
    pub fn resolve_url(&self, url: &str) -> UrlResolution {
        let url = url.trim();

        // Already a raw URL - pass through
        if url.starts_with("https://raw.githubusercontent.com/") {
            return UrlResolution::RawUrl(url.to_string());
        }

        // GitHub blob URL - convert to raw
        // https://github.com/user/repo/blob/branch/path/to/file.md
        if let Some(raw_url) = self.blob_to_raw(url) {
            return UrlResolution::RawUrl(raw_url);
        }

        // GitHub repo root
        // https://github.com/user/repo or https://github.com/user/repo/
        if let Some((user, repo)) = self.parse_repo_root(url) {
            return UrlResolution::RepoRoot { user, repo };
        }

        // Any other raw URL (non-GitHub markdown)
        if (url.starts_with("https://") || url.starts_with("http://"))
            && (url.ends_with(".md") || url.ends_with(".MD") || url.ends_with(".markdown"))
        {
            return UrlResolution::RawUrl(url.to_string());
        }

        UrlResolution::NotGitHub
    }

    /// Convert GitHub blob URL to raw.githubusercontent.com URL
    /// https://github.com/user/repo/blob/branch/path.md
    /// -> https://raw.githubusercontent.com/user/repo/branch/path.md
    fn blob_to_raw(&self, url: &str) -> Option<String> {
        let url = url.trim_end_matches('/');

        // Check if it's a GitHub blob URL
        if !url.starts_with("https://github.com/") && !url.starts_with("http://github.com/") {
            return None;
        }

        // Parse: github.com/user/repo/blob/branch/path...
        let path = url
            .strip_prefix("https://github.com/")
            .or_else(|| url.strip_prefix("http://github.com/"))?;

        let parts: Vec<&str> = path.split('/').collect();

        // Need at least: user/repo/blob/branch/file
        if parts.len() < 5 {
            return None;
        }

        // Check for /blob/ in the path
        if parts[2] != "blob" {
            return None;
        }

        let user = parts[0];
        let repo = parts[1];
        let branch = parts[3];
        let file_path = parts[4..].join("/");

        Some(format!(
            "https://raw.githubusercontent.com/{}/{}/{}/{}",
            user, repo, branch, file_path
        ))
    }

    /// Parse GitHub repo root URL
    /// https://github.com/user/repo -> Some((user, repo))
    fn parse_repo_root(&self, url: &str) -> Option<(String, String)> {
        let url = url.trim_end_matches('/');

        let path = url
            .strip_prefix("https://github.com/")
            .or_else(|| url.strip_prefix("http://github.com/"))?;

        let parts: Vec<&str> = path.split('/').collect();

        // Exactly user/repo (no additional path)
        if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
            return Some((parts[0].to_string(), parts[1].to_string()));
        }

        None
    }

    /// Fetch content from URL (checks cache first)
    pub fn fetch(&mut self, url: &str) -> Result<String, FetchError> {
        // Check cache first
        if let Some(cached) = self.cache.get(url) {
            return Ok(cached.content.clone());
        }

        // Resolve the URL
        let resolved = self.resolve_url(url);

        let content = match resolved {
            UrlResolution::RawUrl(raw_url) => self.fetch_raw(&raw_url)?,
            UrlResolution::RepoRoot { user, repo } => self.fetch_repo_readme(&user, &repo)?,
            UrlResolution::NotGitHub => {
                // Try to fetch anyway if it looks like a URL
                if url.starts_with("http://") || url.starts_with("https://") {
                    self.fetch_raw(url)?
                } else {
                    return Err(FetchError::InvalidUrl);
                }
            }
        };

        // Cache the result
        self.cache.insert(
            url.to_string(),
            CachedContent {
                content: content.clone(),
                fetch_time: SystemTime::now(),
            },
        );

        Ok(content)
    }

    /// Fetch raw content from a URL
    fn fetch_raw(&self, url: &str) -> Result<String, FetchError> {
        let response = self.client.get(url).send().map_err(|e| {
            if e.is_timeout() {
                FetchError::Timeout
            } else {
                FetchError::Network(e.to_string())
            }
        })?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(FetchError::NotFound);
        }

        if !response.status().is_success() {
            return Err(FetchError::Network(format!("HTTP {}", response.status())));
        }

        response
            .text()
            .map_err(|e| FetchError::Network(e.to_string()))
    }

    /// Fetch README from repository root
    /// Tries common branch names and README filename variants
    /// Also handles symlinks (GitHub returns symlink target as content)
    pub fn fetch_repo_readme(&mut self, user: &str, repo: &str) -> Result<String, FetchError> {
        let branches = [
            "HEAD", "main", "master", "canary", "develop", "dev", "trunk",
        ];
        let readme_variants = ["README.md", "readme.md", "README.MD", "Readme.md", "README"];

        for branch in &branches {
            for variant in &readme_variants {
                let url = format!(
                    "https://raw.githubusercontent.com/{}/{}/{}/{}",
                    user, repo, branch, variant
                );

                match self.fetch_raw(&url) {
                    Ok(content) => {
                        // Check if this is a symlink (very short, looks like a path)
                        let trimmed = content.trim();
                        if trimmed.len() < 100
                            && !trimmed.contains('\n')
                            && (trimmed.ends_with(".md") || trimmed.ends_with(".MD"))
                            && !trimmed.starts_with('#')
                            && !trimmed.starts_with('<')
                        {
                            // This looks like a symlink - follow it
                            let symlink_url = format!(
                                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                                user, repo, branch, trimmed
                            );
                            if let Ok(real_content) = self.fetch_raw(&symlink_url) {
                                return Ok(real_content);
                            }
                        }
                        return Ok(content);
                    }
                    Err(FetchError::NotFound) => continue,
                    Err(e) => return Err(e),
                }
            }
        }

        Err(FetchError::NotFound)
    }

    /// Clear the cache
    #[allow(dead_code)]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    #[allow(dead_code)]
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

/// Check if a string looks like a URL
#[allow(dead_code)]
pub fn is_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

/// Check if a URL is a GitHub URL
#[allow(dead_code)]
pub fn is_github_url(url: &str) -> bool {
    url.starts_with("https://github.com/")
        || url.starts_with("http://github.com/")
        || url.starts_with("https://raw.githubusercontent.com/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_url() {
        assert!(is_url("https://example.com"));
        assert!(is_url("http://example.com"));
        assert!(!is_url("ftp://example.com"));
        assert!(!is_url("/path/to/file.md"));
        assert!(!is_url("file.md"));
    }

    #[test]
    fn test_is_github_url() {
        assert!(is_github_url("https://github.com/user/repo"));
        assert!(is_github_url("http://github.com/user/repo"));
        assert!(is_github_url(
            "https://raw.githubusercontent.com/user/repo/main/README.md"
        ));
        assert!(!is_github_url("https://gitlab.com/user/repo"));
        assert!(!is_github_url("https://example.com/file.md"));
    }

    #[test]
    fn test_resolve_url_raw_githubusercontent() {
        let fetcher = GitHubFetcher::new();
        let url = "https://raw.githubusercontent.com/user/repo/main/README.md";

        match fetcher.resolve_url(url) {
            UrlResolution::RawUrl(resolved) => assert_eq!(resolved, url),
            _ => panic!("Expected RawUrl"),
        }
    }

    #[test]
    fn test_resolve_url_github_blob() {
        let fetcher = GitHubFetcher::new();
        let url = "https://github.com/user/repo/blob/main/docs/guide.md";

        match fetcher.resolve_url(url) {
            UrlResolution::RawUrl(resolved) => {
                assert_eq!(
                    resolved,
                    "https://raw.githubusercontent.com/user/repo/main/docs/guide.md"
                );
            }
            _ => panic!("Expected RawUrl"),
        }
    }

    #[test]
    fn test_resolve_url_github_repo_root() {
        let fetcher = GitHubFetcher::new();

        // Without trailing slash
        match fetcher.resolve_url("https://github.com/user/repo") {
            UrlResolution::RepoRoot { user, repo } => {
                assert_eq!(user, "user");
                assert_eq!(repo, "repo");
            }
            _ => panic!("Expected RepoRoot"),
        }

        // With trailing slash
        match fetcher.resolve_url("https://github.com/user/repo/") {
            UrlResolution::RepoRoot { user, repo } => {
                assert_eq!(user, "user");
                assert_eq!(repo, "repo");
            }
            _ => panic!("Expected RepoRoot"),
        }
    }

    #[test]
    fn test_resolve_url_non_github_markdown() {
        let fetcher = GitHubFetcher::new();

        match fetcher.resolve_url("https://example.com/docs/guide.md") {
            UrlResolution::RawUrl(resolved) => {
                assert_eq!(resolved, "https://example.com/docs/guide.md");
            }
            _ => panic!("Expected RawUrl for .md URL"),
        }

        match fetcher.resolve_url("https://example.com/docs/GUIDE.MD") {
            UrlResolution::RawUrl(resolved) => {
                assert_eq!(resolved, "https://example.com/docs/GUIDE.MD");
            }
            _ => panic!("Expected RawUrl for .MD URL"),
        }
    }

    #[test]
    fn test_resolve_url_not_github() {
        let fetcher = GitHubFetcher::new();

        match fetcher.resolve_url("https://example.com/page") {
            UrlResolution::NotGitHub => {}
            _ => panic!("Expected NotGitHub"),
        }

        match fetcher.resolve_url("/local/path/file.md") {
            UrlResolution::NotGitHub => {}
            _ => panic!("Expected NotGitHub for local path"),
        }
    }

    #[test]
    fn test_blob_to_raw_conversion() {
        let fetcher = GitHubFetcher::new();

        // Basic conversion
        let result = fetcher.blob_to_raw("https://github.com/rust-lang/rust/blob/master/README.md");
        assert_eq!(
            result,
            Some("https://raw.githubusercontent.com/rust-lang/rust/master/README.md".to_string())
        );

        // Nested path
        let result =
            fetcher.blob_to_raw("https://github.com/user/repo/blob/main/docs/api/guide.md");
        assert_eq!(
            result,
            Some("https://raw.githubusercontent.com/user/repo/main/docs/api/guide.md".to_string())
        );

        // HTTP (not HTTPS)
        let result = fetcher.blob_to_raw("http://github.com/user/repo/blob/main/README.md");
        assert_eq!(
            result,
            Some("https://raw.githubusercontent.com/user/repo/main/README.md".to_string())
        );

        // Not a blob URL
        let result = fetcher.blob_to_raw("https://github.com/user/repo/tree/main");
        assert_eq!(result, None);

        // Not GitHub
        let result = fetcher.blob_to_raw("https://gitlab.com/user/repo/blob/main/README.md");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_repo_root() {
        let fetcher = GitHubFetcher::new();

        // Valid repo root
        let result = fetcher.parse_repo_root("https://github.com/rust-lang/rust");
        assert_eq!(result, Some(("rust-lang".to_string(), "rust".to_string())));

        // With trailing slash
        let result = fetcher.parse_repo_root("https://github.com/user/repo/");
        assert_eq!(result, Some(("user".to_string(), "repo".to_string())));

        // HTTP
        let result = fetcher.parse_repo_root("http://github.com/user/repo");
        assert_eq!(result, Some(("user".to_string(), "repo".to_string())));

        // Too many path segments (not root)
        let result = fetcher.parse_repo_root("https://github.com/user/repo/issues");
        assert_eq!(result, None);

        // Too few path segments
        let result = fetcher.parse_repo_root("https://github.com/user");
        assert_eq!(result, None);

        // Not GitHub
        let result = fetcher.parse_repo_root("https://gitlab.com/user/repo");
        assert_eq!(result, None);
    }

    #[test]
    fn test_url_with_whitespace() {
        let fetcher = GitHubFetcher::new();

        // Leading/trailing whitespace should be trimmed
        match fetcher.resolve_url("  https://github.com/user/repo  ") {
            UrlResolution::RepoRoot { user, repo } => {
                assert_eq!(user, "user");
                assert_eq!(repo, "repo");
            }
            _ => panic!("Expected RepoRoot"),
        }
    }

    #[test]
    fn test_fetch_error_display() {
        assert_eq!(
            FetchError::Network("connection reset".to_string()).to_string(),
            "Network error: connection reset"
        );
        assert_eq!(FetchError::NotFound.to_string(), "File not found (404)");
        assert_eq!(FetchError::InvalidUrl.to_string(), "Invalid URL format");
        assert_eq!(FetchError::Timeout.to_string(), "Request timed out");
    }

    #[test]
    fn test_cache_operations() {
        let mut fetcher = GitHubFetcher::new();

        assert_eq!(fetcher.cache_size(), 0);

        // Manually insert into cache to test
        fetcher.cache.insert(
            "test_url".to_string(),
            CachedContent {
                content: "cached content".to_string(),
                fetch_time: SystemTime::now(),
            },
        );

        assert_eq!(fetcher.cache_size(), 1);

        fetcher.clear_cache();
        assert_eq!(fetcher.cache_size(), 0);
    }
}
