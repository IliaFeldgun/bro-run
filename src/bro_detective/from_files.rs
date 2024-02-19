use std::collections::HashMap;
use regex::Regex;
use std::fs;

extern crate yaml_rust;
use yaml_rust::YamlLoader;


pub fn detect_potential_makers(detectors: &HashMap<String, Vec<String>>, project_dir: String) -> Result<Vec<String>, String> {
    let mut results = Vec::new();
    let project_contents = fs::read_dir(project_dir).unwrap();
    let mut project_root_filenames = String::new();
    for path in project_contents {
        let path = path.unwrap().file_name();
        let path = path.to_str().unwrap();
        project_root_filenames.push_str(&format!("{}\n", path));
    }
    for (detector_name, detect_files) in detectors {
        for detect_file in detect_files {
            let re = Regex::new(&detect_file).unwrap();
            if re.is_match(&project_root_filenames) {
                results.push(String::from(detector_name));
            }
        }
    }
    if results.len() > 0 {
        return Ok(results);
    } else {
        return Err(String::from("No detectors found"));
    }
}


pub fn load_detectors() -> HashMap<String, Vec<String>> {
    let mut detectors = HashMap::new();
    let detector_dir = String::from("yaml_lib/detectors/");
    let paths = fs::read_dir(detector_dir).unwrap();
    for path in paths {
        let path = path.unwrap();
        let file_name = String::from(path.file_name().to_str().unwrap());
        let path = String::from(path.path().to_str().unwrap());
        let detector = fs::read_to_string(path).unwrap();
        println!("Bro, I'm loading detectors from: {}", file_name);
        let detector = YamlLoader::load_from_str(&detector).unwrap();
        let detect_files_list_yaml = detector[0].clone();
        let detect_files_list = detect_files_list_yaml.as_vec().unwrap();
        let mut detect_files = Vec::new();
        for detect_file_yaml in detect_files_list {
            let detect_file = String::from(detect_file_yaml.clone().as_str().unwrap());
            detect_files.push(String::from(detect_file));
        }
        let detector_name = String::from(file_name);
        detectors.insert(detector_name, detect_files);
    }
    if detectors.len() > 0 {
        return detectors;
    } else {
        panic!("No detectors found");
    }
}
