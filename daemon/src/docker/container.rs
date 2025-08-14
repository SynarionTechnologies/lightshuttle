use std::{collections::HashMap, process::Command};

use crate::{errors::Error, services::docker::DockerClient};

use super::{
    models::{AppInstance, AppStatus},
    ContainerConfig,
};

/// Recreates a Docker container by name: stops, deletes, and restarts it with same config.
///
/// # Arguments
/// - `name`: The container to recreate
///
/// # Returns
/// - `Ok(container_id)` if successful
/// - `Err(Error)` if failed
pub fn recreate_container(client: &dyn DockerClient, name: &str) -> Result<String, Error> {
    let output = client.inspect(name)?;
    let container: Vec<serde_json::Value> =
        serde_json::from_str(&output).map_err(|e| Error::DockerOutputParse(e.to_string()))?;

    if container.is_empty() {
        return Err(Error::ContainerNotFound);
    }

    let cfg = &container[0];

    let image = cfg["Config"]["Image"]
        .as_str()
        .ok_or_else(|| Error::DockerOutputParse("Missing image".into()))?;

    let ports = cfg["NetworkSettings"]["Ports"]
        .as_object()
        .ok_or_else(|| Error::DockerOutputParse("Missing ports".into()))?;

    let container_port = ports
        .keys()
        .filter_map(|k| k.split('/').next())
        .filter_map(|p| p.parse::<u16>().ok())
        .next()
        .ok_or_else(|| Error::DockerOutputParse("No container port found".into()))?;

    let host_ports: Vec<u16> = ports
        .values()
        .filter_map(|v| v.as_array())
        .flatten()
        .filter_map(|binding| binding["HostPort"].as_str()?.parse().ok())
        .collect();

    let labels = cfg["Config"]["Labels"].as_object().map(|map| {
        map.iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
            .collect::<HashMap<String, String>>()
    });

    let env_vars = cfg["Config"]["Env"].as_array().map(|vars| {
        vars.iter()
            .filter_map(|v| v.as_str())
            .filter_map(|kv| {
                let mut split = kv.splitn(2, '=');
                let k = split.next()?;
                let v = split.next().unwrap_or("");
                Some((k.to_string(), v.to_string()))
            })
            .collect::<HashMap<String, String>>()
    });

    let volumes = cfg["HostConfig"]["Binds"].as_array().map(|items| {
        items
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    });

    let restart_policy = cfg["HostConfig"]["RestartPolicy"]["Name"]
        .as_str()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());

    remove_container(name)?;

    client.run(ContainerConfig {
        name,
        image,
        host_ports: &host_ports,
        container_port,
        labels: labels.as_ref(),
        env: env_vars.as_ref(),
        volumes: volumes.as_ref(),
        restart_policy: restart_policy.as_deref(),
    })
}

/// Lists running Docker containers using `docker ps`.
///
/// # Returns
/// - `Ok(Vec<AppInstance>)` containing all running containers
/// - `Err(Error)` if the Docker command fails
pub fn get_running_containers() -> Result<Vec<AppInstance>, Error> {
    let output = Command::new("docker")
        .args([
            "ps",
            "--format",
            "{{.ID}};{{.Names}};{{.Image}};{{.Status}};{{.Ports}}",
        ])
        .output()
        .map_err(|_| Error::DockerCommandFailed)?;

    if !output.status.success() {
        return Err(Error::DockerCommandFailed);
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
                id: idx as u32 + 1,
                name: parts[1].to_string(),
                image: parts[2].to_string(),
                status: parse_status(parts[3]),
                ports: parse_ports(parts[4]),
                created_at: "".to_string(),
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
/// - `Err(Error)` if an error occurred
pub fn get_container_by_name(
    client: &dyn DockerClient,
    name: &str,
) -> Result<Option<AppInstance>, Error> {
    let output = match client.inspect(name) {
        Ok(o) => o,
        Err(Error::ContainerNotFound) => return Ok(None),
        Err(e) => return Err(e),
    };

    let containers: Vec<serde_json::Value> =
        serde_json::from_str(&output).map_err(|e| Error::DockerOutputParse(e.to_string()))?;

    if containers.is_empty() {
        return Ok(None);
    }

    let container = &containers[0];
    let name = container["Name"]
        .as_str()
        .unwrap_or_default()
        .trim_start_matches('/')
        .to_string();
    let image = container["Config"]["Image"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let created_at = container["Created"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let ports = if let Some(ports) = container["NetworkSettings"]["Ports"].as_object() {
        ports
            .keys()
            .filter_map(|k| {
                k.split('/')
                    .next()
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

/// Returns the status of a container by name using `docker inspect`.
///
/// # Arguments
/// - `name`: Container name
///
/// # Returns
/// - `Ok(status)` if found (e.g., "running", "exited", etc.)
/// - `Err(ContainerNotFound)` if not found
pub fn get_container_status(client: &dyn DockerClient, name: &str) -> Result<String, Error> {
    let output = client.inspect(name)?;
    let containers: Vec<serde_json::Value> =
        serde_json::from_str(&output).map_err(|e| Error::DockerOutputParse(e.to_string()))?;

    if containers.is_empty() {
        return Err(Error::ContainerNotFound);
    }

    let status = containers[0]["State"]["Status"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    Ok(status)
}

/// Removes a Docker container by name.
///
/// # Arguments
/// - `name`: The container name to remove.
///
/// # Returns
/// - `Ok(())` if deleted successfully
/// - `Err(Error)` if failed
pub fn remove_container(name: &str) -> Result<(), Error> {
    let output = Command::new("docker")
        .args(["rm", "-f", name])
        .output()
        .map_err(|_| Error::DockerCommandFailed)?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    if stderr.contains("No such container") || stderr.contains("Error: No such container") {
        return Err(Error::ContainerNotFound);
    }

    if output.status.success() {
        Ok(())
    } else {
        Err(Error::Unexpected(stderr.trim().to_string()))
    }
}

/// Fetch the logs of a container using `docker logs`.
///
/// # Arguments
/// - `name`: Container name.
///
/// # Returns
/// - `Ok(logs)` if successful.
/// - `Err(Error)` if failed.
pub fn get_container_logs(name: &str) -> Result<String, Error> {
    let output = Command::new("docker")
        .args(["logs", name])
        .output()
        .map_err(|_| Error::DockerCommandFailed)?;

    if output.status.success() {
        let logs = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(logs)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
        if stderr.contains("no such container") {
            Err(Error::ContainerNotFound)
        } else {
            Err(Error::Unexpected(stderr.trim().to_string()))
        }
    }
}

/// Parses the status string from `docker ps` into an `AppStatus`.
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
fn parse_ports(ports_info: &str) -> Vec<u16> {
    ports_info
        .split(',')
        .filter_map(|entry| {
            let parts: Vec<&str> = entry.trim().split("->").collect();
            if !parts.is_empty() {
                if let Some(port_part) = parts[0].split(':').next_back() {
                    return port_part.parse::<u16>().ok();
                }
            }
            None
        })
        .collect()
}
