extern crate hddl_analyzer;

use hddl_analyzer::{HDDLProgram, ParsingError, SemanticErrorType, Transpiler};
use serde_json::Value;
use std::fs;

mod common;

#[test]
pub fn json_round_trip_single_domain() {
    let domain = fs::read("tests/ipc/Blocksworld-GTOHP/domain.hddl").unwrap();
    let program = HDDLProgram::from_hddl(&domain, None).unwrap();
    let exported = Transpiler::new(program).to_json();
    let reimported = HDDLProgram::from_json(&exported).unwrap();
    let re_exported = Transpiler::new(reimported).to_json();
    assert_eq!(exported, re_exported);
}

#[test]
pub fn json_round_trip_ipc() {
    for case in common::fast_corpus_cases().expect("fast corpus selection should be valid") {
        let domain = fs::read(&case.domain_path)
            .unwrap_or_else(|error| panic!("{}: failed to read domain: {error}", case.id));
        let problem = fs::read(&case.problem_path)
            .unwrap_or_else(|error| panic!("{}: failed to read problem: {error}", case.id));
        let program = HDDLProgram::from_hddl(&domain, Some(&problem))
            .unwrap_or_else(|error| panic!("{}: HDDL parse error: {:?}", case.id, error));
        let exported = Transpiler::new(program).to_json();
        let reimported = HDDLProgram::from_json(&exported)
            .unwrap_or_else(|error| panic!("{}: JSON import error: {:?}", case.id, error));
        reimported.verify().unwrap_or_else(|error| {
            panic!("{}: reimport verification error: {:?}", case.id, error)
        });
        let re_exported = Transpiler::new(reimported).to_json();
        let exported_value = serde_json::from_str::<Value>(&exported)
            .unwrap_or_else(|error| panic!("{}: exported JSON parse error: {error}", case.id));
        let re_exported_value = serde_json::from_str::<Value>(&re_exported)
            .unwrap_or_else(|error| panic!("{}: re-exported JSON parse error: {error}", case.id));
        let exact_equal = exported == re_exported;
        let structural_equal = exported_value == re_exported_value;

        assert!(
            exact_equal && structural_equal,
            "{}: JSON round trip mismatch: exact_equal={exact_equal}, structural_equal={structural_equal}",
            case.id
        );
    }
}

#[test]
pub fn json_round_trip_metadata() {
    let domain = fs::read("tests/ipc/Blocksworld-GTOHP/domain.hddl").unwrap();
    let program = HDDLProgram::from_hddl(&domain, None).unwrap();
    let original = program.metadata().unwrap();
    let exported = Transpiler::new(program).to_json();
    let reimported = HDDLProgram::from_json(&exported).unwrap();
    let recovered = reimported.metadata().unwrap();
    assert_eq!(original.domain_name, recovered.domain_name);
    assert_eq!(original.n_actions, recovered.n_actions);
    assert_eq!(original.n_tasks, recovered.n_tasks);
    assert_eq!(original.n_methods, recovered.n_methods);
    assert_eq!(original.recursion, recovered.recursion);
    // nullables are collected from a set, so their order is nondeterministic
    let mut original_nullables = original.nullables.clone();
    let mut recovered_nullables = recovered.nullables.clone();
    original_nullables.sort();
    recovered_nullables.sort();
    assert_eq!(original_nullables, recovered_nullables);
}

#[test]
pub fn json_minimal_handwritten() {
    let json = r#"{
        "domain": {
            "name": "minimal",
            "requirements": [],
            "predicates": [],
            "compound_tasks": [],
            "methods": [],
            "actions": [{"name": "noop", "parameters": []}],
            "functions": []
        }
    }"#;
    let program = HDDLProgram::from_json(json).unwrap();
    assert_eq!(program.domain.actions.len(), 1);
    assert_eq!(program.domain.actions[0].name, "noop");
    // omitted positions default to the "synthesized" sentinel, line 0
    assert_eq!(program.domain.actions[0].name_pos.line, 0);
    assert!(program.problem.is_none());
    program.verify().unwrap();
}

