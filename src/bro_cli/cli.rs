use crate::bro_sandboxes::from_files::get_sandboxes;
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
            println!(
                "Running git repo {}, bro!",
                matches.get_one::<String>("git_repo").unwrap()
            );
        }
    }
}
