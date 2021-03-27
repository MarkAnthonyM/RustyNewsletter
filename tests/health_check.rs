#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    spawn_app();

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");

    // Launch server as a background task
    // tokio::spawn returns a handle to spawned future,
    // but we have no use for it here
    let _ = tokio::spawn(server);
}