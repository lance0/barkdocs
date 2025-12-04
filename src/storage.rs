use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// A history entry (recently opened file or URL)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// File path or URL
    pub location: String,
    /// Whether this is a URL (vs local file)
    pub is_url: bool,
    /// Display name (filename or document title)
    pub display_name: String,
    /// Timestamp of last access (unix epoch seconds)
    pub last_accessed: u64,
}

/// A bookmark entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bookmark {
    /// File path or URL
    pub location: String,
    /// Whether this is a URL (vs local file)
    pub is_url: bool,
    /// User-provided name (or auto-generated)
    pub name: String,
    /// Timestamp when bookmarked (unix epoch seconds)
    pub created_at: u64,
}

/// History manager
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct History {
    entries: Vec<HistoryEntry>,
    #[serde(default = "default_max_entries")]
    max_entries: usize,
}

fn default_max_entries() -> usize {
    100
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 100,
        }
    }

    /// Load history from disk
    pub fn load() -> Self {
        let Some(path) = Self::storage_path() else {
            return Self::new();
        };

        if !path.exists() {
            return Self::new();
        }

        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Self::new()),
            Err(_) => Self::new(),
        }
    }

    /// Save history to disk
    pub fn save(&self) -> Result<(), std::io::Error> {
        let Some(path) = Self::storage_path() else {
            return Ok(());
        };

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }

    /// Add or update an entry in history
    pub fn add(&mut self, location: &str, is_url: bool, display_name: &str) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Check if entry already exists
        if let Some(existing) = self.entries.iter_mut().find(|e| e.location == location) {
            existing.last_accessed = now;
            existing.display_name = display_name.to_string();
        } else {
            self.entries.push(HistoryEntry {
                location: location.to_string(),
                is_url,
                display_name: display_name.to_string(),
                last_accessed: now,
            });
        }

        // Sort by most recent first
        self.entries
            .sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));

        // Trim to max entries
        if self.entries.len() > self.max_entries {
            self.entries.truncate(self.max_entries);
        }
    }

    /// Get all entries (most recent first)
    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    /// Clear all history
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get storage path
    fn storage_path() -> Option<PathBuf> {
        dirs::data_dir().map(|d| d.join("barkdocs").join("history.json"))
    }
}

/// Bookmarks manager
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Bookmarks {
    entries: Vec<Bookmark>,
}

