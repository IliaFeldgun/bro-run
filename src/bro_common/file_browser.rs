use std::{
    fs::{self, DirEntry},
    io::Error,
    time::SystemTime,
};

pub struct FileInfo {
    pub name: String,
    pub full_known_path: String,
}

fn get_file_info(dir_entry: Result<DirEntry, Error>) -> Result<FileInfo, Error> {
    match dir_entry {
        Ok(path) => {
            let info = FileInfo {
                name: match path.file_name().to_str() {
                    Some(file_name) => String::from(file_name),
                    None => return Err(Error::other("Failed to parse file name")),
                },
                full_known_path: match path.path().to_str() {
                    Some(file_path) => String::from(file_path),
                    None => return Err(Error::other("Failed to parse file path")),
                },
            };
            Ok(info)
        }
        Err(e) => Err(e),
    }
}

pub fn dir_contents_to_strings(path: String) -> Result<Vec<FileInfo>, String> {
    let mut output: Vec<FileInfo> = Vec::new();
    let dir_contents =
        fs::read_dir(path).map_err(|e| format!("Failed to read directory: {}", e))?;
    for subpath in dir_contents {
        match get_file_info(subpath) {
            Ok(info) => output.push(info),
            Err(e) => {
                return Err(format!("{}", e));
            }
        }
    }
    Ok(output)
}

pub fn get_full_path(path: String) -> Option<String> {
    match fs::canonicalize(path) {
        Ok(full_path) => Some(String::from(full_path.to_str()?)),
        Err(_) => None,
    }
}

pub fn get_changes_since(directory: String, time: SystemTime) -> Result<Vec<FileInfo>, String> {
    let mut dirs_changed: Vec<FileInfo> = Vec::new();
    let dir_contents = fs::read_dir(directory).map_err(|e| format!("Failed to read dir {}", e))?;
    for subpath in dir_contents {
        let subpath_entry = subpath.map_err(|e| format!("Failed to read path metadata {}", e))?;
        let metadata = subpath_entry
            .metadata()
            .map_err(|e| format!("Failed to read metadata {}", e))?;
        if metadata
            .modified()
            .map_err(|e| format!("Failed to get modified date {}", e))?
            .ge(&time)
        {
            if let Ok(info) = get_file_info(Ok(subpath_entry)) {
                dirs_changed.push(info)
            }
        }
    }
    Ok(dirs_changed)
}
