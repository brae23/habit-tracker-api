use crate::helpers::{assert_is_success, assert_is_unauthorized, spawn_app};

#[tokio::test]
async fn logout_works_when_user_already_logged_in() {
    // Arrange
    let app = spawn_app().await;
    let body = serde_json::json!({
        "username": &app.test_user.user_name,
        "password": &app.test_user.password,
    });
    app.post_login(&body).await;

    // Act
    let response = app.post_logout().await;

    // Assert
    assert_is_success(&response);
}

#[tokio::test]
async fn logout_fails_when_user_not_already_logged_in() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.post_logout().await;

    // Assert
    assert_is_unauthorized(&response);
}
