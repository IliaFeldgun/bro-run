use bro_cli::cli::build_cli;
use bro_cli::cli::process_cli;
use bro_sandbox::from_files::get_sandboxes;

mod bro_cli;
mod bro_config;
mod bro_common;
mod bro_detective;
mod bro_maker;
mod bro_sandbox;
mod bro_scm;

fn main() {
    let sandboxes = get_sandboxes().expect("Failed to initialize sandboxes argument");
    match process_cli(build_cli(sandboxes)) {
        Ok(status) => {
            if status == 0 {
                println!("Looks good, bro!")
            } else {
                println!("Failed with status {}", status)
            }
        }
        Err(e) => print!("Fatal Error {}", e),
    }
}
