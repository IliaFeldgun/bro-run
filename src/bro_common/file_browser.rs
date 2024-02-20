use std::fs;
pub struct FileInfo {
    pub name: String,
    pub full_known_path: String,
}

pub fn dir_contents_to_strings(path: String) -> Result<Vec<FileInfo>, String> {
    let mut output: Vec<FileInfo> = Vec::new();
    let dir_contents =
        fs::read_dir(path).map_err(|e| format!("Failed to read directory: {}", e))?;
    for subpath in dir_contents {
        match subpath {
            Ok(path) => {
                let info = FileInfo {
                    name: match path.file_name().to_str() {
                        Some(file_name) => String::from(file_name),
                        None => return Err(String::from("Failed to get file name")),
                    },
                    full_known_path: match path.path().to_str() {
                        Some(file_path) => String::from(file_path),
                        None => return Err(String::from("Failed to get file path")),
                    },
                };
                output.push(info)
            }
            Err(e) => {
                return Err(format!("{}", e));
            }
        }
    }
    Ok(output)
}
