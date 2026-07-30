use std::println;

use super::{Input, Transpiler};
use crate::{
    transpiler::transform, AbstractSyntaxTree, Action, Formula, HDDLProgram, LexicalAnalyzer,
    NumberType, Parser, Predicate, Subtask, Symbol, TokenPosition,
};

fn symbol(name: &'static str) -> Symbol<'static> {
    Symbol::new(name, TokenPosition { line: 0 }, None, None)
}

fn atom(name: &'static str, vars: Vec<&'static str>) -> Box<Formula<'static>> {
    Box::new(Formula::Atom(Predicate::new(
        name,
        TokenPosition { line: 0 },
        vars.into_iter().map(symbol).collect(),
    )))
}

fn parse_domain(program: &Vec<u8>) -> crate::DomainAST {
    let lexer = LexicalAnalyzer::new(program);
    match Parser::new(lexer).parse() {
        Ok(AbstractSyntaxTree::Domain(ast)) => ast,
        _ => panic!("domain parsing errors"),
    }
}

fn parse_problem(program: &Vec<u8>) -> crate::ProblemAST {
    let lexer = LexicalAnalyzer::new(program);
    match Parser::new(lexer).parse() {
        Ok(AbstractSyntaxTree::Problem(ast)) => ast,
        _ => panic!("problem parsing errors"),
    }
}

fn assert_domain_round_trip(canonical: &str) {
    let program = canonical.as_bytes().to_vec();
    assert_eq!(parse_domain(&program).to_string(), canonical);
}

fn assert_problem_round_trip(canonical: &str) {
    let program = canonical.as_bytes().to_vec();
    assert_eq!(parse_problem(&program).to_string(), canonical);
}

#[test]
pub fn pddl_display_test() {
    let formula = Formula::And(vec![
        atom("at", vec!["p_1", "p_2"]),
        Box::new(Formula::Not(atom("clear", vec![]))),
        Box::new(Formula::Probabilistic(
            NumberType::Real(0.5),
            atom("holding", vec![]),
        )),
        Box::new(Formula::Equals("a", "b")),
    ]);
    assert_eq!(
        formula.to_string(),
        "(and (at p_1 p_2) (not (clear)) (probabilistic 0.5 (holding)) (= a b))"
    );

    let var = Symbol::new("x", TokenPosition { line: 0 }, Some("loc"), None);
    let for_all = Formula::ForAll(vec![var], atom("p_1", vec![]));
    assert_eq!(for_all.to_string(), "(forall (x - loc) (p_1))");

    let when = Formula::Imply(
        vec![atom("p_1", vec![])],
        vec![atom("p_2", vec![]), atom("p_3", vec![])],
    );
    assert_eq!(when.to_string(), "(when (p_1) (and (p_2) (p_3)))");

    let oneof = Formula::Xor(vec![atom("p_1", vec![]), atom("p_2", vec![])]);
    assert_eq!(oneof.to_string(), "(oneof (p_1) (p_2))");
}

#[test]
pub fn action_display_test() {
    let action = Action {
        name: "pickup",
        name_pos: TokenPosition { line: 0 },
        parameters: vec![Symbol::new(
            "?x",
            TokenPosition { line: 0 },
            Some("block"),
            None,
        )],
        preconditions: Some(Formula::And(vec![
            atom("clear", vec!["?x"]),
            atom("handempty", vec![]),
        ])),
        effects: Some(Formula::And(vec![
            atom("holding", vec!["?x"]),
            Box::new(Formula::Not(atom("clear", vec!["?x"]))),
        ])),
    };
    assert_eq!(
        action.to_string(),
        "(:action pickup\n :parameters (?x - block)\n \
        :precondition (and (clear ?x) (handempty))\n \
        :effect (and (holding ?x) (not (clear ?x)))\n)"
    );
}

#[test]
pub fn action_display_omits_empty_test() {
    let action = Action {
        name: "noop",
        name_pos: TokenPosition { line: 0 },
        parameters: vec![],
        preconditions: None,
        effects: None,
    };
    assert_eq!(action.to_string(), "(:action noop\n :parameters ()\n)");
}

#[test]
pub fn subtask_display_test() {
    let with_id = Subtask {
        id: Some(symbol("task0")),
        task: symbol("deliver"),
        terms: vec![symbol("?p"), symbol("?l")],
    };
    assert_eq!(with_id.to_string(), "(task0 (deliver ?p ?l))");

    let bare = Subtask {
        id: None,
        task: symbol("noop"),
        terms: vec![],
    };
    assert_eq!(bare.to_string(), "(noop)");
}

