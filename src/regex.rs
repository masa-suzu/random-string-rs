#[derive(PartialEq, Clone, Debug)]
pub enum Primitive {
    Digit,
    Loop(Box<Primitive>, u64),
}
