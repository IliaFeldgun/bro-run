use std::{fs, process::Output, time::SystemTime};

use crate::bro_common::{command::run_command_sync, file_browser::get_changes_since};

use super::from_files::{get_scm_command, get_scms};

const SCM_DIR: &str = "scm_dir";

pub fn get_source(source: String) -> Result<String, String> {
    let mut time = SystemTime::now();
    let command_output = match detect_scm(source.clone()) {
        Some(scm) => {
            time = SystemTime::now();
            run_scm_command(source.clone(), scm)
        }
        None => Err(format!("Failed to detect scm of {}", source)),
    };
    match command_output {
        Ok(output) => {
            if output.status.success() {
                match get_changes_since(SCM_DIR.to_string(), time) {
                    Ok(paths) => {
                        for path in paths {
                            if source.contains(&path.name) {
                                return Ok(path.full_known_path);
                            }
                        }
                        Err(format!("Failed to detect dir after getting {}", source))
                    }
                    Err(e) => Err(e),
                }
            } else {
                Err(format!(
                    "Failed to run source get command {}",
                    output.status
                ))
            }
        }
        Err(e) => Err(format!("Failed to run source get command {}", e)),
    }
}

fn run_scm_command(source: String, scm: String) -> Result<Output, String> {
    let get_command = match get_scm_command(scm) {
        Ok(prefix) => {
            println!("{}-ing, bro!", prefix);
            format!("{} {}", prefix, source)
        }
        Err(e) => return Err(e),
    };
    match fs::create_dir_all(SCM_DIR) {
        Ok(_) => run_command_sync(get_command, Some(SCM_DIR.to_string())),
        Err(e) => Err(format!("Failed to create dir {} for scm checkouts", e)),
    }
}

fn detect_scm(source: String) -> Option<String> {
    match get_scms() {
        Ok(scms) => {
            for scm in scms {
                if source.contains(&scm) {
                    return Some(scm);
                }
            }
            None
        }
        Err(_) => None,
    }
}
