use core::panic;
use std::collections::{HashMap, HashSet};
use std::vec;

use crate::semantic_analyzer::TypeChecker;
use crate::{Formula, HDDLProgram, Predicate, RequirementType, Symbol, TokenPosition};

pub fn remove_types<'a>(program: &mut HDDLProgram<'a>) {
    if program.domain.types.is_none() {
        return;
    }
    // util for creating predicate calls
    let type_atom = |ty, name| {
        Predicate::new(
            ty,
            TokenPosition::default(),
            vec![Symbol::new(name, TokenPosition::default(), None, None)],
        )
    };
    let checker = TypeChecker::new(&program.domain.types);
    // Add types as predicates
    for type_name in checker.get_types() {
        let already_declared = program
            .domain
            .predicates
            .iter()
            .any(|predicate| predicate.name == type_name);
        if !already_declared {
            program.domain.add_predicate(Predicate::new(
                type_name,
                TokenPosition::default(),
                vec![Symbol::new("?var", TokenPosition::default(), None, None)],
            ));
        } else {
            panic!("unable to convert. {} is already defined.", type_name)
        }
    }
    // Drop typings
    program.domain.types = None;
    program
        .domain
        .requirements
        .retain(|x| *x != RequirementType::TypedObjects);
    // convert typed domain constants to untyped
    let mut init = Vec::new();
    if let Some(constants) = &mut program.domain.constants {
        match &mut program.problem {
            Some(problem) => {
                for constant in constants.iter_mut() {
                    constant.type_pos = None;
                    let name = constant.name;
                    let Some(t) = constant.symbol_type.take() else {
                        continue;
                    };
                    init.push(type_atom(t, name));
                    init.extend(
                        std::iter::once(t)
                            .chain(checker.get_supertypes(t))
                            .map(|ty| type_atom(ty, name)),
                    );
                }
            }
            None => {
                let untyped_constants = constants.iter().all(|x| x.symbol_type.is_none());
                if !untyped_constants {
                    panic!("Typed constants can only be dropped from a domain if a problem is provided")
                }
            }
        }
    }
    // convert typed action params to untyped
    for action in program.domain.actions.iter_mut() {
        let constraints = collect_param_constraints(&mut action.parameters);
        apply_constraints(
            &mut action.preconditions,
            minimize_constraints(constraints, &checker),
        );
    }
    // convert typed tasks to untyped
    let mut task_param_types: HashMap<&'a str, Vec<Option<&'a str>>> = HashMap::new();
    for task in program.domain.compound_tasks.iter_mut() {
        let mut types = vec![];
        for param in task.parameters.iter_mut() {
            param.type_pos = None;
            types.push(param.symbol_type.take());
        }
        task_param_types.insert(task.name, types);
    }
    // convert typed methods to untyped;
    // each method also receives the typings of its task's terms
    for method in program.domain.methods.iter_mut() {
        let mut constraints = collect_param_constraints(&mut method.params);
        if let Some(types) = task_param_types.get(method.task.name) {
            for (term, t) in method.task_terms.iter().zip(types) {
                if let Some(t) = *t {
                    constraints.push((term.name, t));
                }
            }
        }
        apply_constraints(
            &mut method.precondition,
            minimize_constraints(constraints, &checker),
        );
    }
    // Convert typed objects to untyped
    if let Some(problem) = &mut program.problem {
        for object in problem.objects.iter_mut() {
            object.type_pos = None;
            let name = object.name;
            let Some(t) = object.symbol_type.take() else {
                continue;
            };
            init.push(type_atom(t, name));
            init.extend(
                std::iter::once(t)
                    .chain(checker.get_supertypes(t))
                    .map(|ty| type_atom(ty, name)),
            );
        }
        problem.add_init_state(init);
    }
}

// strips the parameter typings, returning them as (variable, type) constraints
fn collect_param_constraints<'a>(parameters: &mut Vec<Symbol<'a>>) -> Vec<(&'a str, &'a str)> {
    let mut constraints = vec![];
    for param in parameters.iter_mut() {
        param.type_pos = None;
        let Some(t) = param.symbol_type.take() else {
            continue;
        };
        constraints.push((param.name, t));
    }
    constraints
}

// drops constraints implied by a more specific constraint on the same variable
fn minimize_constraints<'a>(
    constraints: Vec<(&'a str, &'a str)>,
    checker: &TypeChecker<'a>,
) -> Vec<(&'a str, &'a str)> {
    let mut kept: Vec<(&'a str, &'a str)> = vec![];
    for (var, t) in constraints {
        let implied = kept
            .iter()
            .any(|(kv, kt)| *kv == var && (*kt == t || checker.get_supertypes(kt).contains(t)));
        if implied {
            continue;
        }
        kept.retain(|(kv, kt)| !(*kv == var && checker.get_supertypes(t).contains(kt)));
        kept.push((var, t));
    }
    kept
}

fn apply_constraints<'a>(
    precondition: &mut Option<Formula<'a>>,
    constraints: Vec<(&'a str, &'a str)>,
) {
    let conjuncts = constraints
        .into_iter()
        .map(|(var, t)| {
            Box::new(Formula::Atom(Predicate::new(
                t,
                TokenPosition::default(),
                vec![Symbol::new_untyped(var, TokenPosition::default())],
            )))
        })
        .collect();
    let conjuncts = Formula::And(conjuncts);
    *precondition = Some(match precondition.take() {
        Some(prec) => prec.and(conjuncts),
        None => conjuncts,
    });
}
