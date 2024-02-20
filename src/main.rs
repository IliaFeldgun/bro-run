use bro_cli::cli::build_cli;
use bro_cli::cli::process_cli;

mod bro_cli;
mod bro_sandbox;
mod bro_detective;
mod bro_maker;

fn main() {
    process_cli(build_cli());
}