#[test]
pub fn predicate_display_test() {
    let predicate = Predicate::new(
        "at",
        TokenPosition { line: 0 },
        vec![
            Symbol::new("?x", TokenPosition { line: 0 }, Some("loc"), None),
            symbol("?y"),
        ],
    );
    assert_eq!(predicate.to_string(), "(at ?x - loc ?y)");
}

#[test]
pub fn action_display_round_trip_test() {
    assert_domain_round_trip(
        "(define (domain d)

 (:action pickup
  :parameters (?x - block)
  :precondition (and (clear ?x) (handempty))
  :effect (and (holding ?x) (not (clear ?x)))
 )

)",
    );
}

#[test]
pub fn method_display_round_trip_test() {
    assert_domain_round_trip(
        "(define (domain d)

 (:method m_1
  :parameters (?p1 - p ?l1 - loc ?l2 - loc)
  :task (deliver ?p1 ?l1)
  :precondition (at ?p1 ?l1)
  :subtasks (and (task0 (pickup ?p1 ?l1)) (task1 (drop ?p1 ?l2)))
  :ordering (and (< task0 task1))
  :constraints (and (not (= ?l1 ?l2)))
 )

)",
    );
}

#[test]
pub fn ordered_subtasks_round_trip_test() {
    assert_domain_round_trip(
        "(define (domain d)

 (:method m_2
  :parameters (?p - p)
  :task (deliver ?p)
  :ordered-subtasks (and (pickup ?p) (drop ?p))
 )

)",
    );
}

#[test]
pub fn init_tn_display_round_trip_test() {
    assert_problem_round_trip(
        "(define (problem p1) (:domain d)

 (:htn
  :parameters (?v)
  :subtasks (and (task0 (deliver pkg_0 loc_0)) (task1 (deliver pkg_1 loc_1)))
  :ordering (and (< task0 task1))
 )

)",
    );
}

#[test]
pub fn domain_to_hddl_round_trip_test() {
    assert_domain_round_trip(
        "(define (domain transport)

 (:requirements :typing :hierarchy)

 (:types loc - object pkg - object)

 (:constants depot - loc)

 (:predicates (at ?p - pkg ?l - loc) (road ?l1 - loc ?l2 - loc))

 (:functions (fuel ?l - loc))

 (:task deliver
  :parameters (?p - pkg ?l - loc)
 )

 (:method m_deliver
  :parameters (?p - pkg ?l1 - loc ?l2 - loc)
  :task (deliver ?p ?l2)
  :ordered-subtasks (and (pickup ?p ?l1) (drop ?p ?l2))
 )

 (:action pickup
  :parameters (?p - pkg ?l - loc)
  :precondition (at ?p ?l)
  :effect (not (at ?p ?l))
 )

)",
    );
}

#[test]
pub fn problem_to_hddl_round_trip_test() {
    assert_problem_round_trip(
        "(define (problem p_transport) (:domain transport)

 (:requirements :typing)

 (:objects pkg_0 - pkg loc_0 - loc loc_1 - loc)

 (:htn
  :subtasks (and (task0 (deliver pkg_0 loc_1)))
 )

 (:init (at pkg_0 loc_0) (road loc_0 loc_1))

 (:goal (at pkg_0 loc_1))

)",
    );
}

#[test]
pub fn transpiler_pipeline_test() {
    let domain = "(define (domain transport)

 (:predicates (at ?p ?l) (road ?l1 ?l2))

 (:task deliver
  :parameters (?p ?l)
 )

 (:method m_deliver
  :parameters (?p ?l)
  :task (deliver ?p ?l)
  :ordered-subtasks (and (pickup ?p ?l))
 )

 (:action pickup
  :parameters (?p ?l)
  :precondition (at ?p ?l)
  :effect (not (at ?p ?l))
 )

)";
    let problem = "(define (problem p_transport) (:domain transport)

 (:objects pkg_0 loc_0)

 (:htn
  :subtasks (and (deliver pkg_0 loc_0))
 )

 (:init (at pkg_0 loc_0))

)";
    let domain_bytes = domain.as_bytes().to_vec();
    let problem_bytes = problem.as_bytes().to_vec();

    // HDDL in, both outputs
    let from_hddl = Transpiler::from_hddl(&domain_bytes, Some(&problem_bytes)).unwrap();
    assert_eq!(
        from_hddl.to_hddl(),
        (domain.to_string(), Some(problem.to_string()))
    );
    let json = from_hddl.to_json();

    // JSON in, both outputs; agrees with the HDDL path
    let from_json = Transpiler::from_json(&json).unwrap();
    assert_eq!(
        from_json.to_hddl(),
        (domain.to_string(), Some(problem.to_string()))
    );
    assert_eq!(from_json.to_json(), json);

    // verify and metadata work through the facade
    assert!(from_json.verify().is_ok());
    assert_eq!(from_json.metadata().unwrap().domain_name, "transport");

    // from_input dispatches to the same constructors
    let from_hddl_input = Transpiler::from_input(Input::Hddl {
        domain: &domain_bytes,
        problem: Some(&problem_bytes),
    })
    .unwrap();
    assert_eq!(from_hddl_input.to_json(), json);
    let from_json_input = Transpiler::from_input(Input::Json(&json)).unwrap();
    assert_eq!(from_json_input.to_json(), json);
}

