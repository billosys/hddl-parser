use petgraph::prelude::GraphMap;
use petgraph::algo::toposort;
use petgraph::Directed;
use serde::{Deserialize, Serialize};

use crate::TokenPosition;
use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialTaskNetwork<'a> {
    #[serde(default)]
    pub parameters: Option<Vec<Symbol<'a>>>,
    #[serde(borrow)]
    pub tn: HTN<'a>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTN<'a> {
    #[serde(borrow)]
    pub subtasks: Vec<Subtask<'a>>,
    #[serde(skip)]
    pub ordering_pos: Option<TokenPosition>,
    pub orderings: TaskOrdering<'a>,
    #[serde(default)]
    pub constraints: Option<Vec<Constraint<'a>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtask<'a> {
    #[serde(default)]
    pub id: Option<Symbol<'a>>,
    #[serde(borrow)]
    pub task: Symbol<'a>,
    pub terms: Vec<Symbol<'a>>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constraint<'a> {
    Equal(#[serde(borrow)] &'a str, &'a str),
    NotEqual(#[serde(borrow)] &'a str, &'a str)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskOrdering<'a> {
    Total,
    Partial(#[serde(borrow)] Vec<(&'a str, &'a str)>)
}

impl <'a> TaskOrdering<'a> {
    pub fn is_acyclic(&self) -> bool {
        match &self {
            TaskOrdering::Total => { true }
            TaskOrdering::Partial(orderings) => {
                let ordering_graph = GraphMap::<_, (), Directed>::from_edges(orderings);
                match toposort(&ordering_graph, None) {
                    Ok(_) => { true }
                    Err(_) => {
                        return false;
                    }
                }
            }
        }
    }
}