use crate::bro_common::{file_browser::dir_contents_to_strings, yaml_loader::load_yaml_as_vec};

const SCM_DIR: &str = "yaml_lib/source-control/";

pub fn get_scms() -> Result<Vec<String>, String> {
    let mut scms: Vec<String> = Vec::new();
    match dir_contents_to_strings(SCM_DIR.to_string()) {
        Ok(info_vector) => {
            for file_info in info_vector {
                scms.push(file_info.name);
            }
        }
        Err(e) => return Err(format!("Failed to detect scms {}", e)),
    }
    if scms.is_empty() {
        return Err("No scms found".to_string());
    }
    Ok(scms)
}

pub fn get_scm_command(scm: String) -> Result<String, String> {
    let get_command_path = format!("{}/{}/{}.yaml", SCM_DIR, scm, "get");
    match load_yaml_as_vec(get_command_path) {
        Ok(commands) => {
            if let Some(cmd) = commands.first() {
                Ok(String::from(cmd))
            } else {
                Err(format!("Failed to get {} get command", scm))
            }
        }
        Err(e) => Err(format!("Failed to get {} get command {}", scm, e)),
    }
}
