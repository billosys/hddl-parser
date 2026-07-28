use serde::{Deserialize, Serialize};

use crate::TokenPosition;
use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Function<'a> {
    pub name: &'a str,
    #[serde(skip)]
    pub name_pos: TokenPosition,
    pub variables: Vec<Symbol<'a>>
}