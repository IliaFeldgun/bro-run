extern crate yaml_rust;
use std::{fs, process::Command};
use yaml_rust::YamlLoader;

pub fn build(makers: Vec<String>) {
    for maker in makers {
        install_maker(maker.clone()).unwrap();
        let commands = get_maker_commands(maker.clone());
        for build_command in commands {
            println!("{}-ing, bro!", build_command);
            let build_args = ["-c", &build_command];
            let command = Command::new("bash").args(build_args).spawn().unwrap();
            let output = command.wait_with_output().unwrap();
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }        
}

fn install_maker(maker: String) -> Result<String, String> {
    return Ok(maker);
}


fn get_maker_commands(maker: String) -> Vec<String> {
    let maker_dir = String::from("yaml_lib/makers");
    let maker = maker.replace(".yaml", "");
    let maker_build_path = format!("{}/{}/build.yaml", maker_dir, maker);
    println!("Bro, I can make this, detected {}!", maker);
    let maker_build_file = fs::read_to_string(maker_build_path).unwrap();
    let maker_build_yaml = YamlLoader::load_from_str(&maker_build_file).unwrap();
    let maker_build_commands_yaml = maker_build_yaml[0].clone();
    let maker_build_commands = maker_build_commands_yaml.as_vec().unwrap();
    let mut commands = Vec::new();
    for command_yaml in maker_build_commands {
        let command = String::from(command_yaml.clone().as_str().unwrap());
        commands.push(command);
    }
    return commands;
}