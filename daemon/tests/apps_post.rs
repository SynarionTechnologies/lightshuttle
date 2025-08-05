use axum::{
    body,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use lightshuttle_core::{
    app::build_router,
    docker::{create_and_run_container, remove_container, ContainerConfig},
};
use serde_json::{json, Value};
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

    println!("Status: {status}");
    println!("Body: {body}");

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
        .uri(format!("/apps/{container_name}/start"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body_bytes = body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body = String::from_utf8_lossy(&body_bytes);

    println!("Status: {status}");
    println!("Body: {body}");

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
        .uri(format!("/apps/{container_name}/start"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();

    println!("Status: {status}");

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
        .uri(format!("/apps/{container_name}/stop"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body_bytes = body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body = String::from_utf8_lossy(&body_bytes);

    println!("Status: {status}");
    println!("Body: {body}");

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
        .uri(format!("/apps/{container_name}/stop"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();

    println!("Status: {status}");

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn post_apps_name_recreate_should_restart_container() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭️ Skipping test: set DOCKER_TEST=1 to run it");
        return;
    }

    let name = "test-recreate-nginx";
    let _ = remove_container(name);

    let config = ContainerConfig {
        name,
        image: "nginx:latest",
        host_ports: &[8089],
        container_port: 80,
        labels: None,
        env: None,
        volumes: None,
        restart_policy: None,
    };

    create_and_run_container(config).expect("Failed to create container");

    let app = build_router();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/apps/{name}/recreate"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&bytes).unwrap();

    assert!(json["container_id"].as_str().unwrap().is_empty());

    let _ = remove_container(name);
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

#[tokio::test]
async fn post_apps_should_set_environment_variables() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭️ Skipping Docker test (DOCKER_TEST not set)");
        return;
    }

    let name = "test-env-nginx";
    let _ = remove_container(name);

    let payload = json!({
        "name": name,
        "image": "nginx:latest",
        "ports": [8088],
        "container_port": 80,
        "env": {
            "FOO": "bar",
            "LIGHTSHUTTLE": "true"
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
        .args(["exec", name, "printenv", "FOO"])
        .output()
        .expect("Failed to exec into container");

    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "bar");

    let _ = remove_container(name);
}

#[tokio::test]
async fn post_apps_should_mount_volume() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭️ Skipping test: set DOCKER_TEST=1 to run it");
        return;
    }

    let name = "test-volume-nginx";
    let host_path = "/tmp/lightshuttle-test-volume";
    let container_path = "/data";

    std::fs::create_dir_all(host_path).unwrap();
    std::fs::write(format!("{host_path}/hello.txt"), "Hello LightShuttle!").unwrap();

    let payload = json!({
        "name": name,
        "image": "nginx:latest",
        "ports": [8090],
        "container_port": 80,
        "volumes": [format!("{host_path}:{container_path}")]
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
        .args(["exec", name, "cat", "/data/hello.txt"])
        .output()
        .expect("Failed to exec into container");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "Hello LightShuttle!"
    );

    let _ = remove_container(name);
    let _ = std::fs::remove_dir_all(host_path);
}

#[tokio::test]
async fn post_apps_should_apply_restart_policy() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭ Skipping test: set DOCKER_TEST=1 to run it");
        return;
    }

    let name = "test-restart-policy";
    let _ = remove_container(name);

    let payload = json!({
        "name": name,
        "image": "nginx:latest",
        "ports": [8093],
        "container_port": 80,
        "restart_policy": "always"
    });

    let app = build_router();
    let request = Request::builder()
        .method("POST")
        .uri("/apps")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let output = std::process::Command::new("docker")
        .args([
            "inspect",
            name,
            "--format",
            "{{.HostConfig.RestartPolicy.Name}}",
        ])
        .output()
        .expect("Failed to inspect");

    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    assert_eq!(value, "always");

    let _ = remove_container(name);
}

#[tokio::test]
async fn post_apps_name_recreate_should_preserve_volumes_and_env() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭ Skipping Docker test (DOCKER_TEST not set)");
        return;
    }

    let name = "test-recreate-persist";
    let host_path = "/tmp/lightshuttle-recreate";
    let container_path = "/data";

    std::fs::create_dir_all(host_path).unwrap();
    let _ = std::fs::write(format!("{host_path}/original.txt"), "before recreate");

    let payload = json!({
        "name": name,
        "image": "nginx:latest",
        "ports": [8091],
        "container_port": 80,
        "env": {
            "RECREATE_TEST": "1"
        },
        "volumes": [format!("{host_path}:{container_path}")]
    });

    let app = build_router();
    let request = Request::builder()
        .method("POST")
        .uri("/apps")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    std::process::Command::new("docker")
        .args([
            "exec",
            name,
            "sh",
            "-c",
            "echo modified > /data/original.txt",
        ])
        .output()
        .unwrap();

    let recreate = Request::builder()
        .method("POST")
        .uri(format!("/apps/{name}/recreate"))
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(recreate).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let out = std::process::Command::new("docker")
        .args(["exec", name, "cat", "/data/original.txt"])
        .output()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "modified");

    let out = std::process::Command::new("docker")
        .args(["exec", name, "printenv", "RECREATE_TEST"])
        .output()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "1");

    let _ = remove_container(name);
    let _ = std::fs::remove_dir_all(host_path);
}

#[tokio::test]
async fn post_apps_should_fail_on_invalid_volume_format() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭ Skipping Docker test (DOCKER_TEST not set)");
        return;
    }

    let name = "test-bad-volume";

    let payload = json!({
        "name": name,
        "image": "nginx:latest",
        "ports": [8092],
        "container_port": 80,
        "volumes": [":/data"]
    });

    let app = build_router();
    let request = Request::builder()
        .method("POST")
        .uri("/apps")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
