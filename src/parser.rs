use crate::regex::Primitive;

extern crate nom;
use nom::{bytes::complete::tag, character::complete::digit1, IResult};

pub fn parse_digits(s: &str) -> IResult<&str, Primitive> {
    let (s, _) = parse_digit(s)?;

    let (s, _) = tag("{")(s)?;
    let (s, n) = digit1(s)?;
    let (s, _) = tag("}")(s)?;

    Ok((
        s,
        Primitive::Loop(Box::new(Primitive::Digit), n.parse().unwrap()),
    ))
}

pub fn parse_digit(s: &str) -> IResult<&str, Primitive> {
    let (s, _) = tag("\\b")(s)?;
    Ok((s, Primitive::Digit))
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_digit, parse_digits};
    use crate::regex::Primitive;
    #[test]
    fn test_parse_digit() {
        assert_eq!(parse_digit(r"\b"), Ok(("", Primitive::Digit)));
    }

    #[test]
    fn test_parse_digits() {
        assert_eq!(
            parse_digits(r"\b{1}"),
            Ok(("", Primitive::Loop(Box::new(Primitive::Digit), 1)))
        );
        assert_eq!(
            parse_digits(r"\b{10}"),
            Ok(("", Primitive::Loop(Box::new(Primitive::Digit), 10)))
        );
    }
}
