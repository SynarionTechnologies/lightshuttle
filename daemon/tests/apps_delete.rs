use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use lightshuttle_core::{
    api::routes::router,
    docker::{remove_container, ContainerConfig},
    services::docker::{DockerClient, ShellDockerClient},
};
use tower::ServiceExt;

#[tokio::test]
async fn delete_existing_app_should_succeed() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭ Skipping test: set DOCKER_TEST=1 to run it");
        return;
    }

    let container_name = "test-delete-lightshuttle";
    let _ = remove_container(container_name);

    let config = ContainerConfig {
        name: container_name,
        image: "nginx:latest",
        host_ports: &[8088],
        container_port: 80,
        labels: None,
        env: None,
        volumes: None,
        restart_policy: None,
    };

    let docker = ShellDockerClient;
    docker.run(config).expect("Failed to launch container");

    let app = router();
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/apps/{container_name}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    println!("Status: {status}");

    assert_eq!(status, StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn delete_non_existing_app_should_return_404() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭ Skipping test: set DOCKER_TEST=1 to run it");
        return;
    }

    let app = router();
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/apps/i-do-not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    println!("Status: {status}");

    assert_eq!(status, StatusCode::NOT_FOUND);
}