#[test]
pub fn transpiler_to_hddl_test() {
    let domain = "(define (domain transport)

 (:predicates (at ?p ?l))

)";
    let problem = "(define (problem p_transport) (:domain transport)

 (:objects pkg_0 loc_0)

 (:init (at pkg_0 loc_0))

)";
    let domain_bytes = domain.as_bytes().to_vec();
    let problem_bytes = problem.as_bytes().to_vec();
    let program = HDDLProgram::from_hddl(&domain_bytes, Some(&problem_bytes)).unwrap();
    let (domain_hddl, problem_hddl) = Transpiler::new(program).to_hddl();
    assert_eq!(domain_hddl, domain);
    assert_eq!(problem_hddl.as_deref(), Some(problem));
}

#[test]
pub fn untyping_test() {
    let domain = "(define (domain transport)
        (:types car truck - vehicle vehicle - something location) 
        (:constants up - location)
        (:predicates (at ?c - car ?l - location ?a))
    )";
    let problem = "(define (problem p_transport) (:domain transport)
        (:objects c1 - car loc_0 - location pkg_0)
        (:init (at pkg_0 loc_0))
    )";
    let domain_bytes = domain.as_bytes().to_vec();
    let problem_bytes = problem.as_bytes().to_vec();
    let program = HDDLProgram::from_hddl(&domain_bytes, Some(&problem_bytes)).unwrap();
    let result = Transpiler::new(program)
        .transform(crate::Transformation::RemoveTypes)
        .into_program();
    assert!(result.domain.constants.unwrap().iter().all(|x| {
        x.symbol_type.is_none() && x.type_pos.is_none()
    }));
    let predicates = &result.domain.predicates;
    assert!(predicates.contains(&Predicate {
        name: "object",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("?var", TokenPosition::default())]
    }));
    assert!(predicates.contains(&Predicate {
        name: "something",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("?var", TokenPosition::default())]
    }));
    assert!(predicates.contains(&Predicate {
        name: "location",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("?var", TokenPosition::default())]
    }));
    assert!(predicates.contains(&Predicate {
        name: "vehicle",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("?var", TokenPosition::default())]
    }));
    assert!(predicates.contains(&Predicate {
        name: "car",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("?var", TokenPosition::default())]
    }));
    assert!(predicates.contains(&Predicate {
        name: "truck",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("?var", TokenPosition::default())]
    }));
    let problem = &result.problem.unwrap();
    let objects = &problem.objects;
    for object in objects {
        assert!(object.symbol_type.is_none());
        assert!(object.type_pos.is_none());
    }
    let init = &problem.init_state;
    assert!(init.contains(&Predicate {
        name: "object",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("up", TokenPosition::default())]
    }));
    assert!(init.contains(&Predicate {
        name: "location",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("up", TokenPosition::default())]
    }));
    assert!(init.contains(&Predicate {
        name: "object",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("c1", TokenPosition::default())]
    }));
    assert!(init.contains(&Predicate {
        name: "car",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("c1", TokenPosition::default())]
    }));
    assert!(init.contains(&Predicate {
        name: "vehicle",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("c1", TokenPosition::default())]
    }));
    assert!(init.contains(&Predicate {
        name: "object",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("loc0", TokenPosition::default())]
    }));
    assert!(init.contains(&Predicate {
        name: "location",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("loc0", TokenPosition::default())]
    }));
    assert!(init.contains(&Predicate {
        name: "object",
        name_pos: TokenPosition::default(),
        variables: vec![Symbol::new_untyped("pkg0", TokenPosition::default())]
    }));
}
