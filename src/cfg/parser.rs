use super::{is_nonterminal, CFGRule};
/**
* A module to parse strings as CFG rules
*/
pub fn parse(s: &str) -> Option<Vec<CFGRule>> {
    let (lhs, rhs_entire) = s.split_once("->").map(|p| {
        let (x, y) = p;
        (x.trim(), y.trim())
    })?;

    if !is_nonterminal(lhs) {
        return None;
    }

    let rhs_rules = rhs_entire.split("|").map(|rule| rule.trim());

    let rhs_rules_split: Vec<Vec<String>> = rhs_rules
        .map(tokenize_rule)
        // .filter(|x| x.is_some())
        // .map(|x| x.unwrap())
        .collect();

    Some(
        rhs_rules_split
            .into_iter()
            .map(|rhs| CFGRule {
                lhs: String::from(lhs),
                rhs,
            })
            .collect::<Vec<CFGRule>>(),
    )
}

fn tokenize_rule(rule_str: &str) -> Vec<String> {
    let items = rule_str.split(" ");
    let mut elements = Vec::<String>::new();

    for item in items {
        if is_nonterminal(item) {
            elements.push(String::from(item));
        } else {
            let mut strs: Vec<String> = item.chars().map(|i| String::from(i)).collect();
            elements.append(&mut strs);
        }
    }

    elements
}

#[cfg(test)]
mod test_parse {
    use super::*;

    #[test]
    fn test_tokenize() {
        let s = "A010 U_1";
        let tokenized = tokenize_rule(s);
        let expected = vec!["A", "0", "1", "0", "U_1"];
        assert_eq!(tokenized, expected);
    }

    #[test]
    fn test_parse() {
        let rule_str = "A -> 012 U_0 1324 | AB";
        let expected = vec![
            CFGRule {
                lhs: String::from("A"),
                rhs: vec!["0", "1", "2", "U_0", "1", "3", "2", "4"]
                    .iter()
                    .map(|i| i.to_string())
                    .collect(),
            },
            CFGRule {
                lhs: "A".to_string(),
                rhs: vec!["A".to_string(), "B".to_string()],
            },
        ];
        let parsed = parse(rule_str);
        assert!(parsed.is_some());
        assert_eq!(parsed.unwrap(), expected);
    }
}
