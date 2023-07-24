use std::{
    env,
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Context;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Router, Server,
};
use tokio::sync::Mutex;

use crate::pi::{choose, gen_date_of_birth, gen_names, gen_sex, KanaForm, Name, Sex, PI};

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
    let shared_state = AppState {
        female_names: Arc::new(Mutex::new(None)),
        male_names: Arc::new(Mutex::new(None)),
    };
    let app = Router::new()
        .route("/", get(get_root))
        .route("/healthz", get(|| async { "OK" }))
        .with_state(shared_state);

    let port_as_string = env::var("PORT").or_else(|e| match e {
        env::VarError::NotPresent => Ok("3000".to_owned()),
        env::VarError::NotUnicode(_) => anyhow::bail!("PORT is not unicode"),
    })?;
    let port = u16::from_str(port_as_string.as_str()).context("PORT range is (0..=65535)")?;
    let socket_addr = SocketAddr::new(
        IpAddr::from_str("0.0.0.0").expect("0.0.0.0 is valid host"),
        port,
    );

    Ok(Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?)
}
