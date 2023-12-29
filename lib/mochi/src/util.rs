use std::error::Error;

use pest::iterators::Pair;

use crate::{error::ProgrammerError, Rule};

pub fn unwrap_rule_as_token(opt_rule: Option<Pair<'_, Rule>>, rule_type: Rule) -> Result<Pair<'_, Rule>, Box<dyn std::error::Error>> {
    if opt_rule.is_none() {
        return Err(Box::new(ProgrammerError));
    }
    let rule = opt_rule.unwrap();
    if rule.as_rule() != rule_type {
        return Err(Box::new(ProgrammerError));
    }
    return Ok(rule);
}

pub fn unwrap_rule_as_token_or_err<E: Error>(opt_rule: Option<Pair<'_, Rule>>, rule_type: Rule, error: E) -> Result<Pair<'_, Rule>, Box<dyn std::error::Error>> where E: 'static {
    if opt_rule.is_none() {
        return Err(Box::new(ProgrammerError));
    }
    let rule = opt_rule.unwrap();
    if rule.as_rule() != rule_type {
        return Err(Box::new(error));
    }
    return Ok(rule);
}