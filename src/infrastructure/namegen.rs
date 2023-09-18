use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{bail, ensure, Context};
use rand::{thread_rng, Rng};
use scraper::{Html, Selector};
use tokio::sync::Mutex;

use crate::{
    model::{DateOfBirth, GenNameError, GenPiError, KanaForm, Name, NameGenerator, Sex, PI},
    use_case::GeneratePiUseCase,
};

type Names = Vec<Name>;

#[derive(Clone, Debug)]
pub struct NamesCache {
    female_names: Arc<Mutex<Option<(Instant, Names)>>>,
    male_names: Arc<Mutex<Option<(Instant, Names)>>>,
}

#[async_trait::async_trait]
impl NameGenerator for NamesCache {
    async fn generate(&self, sex: Sex) -> Result<Name, GenNameError> {
        gen_name(self, sex).await
    }
}

impl Default for NamesCache {
    fn default() -> Self {
        Self {
            female_names: Arc::new(Mutex::new(None)),
            male_names: Arc::new(Mutex::new(None)),
        }
    }
}

fn choose<T>(a: &[T]) -> &T {
    &a[thread_rng().gen_range(0..a.len())]
}

async fn gen_name(cache: &NamesCache, sex: Sex) -> Result<Name, GenNameError> {
    let mut locked = match sex {
        Sex::Female => cache
            .female_names
            .try_lock()
            .map_err(|_| GenNameError::Conflict)?,
        Sex::Male => cache
            .male_names
            .try_lock()
            .map_err(|_| GenNameError::Conflict)?,
    };
    let name = match locked.as_mut() {
        Some((instant, names)) => {
            if instant.elapsed() > Duration::new(5, 0) {
                *instant = Instant::now();
                *names = gen_names(sex)
                    .await
                    .map_err(|_| GenNameError::RequestFailure)?
            }
            choose(names).clone()
        }
        None => {
            let instant = Instant::now();
            let names = gen_names(sex)
                .await
                .map_err(|_| GenNameError::RequestFailure)?;
            let name = choose(&names).clone();
            *locked = Some((instant, names));
            name
        }
    };
    Ok(name)
}

async fn gen_names(sex: Sex) -> anyhow::Result<Names> {
    let sex = match sex {
        Sex::Female => "female",
        Sex::Male => "male",
    };
    let url = format!("https://namegen.jp/?country=japan&sex={sex}&middlename=&middlename_cond=fukumu&middlename_rarity=&middlename_rarity_cond=ika&lastname=&lastname_cond=fukumu&lastname_rarity=&lastname_rarity_cond=ika&lastname_type=name&firstname=&firstname_cond=fukumu&firstname_rarity=&firstname_rarity_cond=ika&firstname_type=name");
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        bail!("Error: {}", response.status());
    }
    let response_body = response.text().await?;
    let html = Html::parse_document(&response_body);
    let table_selector =
        Selector::parse("table.gen-table-1").expect("selector 'table.gen-table-1' is valid");
    let table = html
        .select(&table_selector)
        .next()
        .context("div.gen-table-1 not found")?;
    let tr_selector = Selector::parse("tr").expect("selector 'tr' is valid");
    let mut names = vec![];
    // skip(1) to skip the first row (header)
    for tr in table.select(&tr_selector).skip(1) {
        let name_selector = Selector::parse("td.name").expect("selector 'td.name' is valid");
        let name = tr
            .select(&name_selector)
            .next()
            .context("td.name not found")?;
        let sei_mei = name
            .text()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        ensure!(sei_mei.len() == 2, "sei_mei.len() != 2");

        let pron_selector = Selector::parse("td.pron").expect("selector 'td.pron' is valid");
        let pron = tr
            .select(&pron_selector)
            .next()
            .context("td.pron not found")?;
        let sei_mei_kana = pron
            .inner_html()
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        ensure!(sei_mei_kana.len() == 2, "sei_mei_kana.len() != 2");

        names.push(Name {
            first_name: sei_mei[1].clone(),
            first_name_kana: sei_mei_kana[1].clone(),
            last_name: sei_mei[0].clone(),
            last_name_kana: sei_mei_kana[0].clone(),
        });
    }
    Ok(names)
}

#[async_trait::async_trait]
impl GeneratePiUseCase for NamesCache {
    #[tracing::instrument(skip(self), err, ret)]
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