#[test]
pub fn json_symbol_type_without_type_pos_no_panic() {
    // "vehicle" is not a declared type, and the variable carries no type_pos;
    let json = r#"{
        "domain": {
            "name": "d",
            "requirements": [],
            "predicates": [{
                "name": "at",
                "variables": [{"name": "v", "symbol_type": "vehicle"}]
            }],
            "compound_tasks": [],
            "methods": [],
            "actions": [],
            "functions": []
        }
    }"#;
    let program = HDDLProgram::from_json(json).unwrap();
    match program.verify() {
        Err(ParsingError::Semantic(SemanticErrorType::UndefinedType(undefined))) => {
            assert_eq!(undefined.symbol, "vehicle");
            assert_eq!(undefined.position.line, 0);
        }
        other => panic!("expected an UndefinedType error, got {:?}", other),
    }
}

#[test]
pub fn json_cyclic_ordering_no_pos_domain_no_panic() {
    let json = r#"{
        "domain": {
            "name": "d",
            "requirements": [],
            "predicates": [],
            "compound_tasks": [{"name": "ct", "parameters": []}],
            "methods": [{
                "name": {"name": "m"},
                "task": {"name": "ct"},
                "task_terms": [],
                "params": [],
                "tn": {
                    "subtasks": [
                        {"id": {"name": "t1"}, "task": {"name": "a1"}, "terms": []},
                        {"id": {"name": "t2"}, "task": {"name": "a1"}, "terms": []}
                    ],
                    "orderings": {"Partial": [["t1", "t2"], ["t2", "t1"]]}
                }
            }],
            "actions": [{"name": "a1", "parameters": []}],
            "functions": []
        }
    }"#;
    let program = HDDLProgram::from_json(json).unwrap();
    match program.verify() {
        Err(ParsingError::Semantic(SemanticErrorType::CyclicOrderingDeclaration(pos))) => {
            assert_eq!(pos.line, 0);
        }
        other => panic!(
            "expected a CyclicOrderingDeclaration error, got {:?}",
            other
        ),
    }
}

#[test]
pub fn json_cyclic_ordering_no_pos_problem_no_panic() {
    let json = r#"{
        "domain": {
            "name": "d",
            "requirements": [],
            "predicates": [],
            "compound_tasks": [],
            "methods": [],
            "actions": [{"name": "a1", "parameters": []}],
            "functions": []
        },
        "problem": {
            "requirements": [],
            "init_tn": {
                "tn": {
                    "subtasks": [
                        {"id": {"name": "t1"}, "task": {"name": "a1"}, "terms": []},
                        {"id": {"name": "t2"}, "task": {"name": "a1"}, "terms": []}
                    ],
                    "orderings": {"Partial": [["t1", "t2"], ["t2", "t1"]]}
                }
            },
            "init_state": [],
            "objects": []
        }
    }"#;
    let program = HDDLProgram::from_json(json).unwrap();
    match program.verify() {
        Err(ParsingError::Semantic(SemanticErrorType::CyclicOrderingDeclaration(pos))) => {
            assert_eq!(pos.line, 0);
        }
        other => panic!(
            "expected a CyclicOrderingDeclaration error, got {:?}",
            other
        ),
    }
}

#[test]
pub fn json_malformed_input() {
    match HDDLProgram::from_json("{ not json") {
        Err(ParsingError::JSON(error)) => {
            assert!(!error.to_string().is_empty());
            assert!(error.line > 0);
        }
        other => panic!("expected a Json error, got {:?}", other.map(|_| ())),
    }
}

#[test]
pub fn json_wrong_shape() {
    assert!(matches!(
        HDDLProgram::from_json(r#"{"domain": []}"#),
        Err(ParsingError::JSON(_))
    ));
    let unknown_formula_variant = r#"{
        "domain": {
            "name": "d",
            "requirements": [],
            "predicates": [],
            "compound_tasks": [],
            "methods": [],
            "actions": [{"name": "a", "parameters": [], "preconditions": {"Nand": []}}],
            "functions": []
        }
    }"#;
    assert!(matches!(
        HDDLProgram::from_json(unknown_formula_variant),
        Err(ParsingError::JSON(_))
    ));
}
