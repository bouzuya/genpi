use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use axum::{routing::get, Router, Server};

use crate::{
    config::Config,
    handler::{generate_pi, AppState},
};

pub async fn run_server() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    let shared_state = AppState::default();
    let router = Router::new()
        .route("/", get(generate_pi))
        .route("/healthz", get(|| async { "OK" }));
    let router = if config.base_path.is_empty() {
        router
    } else {
        Router::new()
            .route("/", get(generate_pi))
            .nest(&config.base_path, router)
    }
    .with_state(shared_state);

    let socket_addr = SocketAddr::new(
        IpAddr::from_str("0.0.0.0").expect("0.0.0.0 is valid host"),
        config.port,
    );

    Ok(Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await?)
}
