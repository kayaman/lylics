use axum::{
    extract::Query,
    http::StatusCode,
    response::{
        sse::{Event, Sse},
        IntoResponse, Json,
    },
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio_stream::{wrappers::IntervalStream, StreamExt};
use tower_http::cors::CorsLayer;
use tracing::info;

mod lyrics;

use lyrics::LyricsStore;

#[derive(Clone)]
struct AppState {
    store: Arc<LyricsStore>,
}

#[derive(Deserialize)]
struct SseParams {
    #[serde(default = "default_interval")]
    interval: u64,
}

fn default_interval() -> u64 {
    10
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    lyrics_count: usize,
}

#[derive(Serialize)]
struct LyricChunk {
    artist: String,
    song: String,
    chunk: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lylics=info".into()),
        )
        .compact()
        .init();

    let store = LyricsStore::load();
    info!("loaded {} lyrics entries", store.len());

    let state = AppState {
        store: Arc::new(store),
    };

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/random", get(random_lyric))
        .route("/api/v1/stream", get(stream_lyrics))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let host = std::env::var("LYLICS_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("LYLICS_PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("{host}:{port}");

    info!("lylics listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn healthz() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn readyz(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let resp = HealthResponse {
        status: "ready".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        lyrics_count: state.store.len(),
    };
    (StatusCode::OK, Json(resp))
}

/// GET /api/v1/random — returns a single random lyric chunk as JSON
async fn random_lyric(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    match state.store.random_chunk() {
        Some(chunk) => (
            StatusCode::OK,
            Json(LyricChunk {
                artist: chunk.0.clone(),
                song: chunk.1.clone(),
                chunk: chunk.2.clone(),
            }),
        ),
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(LyricChunk {
                artist: String::new(),
                song: String::new(),
                chunk: "no lyrics loaded".into(),
            }),
        ),
    }
}

/// GET /api/v1/stream?interval=10 — SSE stream of random lyric chunks
async fn stream_lyrics(
    axum::extract::State(state): axum::extract::State<AppState>,
    Query(params): Query<SseParams>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let interval_secs = params.interval.clamp(1, 300);
    info!("new SSE client connected, interval={}s", interval_secs);

    let stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(interval_secs)))
        .map(move |_| {
            let event = match state.store.random_chunk() {
                Some((artist, song, chunk)) => {
                    let payload = serde_json::json!({
                        "artist": artist,
                        "song": song,
                        "chunk": chunk,
                    });
                    Event::default()
                        .event("lyric")
                        .json_data(payload)
                        .unwrap()
                }
                None => Event::default()
                    .event("error")
                    .data("no lyrics available"),
            };
            Ok(event)
        });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}
