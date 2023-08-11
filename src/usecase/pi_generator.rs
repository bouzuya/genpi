use crate::model::{DateOfBirth, GenPiError, KanaForm, NameGenerator, NamesCache, Sex, PI};

#[async_trait::async_trait]
pub trait PiGenerator {
    async fn generate(&self, kana_form: KanaForm) -> Result<PI, GenPiError>;
}

pub trait HasPiGenerator {
    type PiGenerator: PiGenerator + Send + Sync;
    fn pi_generator(&self) -> &Self::PiGenerator;
}

#[async_trait::async_trait]
impl PiGenerator for NamesCache {
    async fn generate(&self, kana_form: KanaForm) -> Result<PI, GenPiError> {
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
