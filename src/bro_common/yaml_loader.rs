use std::fs;

use yaml_rust2::{Yaml, YamlLoader};

fn get_yaml_docs(yaml_path: String) -> Result<Vec<Yaml>, String> {
    let file = fs::read_to_string(yaml_path).map_err(|e| format!("Could not read file {}", e))?;
    let yaml_docs =
        YamlLoader::load_from_str(&file).map_err(|e| format!("Could not load yaml {}", e))?;
    Ok(yaml_docs)
}

pub fn load_yaml_as_vec(yaml_path: String) -> Result<Vec<String>, String> {
    let first_doc = get_yaml_docs(yaml_path)
        .map_err(|e| format!("Failed to load yaml docs  {}", e))?[0]
        .clone();
    let mut output = Vec::new();
    match first_doc.as_vec() {
        Some(yaml_vector) => {
            for string_yaml in yaml_vector {
                match string_yaml.as_str() {
                    Some(parsed_string) => output.push(String::from(parsed_string)),
                    None => return Err(String::from("Failed to load as string")),
                }
            }
        }
        None => return Err(String::from("Failed to load as vector")),
    }
    Ok(output)
}
