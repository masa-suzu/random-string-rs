use crate::regex::{Pattern, Primitive};

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use std::char::from_u32;

pub fn generate(p: Pattern, seed: u64) -> String {
    let mut rng = Xoshiro256StarStar::seed_from_u64(seed);
    match p {
        Pattern::Word(w) => generate_word::<Xoshiro256StarStar>(*w, &mut rng),
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

fn generate_word<T>(p: Primitive, rng: &mut T) -> String
where
    T: Rng,
{
    match p {
        Primitive::Digit => rng.gen_range(0, 9).to_string(),
        Primitive::Alphabetic => from_u32(rng.gen_range(97, 122) as u32).unwrap().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::generator::generate;
    use crate::regex::{Pattern, Primitive};
    #[test]
    fn test_generate() {
        assert_eq!(
            generate(Pattern::Word(Box::new(Primitive::Digit)), 100),
            "2"
        );
        assert_eq!(
            generate(Pattern::Word(Box::new(Primitive::Alphabetic)), 100),
            "f"
        );
        assert_eq!(
            generate(Pattern::Loop(Box::new(Primitive::Digit), 10, 10), 1),
            "0205262863"
        );
        assert_eq!(
            generate(Pattern::Loop(Box::new(Primitive::Digit), 2, 2), 100),
            "25"
        );
        assert_eq!(
            generate(Pattern::Loop(Box::new(Primitive::Digit), 1, 10), 10202),
            "8582722"
        );
    }
}
