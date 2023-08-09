use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::pi::{GenNameError, GenPiError, HasPiGenerator, KanaForm, PiGenerator, PI};

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
    T: Clone + HasPiGenerator + Send + Sync,
{
    let pi_generator = state.pi_generator();

    let is_katakana = q.katakana.unwrap_or_default();
    let is_halfwidth = q.halfwidth.unwrap_or_default();
    let kana_form = match (is_halfwidth, is_katakana) {
        (false, false) => KanaForm::Hiragana,
        (false, true) => KanaForm::Katakana,
        (true, false) => return Err(StatusCode::BAD_REQUEST),
        (true, true) => KanaForm::HalfwidthKana,
    };

    let pi = pi_generator
        .generate(kana_form)
        .await
        .map_err(|e| match e {
            GenPiError::GenNameError(e) => match e {
                GenNameError::RequestFailure => StatusCode::INTERNAL_SERVER_ERROR,
                GenNameError::Conflict => StatusCode::CONFLICT,
            },
        })?;
    Ok(Json(pi))
}

pub fn route<T>() -> Router<T>
where
    T: Clone + HasPiGenerator + Send + Sync + 'static,
{
    Router::new().route("/", get(handler::<T>))
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    use crate::{
        model::Name,
        pi::{gen_date_of_birth, Sex},
    };

    use super::*;

    #[derive(Clone, Debug)]
    struct MockPiGenerator;

    #[async_trait::async_trait]
    impl PiGenerator for MockPiGenerator {
        async fn generate(&self, _kana_form: KanaForm) -> Result<PI, GenPiError> {
            let sex = Sex::Male;
            let name = Name {
                first_name: "山田".to_string(),
                first_name_kana: "やまだ".to_string(),
                last_name: "太郎".to_string(),
                last_name_kana: "たろう".to_string(),
            };
            let date_of_birth = gen_date_of_birth();
            Ok(PI {
                date_of_birth,
                first_name: name.first_name,
                first_name_kana: name.first_name_kana,
                last_name: name.last_name,
                last_name_kana: name.last_name_kana,
                sex,
            })
        }
    }

    #[derive(Clone, Debug)]
    struct MockApp {
        pi_generator: MockPiGenerator,
    }

    impl HasPiGenerator for MockApp {
        type PiGenerator = MockPiGenerator;
        fn pi_generator(&self) -> &Self::PiGenerator {
            &self.pi_generator
        }
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let app = route().with_state(MockApp {
            pi_generator: MockPiGenerator,
        });

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        // TODO: test body
        Ok(())
    }
}
