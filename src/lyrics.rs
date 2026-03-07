use rand::seq::SliceRandom;
use tracing::{info, warn};

pub struct LyricsStore {
    entries: Vec<String>,
}

const DEFAULT_LYRICS: &str = include_str!("../lyrics/lyrics.json");

impl LyricsStore {
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
        self.entries.is_empty()
    }

    pub fn random_chunk(&self) -> Option<String> {
        let mut rng = rand::thread_rng();
        self.entries.choose(&mut rng).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_store(entries: Vec<String>) -> LyricsStore {
        LyricsStore { entries }
    }

    #[test]
    fn test_len_with_entries() {
        let store = make_store(vec!["a".into(), "b".into(), "c".into()]);
        assert_eq!(store.len(), 3);
    }

    #[test]
    fn test_len_empty() {
        let store = make_store(vec![]);
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_is_empty_true() {
        let store = make_store(vec![]);
        assert!(store.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let store = make_store(vec!["line".into()]);
        assert!(!store.is_empty());
    }

    #[test]
    fn test_random_chunk_returns_some() {
        let store = make_store(vec!["only line".into()]);
        let result = store.random_chunk();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "only line");
    }

    #[test]
    fn test_random_chunk_returns_none_when_empty() {
        let store = make_store(vec![]);
        assert!(store.random_chunk().is_none());
    }

    #[test]
    fn test_random_chunk_multiple_entries() {
        let store = make_store(vec!["line1".into(), "line2".into(), "line3".into()]);
        for _ in 0..20 {
            let chunk = store.random_chunk().unwrap();
            assert!(chunk == "line1" || chunk == "line2" || chunk == "line3");
        }
    }

    #[test]
    fn test_load_defaults() {
        let defaults = LyricsStore::load_defaults();
        assert!(!defaults.is_empty());
    }

    #[test]
    fn test_load_no_env_var_uses_defaults() {
        std::env::remove_var("LYLICS_DATA_PATH");
        let store = LyricsStore::load();
        assert!(store.len() > 0);
        assert!(!store.is_empty());
    }

    #[test]
    fn test_load_with_valid_file() {
        let dir = std::env::temp_dir();
        let path = dir.join("lylics_test_valid.json");
        let data = r#"["hello world", "another line"]"#;
        std::fs::write(&path, data).unwrap();

        std::env::set_var("LYLICS_DATA_PATH", path.to_str().unwrap());
        let store = LyricsStore::load();
        assert_eq!(store.len(), 2);
        let chunk = store.random_chunk().unwrap();
        assert!(chunk == "hello world" || chunk == "another line");

        std::env::remove_var("LYLICS_DATA_PATH");
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_load_with_invalid_json_file() {
        let dir = std::env::temp_dir();
        let path = dir.join("lylics_test_invalid.json");
        std::fs::write(&path, "not valid json {{{").unwrap();

        std::env::set_var("LYLICS_DATA_PATH", path.to_str().unwrap());
        let store = LyricsStore::load();
        assert!(store.len() > 0, "should fall back to defaults");

        std::env::remove_var("LYLICS_DATA_PATH");
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_load_with_nonexistent_file() {
        std::env::set_var("LYLICS_DATA_PATH", "/tmp/lylics_nonexistent_file.json");
        let store = LyricsStore::load();
        assert!(store.len() > 0, "should fall back to defaults");

        std::env::remove_var("LYLICS_DATA_PATH");
    }
}
