use std::env;

use lightshuttle_core::docker::create_and_run_container;

#[tokio::test]
async fn test_launch_container_via_cli() {
    if env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭ Skipping test: set DOCKER_TEST=1 to run it");
        return;
    }

    let result = create_and_run_container(
        "test-nginx-lightshuttle",
        "nginx:latest",
        &[8089],
        80,
        None,
        None,
    );

    match result {
        Ok(container_id) => {
            println!("✅ Launched container: {}", container_id);
            assert!(!container_id.is_empty());
        }
        Err(e) => {
            eprintln!("❌ Failed to launch container: {}", e);
            panic!("Container launch failed");
        }
    }
}
