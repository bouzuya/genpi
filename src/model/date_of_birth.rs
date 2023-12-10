use std::{ops::Range, str::FromStr};

use rand::Rng;
use time::{macros::format_description, Date, Duration};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateOfBirth(Date);

impl From<Date> for DateOfBirth {
    fn from(date: Date) -> Self {
        Self(date)
    }
}

impl FromStr for DateOfBirth {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let format = format_description!("[year]-[month]-[day]");
        let date = Date::parse(s, &format)?;
        Ok(Self(date))
    }
}

impl serde::Serialize for DateOfBirth {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(
            &self
                .0
                .format(&format_description!("[year]-[month]-[day]"))
                .expect("invalid format"),
        )
    }
}

pub struct UniformDateOfBirth(Range<Date>);

impl rand::distributions::uniform::UniformSampler for UniformDateOfBirth {
    type X = DateOfBirth;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        let (low, high) = (low.borrow().0, high.borrow().0);
        if low >= high {
            panic!("low must be less than high")
        }
        Self(low..high)
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        let (low, high) = (
            low.borrow().0,
            high.borrow().0.saturating_add(Duration::days(1)),
        );
        if low > high {
            panic!("low must be less than or equal to high")
        }
        Self(low..high)
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        DateOfBirth(
            Date::from_julian_day(
                rng.gen_range(self.0.start.to_julian_day()..self.0.end.to_julian_day()),
            )
            .expect("invalid date"),
        )
    }
}

impl rand::distributions::uniform::SampleUniform for DateOfBirth {
    type Sampler = UniformDateOfBirth;
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_from_date() -> anyhow::Result<()> {
        let date = Date::parse("2020-01-02", format_description!("[year]-[month]-[day]"))?;
        let dob = DateOfBirth::from(date);
        let s = serde_json::to_string(&dob)?;
        assert_eq!(s, r#""2020-01-02""#);
        Ok(())
    }

    #[test]
    fn test_from_str() -> anyhow::Result<()> {
        let dob: DateOfBirth = "2020-01-02".parse()?;
        let s = serde_json::to_string(&dob)?;
        assert_eq!(s, r#""2020-01-02""#);
        Ok(())
    }

    #[test]
    fn test_gen_range() -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();

        let low = "2020-01-01".parse::<DateOfBirth>()?;
        let high = "2020-01-02".parse::<DateOfBirth>()?;
        let gen = rng.gen_range(low..high);
        assert_eq!(gen, low);

        let mut set = HashSet::new();
        for _ in 0..100 {
            let gen = rng.gen_range(low..=high);
            set.insert(gen);
        }
        assert_eq!(set.len(), 2);

        Ok(())
    }
}
