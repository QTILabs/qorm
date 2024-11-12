use crate::Bind;

#[derive(Clone)]
pub struct Or<'a> {
    pub column: &'a str,
    pub operator: &'a str,
    pub value: Bind,
}
