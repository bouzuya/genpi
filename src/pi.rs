use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{bail, ensure, Context};
use rand::{thread_rng, Rng};
use scraper::{Html, Selector};
use time::OffsetDateTime;
use tokio::sync::Mutex;

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

#[derive(Debug, serde::Serialize)]
pub struct DateOfBirth(String);

#[derive(Clone, Copy, Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    Female,
    Male,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Name {
    pub first_name: String,
    pub first_name_kana: String,
    pub last_name: String,
    pub last_name_kana: String,
}

impl Name {
    pub fn in_katakana(self) -> Self {
        Self {
            first_name_kana: Self::hiragana_to_katakana(&self.first_name_kana).unwrap(),
            last_name_kana: Self::hiragana_to_katakana(&self.last_name_kana).unwrap(),
            ..self
        }
    }

    pub fn in_halfwidth_kana(self) -> Self {
        Self {
            first_name_kana: Self::hiragana_to_halfwidth_kana(&self.first_name_kana).unwrap(),
            last_name_kana: Self::hiragana_to_halfwidth_kana(&self.last_name_kana).unwrap(),
            ..self
        }
    }

    fn hiragana_to_halfwidth_kana(s: &str) -> anyhow::Result<String> {
        let map = {
            let mut map = HashMap::new();
            map.insert('ぁ', "ｧ");
            map.insert('あ', "ｱ");
            map.insert('ぃ', "ｨ");
            map.insert('い', "ｲ");
            map.insert('ぅ', "ｩ");
            map.insert('う', "ｳ");
            map.insert('ぇ', "ｪ");
            map.insert('え', "ｴ");
            map.insert('ぉ', "ｫ");
            map.insert('お', "ｵ");
            map.insert('か', "ｶ");
            map.insert('が', "ｶﾞ");
            map.insert('き', "ｷ");
            map.insert('ぎ', "ｷﾞ");
            map.insert('く', "ｸ");
            map.insert('ぐ', "ｸﾞ");
            map.insert('け', "ｹ");
            map.insert('げ', "ｹﾞ");
            map.insert('こ', "ｺ");
            map.insert('ご', "ｺﾞ");
            map.insert('さ', "ｻ");
            map.insert('ざ', "ｻﾞ");
            map.insert('し', "ｼ");
            map.insert('じ', "ｼﾞ");
            map.insert('す', "ｽ");
            map.insert('ず', "ｽﾞ");
            map.insert('せ', "ｾ");
            map.insert('ぜ', "ｾﾞ");
            map.insert('そ', "ｿ");
            map.insert('ぞ', "ｿﾞ");
            map.insert('た', "ﾀ");
            map.insert('だ', "ﾀﾞ");
            map.insert('ち', "ﾁ");
            map.insert('ぢ', "ﾁﾞ");
            map.insert('っ', "ｯ");
            map.insert('つ', "ﾂ");
            map.insert('づ', "ﾂﾞ");
            map.insert('て', "ﾃ");
            map.insert('で', "ﾃﾞ");
            map.insert('と', "ﾄ");
            map.insert('ど', "ﾄﾞ");
            map.insert('な', "ﾅ");
            map.insert('に', "ﾆ");
            map.insert('ぬ', "ﾇ");
            map.insert('ね', "ﾈ");
            map.insert('の', "ﾉ");
            map.insert('は', "ﾊ");
            map.insert('ば', "ﾊﾞ");
            map.insert('ぱ', "ﾊﾟ");
            map.insert('ひ', "ﾋ");
            map.insert('び', "ﾋﾞ");
            map.insert('ぴ', "ﾋﾟ");
            map.insert('ふ', "ﾌ");
            map.insert('ぶ', "ﾌﾞ");
            map.insert('ぷ', "ﾌﾟ");
            map.insert('へ', "ﾍ");
            map.insert('べ', "ﾍﾞ");
            map.insert('ぺ', "ﾍﾟ");
            map.insert('ほ', "ﾎ");
            map.insert('ぼ', "ﾎﾞ");
            map.insert('ぽ', "ﾎﾟ");
            map.insert('ま', "ﾏ");
            map.insert('み', "ﾐ");
            map.insert('む', "ﾑ");
            map.insert('め', "ﾒ");
            map.insert('も', "ﾓ");
            map.insert('ゃ', "ｬ");
            map.insert('や', "ﾔ");
            map.insert('ゅ', "ｭ");
            map.insert('ゆ', "ﾕ");
            map.insert('ょ', "ｮ");
            map.insert('よ', "ﾖ");
            map.insert('ら', "ﾗ");
            map.insert('り', "ﾘ");
            map.insert('る', "ﾙ");
            map.insert('れ', "ﾚ");
            map.insert('ろ', "ﾛ");
            map.insert('ゎ', "ﾜ"); // not found
            map.insert('わ', "ﾜ");
            map.insert('ゐ', "ｲ"); // not found
            map.insert('ゑ', "ｴ"); // not found
            map.insert('を', "ｦ");
            map.insert('ん', "ﾝ");
            map.insert('ゔ', "ｳﾞ");
            map.insert('ゕ', "ｶ"); // not found
            map.insert('ゖ', "ｹ"); // not found
            map
        };
        s.chars().try_fold(String::new(), |mut acc, c: char| {
            let b = c as u32;
            if !(0x3041..=0x3096).contains(&b) {
                Err(anyhow::anyhow!("{} is not hiragana", c))
            } else {
                acc.push_str(map.get(&c).unwrap());
                Ok(acc)
            }
        })
    }

