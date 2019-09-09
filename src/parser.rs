use crate::regex::{Pattern, Primitive};

extern crate nom;
use nom::{
    bytes::complete::is_not,
    bytes::complete::tag,
    character::complete::{char, digit1},
    sequence::delimited,
    IResult,
};

#[derive(PartialEq, Clone, Debug)]
pub enum Error {
    UnTerminatedError(String),
    ParseError,
}

pub fn parse(s: &str) -> Result<Pattern, Error> {
    match parse_sequence(s) {
        Ok(("\r\n", p)) => Ok(p),
        Ok(("\n", p)) => Ok(p),
        Ok(("\r", p)) => Ok(p),
        Ok(("", p)) => Ok(p),

        Ok((s, _)) => Err(Error::UnTerminatedError(s.to_string())),
        _ => Err(Error::ParseError),
    }
}

fn parse_sequence(s: &str) -> IResult<&str, Pattern> {
    let mut vec = Vec::new();
    let mut x = s;
    loop {
        match nom::branch::alt((parse_loop, parse_primitive))(x) {
            Ok((s, p)) => {
                x = s;
                vec.push(Box::new(p));
            }
            Err(_) => {
                if vec.len() == 1 {
                    return Ok((x, *vec.pop().unwrap()));
                }
                return Ok((x, Pattern::Sequence(vec)));
            }
        }
    }
}

fn parse_loop(s: &str) -> IResult<&str, Pattern> {
    let (s, pattern) = parse_primitive(s)?;
    let word = match pattern {
        Pattern::Word(w) => *w,
        _ => unreachable!(),
    };

    if let Ok((s, _)) = tag::<&str, &str, (&str, nom::error::ErrorKind)>("*")(s) {
        return Ok((s, Pattern::Loop(Box::new(word), 0, 100)));
    };

    if let Ok((s, _)) = tag::<&str, &str, (&str, nom::error::ErrorKind)>("+")(s) {
        return Ok((s, Pattern::Loop(Box::new(word), 1, 100)));
    };

    let (s, _) = tag("{")(s)?;

    let (s, from) = digit1(s)?;

    let (s, to) = if let Ok((s, _)) = tag::<&str, &str, (&str, nom::error::ErrorKind)>(",")(s) {
        digit1(s)?
    } else {
        (s, from)
    };

    let (s, _) = tag("}")(s)?;

    let from = from.parse().unwrap();
    let to = to.parse().unwrap();

    if from > to {
        // TODO: IResult<&str, Pattern>ではなくて、Result<Pattern, Error>を使う。
        return Err(nom::Err::Error((s, nom::error::ErrorKind::Verify)));
    }

    Ok((s, Pattern::Loop(Box::new(word), from, to)))
}

fn parse_primitive(s: &str) -> IResult<&str, Pattern> {
    let (s, p) = nom::branch::alt((
        parse_digit,
        parse_alphabetic,
        parse_group,
        parse_alt,
        parse_char,
    ))(s)?;
    Ok((s, Pattern::Word(Box::new(p))))
}

fn parse_group(s: &str) -> IResult<&str, Primitive> {
    let (s, _) = tag("(")(s)?;

    let (s, p) = parse_sequence(s)?;

    let (s, _) = tag(")")(s)?;

    Ok((s, Primitive::Group(Box::new(p))))
}

fn parse_alt(s: &str) -> IResult<&str, Primitive> {
    let (s, x) = delimited(char('['), is_not("]"), char(']'))(s)?;

    Ok((s, Primitive::Alt(x.to_string())))
}

fn parse_char(s: &str) -> IResult<&str, Primitive> {
    match nom::character::streaming::anychar(s) {
        Ok((s, ch)) => {
            if reserved(ch) {
                return Err(nom::Err::Error((s, nom::error::ErrorKind::Verify)));
            }
            Ok((s, Primitive::Char(ch)))
        }
        Err(e) => Err(e),
    }
}

