use std::process::{Command, Output};

pub fn run_command_sync(command: String, dir: Option<String>) -> Result<Output, String> {
    let build_args = ["-c", &command];
    let mut cmd_to_run = Command::new("bash");
    cmd_to_run.args(build_args);
    if let Some(run_dir) = dir {
        cmd_to_run.current_dir(run_dir);
    }
    match cmd_to_run.spawn() {
        Ok(process) => match process.wait_with_output() {
            Ok(out) => Ok(out),
            Err(e) => Err(format!("Failed to execute {}", e)),
        },
        Err(e) => Err(format!("Failed to execute {}", e)),
    }
}
