use bro_cli::cli::build_cli;
use bro_cli::cli::process_cli;
use bro_sandbox::from_files::get_sandboxes;

mod bro_cli;
mod bro_common;
mod bro_detective;
mod bro_maker;
mod bro_sandbox;

fn main() {
    let sandboxes = get_sandboxes().expect("Failed to initialize sandboxes argument");
    let _ = process_cli(build_cli(sandboxes));
}
