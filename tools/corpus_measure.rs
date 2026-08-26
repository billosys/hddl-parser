// Run with `cargo run --locked --example corpus_measure`.
use hddl_analyzer::{HDDLProgram, Transpiler};
use serde_json::Value;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const CORPUS_ROOT: &str = "tests/ipc";

#[derive(Debug)]
struct CorpusCase {
    id: String,
    domain_path: PathBuf,
    problem_path: PathBuf,
}

#[derive(Debug)]
struct Config {
    filter: Option<String>,
    limit: Option<usize>,
    report_path: Option<PathBuf>,
}

#[derive(Debug)]
struct CaseMeasurement {
    case_id: String,
    domain_path: String,
    problem_path: String,
    hddl_parse_ms: Option<f64>,
    verify_ms: Option<f64>,
    json_export_ms: Option<f64>,
    json_import_ms: Option<f64>,
    reimport_verify_ms: Option<f64>,
    json_reexport_ms: Option<f64>,
    json_string_compare_ms: Option<f64>,
    json_value_compare_ms: Option<f64>,
    parse_outcome: String,
    verify_outcome: String,
    json_export_outcome: String,
    json_import_outcome: String,
    reimport_verify_outcome: String,
    json_reexport_outcome: String,
    json_string_equal: Option<bool>,
    json_value_equal: Option<bool>,
    error: Option<String>,
}

impl CaseMeasurement {
    fn new(case: &CorpusCase) -> Self {
        Self {
            case_id: case.id.clone(),
            domain_path: case.domain_path.display().to_string(),
            problem_path: case.problem_path.display().to_string(),
            hddl_parse_ms: None,
            verify_ms: None,
            json_export_ms: None,
            json_import_ms: None,
            reimport_verify_ms: None,
            json_reexport_ms: None,
            json_string_compare_ms: None,
            json_value_compare_ms: None,
            parse_outcome: "not_run".to_string(),
            verify_outcome: "not_run".to_string(),
            json_export_outcome: "not_run".to_string(),
            json_import_outcome: "not_run".to_string(),
            reimport_verify_outcome: "not_run".to_string(),
            json_reexport_outcome: "not_run".to_string(),
            json_string_equal: None,
            json_value_equal: None,
            error: None,
        }
    }

    fn completed(&self) -> bool {
        self.parse_outcome == "ok"
            && self.verify_outcome == "ok"
            && self.json_export_outcome == "ok"
            && self.json_import_outcome == "ok"
            && self.reimport_verify_outcome == "ok"
            && self.json_reexport_outcome == "ok"
            && self.json_string_equal.is_some()
            && self.json_value_equal.is_some()
    }

    fn total_measured_ms(&self) -> f64 {
        [
            self.hddl_parse_ms,
            self.verify_ms,
            self.json_export_ms,
            self.json_import_ms,
            self.reimport_verify_ms,
            self.json_reexport_ms,
            self.json_string_compare_ms,
            self.json_value_compare_ms,
        ]
        .into_iter()
        .flatten()
        .sum()
    }

    fn slowest_phase(&self) -> (&'static str, f64) {
        [
            ("hddl_parse", self.hddl_parse_ms),
            ("verify", self.verify_ms),
            ("json_export", self.json_export_ms),
            ("json_import", self.json_import_ms),
            ("reimport_verify", self.reimport_verify_ms),
            ("json_reexport", self.json_reexport_ms),
            ("json_string_compare", self.json_string_compare_ms),
            ("json_value_compare", self.json_value_compare_ms),
        ]
        .into_iter()
        .filter_map(|(phase, elapsed)| elapsed.map(|ms| (phase, ms)))
        .max_by(|(_, lhs), (_, rhs)| lhs.total_cmp(rhs))
        .unwrap_or(("none", 0.0))
    }

    fn failure(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

fn main() -> io::Result<()> {
    let config = Config::from_env()?;
    let all_cases = discover_cases(Path::new(CORPUS_ROOT))?;
    let discovered_cases = all_cases.len();
    let selected_cases = select_cases(all_cases, &config);
    let mut report = CsvReport::new(config.report_path.as_deref())?;

    println!(
        "discovered_cases={}, selected_cases={}",
        discovered_cases,
        selected_cases.len()
    );

    let mut measurements = Vec::with_capacity(selected_cases.len());
    for case in &selected_cases {
        let measurement = measure_case(case);
        println!(
            "{} completed={} total_ms={:.3} slowest_phase={}:{:.3}{}",
            measurement.case_id,
            measurement.completed(),
            measurement.total_measured_ms(),
            measurement.slowest_phase().0,
            measurement.slowest_phase().1,
            measurement
                .failure()
                .map(|error| format!(" error={error}"))
                .unwrap_or_default()
        );
        report.write_case(&measurement)?;
        measurements.push(measurement);
    }

    print_summary(&measurements);
    Ok(())
}

impl Config {
    fn from_env() -> io::Result<Self> {
        let filter = env::var("HDDL_CORPUS_FILTER")
            .ok()
            .filter(|s| !s.is_empty());
        let limit = match env::var("HDDL_CORPUS_LIMIT") {
            Ok(value) if value.is_empty() => None,
            Ok(value) => {
                let parsed = value.parse::<usize>().map_err(|error| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("HDDL_CORPUS_LIMIT must be a positive integer: {error}"),
                    )
                })?;
                if parsed == 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "HDDL_CORPUS_LIMIT must be greater than zero",
                    ));
                }
                Some(parsed)
            }
            Err(_) => None,
        };
        let report_path = env::var_os("HDDL_CORPUS_REPORT")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from);

        Ok(Self {
            filter,
            limit,
            report_path,
        })
    }
}

