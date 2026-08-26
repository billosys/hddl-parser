use std::fs;
use std::panic;

use hddl_analyzer::{
    Formula, HDDLProgram, Input, LexicalAnalyzer, LexicalError, LexicalErrorType, NumberType,
    Parser, ParsingError, Predicate, TokenPosition, Transformation, Transpiler,
};

fn ipc_domain() -> Vec<u8> {
    fs::read("tests/ipc/Blocksworld-GTOHP/domain.hddl").unwrap()
}

fn ipc_problem() -> Vec<u8> {
    fs::read("tests/ipc/Blocksworld-GTOHP/p01.hddl").unwrap()
}

fn expect_transformation_error<T>(result: Result<T, ParsingError>) -> String {
    match result {
        Err(ParsingError::Transformation(message)) => message,
        Err(other) => panic!("expected transformation error, got {other:?}"),
        Ok(_) => panic!("expected transformation error"),
    }
}

fn atom(name: &str) -> Box<Formula<'_>> {
    Box::new(Formula::Atom(Predicate::new_dummy(name)))
}

#[test]
fn vec_backed_hddl_program_api_parses_domain_and_problem() {
    let domain = ipc_domain();
    let problem = ipc_problem();

    let program = HDDLProgram::from_hddl(&domain, Some(&problem)).unwrap();
    let metadata = program.metadata().unwrap();

    assert_eq!(metadata.domain_name, "BLOCKS");
    assert_eq!(metadata.n_actions, 5);
    assert_eq!(metadata.n_tasks, 4);
}

#[test]
fn byte_slice_hddl_program_api_parses_domain_and_problem() {
    let domain = ipc_domain();
    let problem = ipc_problem();

    let program = HDDLProgram::from_hddl(domain.as_slice(), Some(problem.as_slice())).unwrap();
    let metadata = program.metadata().unwrap();

    assert_eq!(metadata.domain_name, "BLOCKS");
    assert_eq!(metadata.n_actions, 5);
    assert_eq!(metadata.n_tasks, 4);
}

#[test]
fn vec_backed_transpiler_and_input_hddl_apis_parse_domain_and_problem() {
    let domain = ipc_domain();
    let problem = ipc_problem();

    let direct = Transpiler::from_hddl(&domain, Some(&problem)).unwrap();
    let from_input = Transpiler::from_input(Input::Hddl {
        domain: &domain,
        problem: Some(&problem),
    })
    .unwrap();

    assert_eq!(direct.metadata().unwrap().domain_name, "BLOCKS");
    assert_eq!(from_input.metadata().unwrap().domain_name, "BLOCKS");
}

#[test]
fn byte_slice_transpiler_and_input_hddl_apis_parse_domain_and_problem() {
    let domain = ipc_domain();
    let problem = ipc_problem();

    let direct = Transpiler::from_hddl(domain.as_slice(), Some(problem.as_slice())).unwrap();
    let from_input = Transpiler::from_input(Input::Hddl {
        domain: domain.as_slice(),
        problem: Some(problem.as_slice()),
    })
    .unwrap();

    assert_eq!(direct.metadata().unwrap().domain_name, "BLOCKS");
    assert_eq!(from_input.metadata().unwrap().domain_name, "BLOCKS");
}

#[test]
fn representative_crate_root_exports_are_importable() {
    let source = b"(define (domain import-smoke))".to_vec();
    let lexer = LexicalAnalyzer::new(&source);
    let parser = Parser::new(lexer);
    let formula = Formula::Atom(Predicate::new_dummy("reachable"));

    assert!(parser.parse().is_ok());
    assert_eq!(formula.to_string(), "(reachable)");
}

#[test]
fn malformed_problem_top_level_token_returns_syntactic_error() {
    let problem = b"(define (problem malformed) (:domain d) unexpected)".to_vec();

    let result = panic::catch_unwind(|| HDDLProgram::from_hddl(&problem, None));

    let error = match result.expect("malformed problem should return an error instead of panicking")
    {
        Err(error) => error,
        Ok(_) => panic!("malformed problem should not parse successfully"),
    };

    match error {
        ParsingError::Syntactic(error) => {
            assert_eq!(
                error.expected,
                "either ')' to close the definition of malformed, or '(' to start defining new components"
            );
            assert_eq!(error.found, "Identifier unexpected");
        }
        other => panic!("expected syntactic error, got {other:?}"),
    }
}

#[test]
fn current_transformation_error_variant_and_messages_are_pinned() {
    let domain = ipc_domain();
    let problem = ipc_problem();

    let problem_as_domain = expect_transformation_error(HDDLProgram::from_hddl(&problem, None));
    assert_eq!(problem_as_domain, "expected domain input, found problem");

    let domain_as_problem =
        expect_transformation_error(HDDLProgram::from_hddl(&domain, Some(&domain)));
    assert_eq!(domain_as_problem, "expected problem input, found domain");

    let remove_equality_domain_only = expect_transformation_error(
        Transpiler::from_hddl(&domain, None)
            .unwrap()
            .transform(Transformation::RemoveEqualityConstraints),
    );
    assert_eq!(
        remove_equality_domain_only,
        "remove-equality-constraints requires a problem input"
    );
}

#[test]
fn current_public_misspelled_variants_are_available() {
    let lexical_error = LexicalError {
        error_type: LexicalErrorType::InvalidKeyword,
        lexeme: ":bad-keyword".to_string(),
        position: TokenPosition { line: 42 },
    };
    let parsing_error = ParsingError::Lexiacal(lexical_error);
    let transformation = Transformation::QuantifierElimintation;

    assert!(parsing_error.to_string().contains(":bad-keyword"));
    assert!(matches!(
        transformation,
        Transformation::QuantifierElimintation
    ));
}

#[test]
fn formula_to_dnf_currently_panics_on_equality_literals() {
    let formula = Formula::Equals("?x", "?y");

    let result = panic::catch_unwind(|| formula.to_dnf());

    assert!(result.is_err());
}

#[test]
fn formula_to_dnf_currently_panics_on_non_nnf_negation() {
    let formula = Formula::Not(Box::new(Formula::Equals("?x", "?y")));

    let result = panic::catch_unwind(|| formula.to_dnf());

    assert!(result.is_err());
}

#[test]
fn formula_to_nnf_currently_panics_on_probabilistic_formulae() {
    let formula = Formula::Probabilistic(NumberType::Real(0.5), atom("p"));

    let result = panic::catch_unwind(|| formula.to_nnf(false));

    assert!(result.is_err());
}
