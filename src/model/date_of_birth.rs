use std::str::FromStr;

use rand::{thread_rng, Rng};
use time::{macros::format_description, Date, OffsetDateTime};

#[derive(Debug, Eq, PartialEq, serde::Serialize)]
pub struct DateOfBirth(String);

impl DateOfBirth {
    pub fn gen() -> Self {
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
        Self(format!("{:04}-{:02}-{:02}", year, month, day))
    }
}

impl FromStr for DateOfBirth {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let format = format_description!("[year]-[month]-[day]");
        let date = Date::parse(s, &format)?;
        let s = date.format(&format)?;
        Ok(Self(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() -> anyhow::Result<()> {
        let dob: DateOfBirth = "2020-01-02".parse()?;
        let s = serde_json::to_string(&dob)?;
        assert_eq!(s, r#""2020-01-02""#);
        Ok(())
    }

    #[test]
    fn test_gen() {
        assert_ne!(DateOfBirth::gen(), DateOfBirth::gen());
    }
}
