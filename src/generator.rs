use crate::regex::Primitive;

pub fn generate(p: Primitive) -> String {
    match p {
        Primitive::Digit => "1".to_string(),
        Primitive::Loop(_, n) => (0..n).map(|_| "2").collect::<Vec<&str>>().join(""),
    }
}

#[cfg(test)]
mod tests {
    use crate::generator::generate;
    use crate::regex::Primitive;
    #[test]
    fn test_generate() {
        assert_eq!(generate(Primitive::Digit), "1");
        assert_eq!(
            generate(Primitive::Loop(Box::new(Primitive::Digit), 10)),
            "2222222222"
        );
    }
}
