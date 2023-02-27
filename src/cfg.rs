mod cnf;
mod parser;

use lazy_static::lazy_static;
use regex::Regex;

const BLANK: &str = ".";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CFGRule {
    lhs: String,
    rhs: Vec<String>,
}

pub struct CFG {
    start_symbol: String,
    rules: Vec<CFGRule>,
}

/**
* Returns whether a string represents a SINGLE nonterminal.
*
* A nonterminal consists of an uppercase character potentiall followed by
*/
pub fn is_nonterminal(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[A-Z](_[A-Za-z0-9,]+)*$").unwrap();
    }
    RE.is_match(s)
}
