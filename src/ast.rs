//! Abstract syntax tree for a Rhiz file.
use pest::iterators::{Pair, Pairs};
use pest::Parser;

use crate::parser::{RhizParser, Rule};

/// Elements of a Rhizfile.
#[derive(Debug, PartialEq)]
pub enum RhizValue {
    Program(Vec<RhizValue>),
    SExpr(Vec<RhizValue>),
    Symbol(String),
    String(String),
}

impl std::convert::From<&RhizValue> for String {
    fn from(v: &RhizValue) -> String {
        match v {
            RhizValue::String(s) => format!("\"{}\"", s),
            RhizValue::Symbol(s) => s.to_owned(),
            RhizValue::SExpr(contents) => {
                let mut outp = String::new();
                let items = contents.iter();
                for i in items {
                    let s: String = i.into();
                    outp.push_str(&s);
                    outp.push(' ');
                }
                outp
            }
            RhizValue::Program(sexprs) => {
                let mut outp = String::new();
                let items = sexprs.iter();
                for i in items {
                    let s: String = i.into();
                    outp.push_str(&s);
                    outp.push(' ');
                }
                outp
            }
        }
    }
}

/// Convert
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

/// Extract a (possibly nested) `RhizValue` from a Pest parser pair.
fn parse_value(pair: Pair<Rule>) -> Result<RhizValue, String> {
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
        RhizParser::parse(Rule::file, src).map_err(|e| format!("Parsing error: {}", e))?;
    let prog = parse_tree.next().expect("Expected a program");
    parse_value(prog)
}

#[test]
fn test_parse_values() {
    let example_src = r#"(Once there was) (a "way" to get "back home")"#;
    let expected = RhizValue::Program(vec![
        RhizValue::SExpr(vec![
            RhizValue::Symbol("Once".to_owned()),
            RhizValue::Symbol("there".to_owned()),
            RhizValue::Symbol("was".to_owned()),
        ]),
        RhizValue::SExpr(vec![
            RhizValue::Symbol("a".to_owned()),
            RhizValue::String("way".to_owned()),
            RhizValue::Symbol("to".to_owned()),
            RhizValue::Symbol("get".to_owned()),
            RhizValue::String("back home".to_owned()),
        ]),
    ]);
    debug_assert_eq!(
        parse_rhiz_program(example_src).expect("Failed to prase example program"),
        expected
    );
}
