use serde::{Deserialize, Serialize};

use crate::TokenPosition;

use super::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct Method<'a> {
    #[serde(borrow)]
    pub name: Symbol<'a>,
    pub task: Symbol<'a>,
    pub task_terms: Vec<Symbol<'a>>,
    pub params: Vec<Symbol<'a>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precondition: Option<Formula<'a>>,
    pub tn: HTN<'a>
}