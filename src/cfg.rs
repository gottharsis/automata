mod parser;

#[derive(Debug, PartialEq, Eq)]
pub struct CFGRule {
    lhs: String,
    rhs: Vec<String>,
}
