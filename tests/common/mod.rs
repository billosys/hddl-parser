use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::PathBuf;

const CORPUS_ROOT: &str = "tests/ipc";
const FAST_SELECTION_PATH: &str = "tests/ipc/corpus-selections/fast.txt";

#[derive(Debug)]
pub struct CorpusCase {
    pub id: String,
    pub domain_path: PathBuf,
    pub problem_path: PathBuf,
}

pub fn fast_corpus_cases() -> io::Result<Vec<CorpusCase>> {
    let contents = fs::read_to_string(FAST_SELECTION_PATH)?;
    let mut seen = HashSet::new();
    let mut cases = Vec::new();

    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let case_id = line.trim();
        if case_id.is_empty() || case_id.starts_with('#') {
            continue;
        }
        if !seen.insert(case_id.to_string()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("duplicate corpus case ID {case_id:?} on line {line_number}"),
            ));
        }

        let Some((domain_dir, problem_file)) = case_id.split_once('/') else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid corpus case ID {case_id:?} on line {line_number}"),
            ));
        };
        let domain_path = PathBuf::from(CORPUS_ROOT)
            .join(domain_dir)
            .join("domain.hddl");
        let problem_path = PathBuf::from(CORPUS_ROOT)
            .join(domain_dir)
            .join(problem_file);
        if !domain_path.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("missing domain file for corpus case ID {case_id:?}"),
            ));
        }
        if !problem_path.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("missing problem file for corpus case ID {case_id:?}"),
            ));
        }

        cases.push(CorpusCase {
            id: case_id.to_string(),
            domain_path,
            problem_path,
        });
    }

    if cases.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{FAST_SELECTION_PATH} contains no corpus case IDs"),
        ));
    }

    Ok(cases)
}
