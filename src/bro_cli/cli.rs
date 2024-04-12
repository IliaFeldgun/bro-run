use crate::bro_common::file_browser::get_full_path;
use crate::bro_detective::from_files::{detect_potential_makers, load_detectors};
use crate::bro_maker::from_files::build;
use crate::bro_scm::get_source::get_source;

use clap::{arg, command, ArgMatches, Command};

pub fn build_cli(sandbox_list: Vec<String>) -> ArgMatches {
    command!() // requires `cargo` feature
        .subcommand(
            Command::new("config")
                .about("Configure the sandbox, bro!.")
                .arg(arg!([sandbox_name] "Name of the sandbox").value_parser(sandbox_list)),
        )
        .subcommand(
            Command::new("run")
                .about("Bro, run this repo for me")
                .arg(arg!(
                    [git_repo] "git repository url or root directory"
                )),
        )
        .get_matches()
}

pub fn process_cli(matches: clap::ArgMatches) -> Result<i32, String> {
    if let Some(matches) = matches.subcommand_matches("config") {
        if let Some(sandbox_name) = matches.get_one::<String>("sandbox_name") {
            println!("Configuring the sandbox to {}, bro!", sandbox_name);
            return Ok(0);
        }
    }
    if let Some(matches) = matches.subcommand_matches("run") {
        if let Some(git_repo) = matches.get_one::<String>("git_repo") {
            let git_repo = String::from(git_repo);
            println!("Getting git repo {}, bro!", git_repo);
            let source_code_path: String;
            if let Some(repo_path) = get_full_path(git_repo.clone()) {
                source_code_path = repo_path
            } else {
                match get_source(git_repo.clone()) {
                    Ok(repo_path) => source_code_path = repo_path,
                    Err(e) => return Err(format!("Failed to get source {}", e)),
                }
            }
            println!("Running git repo {}, bro!", source_code_path);
            let detectors =
                load_detectors().map_err(|e| format!("Failed to load detectors {}", e))?;
            let makers = detect_potential_makers(&detectors, source_code_path.clone());
            match makers {
                Ok(makers) => {
                    build(makers, source_code_path)
                        .map_err(|e| format!("Failed to build {}", e))?;
                    return Ok(0);
                }
                Err(e) => {
                    return Err(format!("No potential makers found: {}", e));
                }
            }
        }
    }
    Ok(1)
}
