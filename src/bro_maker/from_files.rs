use crate::bro_common::yaml_loader::load_yaml_as_vec;
use std::{
    collections::HashMap,
    process::{Command, Output},
};
type MakerResults = HashMap<String, Result<Output, String>>;

pub fn build(makers: Vec<String>) -> Result<MakerResults, String> {
    let mut results: MakerResults = HashMap::new();
    for maker in makers {
        install_maker(maker.clone()).map_err(|e| format!("Failed to install maker {}", e))?;
        let commands = get_maker_commands(maker.clone())
            .map_err(|e| format!("Failed to get maker commands {}", e))?;
        for build_command in commands {
            results.insert(build_command.clone(), run_command(build_command.clone()));
        }
    }
    Ok(results)
}

fn run_command(command: String) -> Result<Output, String> {
    println!("{}-ing, bro!", command);
    let build_args = ["-c", &command];
    match Command::new("bash").args(build_args).spawn() {
        Ok(process) => match process.wait_with_output() {
            Ok(out) => Ok(out),
            Err(e) => Err(format!("Failed to execute {}", e)),
        },
        Err(e) => Err(format!("Failed to execute {}", e)),
    }
}

fn install_maker(maker: String) -> Result<String, String> {
    Ok(maker)
}

fn get_maker_commands(maker: String) -> Result<Vec<String>, String> {
    let maker_dir = String::from("yaml_lib/makers");
    let maker = maker.replace(".yaml", "");
    let maker_build_path = format!("{}/{}/build.yaml", maker_dir, maker);
    println!("Bro, I can make this, detected {}!", maker);
    load_yaml_as_vec(maker_build_path)
}
