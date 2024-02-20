use crate::bro_common::file_browser::dir_contents_to_strings;
use crate::bro_common::yaml_loader::load_yaml_as_vec;
use regex::Regex;
use std::collections::HashMap;

pub fn detect_potential_makers(
    detectors: &HashMap<String, Vec<String>>,
    project_dir: String,
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();
    let mut project_root_filenames = String::new();
    let project_contents = dir_contents_to_strings(project_dir)
        .map_err(|e| format!("Failed to list project contents {}", e))?;
    for file_info in project_contents {
        project_root_filenames.push_str(&format!("{}\n", file_info.name))
    }
    for (detector_name, detect_files) in detectors {
        for detect_file in detect_files {
            let re =
                Regex::new(detect_file).map_err(|e| format!("Failed to compile regex {}", e))?;
            if re.is_match(&project_root_filenames) {
                results.push(String::from(detector_name));
            }
        }
    }
    if results.is_empty() {
        return Err(String::from("No detectors found"));
    }
    Ok(results)
}

pub fn load_detectors() -> Result<HashMap<String, Vec<String>>, String> {
    let mut detectors = HashMap::new();
    let detector_dir = String::from("yaml_lib/detectors/");
    let detector_files = dir_contents_to_strings(detector_dir)
        .map_err(|e| format!("Failed to list detectors {}", e))?;
    for detector_file in detector_files {
        let file_name = detector_file.name;
        let path = detector_file.full_known_path;
        println!("Bro, I'm loading detectors from: {}", file_name);
        match load_yaml_as_vec(path) {
            Ok(detector_vector) => {
                detectors.insert(file_name, detector_vector);
            }
            Err(e) => return Err(format!("Failed to load detectors vector {}", e)),
        }
    }
    if detectors.is_empty() {
        return Err(String::from("No detectors found"));
    }
    Ok(detectors)
}
