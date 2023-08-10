use rand::{thread_rng, Rng};

#[derive(Clone, Copy, Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    Female,
    Male,
}

impl Sex {
    pub fn gen() -> Self {
        [Self::Female, Self::Male][thread_rng().gen_range(0..1)]
    }
}
