use crate::model::{DateOfBirth, Name, Sex};

#[derive(Debug, serde::Serialize)]
pub struct PI {
    pub date_of_birth: DateOfBirth,
    pub first_name: String,
    pub first_name_kana: String,
    pub last_name: String,
    pub last_name_kana: String,
    pub sex: Sex,
}

impl From<(Name, Sex, DateOfBirth)> for PI {
    fn from((name, sex, date_of_birth): (Name, Sex, DateOfBirth)) -> Self {
        Self {
            date_of_birth,
            first_name: name.first_name,
            first_name_kana: name.first_name_kana,
            last_name: name.last_name,
            last_name_kana: name.last_name_kana,
            sex,
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum GenNameError {
    #[error("request failure")]
    RequestFailure,
    #[error("conflict")]
    Conflict,
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum GenPiError {
    #[error("gen name error")]
    GenNameError(GenNameError),
}

#[async_trait::async_trait]
pub trait NameGenerator {
    async fn generate(&self, sex: Sex) -> Result<Name, GenNameError>;
}

pub trait HasNameGenerator {
    type NameGenerator: NameGenerator + Send + Sync;
    fn name_generator(&self) -> &Self::NameGenerator;
}

#[derive(Debug)]
pub enum KanaForm {
    Hiragana,
    Katakana,
    HalfwidthKana,
}
