use super::*;

#[cfg(test)]
mod tests {

    use super::*;

    fn atom(name: &'static str, vars: Vec<&'static str>) -> Box<Formula<'static>> {
        Box::new(Formula::Atom(Predicate::new(
            name,
            TokenPosition { line: 0 },
            vars.into_iter()
                .map(|v| Symbol::new(v, TokenPosition { line: 0 }, None, None))
                .collect(),
        )))
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
    pub fn action_display_round_trip_test() {
        let program = String::from(
            "(define (domain d)
                (:action pickup
                 :parameters (?x - block)
                 :precondition (and (clear ?x) (handempty))
                 :effect (and (holding ?x) (not (clear ?x)))
                )
             ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(&program);
        let first = match Parser::new(lexer).parse() {
            Ok(AbstractSyntaxTree::Domain(ast)) => {
                assert_eq!(ast.actions.len(), 1);
                assert_eq!(ast.actions[0].parameters[0].name, "?x");
                ast.actions[0].to_string()
            }
            _ => panic!("parsing errors"),
        };
        let reparsed_program = format!("(define (domain d) {} ) ", first).into_bytes();
        let lexer = LexicalAnalyzer::new(&reparsed_program);
        match Parser::new(lexer).parse() {
            Ok(AbstractSyntaxTree::Domain(ast)) => {
                assert_eq!(ast.actions.len(), 1);
                assert_eq!(ast.actions[0].to_string(), first);
            }
            _ => panic!("re-emitted action is not parseable"),
        }
    }
}