struct CsvReport {
    writer: Option<BufWriter<File>>,
}

impl CsvReport {
    fn new(path: Option<&Path>) -> io::Result<Self> {
        let writer = match path {
            Some(path) => {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut writer = BufWriter::new(File::create(path)?);
                writeln!(
                    writer,
                    "case_id,domain_path,problem_path,hddl_parse_ms,verify_ms,json_export_ms,json_import_ms,reimport_verify_ms,json_reexport_ms,json_string_compare_ms,json_value_compare_ms,parse_outcome,verify_outcome,json_export_outcome,json_import_outcome,reimport_verify_outcome,json_reexport_outcome,json_string_equal,json_value_equal,error"
                )?;
                writer.flush()?;
                Some(writer)
            }
            None => None,
        };

        Ok(Self { writer })
    }

    fn write_case(&mut self, measurement: &CaseMeasurement) -> io::Result<()> {
        let Some(writer) = self.writer.as_mut() else {
            return Ok(());
        };

        writeln!(
            writer,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            csv(&measurement.case_id),
            csv(&measurement.domain_path),
            csv(&measurement.problem_path),
            csv_opt_ms(measurement.hddl_parse_ms),
            csv_opt_ms(measurement.verify_ms),
            csv_opt_ms(measurement.json_export_ms),
            csv_opt_ms(measurement.json_import_ms),
            csv_opt_ms(measurement.reimport_verify_ms),
            csv_opt_ms(measurement.json_reexport_ms),
            csv_opt_ms(measurement.json_string_compare_ms),
            csv_opt_ms(measurement.json_value_compare_ms),
            csv(&measurement.parse_outcome),
            csv(&measurement.verify_outcome),
            csv(&measurement.json_export_outcome),
            csv(&measurement.json_import_outcome),
            csv(&measurement.reimport_verify_outcome),
            csv(&measurement.json_reexport_outcome),
            csv_opt_bool(measurement.json_string_equal),
            csv_opt_bool(measurement.json_value_equal),
            csv(measurement.error.as_deref().unwrap_or(""))
        )?;
        writer.flush()
    }
}

fn discover_cases(corpus_root: &Path) -> io::Result<Vec<CorpusCase>> {
    let mut domain_dirs = fs::read_dir(corpus_root)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<io::Result<Vec<_>>>()?;
    domain_dirs.retain(|path| path.is_dir());
    domain_dirs.sort();

    let mut cases = Vec::new();
    for domain_dir in domain_dirs {
        let domain_path = domain_dir.join("domain.hddl");
        if !domain_path.is_file() {
            continue;
        }

        let mut problem_paths = fs::read_dir(&domain_dir)?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<io::Result<Vec<_>>>()?;
        problem_paths.retain(|path| {
            path.is_file() && path.file_name().is_some_and(|name| name != "domain.hddl")
        });
        problem_paths.sort();

        for problem_path in problem_paths {
            let domain_name = domain_dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("<unknown-domain>");
            let problem_name = problem_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("<unknown-problem>");
            cases.push(CorpusCase {
                id: format!("{domain_name}/{problem_name}"),
                domain_path: domain_path.clone(),
                problem_path,
            });
        }
    }

    Ok(cases)
}

fn select_cases(cases: Vec<CorpusCase>, config: &Config) -> Vec<CorpusCase> {
    let filtered = cases.into_iter().filter(|case| {
        config
            .filter
            .as_ref()
            .is_none_or(|filter| case.id.contains(filter))
    });

    match config.limit {
        Some(limit) => filtered.take(limit).collect(),
        None => filtered.collect(),
    }
}

