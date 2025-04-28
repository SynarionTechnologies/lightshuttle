use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use lightshuttle_core::{
    app::build_router,
    docker::{launch_container, remove_container},
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
    launch_container(container_name, "nginx:latest", &[8088], 80)
        .expect("Failed to launch container");

    let app = build_router();
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/apps/{}", container_name))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    println!("Status: {}", status);

    assert_eq!(status, StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn delete_non_existing_app_should_return_404() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭ Skipping test: set DOCKER_TEST=1 to run it");
        return;
    }

    let app = build_router();
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/apps/i-do-not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    println!("Status: {}", status);

    assert_eq!(status, StatusCode::NOT_FOUND);
}
