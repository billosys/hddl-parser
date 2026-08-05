use crate::*;

impl<'a> Transpiler<'a> {
    // TODO: reimplement under the new structure (typed universal expansion,
    // DNF splitting via a compound-task wrapper, existential hoisting)
    pub(crate) fn conjunctive_preconditions(&mut self) -> Result<(), ParsingError> {
        Err(ParsingError::Transformation(
            "the conjunctive-preconditions transformation is not implemented yet".to_string(),
        ))
    }
}