impl Bookmarks {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Load bookmarks from disk
    pub fn load() -> Self {
        let Some(path) = Self::storage_path() else {
            return Self::new();
        };

        if !path.exists() {
            return Self::new();
        }

        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Self::new()),
            Err(_) => Self::new(),
        }
    }

    /// Save bookmarks to disk
    pub fn save(&self) -> Result<(), std::io::Error> {
        let Some(path) = Self::storage_path() else {
            return Ok(());
        };

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }

    /// Add a new bookmark
    pub fn add(&mut self, location: &str, is_url: bool, name: &str) {
        // Check if already bookmarked
        if self.find_by_location(location).is_some() {
            return;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.entries.push(Bookmark {
            location: location.to_string(),
            is_url,
            name: name.to_string(),
            created_at: now,
        });
    }

    /// Remove a bookmark by index
    pub fn remove(&mut self, index: usize) {
        if index < self.entries.len() {
            self.entries.remove(index);
        }
    }

    /// Get all bookmarks
    pub fn entries(&self) -> &[Bookmark] {
        &self.entries
    }

    /// Find a bookmark by location
    pub fn find_by_location(&self, location: &str) -> Option<&Bookmark> {
        self.entries.iter().find(|b| b.location == location)
    }

    /// Check if a location is bookmarked
    pub fn is_bookmarked(&self, location: &str) -> bool {
        self.find_by_location(location).is_some()
    }

    /// Get storage path
    fn storage_path() -> Option<PathBuf> {
        dirs::data_dir().map(|d| d.join("barkdocs").join("bookmarks.json"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_new() {
        let history = History::new();
        assert!(history.entries().is_empty());
        assert_eq!(history.max_entries, 100);
    }

    #[test]
    fn test_history_add() {
        let mut history = History::new();

        history.add("/path/to/file.md", false, "file.md");
        assert_eq!(history.entries().len(), 1);
        assert_eq!(history.entries()[0].location, "/path/to/file.md");
        assert!(!history.entries()[0].is_url);
        assert_eq!(history.entries()[0].display_name, "file.md");
    }

    #[test]
    fn test_history_add_url() {
        let mut history = History::new();

        history.add("https://github.com/user/repo", true, "repo README");
        assert_eq!(history.entries().len(), 1);
        assert!(history.entries()[0].is_url);
    }

    #[test]
    fn test_history_update_existing() {
        let mut history = History::new();

        history.add("/path/to/file.md", false, "old name");
        let first_access = history.entries()[0].last_accessed;

        history.add("/path/to/file.md", false, "new name");

        // Should still be one entry, with updated name and timestamp
        assert_eq!(history.entries().len(), 1);
        assert_eq!(history.entries()[0].display_name, "new name");
        assert!(history.entries()[0].last_accessed >= first_access);
    }

    #[test]
    fn test_history_ordering_by_timestamp() {
        // Directly create history entries with known timestamps
        let mut history = History::new();

        // Manually insert entries with controlled timestamps
        history.entries.push(HistoryEntry {
            location: "/first.md".to_string(),
            is_url: false,
            display_name: "first".to_string(),
            last_accessed: 1000,
        });
        history.entries.push(HistoryEntry {
            location: "/second.md".to_string(),
            is_url: false,
            display_name: "second".to_string(),
            last_accessed: 2000,
        });
        history.entries.push(HistoryEntry {
            location: "/third.md".to_string(),
            is_url: false,
            display_name: "third".to_string(),
            last_accessed: 3000,
        });

        // Sort by most recent (like add() does)
        history
            .entries
            .sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));

        // Most recent should be first
        assert_eq!(history.entries()[0].location, "/third.md");
        assert_eq!(history.entries()[1].location, "/second.md");
        assert_eq!(history.entries()[2].location, "/first.md");
    }

    #[test]
    fn test_history_max_entries() {
        let mut history = History::new();
        history.max_entries = 3;

        // Manually insert entries with controlled timestamps
        history.entries.push(HistoryEntry {
            location: "/1.md".to_string(),
            is_url: false,
            display_name: "1".to_string(),
            last_accessed: 1000,
        });
        history.entries.push(HistoryEntry {
            location: "/2.md".to_string(),
            is_url: false,
            display_name: "2".to_string(),
            last_accessed: 2000,
        });
        history.entries.push(HistoryEntry {
            location: "/3.md".to_string(),
            is_url: false,
            display_name: "3".to_string(),
            last_accessed: 3000,
        });
        history.entries.push(HistoryEntry {
            location: "/4.md".to_string(),
            is_url: false,
            display_name: "4".to_string(),
            last_accessed: 4000,
        });

        // Sort and truncate (like add() does)
        history
            .entries
            .sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
        history.entries.truncate(history.max_entries);

        // Should only keep 3 most recent
        assert_eq!(history.entries().len(), 3);
        assert_eq!(history.entries()[0].location, "/4.md");
        assert_eq!(history.entries()[1].location, "/3.md");
        assert_eq!(history.entries()[2].location, "/2.md");
    }

    #[test]
    fn test_history_clear() {
        let mut history = History::new();
        history.add("/file.md", false, "file");
        history.clear();
        assert!(history.entries().is_empty());
    }

    #[test]
    fn test_bookmarks_new() {
        let bookmarks = Bookmarks::new();
        assert!(bookmarks.entries().is_empty());
    }

    #[test]
    fn test_bookmarks_add() {
        let mut bookmarks = Bookmarks::new();

        bookmarks.add("/path/to/file.md", false, "My File");
        assert_eq!(bookmarks.entries().len(), 1);
        assert_eq!(bookmarks.entries()[0].location, "/path/to/file.md");
        assert_eq!(bookmarks.entries()[0].name, "My File");
        assert!(!bookmarks.entries()[0].is_url);
    }

    #[test]
    fn test_bookmarks_no_duplicates() {
        let mut bookmarks = Bookmarks::new();

        bookmarks.add("/file.md", false, "First");
        bookmarks.add("/file.md", false, "Second");

        // Should not add duplicate
        assert_eq!(bookmarks.entries().len(), 1);
        assert_eq!(bookmarks.entries()[0].name, "First");
    }

    #[test]
    fn test_bookmarks_remove() {
        let mut bookmarks = Bookmarks::new();

        bookmarks.add("/1.md", false, "1");
        bookmarks.add("/2.md", false, "2");
        bookmarks.add("/3.md", false, "3");

        bookmarks.remove(1);

        assert_eq!(bookmarks.entries().len(), 2);
        assert_eq!(bookmarks.entries()[0].location, "/1.md");
        assert_eq!(bookmarks.entries()[1].location, "/3.md");
    }

    #[test]
    fn test_bookmarks_remove_out_of_bounds() {
        let mut bookmarks = Bookmarks::new();
        bookmarks.add("/file.md", false, "file");

        // Should not panic
        bookmarks.remove(100);
        assert_eq!(bookmarks.entries().len(), 1);
    }

    #[test]
    fn test_bookmarks_find_by_location() {
        let mut bookmarks = Bookmarks::new();

        bookmarks.add("/file1.md", false, "File 1");
        bookmarks.add("/file2.md", false, "File 2");

        let found = bookmarks.find_by_location("/file2.md");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "File 2");

        let not_found = bookmarks.find_by_location("/nonexistent.md");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_bookmarks_is_bookmarked() {
        let mut bookmarks = Bookmarks::new();
        bookmarks.add("/file.md", false, "file");

        assert!(bookmarks.is_bookmarked("/file.md"));
        assert!(!bookmarks.is_bookmarked("/other.md"));
    }

    #[test]
    fn test_history_entry_serialization() {
        let entry = HistoryEntry {
            location: "/test.md".to_string(),
            is_url: false,
            display_name: "test".to_string(),
            last_accessed: 1234567890,
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: HistoryEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.location, entry.location);
        assert_eq!(deserialized.is_url, entry.is_url);
        assert_eq!(deserialized.display_name, entry.display_name);
        assert_eq!(deserialized.last_accessed, entry.last_accessed);
    }

    #[test]
    fn test_bookmark_serialization() {
        let bookmark = Bookmark {
            location: "https://github.com/user/repo".to_string(),
            is_url: true,
            name: "My Repo".to_string(),
            created_at: 1234567890,
        };

        let json = serde_json::to_string(&bookmark).unwrap();
        let deserialized: Bookmark = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.location, bookmark.location);
        assert_eq!(deserialized.is_url, bookmark.is_url);
        assert_eq!(deserialized.name, bookmark.name);
        assert_eq!(deserialized.created_at, bookmark.created_at);
    }
}
