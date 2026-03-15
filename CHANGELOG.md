# Changelog

## [v0.3.0](https://github.com/kayaman/lylics/releases/tag/v0.3.0) — 2026-03-14

### Features

- Enhance Helm deployment configuration: conditional environment variable injection based on `lyricsData` and `env` values.
- Add support for `nodeSelector`, `affinity`, and `tolerations` in the Helm deployment spec.

## [v0.2.0](https://github.com/kayaman/lylics/releases/tag/v0.2.0) — 2026-03-07

### Breaking Changes

- Simplified data model: lyric entries are now plain strings instead of structured `{artist, song, chunks}` objects. Update any data files from the old format to a flat JSON array of strings.
- `GET /api/v1/random` response changed from `{"artist": ..., "song": ..., "chunk": ...}` to `{"text": "..."}`.
- `GET /api/v1/stream` SSE event payload changed from `{"artist": ..., "song": ..., "chunk": ...}` to `{"text": "..."}`.

### Features

- Replace structured `LyricEntry` model with `Vec<String>` for simpler data management.
- Provide `lyrics/default.json` as a flat string array embedded at compile time.
- Add `LyricsStore::from_entries` constructor (test-only) to support integration testing.

### Fixes

- Fix route registration: `/health` now correctly dispatches to the `health()` handler.
- Fix `random_lyric` handler: returns the lyric text instead of the artist name.
- Fix `random_lyric` error branch: uses the correct `Lyrics` response type (removed non-existent `Ly` type).
- Fix `stream_lyrics` handler: destructures `Option<String>` from updated `random_chunk()` signature.

### Infrastructure

- Add 21 tests covering 100% of application code paths across `lyrics.rs` and `main.rs`.
- Add `[dev-dependencies]`: `tower` (for `ServiceExt::oneshot`) and `http-body-util` (for response body reading in integration tests).

## [v0.1.0](https://github.com/kayaman/lylics/releases/tag/v0.1.0)

- Initial release: HTTP microservice serving random lyrics via REST and SSE with structured `{artist, song, chunks}` data model.
