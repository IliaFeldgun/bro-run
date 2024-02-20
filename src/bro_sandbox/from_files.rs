use std::fs;

pub fn get_sandboxes() -> Vec<String> {
    let sandbox_dir = String::from("yaml_lib/sandboxes/");
    let mut sandboxes = Vec::new();
    let paths = fs::read_dir(sandbox_dir).unwrap();
    for path in paths {
        let path = path.unwrap().file_name();
        let path = path.to_str().unwrap();
        sandboxes.push(String::from(path));
    }
    return sandboxes;
}
