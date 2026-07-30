use crate::{HDDLProgram, MetaData, ParsingError, WarningType};

use super::{Input, Transformation};

pub struct Transpiler<'a> {
    program: HDDLProgram<'a>,
}

impl<'a> Transpiler<'a> {
    pub fn new(program: HDDLProgram<'a>) -> Transpiler<'a> {
        Transpiler { program }
    }

    pub fn from_hddl(
        domain: &'a Vec<u8>,
        problem: Option<&'a Vec<u8>>,
    ) -> Result<Transpiler<'a>, ParsingError> {
        Ok(Transpiler::new(HDDLProgram::from_hddl(domain, problem)?))
    }

    pub fn from_json(json: &'a str) -> Result<Transpiler<'a>, ParsingError> {
        Ok(Transpiler::new(HDDLProgram::from_json(json)?))
    }

    pub fn from_input(input: Input<'a>) -> Result<Transpiler<'a>, ParsingError> {
        match input {
            Input::Hddl { domain, problem } => Transpiler::from_hddl(domain, problem),
            Input::Json(json) => Transpiler::from_json(json),
        }
    }

    // applies a transformation to the program; chainable before emission
    pub fn transform(mut self, transformation: Transformation) -> Transpiler<'a> {
        match transformation {
            Transformation::RemoveTypes => super::remove_types::remove_types(&mut self.program),
        }
        self
    }

    pub fn verify(&self) -> Result<Vec<WarningType>, ParsingError> {
        self.program.verify()
    }

    pub fn metadata(&self) -> Result<MetaData, ParsingError> {
        self.program.metadata()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.program).unwrap()
    }

    pub fn to_hddl(&self) -> (String, Option<String>) {
        (
            self.program.domain.to_string(),
            self.program.problem.as_ref().map(|problem| problem.to_string()),
        )
    }

    pub fn into_program(self) -> HDDLProgram<'a> {
        self.program
    }
}
