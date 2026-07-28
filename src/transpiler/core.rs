use crate::HDDLProgram;

pub struct Transpiler<'a> {
    program: HDDLProgram<'a>
}

impl <'a> Transpiler<'a> {
    pub fn new(program: HDDLProgram<'a>) -> Transpiler<'a> {
        Transpiler { program }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.program).unwrap()
    }
}