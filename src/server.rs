use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use axum::{extract::MatchedPath, routing::get, Router, Server};
use hyper::Request;
use tower_http::trace::TraceLayer;
use tracing::info_span;

use crate::{
    config::Config, handler::generate_pi, infrastructure::NamesCache, model::HasNameGenerator,
    use_case::HasGeneratePiUseCase,
};

#[derive(Clone, Debug)]
pub struct AppState {
    name_generator: NamesCache,
}

impl HasNameGenerator for AppState {
    type NameGenerator = NamesCache;

    fn name_generator(&self) -> &Self::NameGenerator {
        &self.name_generator
    }
}

impl HasGeneratePiUseCase for AppState {
    type GeneratePiUseCase = NamesCache;

    fn generate_pi_use_case(&self) -> &Self::GeneratePiUseCase {
        &self.name_generator
    }
}

pub async fn run_server() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    let state = AppState {
        name_generator: NamesCache::default(),
    };
    let router = Router::new().merge(generate_pi::route::<AppState>());
    let router = if config.base_path.is_empty() {
        router
    } else {
        Router::new()
            .route("/", get(|| async { "OK" }))
            .nest(&config.base_path, router)
    }
    .with_state(state)
    .layer(
        TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);
            info_span!(
                "http_request",
                method = ?request.method(),
                matched_path,
            )
        }),
    );

    let socket_addr = SocketAddr::new(
        IpAddr::from_str("0.0.0.0").expect("0.0.0.0 is valid host"),
        config.port,
    );

    Ok(Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await?)
}
