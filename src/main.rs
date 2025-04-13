mod domain;
mod paymentprocessors;
mod dtos;

use std::env;

use axum_prometheus::PrometheusMetricLayer;
use dotenv::dotenv;
use axum::{http::Method, middleware::from_fn_with_state, routing::{get, post, put}, Router};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::
    fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_target(false)
    .with_ansi(false)
    .json()
    .with_file(true)
    .with_line_number(true)
    .with_current_span(true)
    .with_writer(std::fs::File::create(String::from(env::var("LOG_PATH").unwrap())).unwrap())
    .init();

    let (prometheus_layer, metrics_handle) = PrometheusMetricLayer::pair();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env::var("AXUM_PORT").unwrap())).await.unwrap();

    axum::serve(listener, Router::new()).await.unwrap();
}
