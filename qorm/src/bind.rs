#[derive(Clone, Debug, PartialEq)]
pub enum Bind {
    Null,
    String(String),
    Int(i32),
    Bool(bool),
    Raw(String),
}
