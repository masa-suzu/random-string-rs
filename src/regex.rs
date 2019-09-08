#[derive(PartialEq, Clone, Debug)]
pub enum Primitive {
    Char(char),
    Digit,
    Alphabetic,
    Group(Box<Pattern>),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Pattern {
    Word(Box<Primitive>),
    Loop(Box<Primitive>, u64, u64),
    Sequence(Vec<Box<Pattern>>),
}
