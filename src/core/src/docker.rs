use std::process::Command;

/// Launches a Docker container using the `docker` CLI.
/// 
/// # Arguments
/// - `name`: Name to assign to the container
/// - `image`: Docker image to run (e.g., `nginx:latest`)
/// - `ports`: List of ports to expose (host:container binding)
/// 
/// # Returns
/// - `Ok(container_name)` on success
/// - `Err(message)` on failure
pub fn launch_container(name: &str, image: &str, ports: &[u16]) -> Result<String, String> {
    let port_args: Vec<String> = ports
        .iter()
        .flat_map(|port| vec!["-p".to_string(), format!("{}:{}", port, port)])
        .collect();

    let mut args = vec!["run", "-d", "--rm", "--name", name];
    args.extend(port_args.iter().map(|s| s.as_str()));
    args.push(image);

    let output = Command::new("docker")
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to execute docker command: {}", e))?;

    if output.status.success() {
        let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(container_id)
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Docker error: {}", err.trim()))
    }
}
