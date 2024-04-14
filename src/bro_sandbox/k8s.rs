use std::{borrow::Borrow, collections::HashMap, fmt::format, io::Read, thread, time::Duration};

use crate::bro_common::{command::run_command_sync, file_browser::get_full_path, templating::generate_file_from_template};

const JOB_YAML_TEMPLATE: &str = "yaml_lib/sandboxes/minikube/resources/one-job.template.yaml";
const K8S_NAMESPACE: &str = "bro-sandbox";
const POD_NAME: &str = "bro-sandbox-pod";
const IMAGE_NAME: &str = "bro-sandbox:latest";


pub fn run_command_job(command: String, path: Option<String>) -> Result<i32, String> {
    let mut template_map = HashMap::new();
    template_map.insert("image_name", IMAGE_NAME.to_string());
    template_map.insert("name", K8S_NAMESPACE.to_string());
    template_map.insert("bro_cmd", command.clone());
    if let Some(mount_path) = get_full_path(path) {
        template_map.insert("mount_path", mount_path.to_string());
    } else if let Some(mount_path) = get_full_path(Some(String::from("bro_scm"))) {
        template_map.insert("mount_path", mount_path.to_string());
    } else {
        return Err(String::from("Failed to get mount path"));
    }
    let job_yaml = match generate_file_from_template(JOB_YAML_TEMPLATE.to_string(), template_map) {
        Ok(yaml_path) => yaml_path,
        Err(e) => return Err(e)
    };
    let create_pod_command = format!("apply -f {}", job_yaml);
    let follow_logs_command = format!("logs -f pod/{}", POD_NAME);
    run_k8s_command(create_pod_command, false);
    wait_out_of_pending(POD_NAME.to_string());
    run_k8s_command(follow_logs_command, false);
    // let clean_pod_command = format!("delete pod/{}", POD_NAME);
    // run_k8s_command(clean_pod_command, false);
    Ok(0)
}

fn wait_out_of_pending(pod_name: String) {
    let get_phase_commad = format!("get pod/{} -o=go-template {}", pod_name, "--template='{{.status.phase}}'");
    while is_pending(run_k8s_command(get_phase_commad.clone(), true)) {
        thread::sleep(Duration::from_secs(1));
    }
}

fn is_pending(stdout: Option<Vec<u8>>) -> bool {
    if let Some(output) = stdout {
        match String::from_utf8(output) {
            Ok(string_output) => { 
                println!("Waiting for pod, status: {}", string_output);
                return string_output.contains("Pending"); 
            }
            Err(_) => {return false}
        }
    }
    false
}

fn run_k8s_command(k8s_command: String, capture_output: bool) -> Option<Vec<u8>> {
    let namespace_arg = format!("-n {}", K8S_NAMESPACE);
    let command = format!("minikube kubectl -- {} {}", namespace_arg, k8s_command);
    match run_command_sync(command, None, capture_output) {
        Ok(output) => {
            Some(output.clone().stdout)
        }
        Err(_) => None
    }
}
