use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Default)]
pub struct TokenPosition {
    pub line: u32,
}