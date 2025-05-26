#[derive(Debug, PartialEq)]
pub enum Op {
    Input(String),
    Const(i64),
    Add(String, String),
    Mul(String, String),
    Output(String),
}