fn measure_case(case: &CorpusCase) -> CaseMeasurement {
    let mut measurement = CaseMeasurement::new(case);

    let domain = match fs::read(&case.domain_path) {
        Ok(domain) => domain,
        Err(error) => {
            measurement.error = Some(format!("read_domain: {error}"));
            return measurement;
        }
    };
    let problem = match fs::read(&case.problem_path) {
        Ok(problem) => problem,
        Err(error) => {
            measurement.error = Some(format!("read_problem: {error}"));
            return measurement;
        }
    };

    let (parse_elapsed, program) = timed(|| HDDLProgram::from_hddl(&domain, Some(&problem)));
    measurement.hddl_parse_ms = Some(ms(parse_elapsed));
    let program = match program {
        Ok(program) => {
            measurement.parse_outcome = "ok".to_string();
            program
        }
        Err(error) => {
            measurement.parse_outcome = "error".to_string();
            measurement.error = Some(format!("hddl_parse: {error:?}"));
            return measurement;
        }
    };

    let (verify_elapsed, verify_result) = timed(|| program.verify());
    measurement.verify_ms = Some(ms(verify_elapsed));
    if let Err(error) = verify_result {
        measurement.verify_outcome = "error".to_string();
        measurement.error = Some(format!("verify: {error:?}"));
        return measurement;
    }
    measurement.verify_outcome = "ok".to_string();

    let (json_export_elapsed, exported) = timed(|| Transpiler::new(program).to_json());
    measurement.json_export_ms = Some(ms(json_export_elapsed));
    measurement.json_export_outcome = "ok".to_string();

    let (json_import_elapsed, reimported) = timed(|| HDDLProgram::from_json(&exported));
    measurement.json_import_ms = Some(ms(json_import_elapsed));
    let reimported = match reimported {
        Ok(program) => {
            measurement.json_import_outcome = "ok".to_string();
            program
        }
        Err(error) => {
            measurement.json_import_outcome = "error".to_string();
            measurement.error = Some(format!("json_import: {error:?}"));
            return measurement;
        }
    };

    let (reimport_verify_elapsed, reimport_verify_result) = timed(|| reimported.verify());
    measurement.reimport_verify_ms = Some(ms(reimport_verify_elapsed));
    if let Err(error) = reimport_verify_result {
        measurement.reimport_verify_outcome = "error".to_string();
        measurement.error = Some(format!("reimport_verify: {error:?}"));
        return measurement;
    }
    measurement.reimport_verify_outcome = "ok".to_string();

    let (json_reexport_elapsed, re_exported) = timed(|| Transpiler::new(reimported).to_json());
    measurement.json_reexport_ms = Some(ms(json_reexport_elapsed));
    measurement.json_reexport_outcome = "ok".to_string();

    let (string_compare_elapsed, string_equal) = timed(|| exported == re_exported);
    measurement.json_string_compare_ms = Some(ms(string_compare_elapsed));
    measurement.json_string_equal = Some(string_equal);

    let (value_compare_elapsed, value_equal) = timed(|| {
        let exported_value = serde_json::from_str::<Value>(&exported);
        let re_exported_value = serde_json::from_str::<Value>(&re_exported);
        matches!((exported_value, re_exported_value), (Ok(lhs), Ok(rhs)) if lhs == rhs)
    });
    measurement.json_value_compare_ms = Some(ms(value_compare_elapsed));
    measurement.json_value_equal = Some(value_equal);

    measurement
}

fn timed<T>(operation: impl FnOnce() -> T) -> (Duration, T) {
    let start = Instant::now();
    let output = operation();
    (start.elapsed(), output)
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

fn print_summary(measurements: &[CaseMeasurement]) {
    let completed = measurements
        .iter()
        .filter(|measurement| measurement.completed())
        .count();
    let failures = measurements.len() - completed;
    println!(
        "summary attempted={} completed={} failures={}",
        measurements.len(),
        completed,
        failures
    );

    let mut slowest: Vec<_> = measurements.iter().collect();
    slowest.sort_by(|lhs, rhs| rhs.total_measured_ms().total_cmp(&lhs.total_measured_ms()));
    for measurement in slowest.into_iter().take(10) {
        let (phase, elapsed) = measurement.slowest_phase();
        println!(
            "slow_case {} total_ms={:.3} slowest_phase={}:{:.3}",
            measurement.case_id,
            measurement.total_measured_ms(),
            phase,
            elapsed
        );
    }

    let equality_disagreements = measurements
        .iter()
        .filter(|measurement| {
            matches!(
                (
                    measurement.json_string_equal,
                    measurement.json_value_equal
                ),
                (Some(string_equal), Some(value_equal)) if string_equal != value_equal
            )
        })
        .count();
    println!("json_equality_disagreements={equality_disagreements}");
}

fn csv(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn csv_opt_ms(value: Option<f64>) -> String {
    value.map(|ms| format!("{ms:.3}")).unwrap_or_default()
}

fn csv_opt_bool(value: Option<bool>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}
