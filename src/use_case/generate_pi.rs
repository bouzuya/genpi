use crate::model::{DateOfBirth, GenPiError, KanaForm, NameGenerator, NamesCache, Sex, PI};

#[async_trait::async_trait]
pub trait GeneratePiUseCase {
    async fn generate_pi(&self, kana_form: KanaForm) -> Result<PI, GenPiError>;
}

pub trait HasGeneratePiUseCase {
    type GeneratePiUseCase: GeneratePiUseCase + Send + Sync;
    fn generate_pi_use_case(&self) -> &Self::GeneratePiUseCase;
}

#[async_trait::async_trait]
impl GeneratePiUseCase for NamesCache {
    async fn generate_pi(&self, kana_form: KanaForm) -> Result<PI, GenPiError> {
        let sex = Sex::gen();
        let name = NameGenerator::generate(self, sex)
            .await
            .map_err(GenPiError::GenNameError)?;
        let name = match kana_form {
            KanaForm::Hiragana => name,
            KanaForm::Katakana => name.in_katakana(),
            KanaForm::HalfwidthKana => name.in_halfwidth_kana(),
        };
        Ok(PI::from((name, sex, DateOfBirth::gen())))
    }
}
