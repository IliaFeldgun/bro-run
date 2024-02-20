use crate::bro_common::file_browser::dir_contents_to_strings;

pub fn get_sandboxes() -> Result<Vec<String>, String> {
    let sandbox_dir = String::from("yaml_lib/sandboxes/");
    let mut sandboxes: Vec<String> = Vec::new();
    match dir_contents_to_strings(sandbox_dir) {
        Ok(info_vector) => {
            for file_info in info_vector {
                sandboxes.push(file_info.name);
            }
        }
        Err(e) => return Err(format!("Failed to detect sandboxes {}", e)),
    }
    if sandboxes.is_empty() {
        return Err(String::from("No sandboxes found"));
    }
    Ok(sandboxes)
}
