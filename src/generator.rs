use crate::regex::Primitive;

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

pub fn generate(p: Primitive, seed: u64) -> String {
    let mut rng = Xoshiro256StarStar::seed_from_u64(seed);
    match p {
        Primitive::Digit => rng.gen_range(0, 9).to_string(),
        Primitive::Loop(_, n) => (0..n)
            .map(|_| rng.gen_range(0, 9).to_string())
            .collect::<Vec<String>>()
            .join(""),
    }
}

#[cfg(test)]
mod tests {
    use crate::generator::generate;
    use crate::regex::Primitive;
    #[test]
    fn test_generate() {
        assert_eq!(generate(Primitive::Digit, 100), "2");
        assert_eq!(
            generate(Primitive::Loop(Box::new(Primitive::Digit), 10), 1),
            "0205262863"
        );
    }
}
