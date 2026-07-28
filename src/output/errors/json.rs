use std::fmt;

#[derive(Debug)]
pub struct JsonError {
    /// serde_json's rendered message; already includes "at line L column C".
    pub message: String,
    pub line: u32,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
