#[derive(PartialEq, Clone, Debug)]
pub enum Primitive {
    Digit,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Pattern {
    Word(Box<Primitive>),
    Loop(Box<Primitive>, u64, u64),
}
