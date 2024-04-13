use crate::{bro_common::command::run_command_sync, bro_common::yaml_loader::load_yaml_as_vec};
use std::{collections::HashMap, process::Output};
type MakerResults = HashMap<String, Result<Output, String>>;

pub fn build(makers: Vec<String>, project_path: String) -> Result<MakerResults, String> {
    let mut results: MakerResults = HashMap::new();
    for maker in makers {
        install_maker(maker.clone()).map_err(|e| format!("Failed to install maker {}", e))?;
        let commands = get_maker_commands(maker.clone())
            .map_err(|e| format!("Failed to get maker commands {}", e))?;
        for build_command in commands {
            println!("{}-ing, bro!", build_command);
            results.insert(
                build_command.clone(),
                run_command_sync(build_command.clone(), Some(project_path.clone()), false),
            );
        }
    }
    Ok(results)
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
