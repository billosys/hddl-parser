extern crate hddl_analyzer;

use hddl_analyzer::HDDLProgram;
use std::fs;

mod common;

#[test]
pub fn ipc_validation_test() {
    for case in common::fast_corpus_cases().expect("fast corpus selection should be valid") {
        let domain = fs::read(&case.domain_path)
            .unwrap_or_else(|error| panic!("{}: failed to read domain: {error}", case.id));
        let problem = fs::read(&case.problem_path)
            .unwrap_or_else(|error| panic!("{}: failed to read problem: {error}", case.id));

        if let Err(error) =
            HDDLProgram::from_hddl(&domain, Some(&problem)).and_then(|program| program.verify())
        {
            panic!("{}: parse/verify error: {:?}", case.id, error);
        }
    }
}
