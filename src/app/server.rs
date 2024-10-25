use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use simp::types::config::{Config, Key};
use tokio::net::TcpListener;

async fn gen_sample(State(config): State<Arc<Config>>) -> impl IntoResponse {
    let output = simp::simulate::user::generate_sample(&config.user);

    match output {
        Ok(output) => {
            let output = output
                .into_iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<HashMap<_, _>>();
            Ok(Json(output))
        }
        Err(e) => {
            tracing::error!(?e);
            let output = format!("Error: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, output))
        }
    }
}

async fn resolve_sample(
    State(config): State<Arc<Config>>,
    Path(connector): Path<String>,
    Json(params): Json<HashMap<Key, Key>>,
) -> impl IntoResponse {
    let output = simp::simulate::psp::validate_parameters(&config.psp, connector, params);

    match output {
        Ok(output) => Ok(Json(output)),
        Err(e) => {
            tracing::error!(?e);
            let output = format!("Error: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, output))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::load()?;
    let config = Arc::new(config);

    let host = env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let addr = format!("{}:{}", host, port);

    let router: axum::Router<()> = axum::Router::new()
        .route("/generate", axum::routing::get(gen_sample))
        .route("/resolve/:connector", axum::routing::post(resolve_sample))
        .with_state(config);

    let tcp_listener = TcpListener::bind(&addr).await?;
    axum::serve(tcp_listener, router).await?;
    Ok(())
}
