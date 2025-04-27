use std::process::Command;
use crate::routes::apps::{AppInstance, AppStatus};

/// Launches a Docker container using the `docker` CLI.
///
/// # Arguments
/// - `name`: Name to assign to the container
/// - `image`: Docker image to run (e.g., `nginx:latest`)
/// - `host_ports`: List of ports to expose (host:container binding)
/// - `container_port`: Internal port exposed by the container (e.g., 80 for nginx)
///
/// # Returns
/// - `Ok(container_id)` on success
/// - `Err(message)` on failure
pub fn launch_container(
    name: &str,
    image: &str,
    host_ports: &[u16],
    container_port: u16,
) -> Result<String, String> {
    let port_args: Vec<String> = host_ports
        .iter()
        .flat_map(|host| vec!["-p".to_string(), format!("{host}:{container_port}")])
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

/// Lists running Docker containers using `docker ps`.
///
/// # Returns
/// - `Ok(Vec<AppInstance>)` containing all running containers
/// - `Err(message)` if the Docker command fails
pub fn get_running_containers() -> Result<Vec<AppInstance>, String> {
    let output = Command::new("docker")
        .args(["ps", "--format", "{{.ID}};{{.Names}};{{.Image}};{{.Status}};{{.Ports}}"])
        .output()
        .map_err(|e| format!("Failed to execute docker ps: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Docker error: {}", stderr.trim()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let containers = stdout
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() < 5 {
                return None;
            }

            Some(AppInstance {
                id: idx as u32 + 1, // Locally assigned ID (not Docker ID)
                name: parts[1].to_string(),
                image: parts[2].to_string(),
                status: parse_status(parts[3]),
                ports: parse_ports(parts[4]),
                created_at: "".to_string(), // Creation date not available from `docker ps`
            })
        })
        .collect();

    Ok(containers)
}

/// Retrieves information about a single container by its name.
///
/// # Arguments
/// - `name`: The Docker container name.
///
/// # Returns
/// - `Ok(Some(AppInstance))` if found
/// - `Ok(None)` if not found
/// - `Err(message)` if an error occurred
pub fn get_container_by_name(name: &str) -> Result<Option<AppInstance>, String> {
    let output = Command::new("docker")
        .args(["inspect", name])
        .output()
        .map_err(|e| format!("Failed to execute docker inspect: {}", e))?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let containers: Vec<serde_json::Value> = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse docker inspect output: {}", e))?;

    if containers.is_empty() {
        return Ok(None);
    }

    let container = &containers[0];
    let name = container["Name"].as_str().unwrap_or_default().trim_start_matches('/').to_string();
    let image = container["Config"]["Image"].as_str().unwrap_or_default().to_string();
    let created_at = container["Created"].as_str().unwrap_or_default().to_string();

    let ports = if let Some(ports) = container["NetworkSettings"]["Ports"].as_object() {
        ports.keys()
            .filter_map(|k| {
                k.split('/').next()
                    .and_then(|port| port.parse::<u16>().ok())
            })
            .collect()
    } else {
        vec![]
    };

    Ok(Some(AppInstance {
        id: 0,
        name,
        status: AppStatus::Running,
        image,
        ports,
        created_at,
    }))
}

/// Removes a Docker container by name.
/// 
/// # Arguments
/// - `name`: The container name to remove.
/// 
/// # Returns
/// - `Ok(())` if deleted successfully
/// - `Err(message)` if failed
pub fn remove_container(name: &str) -> Result<(), String> {
    let output = std::process::Command::new("docker")
        .args(["rm", "-f", name])
        .output()
        .map_err(|e| format!("Failed to execute docker rm: {}", e))?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    if stderr.contains("No such container") || stderr.contains("Error: No such container") {
        return Err("No such container".to_string());
    }

    if output.status.success() {
        Ok(())
    } else {
        Err(stderr.trim().to_string())
    }
}

/// Parses the status string from `docker ps` into an `AppStatus`.
///
/// # Arguments
/// - `status`: Raw status string (e.g., "Up 5 minutes" or "Exited (0) 2 hours ago")
///
/// # Returns
/// - `AppStatus::Running`, `Stopped`, or `Error`
fn parse_status(status: &str) -> AppStatus {
    if status.contains("Up") {
        AppStatus::Running
    } else if status.contains("Exited") {
        AppStatus::Stopped
    } else {
        AppStatus::Error
    }
}

/// Parses the ports string from `docker ps` into a list of `u16` host ports.
///
/// # Arguments
/// - `ports_info`: Raw ports string (e.g., "0.0.0.0:8080->80/tcp, :::8080->80/tcp")
///
/// # Returns
/// - `Vec<u16>` list of exposed host ports
fn parse_ports(ports_info: &str) -> Vec<u16> {
    ports_info
        .split(',')
        .filter_map(|entry| {
            let parts: Vec<&str> = entry.trim().split("->").collect();
            if !parts.is_empty() {
                if let Some(port_part) = parts[0].split(':').last() {
                    return port_part.parse::<u16>().ok();
                }
            }
            None
        })
        .collect()
}
