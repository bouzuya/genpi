use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
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
) -> Result<Json<PI>, StatusCode>
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
    Ok(Json(pi))
}

pub fn generate_pi<T>() -> Router<T>
where
    T: Clone + HasNameGenerator + Send + Sync + 'static,
{
    Router::new().route("/", get(handler::<T>))
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    use crate::pi::{Name, Sex};

    use super::*;

    #[derive(Clone, Debug)]
    struct MockNameGenerator;

    #[async_trait::async_trait]
    impl NameGenerator for MockNameGenerator {
        async fn generate(&self, _sex: Sex) -> Result<Name, GenNameError> {
            Ok(Name {
                first_name: "山田".to_string(),
                first_name_kana: "やまだ".to_string(),
                last_name: "太郎".to_string(),
                last_name_kana: "たろう".to_string(),
            })
        }
    }

    #[derive(Clone, Debug)]
    struct MockApp {
        name_generator: MockNameGenerator,
    }

    impl HasNameGenerator for MockApp {
        type NameGenerator = MockNameGenerator;
        fn name_generator(&self) -> &Self::NameGenerator {
            &self.name_generator
        }
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let app = generate_pi().with_state(MockApp {
            name_generator: MockNameGenerator,
        });

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        // TODO: test body
        Ok(())
    }
}
