use crate::model::{GenPiError, KanaForm, PI};

#[async_trait::async_trait]
pub trait GeneratePiUseCase {
    async fn generate_pi(&self, kana_form: KanaForm) -> Result<PI, GenPiError>;
}

pub trait HasGeneratePiUseCase {
    type GeneratePiUseCase: GeneratePiUseCase + Send + Sync;
    fn generate_pi_use_case(&self) -> &Self::GeneratePiUseCase;
}
