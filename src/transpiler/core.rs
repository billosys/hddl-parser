use crate::HDDLProgram;

pub struct Transpiler<'a> {
    program: HDDLProgram<'a>,
}

impl<'a> Transpiler<'a> {
    pub fn new(program: HDDLProgram<'a>) -> Transpiler<'a> {
        Transpiler { program }
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
}
