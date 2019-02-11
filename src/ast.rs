use pest::iterators::{Pair, Pairs};
use pest::Parser;

use crate::parser::{RhizParser, Rule};

#[derive(Debug, PartialEq)]
pub enum RhizValue {
    Program(Vec<RhizValue>),
    SExpr(Vec<RhizValue>),
    Symbol(String),
    String(String),
}

fn collect_or_first_error(pairs: Pairs<Rule>) -> Result<Vec<RhizValue>, String> {
    let mut result = Vec::new();
    for p in pairs {
        match parse_value(p) {
            Ok(v) => result.push(v),
            Err(e) => return Err(e),
        }
    }
    Ok(result)
}

fn parse_value(pair: Pair<Rule>) -> Result<RhizValue, String>
where
{
    match pair.as_rule() {
        Rule::program => {
            let exprs = collect_or_first_error(pair.into_inner())?;
            Ok(RhizValue::Program(exprs))
        }
        Rule::sexpr => {
            let exprs = collect_or_first_error(pair.into_inner())?;
            Ok(RhizValue::SExpr(exprs))
        }
        Rule::symbol => {
            let raw = pair.as_str().to_owned();
            Ok(RhizValue::Symbol(raw))
        }
        Rule::string => {
            let raw = pair.as_str();
            // Drop opening and closing " from string source
            let contents = raw[1..raw.len() - 1].to_owned();
            Ok(RhizValue::String(contents))
        }
        _ => unreachable!("{:?}", pair),
    }
}

pub fn parse_rhiz_program(src: &str) -> Result<RhizValue, String> {
    let mut parse_tree =
        RhizParser::parse(Rule::program, src).map_err(|e| format!("Parsing error: {}", e))?;
    let prog = parse_tree.next().expect("Expected a program");
    parse_value(prog)
}

#[test]
fn test_parse_values() {
    let example_src = r#"(Once there was) (a "way" to get "back home""#;
    assert!(parse_rhiz_program(example_src).is_ok())
}
