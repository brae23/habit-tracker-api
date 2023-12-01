use crate::helpers::{assert_is_bad_request, assert_is_success, spawn_app};

#[tokio::test]
async fn login_works_with_valid_user_data() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let body = serde_json::json!({
        "username": &app.test_user.user_name,
        "password": &app.test_user.password,
    });
    let response = app.post_login(&body).await;

    // Assert
    assert_is_success(&response);
}

#[tokio::test]
async fn login_fails_with_invalid_user_data() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let body = serde_json::json!({
        "username": "random_username",
        "password": "random_password",
    });
    let response = app.post_login(&body).await;

    // Assert
    assert_is_bad_request(&response);
}
