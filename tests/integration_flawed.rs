extern crate hddl_analyzer;

use hddl_analyzer::{HDDLProgram, ParsingError, SemanticErrorType, SyntacticError, WarningType};
use std::fs;

fn verify_flawed_domain(file_name: &str) -> Result<Vec<WarningType>, ParsingError> {
    let domain = fs::read(format!("tests/flawed_domains/{file_name}")).unwrap();
    HDDLProgram::from_hddl(&domain, None).and_then(|program| program.verify())
}

fn expect_semantic_error(file_name: &str, assert_error: impl FnOnce(SemanticErrorType)) {
    match verify_flawed_domain(file_name) {
        Ok(_) => panic!("expected semantic error for {file_name}"),
        Err(ParsingError::Semantic(error)) => assert_error(error),
        Err(error) => panic!("expected semantic error for {file_name}, got {error:?}"),
    }
}

fn expect_syntactic_error(file_name: &str, assert_error: impl FnOnce(SyntacticError)) {
    match verify_flawed_domain(file_name) {
        Ok(_) => panic!("expected syntactic error for {file_name}"),
        Err(ParsingError::Syntactic(error)) => assert_error(error),
        Err(error) => panic!("expected syntactic error for {file_name}, got {error:?}"),
    }
}

fn expect_warnings(file_name: &str) -> Vec<WarningType> {
    match verify_flawed_domain(file_name) {
        Ok(warnings) => warnings,
        Err(error) => panic!("expected warnings for {file_name}, got {error:?}"),
    }
}

