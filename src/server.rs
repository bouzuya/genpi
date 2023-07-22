use std::{
    env,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use anyhow::Context;
use axum::{http::StatusCode, routing::get, Router, Server};

use crate::pi::{KanaForm, PI};

async fn get_root() -> Result<String, StatusCode> {
    // TODO: cache names
    let pi = PI::gen(KanaForm::Hiragana)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    serde_json::to_string(&pi).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn run_server() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(get_root))
        .route("/healthz", get(|| async { "OK" }));

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
