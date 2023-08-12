use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::{
    model::{GenNameError, GenPiError, KanaForm, PI},
    use_case::{GeneratePiUseCase, HasGeneratePiUseCase},
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
    T: Clone + HasGeneratePiUseCase + Send + Sync,
{
    let pi_generator = state.generate_pi_use_case();

    let is_katakana = q.katakana.unwrap_or_default();
    let is_halfwidth = q.halfwidth.unwrap_or_default();
    let kana_form = match (is_halfwidth, is_katakana) {
        (false, false) => KanaForm::Hiragana,
        (false, true) => KanaForm::Katakana,
        (true, false) => return Err(StatusCode::BAD_REQUEST),
        (true, true) => KanaForm::HalfwidthKana,
    };

    let pi = pi_generator
        .generate_pi(kana_form)
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
    T: Clone + HasGeneratePiUseCase + Send + Sync + 'static,
{
    Router::new().route("/", get(handler::<T>))
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    use crate::{
        model::{Name, Sex},
        use_case::GeneratePiUseCase,
    };

    use super::*;

    #[derive(Clone, Debug)]
    struct MockPiGenerator;

    #[async_trait::async_trait]
    impl GeneratePiUseCase for MockPiGenerator {
        async fn generate_pi(&self, _kana_form: KanaForm) -> Result<PI, GenPiError> {
            let sex = Sex::Male;
            let name = Name {
                first_name: "山田".to_string(),
                first_name_kana: "やまだ".to_string(),
                last_name: "太郎".to_string(),
                last_name_kana: "たろう".to_string(),
            };
            let date_of_birth = "2020-01-02".parse().expect("valid date");
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

    impl HasGeneratePiUseCase for MockApp {
        type GeneratePiUseCase = MockPiGenerator;
        fn generate_pi_use_case(&self) -> &Self::GeneratePiUseCase {
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

        let body = hyper::body::to_bytes(response.into_body()).await?;
        let body = String::from_utf8(body[..].to_vec())?;
        assert_eq!(
            body,
            r#"{"date_of_birth":"2020-01-02","first_name":"山田","first_name_kana":"やまだ","last_name":"太郎","last_name_kana":"たろう","sex":"male"}"#
        );
        Ok(())
    }
}
