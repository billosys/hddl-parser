use core::panic;
use std::collections::HashSet;
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
        compile_param_types_to_precondition(&mut action.parameters, &mut action.preconditions);
    }
    // convert typed methods to untyped
    for method in program.domain.methods.iter_mut() {
        compile_param_types_to_precondition(&mut method.params, &mut method.precondition);
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

fn compile_param_types_to_precondition<'a>(
    parameters: &mut Vec<Symbol<'a>>,
    precondition: &mut Option<Formula<'a>>,
) {
    let mut conjuncts = vec![];
    for param in parameters.iter_mut() {
        param.type_pos = None;
        let Some(t) = param.symbol_type.take() else {
            continue;
        };
        conjuncts.push(Box::new(crate::Formula::Atom(Predicate::new(
            t,
            TokenPosition::default(),
            vec![Symbol::new_untyped(param.name, TokenPosition::default())],
        ))));
    }
    let conjuncts = crate::Formula::And(conjuncts);
    *precondition = Some(match precondition.take() {
        Some(prec) => prec.and(conjuncts),
        None => conjuncts,
    });
}
