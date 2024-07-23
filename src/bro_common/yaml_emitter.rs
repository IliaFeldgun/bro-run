use std::{collections::HashMap, fs};

use tera::{Context, Tera};


pub fn generate_yaml_from_template(path: String, kv: HashMap<String, String>) -> Result<String, String> {
    let template_string = fs::read_to_string(path).map_err(|e| format!("Could not read file {}", e))?;
    match populate_template(template_string, kv) {
        Ok(output) => {
            let new_path = path.replace(".template", "");
            fs::write(new_path, output).map_err(|e| format!("Could not write file {}", e))
        }
        Err(e) => {
            Err(e)
        }
    }

}

fn populate_template(source: String, kv: HashMap<String,String>) -> Result<String, String>{
    let mut tera = Tera::default();
    tera.add_raw_template("populating", &source).map_err(|e| format!("Failed to populate template {}", e))?;
    let mut tcontext = Context::new();
    for kv_pair in kv {
        tcontext.insert(kv_pair.0.clone(), &kv_pair.1.clone());
    }
    tera.render("populating", &tcontext).map_err(|e| format!("Failet to populate template {}", e))
}
