#[derive(PartialEq, Clone, Debug)]
pub enum Pattern {
    Digit,
    Loop(Box<Pattern>, u64, u64),
}
