#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    Female,
    Male,
}

impl rand::distributions::Distribution<Sex> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sex {
        if rng.gen::<bool>() {
            Sex::Female
        } else {
            Sex::Male
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rand::Rng;

    use super::*;

    #[test]
    fn test_sample() {
        let mut rng = rand::thread_rng();
        let set = (0..100).map(|_| rng.gen::<Sex>()).collect::<HashSet<Sex>>();
        assert_eq!(set.len(), 2);
    }
}
