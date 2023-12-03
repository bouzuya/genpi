use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

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
        ServiceBuilder::new()
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(
                        DefaultMakeSpan::new()
                            .level(Level::INFO)
                            .include_headers(true),
                    )
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .include_headers(true),
                    ),
            )
            .layer(PropagateRequestIdLayer::x_request_id()),
    );

    let socket_addr = SocketAddr::new(
        IpAddr::from_str("0.0.0.0").expect("0.0.0.0 is valid host"),
        config.port,
    );

    let tcp_listener = TcpListener::bind(socket_addr).await?;
    Ok(axum::serve(tcp_listener, router.into_make_service()).await?)
}
