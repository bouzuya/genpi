use axum::{http::StatusCode, routing::get, Router};

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
    // TODO: use PORT env
    Ok(axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?)
}
