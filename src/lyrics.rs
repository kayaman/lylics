use rand::seq::SliceRandom;
use serde::Deserialize;
use tracing::{info, warn};

#[derive(Deserialize, Clone, Debug)]
pub struct LyricEntry {
    pub artist: String,
    pub song: String,
    pub chunks: Vec<String>,
}

pub struct LyricsStore {
    entries: Vec<LyricEntry>,
}

const DEFAULT_LYRICS: &str = include_str!("../lyrics/default.json");

impl LyricsStore {
    pub fn load() -> Self {
        let entries = if let Ok(path) = std::env::var("LYLICS_DATA_PATH") {
            match std::fs::read_to_string(&path) {
                Ok(data) => match serde_json::from_str::<Vec<LyricEntry>>(&data) {
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

    fn load_defaults() -> Vec<LyricEntry> {
        serde_json::from_str(DEFAULT_LYRICS).expect("embedded default lyrics must be valid JSON")
    }

    pub fn len(&self) -> usize {
        self.entries.iter().map(|e| e.chunks.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn random_chunk(&self) -> Option<(String, String, String)> {
        let mut rng = rand::thread_rng();
        let entry = self.entries.choose(&mut rng)?;
        let chunk = entry.chunks.choose(&mut rng)?;
        Some((entry.artist.clone(), entry.song.clone(), chunk.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_store(entries: Vec<LyricEntry>) -> LyricsStore {
        LyricsStore { entries }
    }

    #[test]
    fn test_lyric_entry_deserialize() {
        let json = r#"{
            "artist": "Artist A",
            "song": "Song A",
            "chunks": ["line 1", "line 2"]
        }"#;
        let entry: LyricEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.artist, "Artist A");
        assert_eq!(entry.song, "Song A");
        assert_eq!(entry.chunks, vec!["line 1", "line 2"]);
    }

    #[test]
    fn test_lyric_entry_clone() {
        let entry = LyricEntry {
            artist: "A".into(),
            song: "S".into(),
            chunks: vec!["c".into()],
        };
        let cloned = entry.clone();
        assert_eq!(cloned.artist, "A");
        assert_eq!(cloned.song, "S");
        assert_eq!(cloned.chunks, vec!["c"]);
    }

    #[test]
    fn test_lyric_entry_debug() {
        let entry = LyricEntry {
            artist: "A".into(),
            song: "S".into(),
            chunks: vec!["c".into()],
        };
        let debug = format!("{:?}", entry);
        assert!(debug.contains("A"));
        assert!(debug.contains("S"));
    }

    #[test]
    fn test_load_defaults() {
        let defaults = LyricsStore::load_defaults();
        assert!(!defaults.is_empty());
        assert_eq!(defaults[0].artist, "Sample Artist");
        assert_eq!(defaults[0].song, "Sample Song");
        assert_eq!(defaults[0].chunks.len(), 3);
        assert_eq!(defaults[1].artist, "Example Band");
        assert_eq!(defaults[1].song, "Example Track");
        assert_eq!(defaults[1].chunks.len(), 3);
    }

    #[test]
    fn test_len_with_entries() {
        let store = make_store(vec![
            LyricEntry {
                artist: "A".into(),
                song: "S1".into(),
                chunks: vec!["a".into(), "b".into()],
            },
            LyricEntry {
                artist: "B".into(),
                song: "S2".into(),
                chunks: vec!["c".into()],
            },
        ]);
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
        let store = make_store(vec![LyricEntry {
            artist: "A".into(),
            song: "S".into(),
            chunks: vec!["c".into()],
        }]);
        assert!(!store.is_empty());
    }

    #[test]
    fn test_random_chunk_returns_some() {
        let store = make_store(vec![LyricEntry {
            artist: "Artist X".into(),
            song: "Song Y".into(),
            chunks: vec!["chunk Z".into()],
        }]);
        let result = store.random_chunk();
        assert!(result.is_some());
        let (artist, song, chunk) = result.unwrap();
        assert_eq!(artist, "Artist X");
        assert_eq!(song, "Song Y");
        assert_eq!(chunk, "chunk Z");
    }

    #[test]
    fn test_random_chunk_returns_none_when_empty() {
        let store = make_store(vec![]);
        assert!(store.random_chunk().is_none());
    }

    #[test]
    fn test_random_chunk_multiple_entries() {
        let store = make_store(vec![
            LyricEntry {
                artist: "A1".into(),
                song: "S1".into(),
                chunks: vec!["c1".into(), "c2".into()],
            },
            LyricEntry {
                artist: "A2".into(),
                song: "S2".into(),
                chunks: vec!["c3".into()],
            },
        ]);
        for _ in 0..20 {
            let (artist, song, chunk) = store.random_chunk().unwrap();
            match artist.as_str() {
                "A1" => {
                    assert_eq!(song, "S1");
                    assert!(chunk == "c1" || chunk == "c2");
                }
                "A2" => {
                    assert_eq!(song, "S2");
                    assert_eq!(chunk, "c3");
                }
                _ => panic!("unexpected artist: {artist}"),
            }
        }
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
        let data = r#"[
            {"artist": "Test", "song": "TestSong", "chunks": ["hello"]}
        ]"#;
        std::fs::write(&path, data).unwrap();

        std::env::set_var("LYLICS_DATA_PATH", path.to_str().unwrap());
        let store = LyricsStore::load();
        assert_eq!(store.len(), 1);
        let (artist, song, chunk) = store.random_chunk().unwrap();
        assert_eq!(artist, "Test");
        assert_eq!(song, "TestSong");
        assert_eq!(chunk, "hello");

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

    #[test]
    fn test_len_single_entry_multiple_chunks() {
        let store = make_store(vec![LyricEntry {
            artist: "A".into(),
            song: "S".into(),
            chunks: vec!["a".into(), "b".into(), "c".into(), "d".into()],
        }]);
        assert_eq!(store.len(), 4);
    }

    #[test]
    fn test_len_entry_with_empty_chunks() {
        let store = make_store(vec![
            LyricEntry {
                artist: "A".into(),
                song: "S".into(),
                chunks: vec![],
            },
            LyricEntry {
                artist: "B".into(),
                song: "S2".into(),
                chunks: vec!["x".into()],
            },
        ]);
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_random_chunk_entry_with_empty_chunks_still_works() {
        let store = make_store(vec![LyricEntry {
            artist: "A".into(),
            song: "S".into(),
            chunks: vec!["only".into()],
        }]);
        for _ in 0..10 {
            let (_, _, chunk) = store.random_chunk().unwrap();
            assert_eq!(chunk, "only");
        }
    }
}
