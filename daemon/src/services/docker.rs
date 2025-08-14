use std::{collections::HashMap, process::Command};

use crate::{docker::ContainerConfig, errors::Error};

/// Abstraction over Docker interactions.
///
/// This trait exposes a minimal set of operations required by the
/// application and allows for future alternative implementations
/// (e.g. talking to a daemon over sockets or HTTP).
pub trait DockerClient: Send + Sync + 'static {
    /// Run a new container.
    fn run(&self, cfg: ContainerConfig) -> Result<String, Error>;
    /// Start an existing container.
    fn start(&self, name: &str) -> Result<(), Error>;
    /// Stop a running container.
    fn stop(&self, name: &str) -> Result<(), Error>;
    /// Inspect a container and return the raw JSON output.
    fn inspect(&self, name: &str) -> Result<String, Error>;
}

/// Docker client backed by shelling out to the `docker` CLI.
pub struct ShellDockerClient;

impl DockerClient for ShellDockerClient {
    fn run(&self, cfg: ContainerConfig) -> Result<String, Error> {
        let port = cfg.container_port;
        let port_args: Vec<String> = cfg
            .host_ports
            .iter()
            .flat_map(|host| vec!["-p".to_string(), format!("{host}:{port}")])
            .collect();

        let label_args: Vec<String> = cfg
            .labels
            .unwrap_or(&HashMap::new())
            .iter()
            .flat_map(|(k, v)| vec!["--label".to_string(), format!("{k}={v}")])
            .collect();

        let env_args: Vec<String> = cfg
            .env
            .unwrap_or(&HashMap::new())
            .iter()
            .flat_map(|(k, v)| vec!["-e".to_string(), format!("{k}={v}")])
            .collect();

        let volume_args: Vec<String> = cfg
            .volumes
            .unwrap_or(&vec![])
            .iter()
            .flat_map(|mount| vec!["-v".to_string(), mount.to_string()])
            .collect();

        if let Some(vols) = cfg.volumes {
            for v in vols {
                if !v.contains(':') || v.starts_with(':') || v.ends_with(':') {
                    return Err(Error::BadRequest(format!("Invalid volume format: '{v}'")));
                }
            }
        }

        if let Some(policy) = cfg.restart_policy {
            let valid = ["no", "always", "on-failure", "unless-stopped"];
            if !valid.contains(&policy) {
                return Err(Error::InvalidRequest(format!(
                    "Invalid restart policy: '{policy}'"
                )));
            }
        }

        let mut args = vec!["run", "-d", "--rm", "--name", cfg.name];
        args.extend(port_args.iter().map(String::as_str));
        args.extend(label_args.iter().map(String::as_str));
        args.extend(env_args.iter().map(String::as_str));
        args.extend(volume_args.iter().map(String::as_str));

        if let Some(policy) = cfg.restart_policy {
            args.push("--restart");
            args.push(policy);
        }

        args.push(cfg.image);

        let output = Command::new("docker")
            .args(&args)
            .output()
            .map_err(|_| Error::DockerCommandFailed)?;

        if output.status.success() {
            let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(container_id)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Err(Error::Unexpected(stderr.trim().to_string()))
        }
    }

    fn start(&self, name: &str) -> Result<(), Error> {
        let output = Command::new("docker")
            .args(["start", name])
            .output()
            .map_err(|_| Error::DockerCommandFailed)?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
            if stderr.contains("no such container") {
                Err(Error::ContainerNotFound)
            } else {
                Err(Error::Unexpected(stderr.trim().to_string()))
            }
        }
    }

    fn stop(&self, name: &str) -> Result<(), Error> {
        let output = Command::new("docker")
            .args(["stop", name])
            .output()
            .map_err(|_| Error::DockerCommandFailed)?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
            if stderr.contains("no such container") {
                Err(Error::ContainerNotFound)
            } else {
                Err(Error::Unexpected(stderr.trim().to_string()))
            }
        }
    }

    fn inspect(&self, name: &str) -> Result<String, Error> {
        let output = Command::new("docker")
            .args(["inspect", name])
            .output()
            .map_err(|_| Error::DockerCommandFailed)?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
            if stderr.contains("no such container") {
                Err(Error::ContainerNotFound)
            } else {
                Err(Error::Unexpected(stderr.trim().to_string()))
            }
        }
    }
}