#[test]
pub fn cyclic_ordering_validation_test() {
    expect_semantic_error("cyclic-ordering-for-subtasks-domain.hddl", |error| {
        if let SemanticErrorType::CyclicOrderingDeclaration(position) = error {
            assert_eq!(position.line, 56);
        } else {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn cyclic_type_validation_test() {
    expect_semantic_error("directly-cyclic-subtypes-domain.hddl", |error| {
        if !matches!(error, SemanticErrorType::CyclicTypeDeclaration) {
            panic!("wrong error {:?}", error)
        }
    });

    expect_semantic_error("indirectly-cyclic-subtypes-domain.hddl", |error| {
        if !matches!(error, SemanticErrorType::CyclicTypeDeclaration) {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn duplicate_action_validation_test() {
    expect_semantic_error("duplicate-action-domain.hddl", |error| {
        if let SemanticErrorType::DuplicateActionDeclaration(duplicate) = error {
            assert_eq!(
                duplicate.symbol,
                "move_seg_twe1_0_200_seg_twe2_0_50_south_south_medium"
            );
        } else {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn duplicate_task_validation_test() {
    expect_semantic_error("duplicate-compound-task-domain.hddl", |error| {
        if let SemanticErrorType::DuplicateCompoundTaskDeclaration(duplicate) = error {
            assert_eq!(duplicate.symbol, "AchieveSomeGoal");
        } else {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn duplicate_method_validation_test() {
    expect_semantic_error("duplicate-decomposition-method-domain.hddl", |error| {
        if let SemanticErrorType::DuplicateMethodDeclaration(duplicate) = error {
            assert_eq!(duplicate.symbol, "ParkAirplane");
        } else {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn duplicate_predicate_validation_test() {
    expect_semantic_error("duplicate-predicate-domain.hddl", |error| {
        if let SemanticErrorType::DuplicatePredicateDeclaration(duplicate) = error {
            assert_eq!(duplicate.symbol, "at-segment");
        } else {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn duplicate_parameter_validation_test() {
    expect_semantic_error("duplicate-parameters-domain.hddl", |error| {
        if let SemanticErrorType::DuplicateParameterDeclaration(duplicate) = error {
            assert_eq!(duplicate.symbol, "?a");
        } else {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn extra_parentheses_validation_test() {
    expect_syntactic_error("extra-parentheses-domain.hddl", |error| {
        assert_eq!(error.found, "Keyword :effect");
    });
}

#[test]
pub fn forgotten_dash_validation_test() {
    expect_syntactic_error("forgotten-dash-domain.hddl", |error| {
        assert_eq!(error.expected, "a variable name starting with '?'");
        assert_eq!(error.found, "Identifier airplane");
        assert_eq!(error.position.line, 33);
    });
}

#[test]
pub fn forgotten_entry_validation_test() {
    expect_syntactic_error("forgotten-entries-domain.hddl", |error| {
        assert_eq!(error.position.line, 63);
    });
}

#[test]
pub fn forgotten_question_mark_validation_test() {
    expect_syntactic_error("forgotten-question-mark-domain.hddl", |error| {
        assert_eq!(error.expected, "a variable name starting with '?'");
        assert_eq!(error.found, "Identifier s");
        assert_eq!(error.position.line, 35);
    });
}

#[test]
pub fn inconsistent_arity_predicate_validation_test() {
    expect_semantic_error(
        "inconsistent-num-parameters-predicate-domain.hddl",
        |error| {
            if let SemanticErrorType::InconsistentPredicateArity(arity) = error {
                assert_eq!(arity.symbol, "at-segment");
            } else {
                panic!("wrong error {:?}", error)
            }
        },
    );
}

#[test]
pub fn inconsistent_type_predicate_validation_test() {
    expect_semantic_error(
        "inconsistent-type-parameters-predicate-domain.hddl",
        |error| {
            if let SemanticErrorType::InconsistentPredicateArgType(type_error) = error {
                assert_eq!(type_error.var_name, "seg_pp_0_60");
            } else {
                panic!("wrong error {:?}", error)
            }
        },
    );
}

#[test]
pub fn inconsistent_arity_task_validation_test() {
    expect_semantic_error("inconsistent-num-parameters-task-domain.hddl", |error| {
        if let SemanticErrorType::InconsistentTaskArity(arity) = error {
            assert_eq!(
                arity.symbol,
                "move_seg_ppdoor_0_40_seg_tww1_0_200_north_south_medium"
            );
        } else {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn inconsistent_type_task_validation_test() {
    expect_semantic_error("inconsistent-type-parameters-task-domain.hddl", |error| {
        if let SemanticErrorType::InconsistentTaskArgType(type_error) = error {
            assert_eq!(type_error.var_name, "?a_0");
        } else {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn undeclared_method_param_validation_test() {
    expect_semantic_error("undeclared-method-parameter-domain.hddl", |error| {
        if let SemanticErrorType::UndefinedParameter(undefined) = error {
            assert_eq!(undefined.symbol, "?d");
        } else {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn undeclared_task_param_validation_test() {
    expect_semantic_error("undeclared-task-parameter-domain.hddl", |error| {
        if let SemanticErrorType::UndefinedParameter(undefined) = error {
            assert_eq!(undefined.symbol, "?s");
        } else {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn undeclared_predicate_validation_test() {
    expect_semantic_error("undefined-predicate-domain.hddl", |error| {
        if let SemanticErrorType::UndefinedPredicate(undefined) = error {
            assert_eq!(undefined.symbol, "occupied");
        } else {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn undeclared_task_validation_test() {
    expect_semantic_error("undefined-task-domain.hddl", |error| {
        if let SemanticErrorType::UndefinedSubtask(undefined) = error {
            assert_eq!(undefined.symbol, "undefined_task");
        } else {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn undeclared_type_validation_test() {
    expect_semantic_error("undefined-type-domain.hddl", |error| {
        if let SemanticErrorType::UndefinedType(undefined) = error {
            assert_eq!(undefined.symbol, "airplane");
        } else {
            panic!("wrong error {:?}", error)
        }
    });
}

#[test]
pub fn no_primitive_refinement_validation_test() {
    let warnings = expect_warnings("abstract-task-without-refinement-domain.hddl");

    assert_eq!(warnings.len(), 1);
    match &warnings[0] {
        WarningType::NoPrimitiveRefinement(info) => {
            assert_eq!(info.symbol, "AchieveSomeGoal")
        }
        warning => panic!("wrong warning {:?}", warning),
    }
}

#[test]
pub fn no_method_validation_test() {
    let warnings = expect_warnings("abstract-task-without-decomposition-domain.hddl");

    assert_eq!(warnings.len(), 1);
    match &warnings[0] {
        WarningType::NoPrimitiveRefinement(info) => {
            assert_eq!(info.symbol, "AchieveSomeGoal")
        }
        warning => panic!("wrong warning {:?}", warning),
    }
}

#[test]
pub fn ignore_possibly_complementary_effects_validation_test() {
    verify_flawed_domain("possible-complementary-effects-domain.hddl").unwrap();
}
