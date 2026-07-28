use clap::Parser;
use hddl_analyzer::{HDDLProgram, Transpiler};
use std::{env, fs};

mod cli_args;

use cli_args::{CLIArgs, Commands};

pub fn main() {
    // ANSI escape color codes
    let yellow = "\x1b[33m";
    let green = "\x1b[32m";
    let red = "\x1b[31m";
    // ANSI escape code to reset text color
    let reset = "\x1b[0m";

    let args = CLIArgs::parse();
    match args.command {
        Commands::Metadata(info) => {
            let domain = fs::read(info.domain_path);
            match domain {
                Ok(domain_content) => match HDDLProgram::from_hddl(&domain_content, None)
                    .and_then(|program| program.metadata())
                {
                    Ok(result) => {
                        print!("{}", result)
                    }
                    Err(error) => {
                        eprintln!("{}[Error]{} {}", red, reset, error)
                    }
                },
                Err(read_error) => {
                    eprintln!("{}[Error]{} {}", red, reset, read_error)
                }
            }
        }
        Commands::Verify(input) => {
            let domain = fs::read(input.domain_path);
            match domain {
                Ok(domain_content) => match input.problem_path {
                    Some(problem_path) => {
                        let problem = fs::read(problem_path);
                        match problem {
                            Ok(problem_content) => {
                                let output =
                                    HDDLProgram::from_hddl(&domain_content, Some(&problem_content))
                                        .and_then(|program| program.verify());
                                match output {
                                    Ok(warnings) => {
                                        for warning in warnings {
                                            println!("{}[Warning]{} {}", yellow, reset, warning);
                                        }
                                        println!("{}[Ok]{}", green, reset);
                                    }
                                    Err(parsing_error) => {
                                        eprintln!("{}[Error]{} {}", red, reset, parsing_error)
                                    }
                                }
                            }
                            Err(read_error) => {
                                eprintln!("{}[Error]{} {}", red, reset, read_error)
                            }
                        }
                    }
                    None => {
                        let output = HDDLProgram::from_hddl(&domain_content, None)
                            .and_then(|program| program.verify());
                        match output {
                            Ok(warnings) => {
                                for warning in warnings {
                                    println!("{}[Warning]{} {}", yellow, reset, warning);
                                }
                                println!("{}[Ok]{}", green, reset);
                            }
                            Err(parsing_error) => {
                                eprintln!("{}[Error]{} {}", red, reset, parsing_error)
                            }
                        }
                    }
                },
                Err(read_error) => {
                    eprintln!("{}[Error]{} {}", red, reset, read_error)
                }
            }
        }
        Commands::Serialize(args) => {
            let domain_bytes = fs::read(args.domain_path);
            match domain_bytes {
                Ok(domain_content) => match args.problem_path {
                    Some(problem_path) => {
                        let problem_bytes = fs::read(problem_path);
                        match problem_bytes {
                            Ok(problem_content) => {
                                let json_string =
                                    HDDLProgram::from_hddl(&domain_content, Some(&problem_content))
                                        .map(|program| Transpiler::new(program).to_json());
                                match json_string {
                                    Ok(output_string) => {
                                        match args.output_file {
                                            None => {
                                                println!("{}[Ok]{}", green, reset);
                                                println!("{}", output_string);
                                            }
                                            Some(output_file) => {
                                                let mut output_path = env::current_dir().unwrap();
                                                output_path.push(&output_file);
                                                match fs::write(&output_path, output_string) {
                                                    Ok(_) => {
                                                        //println!("Result successfully written to {}.", output_file);
                                                        println!("{:?}", output_path);
                                                    }
                                                    Err(err) => {
                                                        eprintln!("{}[Error]{} {}", red, reset, err)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(parsing_error) => {
                                        eprintln!("{}[Error]{} {}", red, reset, parsing_error)
                                    }
                                }
                            }
                            Err(read_error) => {
                                eprintln!("{}[Error]{} {}", red, reset, read_error)
                            }
                        }
                    }
                    None => {
                        let json_string = HDDLProgram::from_hddl(&domain_content, None)
                            .map(|program| Transpiler::new(program).to_json());
                        match json_string {
                            Ok(output_string) => {
                                match args.output_file {
                                    None => {
                                        println!("{}[Ok]{}", green, reset);
                                        println!("{}", output_string);
                                    }
                                    Some(output_file) => {
                                        let mut output_path = env::current_dir().unwrap();
                                        output_path.push(&output_file);
                                        match fs::write(&output_path, output_string) {
                                            Ok(_) => {
                                                println!("Result successfully written to {}", output_file);
                                            }
                                            Err(err) => {
                                                eprintln!("{}[Error]{} {}", red, reset, err)
                                            }
                                        }
                                    }
                                }
                            }
                            Err(parsing_error) => {
                                eprintln!("{}[Error]{} {}", red, reset, parsing_error)
                            }
                        }
                    }
                },
                Err(read_error) => {
                    eprintln!("{}[Error]{} {}", red, reset, read_error)
                }
            }
        }
        Commands::Deserialize(args) => {
            let json = fs::read_to_string(args.domain_path);
            match json {
                Ok(json_content) => {
                    let output = HDDLProgram::from_json(&json_content);
                    match output {
                        Ok(program) => {
                            match program.verify() {
                                Ok(warnings) => {
                                    for warning in warnings {
                                        println!("{}[Warning]{} {}", yellow, reset, warning);
                                    }
                                    println!("{}[Ok]{}", green, reset);
                                }
                                Err(parsing_error) => {
                                    eprintln!("{}[Error]{} {}", red, reset, parsing_error);
                                    return;
                                }
                            }
                            if let Some(output_file) = args.output_file {
                                let output_string = Transpiler::new(program).to_json();
                                let mut output_path = env::current_dir().unwrap();
                                output_path.push(&output_file);
                                match fs::write(&output_path, output_string) {
                                    Ok(_) => {
                                        println!("Result successfully written to {}", output_file);
                                    }
                                    Err(err) => {
                                        eprintln!("{}[Error]{} {}", red, reset, err)
                                    }
                                }
                            }
                        }
                        Err(parsing_error) => {
                            eprintln!("{}[Error]{} {}", red, reset, parsing_error)
                        }
                    }
                }
                Err(read_error) => {
                    eprintln!("{}[Error]{} {}", red, reset, read_error)
                }
            }
        }
    }
}
