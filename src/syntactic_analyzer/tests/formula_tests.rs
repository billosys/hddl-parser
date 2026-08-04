use super::*;

#[cfg(test)]
mod tests {

    use super::*;

    fn atom(name: &str) -> Box<Formula> {
        Box::new(Formula::Atom(Predicate::new_dummy(name)))
    }

    #[test]
    pub fn conjunction_of_literals_test() {
        let formula = Formula::And(vec![
            atom("p_1"),
            Box::new(Formula::Not(atom("p_2"))),
            Box::new(Formula::Equals("a", "b")),
        ]);
        assert_eq!(formula.is_simple_conjunction(), true);
    }

    #[test]
    pub fn bare_literal_conjunction_test() {
        assert_eq!(Formula::Atom(Predicate::new_dummy("p_1")).is_simple_conjunction(), true);
        assert_eq!(Formula::Not(atom("p_1")).is_simple_conjunction(), true);
    }

    #[test]
    pub fn vacuous_conjunction_test() {
        assert_eq!(Formula::Empty.is_simple_conjunction(), true);
        assert_eq!(Formula::And(vec![]).is_simple_conjunction(), true);
    }

    #[test]
    pub fn disjunction_not_conjunction_test() {
        let disjunction = Formula::Or(vec![atom("p_1"), atom("p_2")]);
        assert_eq!(disjunction.is_simple_conjunction(), false);
        let conjoined_disjunction =
            Formula::And(vec![Box::new(Formula::Or(vec![atom("p_1"), atom("p_2")])), atom("p_3")]);
        assert_eq!(conjoined_disjunction.is_simple_conjunction(), false);
        let xor = Formula::Xor(vec![atom("p_1"), atom("p_2")]);
        assert_eq!(xor.is_simple_conjunction(), false);
        let imply = Formula::Imply(atom("p_1"), atom("p_2"));
        assert_eq!(imply.is_simple_conjunction(), false);
    }

    #[test]
    pub fn nested_conjunction_not_simple_test() {
        let nested = Formula::And(vec![
            Box::new(Formula::And(vec![atom("p_1"), atom("p_2")])),
            atom("p_3"),
        ]);
        assert_eq!(nested.is_simple_conjunction(), false);
        let double_negation = Formula::Not(Box::new(Formula::Not(atom("p_1"))));
        assert_eq!(double_negation.is_simple_conjunction(), false);
    }

    #[test]
    pub fn probabilistic_not_conjunction_test() {
        let probabilistic = Formula::And(vec![
            Box::new(Formula::Probabilistic(NumberType::Real(0.5), atom("p_1"))),
            Box::new(Formula::Probabilistic(NumberType::Real(0.5), atom("p_2"))),
        ]);
        assert_eq!(probabilistic.is_simple_conjunction(), false);
    }

    #[test]
    pub fn quantified_not_conjunction_test() {
        let var = Symbol::new("x", TokenPosition { line: 0 }, None, None);
        let exists = Formula::Exists(vec![var.clone()], atom("p_1"));
        assert_eq!(exists.is_simple_conjunction(), false);
        let for_all = Formula::ForAll(vec![var], atom("p_1"));
        assert_eq!(for_all.is_simple_conjunction(), false);
    }

    #[test]
    pub fn parsed_precondition_conjunction_test() {
        let program = String::from(
            "(define (domain bal)
                (:action a_1
                 :parameters (p_1 p_2 - t1)
                 :precondition (and (at p_1) (not (at p_2)))
                 :effect (at p_2)
                )
                (:action a_2
                 :parameters (p_1 p_2 - t1)
                 :precondition (or (at p_1) (at p_2))
                 :effect (at p_2)
                )
             ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(&program);
        match Parser::new(lexer).parse() {
            Ok(AbstractSyntaxTree::Domain(ast)) => {
                assert_eq!(ast.actions.len(), 2);
                let a_1 = &ast.actions[0];
                assert_eq!(a_1.preconditions.as_ref().unwrap().is_simple_conjunction(), true);
                assert_eq!(a_1.effects.as_ref().unwrap().is_simple_conjunction(), true);
                let a_2 = &ast.actions[1];
                assert_eq!(a_2.preconditions.as_ref().unwrap().is_simple_conjunction(), false);
            }
            _ => panic!("parsing errors"),
        }
    }
}
