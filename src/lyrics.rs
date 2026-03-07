use rand::seq::SliceRandom;
use tracing::{info, warn};

pub struct LyricsStore {
    entries: Vec<String>,
}

const DEFAULT_LYRICS: &str = include_str!("../lyrics/default.json");

impl LyricsStore {
    #[cfg(test)]
    pub fn from_entries(entries: Vec<String>) -> Self {
        Self { entries }
    }

    pub fn load() -> Self {
        let entries = if let Ok(path) = std::env::var("LYLICS_DATA_PATH") {
            match std::fs::read_to_string(&path) {
                Ok(data) => match serde_json::from_str::<Vec<String>>(&data) {
                    Ok(e) => {
                        info!("loaded lyrics from {path}");
                        e
                    }
                    Err(err) => {
                        warn!("failed to parse {path}: {err}, using defaults");
                        Self::load_defaults()
                    }
                },
                Err(err) => {
                    warn!("failed to read {path}: {err}, using defaults");
                    Self::load_defaults()
                }
            }
        } else {
            info!("LYLICS_DATA_PATH not set, using embedded defaults");
            Self::load_defaults()
        };

        Self { entries }
    }

    fn load_defaults() -> Vec<String> {
        serde_json::from_str(DEFAULT_LYRICS).expect("embedded default lyrics must be valid JSON")
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn random_chunk(&self) -> Option<String> {
        if self.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        self.entries.choose(&mut rng).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_store(entries: Vec<&str>) -> LyricsStore {
        LyricsStore {
            entries: entries.into_iter().map(String::from).collect(),
        }
    }

    // --- len / is_empty ---

    #[test]
    fn len_empty() {
        let store = make_store(vec![]);
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn len_with_entries() {
        let store = make_store(vec!["a", "b", "c"]);
        assert_eq!(store.len(), 3);
    }

    #[test]
    fn is_empty_true() {
        assert!(make_store(vec![]).is_empty());
    }

    #[test]
    fn is_empty_false() {
        assert!(!make_store(vec!["x"]).is_empty());
    }

    // --- random_chunk ---

    #[test]
    fn random_chunk_returns_none_when_empty() {
        assert!(make_store(vec![]).random_chunk().is_none());
    }

    #[test]
    fn random_chunk_returns_the_entry_for_single_element() {
        let store = make_store(vec!["only lyric"]);
        for _ in 0..10 {
            assert_eq!(store.random_chunk().unwrap(), "only lyric");
        }
    }

    #[test]
    fn random_chunk_returns_one_of_the_entries() {
        let store = make_store(vec!["a", "b", "c"]);
        let allowed = ["a", "b", "c"];
        for _ in 0..20 {
            let chunk = store.random_chunk().unwrap();
            assert!(
                allowed.contains(&chunk.as_str()),
                "unexpected chunk: {chunk}"
            );
        }
    }

    // --- load: default fallback (no env var) ---

    #[test]
    fn load_no_env_var_uses_defaults() {
        std::env::remove_var("LYLICS_DATA_PATH");
        let store = LyricsStore::load();
        assert!(!store.is_empty());
        assert!(store.len() > 0);
    }

    #[test]
    fn load_defaults_contains_expected_lyrics() {
        std::env::remove_var("LYLICS_DATA_PATH");
        let store = LyricsStore::load();
        let chunks: Vec<String> = store.entries.clone();
        assert!(
            chunks.iter().any(|s| s.contains("Nothing really matters")),
            "expected embedded lyric not found"
        );
    }

    // --- load: from file ---

    #[test]
    fn load_with_valid_file() {
        let path = std::env::temp_dir().join("lylics_test_valid.json");
        std::fs::write(&path, r#"["hello world", "second lyric"]"#).unwrap();

        std::env::set_var("LYLICS_DATA_PATH", path.to_str().unwrap());
        let store = LyricsStore::load();
        assert_eq!(store.len(), 2);
        let chunk = store.random_chunk().unwrap();
        assert!(chunk == "hello world" || chunk == "second lyric");

        std::env::remove_var("LYLICS_DATA_PATH");
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn load_with_invalid_json_falls_back_to_defaults() {
        let path = std::env::temp_dir().join("lylics_test_invalid.json");
        std::fs::write(&path, "not valid json {{{").unwrap();

        std::env::set_var("LYLICS_DATA_PATH", path.to_str().unwrap());
        let store = LyricsStore::load();
        assert!(store.len() > 0, "should fall back to defaults");

        std::env::remove_var("LYLICS_DATA_PATH");
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn load_with_nonexistent_file_falls_back_to_defaults() {
        std::env::set_var("LYLICS_DATA_PATH", "/tmp/lylics_nonexistent_12345.json");
        let store = LyricsStore::load();
        assert!(store.len() > 0, "should fall back to defaults");

        std::env::remove_var("LYLICS_DATA_PATH");
    }
}
