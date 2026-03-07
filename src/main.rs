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
struct Lyrics {
    text: String,
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
        .route("/health", get(health))
        .route("/ready", get(readyz))
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

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "ok": "ok" }))
}

async fn readyz(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    let resp = HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        lyrics_count: state.store.len(),
    };
    (StatusCode::OK, Json(resp))
}

/// GET /api/v1/random — returns a single random lyric as JSON
async fn random_lyric(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    match state.store.random_chunk() {
        Some(text) => (StatusCode::OK, Json(Lyrics { text })),
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(Lyrics {
                text: "no lyrics loaded".into(),
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
                Some(text) => {
                    let payload = serde_json::json!({ "text": text });
                    Event::default().event("lyric").json_data(payload).unwrap()
                }
                None => Event::default().event("error").data("no lyrics available"),
            };
            Ok(event)
        });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    fn app_with_lyrics(lyrics: Vec<&'static str>) -> Router {
        let store = LyricsStore::from_entries(lyrics.into_iter().map(String::from).collect());
        let state = AppState {
            store: Arc::new(store),
        };
        Router::new()
            .route("/health", get(health))
            .route("/ready", get(readyz))
            .route("/api/v1/random", get(random_lyric))
            .route("/api/v1/stream", get(stream_lyrics))
            .with_state(state)
    }

    async fn body_string(body: Body) -> String {
        let bytes = body.collect().await.unwrap().to_bytes();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    // --- GET /health ---

    #[tokio::test]
    async fn test_health() {
        let app = app_with_lyrics(vec!["a lyric"]);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = body_string(response.into_body()).await;
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["ok"], "ok");
    }

    // --- GET /ready ---

    #[tokio::test]
    async fn test_readyz() {
        let app = app_with_lyrics(vec!["a", "b", "c"]);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/ready")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = body_string(response.into_body()).await;
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["status"], "ok");
        assert_eq!(json["lyrics_count"], 3);
        assert!(json["version"].is_string());
    }

    // --- GET /api/v1/random ---

    #[tokio::test]
    async fn test_random_lyric_ok() {
        let app = app_with_lyrics(vec!["hello world"]);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/random")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = body_string(response.into_body()).await;
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["text"], "hello world");
    }

    #[tokio::test]
    async fn test_random_lyric_empty_store_returns_503() {
        let app = app_with_lyrics(vec![]);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/random")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body = body_string(response.into_body()).await;
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["text"], "no lyrics loaded");
    }

    // --- GET /api/v1/stream ---

    #[tokio::test]
    async fn test_stream_lyrics_returns_sse_content_type() {
        let app = app_with_lyrics(vec!["streaming lyric"]);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/stream")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(
            content_type.contains("text/event-stream"),
            "expected text/event-stream, got: {content_type}"
        );
    }

    #[tokio::test]
    async fn test_stream_lyrics_custom_interval_returns_sse() {
        let app = app_with_lyrics(vec!["lyric"]);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/stream?interval=5")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_stream_lyrics_interval_clamp_low() {
        // interval=0 should be clamped to 1 — still responds with SSE
        let app = app_with_lyrics(vec!["lyric"]);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/stream?interval=0")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_stream_lyrics_interval_clamp_high() {
        // interval=9999 should be clamped to 300 — still responds with SSE
        let app = app_with_lyrics(vec!["lyric"]);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/stream?interval=9999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_default_interval_value() {
        assert_eq!(default_interval(), 10);
    }
}
