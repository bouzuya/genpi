use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Router, Server,
};
use tokio::sync::Mutex;

use crate::{
    config::Config,
    pi::{choose, gen_date_of_birth, gen_names, gen_sex, KanaForm, Name, Sex, PI},
};

#[derive(serde::Deserialize)]
struct GetRootQuery {
    halfwidth: Option<bool>,
    katakana: Option<bool>,
}

type Names = Vec<Name>;

#[derive(Clone, Debug)]
struct AppState {
    female_names: Arc<Mutex<Option<(Instant, Names)>>>,
    male_names: Arc<Mutex<Option<(Instant, Names)>>>,
}

async fn get_root(
    State(app_state): State<AppState>,
    Query(q): Query<GetRootQuery>,
) -> Result<String, StatusCode> {
    let is_katakana = q.katakana.unwrap_or_default();
    let is_halfwidth = q.halfwidth.unwrap_or_default();
    let kana_form = match (is_halfwidth, is_katakana) {
        (false, false) => KanaForm::Hiragana,
        (false, true) => KanaForm::Katakana,
        (true, false) => return Err(StatusCode::BAD_REQUEST),
        (true, true) => KanaForm::HalfwidthKana,
    };

    let sex = gen_sex();
    let name = {
        let mut locked = match sex {
            Sex::Female => app_state
                .female_names
                .try_lock()
                .map_err(|_| StatusCode::CONFLICT)?,
            Sex::Male => app_state
                .male_names
                .try_lock()
                .map_err(|_| StatusCode::CONFLICT)?,
        };
        let name = match locked.as_mut() {
            Some((instant, names)) => {
                if instant.elapsed() > Duration::new(5, 0) {
                    *instant = Instant::now();
                    *names = gen_names(sex)
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                }
                choose(names).clone()
            }
            None => {
                let instant = Instant::now();
                let names = gen_names(sex)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                let name = choose(&names).clone();
                *locked = Some((instant, names));
                name
            }
        };
        name
    };
    let name = match kana_form {
        KanaForm::Hiragana => name,
        KanaForm::Katakana => name.in_katakana(),
        KanaForm::HalfwidthKana => name.in_halfwidth_kana(),
    };
    let pi = PI::from((name, sex, gen_date_of_birth()));
    serde_json::to_string(&pi).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn run_server() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    let shared_state = AppState {
        female_names: Arc::new(Mutex::new(None)),
        male_names: Arc::new(Mutex::new(None)),
    };
    let router = Router::new()
        .route("/", get(get_root))
        .route("/healthz", get(|| async { "OK" }));
    let router = if config.base_path.is_empty() {
        router
    } else {
        Router::new()
            .route("/", get(get_root))
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
