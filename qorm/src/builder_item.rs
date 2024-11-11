use crate::bind::Bind;

pub struct WhereAndQuery {
    pub column: String,
    pub operator: String,
    pub value: Bind,
}
