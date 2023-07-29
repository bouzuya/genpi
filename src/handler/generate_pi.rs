use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Router,
};

use crate::pi::{
    gen_date_of_birth, gen_sex, GenNameError, HasNameGenerator, KanaForm, NameGenerator, PI,
};

#[derive(serde::Deserialize)]
pub struct GetRootQuery {
    halfwidth: Option<bool>,
    katakana: Option<bool>,
}

async fn handler<T>(
    State(state): State<T>,
    Query(q): Query<GetRootQuery>,
) -> Result<String, StatusCode>
where
    T: Clone + HasNameGenerator + Send + Sync,
{
    let name_generator = state.name_generator();

    let is_katakana = q.katakana.unwrap_or_default();
    let is_halfwidth = q.halfwidth.unwrap_or_default();
    let kana_form = match (is_halfwidth, is_katakana) {
        (false, false) => KanaForm::Hiragana,
        (false, true) => KanaForm::Katakana,
        (true, false) => return Err(StatusCode::BAD_REQUEST),
        (true, true) => KanaForm::HalfwidthKana,
    };

    let sex = gen_sex();

    let name = name_generator.generate(sex).await.map_err(|e| match e {
        GenNameError::RequestFailure => StatusCode::INTERNAL_SERVER_ERROR,
        GenNameError::Conflict => StatusCode::CONFLICT,
    })?;
    let name = match kana_form {
        KanaForm::Hiragana => name,
        KanaForm::Katakana => name.in_katakana(),
        KanaForm::HalfwidthKana => name.in_halfwidth_kana(),
    };
    let pi = PI::from((name, sex, gen_date_of_birth()));
    serde_json::to_string(&pi).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn generate_pi<T>() -> Router<T>
where
    T: Clone + HasNameGenerator + Send + Sync + 'static,
{
    Router::new().route("/", get(handler::<T>))
}
