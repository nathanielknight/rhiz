use pest::Parser;

#[derive(Parser)]
#[grammar = "rhiz.pest"]
pub struct RhizParseTree;

#[test]
fn test_parse_empty_sexpr() {
    let src = "()";
    RhizParseTree::parse(Rule::sexpr, src).expect("Expected to parse an empty sexpr");
}

#[test]
fn test_parse_symbol() {
    let ok_cases = ["asdf ", "JklL)", "x1 ", "kebab-case\n"];
    for case in &ok_cases {
        let msg = format!("Expected '{}' to parse to a symbol", case);
        RhizParseTree::parse(Rule::symbol, case).expect(&msg);
    }
    let err_cases = ["1x", "asdf\"jkl", "no_underscores"];
    for case in &err_cases {
        match RhizParseTree::parse(Rule::symbol, case) {
            Err(_) => continue, // because we expected an error
            Ok(parsed) => {
                let msg = format!(
                    "Unexpected successful parse of symbol: {} ({:?})",
                    case, parsed
                );
                panic!(msg);
            }
        };
    }
}

#[test]
fn test_parse_sexpr() {
    let ok_cases = ["()", "(())", "(asdf jkl)", "(asdf (jkl semi) colon)"];
    for case in &ok_cases {
        let msg = format!("Expected '{}' to parse to a sexpr", case);
        RhizParseTree::parse(Rule::sexpr, case).expect(&msg);
    }
    let err_cases = ["", "(", "(()", "asdf"];
    for case in &err_cases {
        let msg = format!("Expected '{}' to not parse to a sexpr", &case);
        let parsed = RhizParseTree::parse(Rule::sexpr, case);
        assert!(parsed.is_err(), msg);
    }
}

#[test]
fn test_parse_program() {
    let ok_cases = ["", "()", "()()", "(())", "()\n()\n"];
    for case in &ok_cases {
        let msg = format!("Expected '{}' to parse to a program", case);
        RhizParseTree::parse(Rule::program, case).expect(&msg);
    }
    let err_cases = ["(()", "())", "asdf()", "()asdf"];
    for case in &err_cases {
        let msg = format!("Expected '{}' to not parse to a program", case);
        let parsed = RhizParseTree::parse(Rule::program, case);
        assert!(parsed.is_err(), msg);
    }
}
