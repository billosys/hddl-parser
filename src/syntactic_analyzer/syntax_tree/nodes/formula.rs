use std::{fmt, vec};

use serde::{Deserialize, Serialize};

use crate::NumberType;
use crate::transpiler::format_typed_list;

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Formula<'a> {
    Empty,
    Atom(Predicate<'a>),
    Not(Box<Formula<'a>>),
    And(Vec<Box<Formula<'a>>>),
    Or(Vec<Box<Formula<'a>>>),
    Xor(Vec<Box<Formula<'a>>>),
    // formula -> formula'
    Imply(Vec<Box<Formula<'a>>>, Vec<Box<Formula<'a>>>),
    // ∃vars: formula
    Exists(Vec<Symbol<'a>>, Box<Formula<'a>>),
    // ∀vars: formula
    ForAll(Vec<Symbol<'a>>, Box<Formula<'a>>),
    // a formula holding with the given probability in [0, 1]
    Probabilistic(NumberType, Box<Formula<'a>>),
    // formula = formula'
    Equals(#[serde(borrow)] &'a str, &'a str),
}

impl<'a> Formula<'a> {
    pub fn get_propositional_predicates(&self) -> Vec<&Predicate<'a>> {
        let mut predicates = vec![];
        match &*self {
            Formula::Empty => {}
            Formula::Atom(predicate) => {
                predicates.push(predicate);
            }
            Formula::Not(new_formula) => {
                predicates.extend(new_formula.get_propositional_predicates().iter());
            }
            Formula::And(new_formula) | Formula::Or(new_formula) | Formula::Xor(new_formula) => {
                for f in new_formula {
                    predicates.extend(f.get_propositional_predicates().iter());
                }
            }
            Formula::Imply(ps, qs) => {
                for p in ps {
                    predicates.extend(p.get_propositional_predicates().iter());
                }
                for q in qs {
                    predicates.extend(q.get_propositional_predicates().iter());
                }
            },
            Formula::Probabilistic(_, q) => {
                predicates.extend(q.get_propositional_predicates().iter());
            }
            Formula::Equals(_, _) => {}
            // not propositional
            Formula::ForAll(_, _) | Formula::Exists(_, _) => {}
        }
        return predicates;
    }

    pub fn is_simple_conjunction(&self) -> bool {
        match self {
            Formula::Empty => true,
            Formula::And(conjuncts) => conjuncts.iter().all(|c| c.is_literal()),
            other => other.is_literal(),
        }
    }

    pub fn and(self, rhs: Formula<'a>) -> Formula<'a> {
        let mut conjuncts = match self {
            Formula::And(conjuncts) => conjuncts,
            other => vec![Box::new(other)],
        };
        match rhs {
            Formula::And(rhs) => conjuncts.extend(rhs),
            other => conjuncts.push(Box::new(other)),
        }
        Formula::And(conjuncts)
    }

    fn is_literal(&self) -> bool {
        match self {
            Formula::Atom(_) | Formula::Equals(_, _) => true,
            Formula::Not(inner) => matches!(&**inner, Formula::Atom(_)),
            _ => false,
        }
    }
}

impl<'a> fmt::Display for Formula<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn join(terms: &[Box<Formula>]) -> String {
            terms.iter().map(|term| term.to_string()).collect::<Vec<_>>().join(" ")
        }
        // an implication side is a conjunction, but a single conjunct needs no "and" wrapper
        fn implication_side(terms: &[Box<Formula>]) -> String {
            match terms {
                [term] => term.to_string(),
                _ => format!("(and {})", join(terms)),
            }
        }
        match self {
            Formula::Empty => write!(f, "()"),
            Formula::Atom(predicate) => {
                write!(f, "({}", predicate.name)?;
                for var in predicate.variables.iter() {
                    write!(f, " {}", var.name)?;
                }
                write!(f, ")")
            }
            Formula::Not(inner) => write!(f, "(not {})", inner),
            Formula::And(terms) => write!(f, "(and {})", join(terms)),
            Formula::Or(terms) => write!(f, "(or {})", join(terms)),
            Formula::Xor(terms) => write!(f, "(oneof {})", join(terms)),
            Formula::Imply(lhs, rhs) => {
                write!(f, "(when {} {})", implication_side(lhs), implication_side(rhs))
            }
            Formula::Exists(vars, inner) => {
                write!(f, "(exists ({}) {})", format_typed_list(vars), inner)
            }
            Formula::ForAll(vars, inner) => {
                write!(f, "(forall ({}) {})", format_typed_list(vars), inner)
            }
            Formula::Probabilistic(probability, terms) => {
                write!(f, "(probabilistic {} {})", probability, terms)
            }
            Formula::Equals(lhs, rhs) => write!(f, "(= {} {})", lhs, rhs),
        }
    }
}