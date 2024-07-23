
pub fn run(command: String, dir: Option<String>, sanbox_run: fn(String, Option<String>) -> Result<i32, String>) -> Result<i32, String>{
    sanbox_run(command, dir)
}

