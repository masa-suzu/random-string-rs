use crate::regex::Pattern;

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

pub fn generate(p: Pattern, seed: u64) -> String {
    let mut rng = Xoshiro256StarStar::seed_from_u64(seed);
    match p {
        Pattern::Digit => rng.gen_range(0, 9).to_string(),
        Pattern::Loop(_, from, to) => if from != to {
            (0..rng.gen_range(from, to + 1))
        } else {
            (0..from)
        }
        .map(|_| rng.gen_range(0, 9).to_string())
        .collect::<Vec<String>>()
        .join(""),
    }
}

#[cfg(test)]
mod tests {
    use crate::generator::generate;
    use crate::regex::Pattern;
    #[test]
    fn test_generate() {
        assert_eq!(generate(Pattern::Digit, 100), "2");
        assert_eq!(
            generate(Pattern::Loop(Box::new(Pattern::Digit), 10, 10), 1),
            "0205262863"
        );
        assert_eq!(
            generate(Pattern::Loop(Box::new(Pattern::Digit), 2, 2), 100),
            "25"
        );
        assert_eq!(
            generate(Pattern::Loop(Box::new(Pattern::Digit), 1, 10), 10202),
            "8582722"
        );
    }
}
