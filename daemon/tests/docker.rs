use std::env;

use lightshuttle_core::docker::{create_and_run_container, ContainerConfig};

#[tokio::test]
async fn test_launch_container_via_cli() {
    if env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭ Skipping test: set DOCKER_TEST=1 to run it");
        return;
    }

    let config = ContainerConfig {
        name: "test-nginx-lightshuttle",
        image: "nginx:latest",
        host_ports: &[8089],
        container_port: 80,
        labels: None,
        env: None,
        volumes: None,
        restart_policy: None,
    };

    match create_and_run_container(config) {
        Ok(container_id) => {
            println!("✅ Launched container: {container_id}");
            assert!(!container_id.is_empty());
        }
        Err(e) => {
            eprintln!("❌ Failed to launch container: {e}");
            panic!("Container launch failed");
        }
    }
}
