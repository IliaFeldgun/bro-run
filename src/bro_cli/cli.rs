use std::fs;

use crate::bro_maker::from_files::build;
use crate::bro_sandbox::from_files::get_sandboxes;
use crate::bro_detective::from_files::{detect_potential_makers, load_detectors};

use clap::{arg, command, ArgMatches, Command};

pub fn build_cli() -> ArgMatches {
    let matches = command!() // requires `cargo` feature
        .subcommand(
            Command::new("config")
                .about("Configure the sandbox, bro!.")
                .arg(arg!([sandbox_name] "Name of the sandbox").value_parser(get_sandboxes())),
        )
        .subcommand(
            Command::new("run")
                .about("Bro, run this repo for me")
                .arg(arg!(
                    [git_repo] "git repository url or root directory"
                )),
        )
        .get_matches();
    return matches;
}

pub fn process_cli(matches: clap::ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("config") {
        if matches.get_one::<String>("sandbox_name").is_some() {
            println!(
                "Configuring the sandbox to {}, bro!",
                matches.get_one::<String>("sandbox_name").unwrap()
            );
        }
    }
    if let Some(matches) = matches.subcommand_matches("run") {
        if matches.get_one::<String>("git_repo").is_some() {
            let git_repo = String::from(matches.get_one::<String>("git_repo").unwrap());
            println!("Running git repo {}, bro!", fs::canonicalize(git_repo.clone()).unwrap().to_str().unwrap());
            let detectors = load_detectors();
            let makers = detect_potential_makers(&detectors, git_repo);
            match makers {
                Ok(makers) => {
                    build(makers);
                }
                Err(e) => {
                    println!("No potential makers found: {}", e);
                }
            }
        }
    }
}