fn parse_digit(s: &str) -> IResult<&str, Primitive> {
    let (s, _) = tag("\\b")(s)?;
    Ok((s, Primitive::Digit))
}

fn parse_alphabetic(s: &str) -> IResult<&str, Primitive> {
    let (s, _) = tag("\\w")(s)?;
    Ok((s, Primitive::Alphabetic))
}

fn reserved(ch: char) -> bool {
    match ch {
        '\r' | '\n' | '(' | ')' | '{' | '}' | '[' | ']' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use crate::parser::Error;
    use crate::regex::{Pattern, Primitive};

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("b"),
            Ok(Pattern::Word(Box::new(Primitive::Char('b'))))
        );

        assert_eq!(
            parse("\\"),
            Ok(Pattern::Word(Box::new(Primitive::Char('\\'))))
        );

        assert_eq!(parse("\\b"), Ok(Pattern::Word(Box::new(Primitive::Digit))));
        assert_eq!(
            parse("\\w"),
            Ok(Pattern::Word(Box::new(Primitive::Alphabetic)))
        );

        assert_eq!(
            parse("\\b{1}"),
            Ok(Pattern::Loop(Box::new(Primitive::Digit), 1, 1))
        );
        assert_eq!(
            parse("\\b{10}"),
            Ok(Pattern::Loop(Box::new(Primitive::Digit), 10, 10))
        );
        assert_eq!(
            parse("\\w{10}"),
            Ok(Pattern::Loop(Box::new(Primitive::Alphabetic), 10, 10))
        );
        assert_eq!(
            parse("\\b{1,1}"),
            Ok(Pattern::Loop(Box::new(Primitive::Digit), 1, 1))
        );
        assert_eq!(
            parse("\\b{7,10}"),
            Ok(Pattern::Loop(Box::new(Primitive::Digit), 7, 10))
        );

        assert_eq!(
            parse("\\b{1}\r\n"),
            Ok(Pattern::Loop(Box::new(Primitive::Digit), 1, 1))
        );

        assert_eq!(
            parse("\\b{1}\n"),
            Ok(Pattern::Loop(Box::new(Primitive::Digit), 1, 1))
        );

        assert_eq!(
            parse("\\b{1}\r"),
            Ok(Pattern::Loop(Box::new(Primitive::Digit), 1, 1))
        );
        assert_eq!(
            parse("(\\b)\r"),
            Ok(Pattern::Word(Box::new(Primitive::Group(Box::new(
                Pattern::Word(Box::new(Primitive::Digit))
            )))))
        );
        assert_eq!(
            parse("\\b\\w\r"),
            Ok(Pattern::Sequence(vec![
                Box::new(Pattern::Word(Box::new(Primitive::Digit))),
                Box::new(Pattern::Word(Box::new(Primitive::Alphabetic))),
            ]))
        );
        assert_eq!(
            parse("[123]"),
            Ok(Pattern::Word(Box::new(Primitive::Alt("123".to_string()))))
        );
        assert_eq!(
            parse("[ああ]"),
            Ok(Pattern::Word(Box::new(Primitive::Alt(
                "ああ".to_string()
            ))))
        );
        assert_eq!(
            parse("\\b*\r"),
            Ok(Pattern::Loop(Box::new(Primitive::Digit), 0, 100))
        );
        assert_eq!(
            parse("\\b+\r"),
            Ok(Pattern::Loop(Box::new(Primitive::Digit), 1, 100))
        );
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(
            parse("\\b{"),
            Err(Error::UnTerminatedError("{".to_string()))
        );
        assert_eq!(
            parse("\\b{1,"),
            Err(Error::UnTerminatedError("{1,".to_string()))
        );
        assert_eq!(
            parse("\\w{1,"),
            Err(Error::UnTerminatedError("{1,".to_string()))
        );
    }
    #[test]
    fn test_loop_error() {
        assert_eq!(
            parse("\\b{13,12}"),
            Err(Error::UnTerminatedError("{13,12}".to_string()))
        );
    }
}
