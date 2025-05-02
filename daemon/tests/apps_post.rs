use axum::{
    body,
    body::Body,
    http::{Request, StatusCode},
};
use lightshuttle_core::{app::build_router, docker::remove_container};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn post_apps_should_succeed() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭️ Skipping Docker test (DOCKER_TEST not set)");
        return;
    }

    let container_name = "test-nginx";

    let _ = std::process::Command::new("docker")
        .args(["rm", "-f", container_name])
        .output();

    let app = build_router();

    let payload = json!({
        "name": container_name,
        "image": "nginx:latest",
        "ports": [8080],
        "container_port": 80
    });

    let request = Request::builder()
        .method("POST")
        .uri("/apps")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body_bytes = body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body = String::from_utf8_lossy(&body_bytes);

    println!("Status: {}", status);
    println!("Body: {}", body);

    let _ = std::process::Command::new("docker")
        .args(["rm", "-f", container_name])
        .output();

    assert_eq!(status, StatusCode::CREATED);
}

#[tokio::test]
async fn post_apps_name_start_should_succeed() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭️ Skipping Docker test (DOCKER_TEST not set)");
        return;
    }

    let container_name = "test-start-nginx";

    let _ = std::process::Command::new("docker")
        .args(["rm", "-f", container_name])
        .output();

    let _ = std::process::Command::new("docker")
        .args([
            "create",
            "--name",
            container_name,
            "-p",
            "8081:80",
            "nginx:latest",
        ])
        .output();

    let app = build_router();

    let request = Request::builder()
        .method("POST")
        .uri(format!("/apps/{}/start", container_name))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body_bytes = body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body = String::from_utf8_lossy(&body_bytes);

    println!("Status: {}", status);
    println!("Body: {}", body);

    let _ = std::process::Command::new("docker")
        .args(["rm", "-f", container_name])
        .output();

    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn post_apps_name_start_should_404_on_missing_container() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭️ Skipping Docker test (DOCKER_TEST not set)");
        return;
    }

    let container_name = "this-container-does-not-exist";

    let app = build_router();

    let request = Request::builder()
        .method("POST")
        .uri(format!("/apps/{}/start", container_name))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();

    println!("Status: {}", status);

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn post_apps_name_stop_should_succeed() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭️ Skipping Docker test (DOCKER_TEST not set)");
        return;
    }

    let container_name = "test-stop-nginx";

    let _ = std::process::Command::new("docker")
        .args(["rm", "-f", container_name])
        .output();

    // Create and start the container
    let _ = std::process::Command::new("docker")
        .args([
            "run",
            "-d",
            "--name",
            container_name,
            "-p",
            "8082:80",
            "nginx:latest",
        ])
        .output();

    let app = build_router();

    let request = Request::builder()
        .method("POST")
        .uri(format!("/apps/{}/stop", container_name))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body_bytes = body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body = String::from_utf8_lossy(&body_bytes);

    println!("Status: {}", status);
    println!("Body: {}", body);

    let _ = std::process::Command::new("docker")
        .args(["rm", "-f", container_name])
        .output();

    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn post_apps_name_stop_should_404_on_missing_container() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭️ Skipping Docker test (DOCKER_TEST not set)");
        return;
    }

    let container_name = "this-container-does-not-exist";

    let app = build_router();

    let request = Request::builder()
        .method("POST")
        .uri(format!("/apps/{}/stop", container_name))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();

    println!("Status: {}", status);

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn post_apps_should_support_labels() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭ Skipping Docker test (DOCKER_TEST not set)");
        return;
    }

    let container_name = "test-nginx-labels";
    let _ = remove_container(container_name);

    let payload = json!({
        "name": container_name,
        "image": "nginx:latest",
        "ports": [8086],
        "container_port": 80,
        "labels": {
            "app": "lightshuttle",
            "env": "test"
        }
    });

    let app = build_router();
    let request = Request::builder()
        .method("POST")
        .uri("/apps")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let output = std::process::Command::new("docker")
        .args([
            "inspect",
            container_name,
            "--format",
            "{{json .Config.Labels}}",
        ])
        .output()
        .expect("Failed to inspect container");

    assert!(output.status.success());

    let label_json = String::from_utf8_lossy(&output.stdout);
    let labels: serde_json::Value = serde_json::from_str(&label_json).unwrap();

    assert_eq!(labels["app"], "lightshuttle");
    assert_eq!(labels["env"], "test");

    let _ = remove_container(container_name);
}
