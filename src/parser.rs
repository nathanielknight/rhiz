// Need to import this for deriving
#[allow(unused_imports)]
use pest::Parser;

#[derive(Parser)]
#[grammar = "rhiz.pest"]
pub struct RhizParser;

#[test]
fn test_parse_empty_sexpr() {
    let src = "()";
    RhizParser::parse(Rule::sexpr, src).expect("Expected to parse an empty sexpr");
}

#[test]
fn test_parse_symbol() {
    let ok_cases = ["asdf", "JklL)", "x1 ", "kebab-case\n", "-l"];
    for case in &ok_cases {
        let msg = format!("Expected '{}' to parse to a symbol", case);
        RhizParser::parse(Rule::symbol, case).expect(&msg);
    }
}

#[test]
fn test_parse_sexpr() {
    let ok_cases = ["()", "(())", "(asdf jkl)", "(asdf (jkl semi) colon)"];
    for case in &ok_cases {
        let msg = format!("Expected '{}' to parse to a sexpr", case);
        RhizParser::parse(Rule::sexpr, case).expect(&msg);
    }
    let err_cases = ["", "(", "(()", "asdf"];
    for case in &err_cases {
        let msg = format!("Expected '{}' to not parse to a sexpr", &case);
        let parsed = RhizParser::parse(Rule::sexpr, case);
        assert!(parsed.is_err(), msg);
    }
}

#[test]
fn test_parse_program() {
    let ok_cases = ["", "()", "()()", "(())", "()\n()\n"];
    for case in &ok_cases {
        let msg = format!("Expected '{}' to parse to a program", case);
        RhizParser::parse(Rule::file, case).expect(&msg);
    }
    let err_cases = ["(()", "())", "asdf()", "()asdf"];
    for case in &err_cases {
        let msg = format!("Expected '{}' to not parse to a program", case);
        let parsed = RhizParser::parse(Rule::file, case);
        assert!(parsed.is_err(), msg);
    }
}
