mod cqrs;
mod domain;
mod paymentprocessors;
mod dtos;
mod routes;
mod state;
mod auth;
mod events;

use std::{env, sync::Arc};

use axum_prometheus::PrometheusMetricLayer;
use cqrs::CreateCheckoutSessionCommandHandler;
use dotenv::dotenv;
use axum::{http::Method, middleware::from_fn_with_state, routing::{get, post, put}, Router};
use paymentprocessors::StripePaymentProcessor;
use routes::{create_checkout_session, index};
use state::AppState;

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

    let payment_processor = Arc::new(StripePaymentProcessor::new(String::from(env::var("PAYMENT_REDIRECT_BASE_URL").unwrap())));
    let create_checkout_session_command_handler = Arc::new(CreateCheckoutSessionCommandHandler::new(payment_processor));

    let state = Arc::new(AppState {
        create_checkout_session_command_handler: create_checkout_session_command_handler,
        auth0_domain: String::from(env::var("AUTH0_DOMAIN").unwrap()),
        auth0_audience: String::from(env::var("AUTH0_AUDIENCE").unwrap()),
    });

    let (prometheus_layer, metrics_handle) = PrometheusMetricLayer::pair();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env::var("AXUM_PORT").unwrap())).await.unwrap();

    axum::serve(listener, Router::new()
        .route("/", 
            get(index))

        .route("/metrics", 
            get(|| async move {metrics_handle.render()}))
        
        .route("/payments/checkout", 
            post(create_checkout_session)
            .route_layer(from_fn_with_state(state.clone(), auth::authentication_middleware)))
    
        .with_state(state)).await.unwrap();
}