    fn hiragana_to_katakana(s: &str) -> anyhow::Result<String> {
        s.chars()
            .map(|c: char| {
                let b = c as u32;
                if !(0x3041..=0x3096).contains(&b) {
                    Err(anyhow::anyhow!("{} is not hiragana", c))
                } else {
                    Ok(char::from_u32(b + 0x0060).unwrap())
                }
            })
            .collect::<anyhow::Result<String>>()
    }
}

fn choose<T>(a: &[T]) -> &T {
    &a[thread_rng().gen_range(0..a.len())]
}

pub fn gen_date_of_birth() -> DateOfBirth {
    let mut rng = thread_rng();
    let current_year = OffsetDateTime::now_utc().year();
    let year = rng.gen_range(current_year - 120..=current_year);
    let month = rng.gen_range(1..=12);
    let is_leap = (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0);
    let last_day_of_month = match month {
        2 => {
            if is_leap {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    let day = rng.gen_range(1..=last_day_of_month);
    DateOfBirth(format!("{:04}-{:02}-{:02}", year, month, day))
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum GenPiError {
    #[error("gen name error")]
    GenNameError(GenNameError),
}

#[async_trait::async_trait]
pub trait PiGenerator {
    async fn generate(&self, kana_form: KanaForm) -> Result<PI, GenPiError>;
}

pub trait HasPiGenerator {
    type PiGenerator: PiGenerator + Send + Sync;
    fn pi_generator(&self) -> &Self::PiGenerator;
}

#[async_trait::async_trait]
pub trait NameGenerator {
    async fn generate(&self, sex: Sex) -> Result<Name, GenNameError>;
}

pub trait HasNameGenerator {
    type NameGenerator: NameGenerator + Send + Sync;
    fn name_generator(&self) -> &Self::NameGenerator;
}

type Names = Vec<Name>;

#[derive(Clone, Debug)]
pub struct NamesCache {
    female_names: Arc<Mutex<Option<(Instant, Names)>>>,
    male_names: Arc<Mutex<Option<(Instant, Names)>>>,
}

#[async_trait::async_trait]
impl PiGenerator for NamesCache {
    async fn generate(&self, kana_form: KanaForm) -> Result<PI, GenPiError> {
        let sex = gen_sex();
        let name = gen_name(self, sex)
            .await
            .map_err(GenPiError::GenNameError)?;
        let name = match kana_form {
            KanaForm::Hiragana => name,
            KanaForm::Katakana => name.in_katakana(),
            KanaForm::HalfwidthKana => name.in_halfwidth_kana(),
        };
        Ok(PI::from((name, sex, gen_date_of_birth())))
    }
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

#[derive(Clone, Debug, thiserror::Error)]
pub enum GenNameError {
    #[error("request failure")]
    RequestFailure,
    #[error("conflict")]
    Conflict,
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

pub fn gen_sex() -> Sex {
    *choose(&[Sex::Female, Sex::Male])
}

pub enum KanaForm {
    Hiragana,
    Katakana,
    HalfwidthKana,
}

impl PI {
    pub async fn gen(kana_form: KanaForm) -> anyhow::Result<Self> {
        let sex = gen_sex();
        let name = gen_name(&NamesCache::default(), sex).await?;
        let name = match kana_form {
            KanaForm::Hiragana => name,
            KanaForm::Katakana => name.in_katakana(),
            KanaForm::HalfwidthKana => name.in_halfwidth_kana(),
        };
        Ok(PI::from((name, sex, gen_date_of_birth())))
    }
}
