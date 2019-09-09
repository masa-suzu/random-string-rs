use crate::regex::{Pattern, Primitive};

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use std::char::from_u32;

pub fn generate(p: Pattern, seed: u64) -> String {
    let mut rng = Xoshiro256StarStar::seed_from_u64(seed);
    generate_from::<Xoshiro256StarStar>(p, &mut rng)
}

fn generate_from<T>(p: Pattern, rng: &mut T) -> String
where
    T: Rng,
{
    match p {
        Pattern::Word(w) => generate_word::<T>(*w, rng),
        Pattern::Loop(primitive, from, to) => {
            return if from != to {
                (0..rng.gen_range(from, to + 1))
            } else {
                (0..from)
            }
            .map(|_| generate_word(*primitive.clone(), rng))
            .collect::<Vec<String>>()
            .join("");
        }
        Pattern::Sequence(seq) => seq
            .into_iter()
            .map(|p| generate_from(*p.clone(), rng))
            .collect::<Vec<String>>()
            .join(""),
    }
}

fn generate_word<T>(p: Primitive, rng: &mut T) -> String
where
    T: Rng,
{
    match p {
        Primitive::Char(c) => c.to_string(),
        Primitive::Digit => rng.gen_range(0, 9).to_string(),
        Primitive::Alphabetic => from_u32(rng.gen_range(97, 122) as u32).unwrap().to_string(),
        Primitive::Group(p) => generate_from::<T>(*p, rng),
        Primitive::Alt(s) => {
            let v: Vec<char> = s.chars().collect();

            let n = v.len();
            if 0 == n {
                return "".to_string();
            };

            if 1 == n {
                return s;
            }
            let x = rng.gen_range(0, n);

            v[x].to_string()
        }
        Primitive::Or(a, b) => match rng.gen_bool(0.5) {
            true => generate_from(*a, rng),
            false => generate_from(*b, rng),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::generator::generate;
    use crate::regex::{Pattern, Primitive};
    #[test]
    fn test_generate() {
        assert_eq!(
            generate(Pattern::Word(Box::new(Primitive::Char('a'))), 100),
            "a"
        );
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
        assert_eq!(
            generate(
                Pattern::Loop(
                    Box::new(Primitive::Group(Box::new(Pattern::Word(Box::new(
                        Primitive::Alphabetic
                    ))))),
                    3,
                    3
                ),
                10202
            ),
            "wyi"
        );
        assert_eq!(
            generate(
                Pattern::Loop(Box::new(Primitive::Alt("123".to_string())), 4, 5),
                1021
            ),
            "2331"
        );
        assert_eq!(
            generate(
                Pattern::Loop(Box::new(Primitive::Alt("あ".to_string())), 1, 1),
                1021
            ),
            "あ"
        );
        assert_eq!(
            generate(
                Pattern::Word(Box::new(Primitive::Or(
                    Box::new(Pattern::Word(Box::new(Primitive::Digit))),
                    Box::new(Pattern::Word(Box::new(Primitive::Alphabetic))),
                ))),
                101
            ),
            "4"
        );
    }
}
