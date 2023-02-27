use std::collections::{HashMap, HashSet, VecDeque};

use super::{is_nonterminal, CFGRule, BLANK, CFG};
enum CNF_RHS {
    NonTerminals(String, String),
    Terminal(String),
}

pub struct CNFRule {
    lhs: String,
    rhs: CNF_RHS,
}

pub struct CNF {
    rules: Vec<CNFRule>,
    start_symbol: String,
}

impl CFG {
    pub fn to_cnf(&self) -> Option<CNF> {
        // make sure start symbol doesn't appear on the right side of a string
        let new_start = self.start_symbol.clone() + "_newstart";
        let mut rules = self.rules.clone();
        rules.push(CFGRule {
            lhs: new_start.clone(),
            rhs: vec![self.start_symbol.clone()],
        });

        // remove all rules of the form A -> `BLANK`
        rules = remove_blank_rhs(rules, new_start.as_str());
        rules = remove_one_length_rules(rules);
        rules = remove_long_rules(rules);
        rules = replace_nonterminals(rules);

        let cnf_rules = rules.into_iter().map(make_cnf_rule).collect::<Vec<_>>();
        if cnf_rules.iter().any(|i| i.is_none()) {
            None
        } else {
            Some(CNF {
                start_symbol: new_start,
                rules: cnf_rules.into_iter().map(|i| i.unwrap()).collect(),
            })
        }
    }
}

fn remove_blank_rhs(rules: Vec<CFGRule>, start: &str) -> Vec<CFGRule> {
    let (valid, invalid): (Vec<_>, Vec<_>) = rules.iter().partition(|rule| {
        rule.lhs != start && rule.rhs.len() == 1 && rule.rhs.get(0).unwrap() == BLANK
    });

    let mut final_rules: Vec<CFGRule> = valid.into_iter().map(|i| i.clone()).collect();

    let invalid_nonterminals: HashSet<String> =
        invalid.into_iter().map(|i| i.lhs.clone()).collect();

    for bad_symbol in invalid_nonterminals {
        let parent_rules: Vec<&CFGRule> = rules
            .iter()
            .filter(|r| r.rhs.contains(&bad_symbol))
            .collect();
        for parent in parent_rules {
            let indices = parent
                .rhs
                .iter()
                .enumerate()
                .filter(|(_, r)| bad_symbol == r.to_string())
                .map(|(index, _)| index);

            for index in indices {
                let mut new_rhs = parent.rhs.clone();
                new_rhs.remove(index);
                if new_rhs.len() == 0 {
                    continue;
                }

                let new_rule = CFGRule {
                    lhs: parent.lhs.clone(),
                    rhs: new_rhs,
                };
                final_rules.push(new_rule);
            }
        }
    }

    let used_lhs_symbols: HashSet<String> = final_rules.iter().map(|i| i.lhs.clone()).collect();

    final_rules
        .into_iter()
        .filter(|rule| {
            rule.rhs
                .iter()
                .all(|symbol| !is_nonterminal(symbol) || used_lhs_symbols.contains(symbol))
        })
        .collect()
}

/**
* Remove rules of the form A -> B
*/
fn remove_one_length_rules(rules: Vec<CFGRule>) -> Vec<CFGRule> {
    // delete all A -> B
    let (mut final_rules, one_length_rules): (Vec<_>, Vec<_>) =
        rules.into_iter().partition(|i| is_one_length_rule(i));

    // For old A -> B, for all rules B -> w, add A -> w unless already removed
    for one_length_rule in one_length_rules {
        let lhs = one_length_rule.lhs;
        let rhs = one_length_rule.rhs.get(0).unwrap().to_owned();

        let new_rules: Vec<_> = final_rules
            .iter()
            .filter(|r| r.lhs == rhs)
            .map(|r| CFGRule {
                lhs: lhs.clone(),
                rhs: r.rhs.clone(),
            })
            .collect();
        final_rules.extend(new_rules);
    }

    final_rules
}

fn is_one_length_rule(i: &CFGRule) -> bool {
    i.rhs.len() == 1 && is_nonterminal(i.rhs.get(0).unwrap())
}

fn remove_long_rules(rules: Vec<CFGRule>) -> Vec<CFGRule> {
    let (mut final_rules, long_rules): (Vec<_>, Vec<_>) =
        rules.into_iter().partition(|r| r.rhs.len() > 2);

    let mut shortened_rules: Vec<_> = long_rules
        .into_iter()
        .enumerate()
        .flat_map(|(rule_idx, long_rule)| {
            let mut idx = 0;
            let mut rhs_temp: VecDeque<String> = long_rule.rhs.clone().into_iter().collect();
            let mut new_rules = Vec::<CFGRule>::new();
            let mut suffix = "".to_string();
            while rhs_temp.len() > 2 {
                let next_suffix = format!("_extra{},{}", rule_idx, idx);
                let first_char = rhs_temp.pop_front().unwrap();
                let rule = CFGRule {
                    lhs: long_rule.lhs.clone() + &suffix,
                    rhs: vec![first_char, long_rule.lhs.clone() + &next_suffix],
                };
                new_rules.push(rule);
                suffix = next_suffix;
                idx += 1;
            }
            return new_rules;
        })
        .collect();

    final_rules.append(&mut shortened_rules);
    return final_rules;
}

fn replace_nonterminals(mut rules: Vec<CFGRule>) -> Vec<CFGRule> {
    let mut symbols_map = HashMap::<String, String>::new();
    for r in rules.iter_mut() {
        if r.rhs.len() == 1 {
            continue;
        }
        for item in r.rhs.iter_mut() {
            if is_nonterminal(item) {
                continue;
            }
            match symbols_map.get(item.as_str()) {
                Some(nonterm) => {
                    *item = nonterm.clone();
                }
                None => {
                    let key = item.clone();
                    let x = symbols_map
                        .insert(key, format!("U_extra,{}", item))
                        .unwrap();
                    *item = x;
                }
            }
        }
    }
    let mut symbol_rules: Vec<CFGRule> = symbols_map
        .into_iter()
        .map(|(k, v)| CFGRule {
            lhs: v,
            rhs: vec![k],
        })
        .collect();
    rules.append(&mut symbol_rules);
    return rules;
}

fn make_cnf_rule(r: CFGRule) -> Option<CNFRule> {
    match r.rhs.len() {
        1 => Some(CNFRule {
            lhs: r.lhs,
            rhs: CNF_RHS::Terminal(r.rhs.get(0).unwrap().to_owned()),
        }),
        2 => Some(CNFRule {
            lhs: r.lhs,
            rhs: CNF_RHS::NonTerminals(
                r.rhs.get(0).unwrap().to_owned(),
                r.rhs.get(1).unwrap().to_owned(),
            ),
        }),
        _ => None,
    }
}
