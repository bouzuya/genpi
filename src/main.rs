use anyhow::{bail, ensure, Context};
use rand::{thread_rng, Rng};
use scraper::{Html, Selector};
use time::OffsetDateTime;

#[derive(Debug, serde::Serialize)]
struct PI {
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

#[derive(Debug, clap::Parser)]
struct Cli {
    /// Print kana in katakana
    #[arg(long)]
    katakana: bool,
}

async fn gen_pi(is_katakana: bool) -> anyhow::Result<PI> {
    let sex = gen_sex();
    let names = gen_names(sex).await?;
    let name = choose(&names).clone();
    let name = if is_katakana {
        name.in_katakana()
    } else {
        name
    };
    Ok(PI::from((name, sex, gen_date_of_birth())))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    let pi = gen_pi(cli.katakana).await?;
    println!("{}", serde_json::to_string(&pi)?);
    Ok(())
}
