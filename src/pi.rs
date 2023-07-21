use std::collections::HashMap;

use anyhow::{bail, ensure, Context};
use rand::{thread_rng, Rng};
use scraper::{Html, Selector};
use time::OffsetDateTime;

#[derive(Debug, serde::Serialize)]
pub struct PI {
    date_of_birth: DateOfBirth,
    first_name: String,
    first_name_kana: String,
    last_name: String,
    last_name_kana: String,
    sex: Sex,
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
struct DateOfBirth(String);

#[derive(Clone, Copy, Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
enum Sex {
    Female,
    Male,
}

#[derive(Clone, Debug, serde::Serialize)]
struct Name {
    first_name: String,
    first_name_kana: String,
    last_name: String,
    last_name_kana: String,
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

fn gen_date_of_birth() -> DateOfBirth {
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

async fn gen_names(sex: Sex) -> anyhow::Result<Vec<Name>> {
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

fn gen_sex() -> Sex {
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
        let names = gen_names(sex).await?;
        let name = choose(&names).clone();
        let name = match kana_form {
            KanaForm::Hiragana => name,
            KanaForm::Katakana => name.in_katakana(),
            KanaForm::HalfwidthKana => name.in_halfwidth_kana(),
        };
        Ok(PI::from((name, sex, gen_date_of_birth())))
    }
}
