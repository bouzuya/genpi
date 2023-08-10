use rand::{thread_rng, Rng};
use time::OffsetDateTime;

#[derive(Debug, serde::Serialize)]
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
