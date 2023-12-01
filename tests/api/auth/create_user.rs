use rand::{distributions::Alphanumeric, Rng};

use crate::helpers::{assert_is_bad_request, assert_is_success, spawn_app, BadRequestJson};

#[tokio::test]
async fn create_user_works() {
    // Arrange
    let app = spawn_app().await;
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect();
    let body = serde_json::json!({
        "user_name": "testuser",
        "email": "test@example.com",
        "password": password
    });
    let cookie_key = "set-cookie";

    // Act
    let response = app.post_create_user(&body).await;

    // Assert
    assert_is_success(&response);
    assert!(&response.headers().contains_key(cookie_key));
    assert!(&response
        .headers()
        .get(cookie_key)
        .unwrap()
        .to_str()
        .unwrap()
        .contains("session_id"));
}

#[tokio::test]
async fn create_user_fails_with_short_password() {
    //  Arrange
    let app = spawn_app().await;
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(11)
        .map(char::from)
        .collect();
    let body = serde_json::json!({
        "user_name": "testuser",
        "email": "test@example.com",
        "password": password
    });

    // Act
    let response = app.post_create_user(&body).await;

    // Assert
    assert_is_bad_request(&response);
    assert!(response
        .json::<BadRequestJson>()
        .await
        .expect("Failed to parse json into object.")
        .error_message
        .contains(
            "The new password is too short - it must be between 12 and 128 characters long."
        ));
}

#[tokio::test]
async fn create_user_fails_with_long_password() {
    //  Arrange
    let app = spawn_app().await;
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(129)
        .map(char::from)
        .collect();
    let body = serde_json::json!({
        "user_name": "testuser",
        "email": "test@example.com",
        "password": password
    });

    // Acttest
    let response = app.post_create_user(&body).await;

    // Assert
    assert_is_bad_request(&response);
    assert!(response
        .json::<BadRequestJson>()
        .await
        .expect("Failed to parse json into object.")
        .error_message
        .contains("The new password is too long - it must be between 12 and 128 characters long."));
}

#[tokio::test]
async fn create_user_fails_when_user_name_is_empty() {
    //  Arrange
    let app = spawn_app().await;
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect();
    let body = serde_json::json!({
        "user_name": "",
        "email": "test@example.com",
        "password": password
    });

    // Acttest
    let response = app.post_create_user(&body).await;

    // Assert
    assert_is_bad_request(&response);
    assert!(response
        .json::<BadRequestJson>()
        .await
        .expect("Failed to parse json into object.")
        .error_message
        .contains("Username must have a value"));
}

#[tokio::test]
async fn create_user_fails_when_user_already_exists() {
    //  Arrange
    let app = spawn_app().await;
    let body = serde_json::json!({
        "user_name": &app.test_user.user_name,
        "email": "test@example.com",
        "password": &app.test_user.password
    });

    // Act
    let response = app.post_create_user(&body).await;

    // Assert
    assert_is_bad_request(&response);
    assert!(response
        .json::<BadRequestJson>()
        .await
        .expect("Failed to parse json into object.")
        .error_message
        .contains("User already exists"));
}

#[tokio::test]
async fn create_user_fails_with_invalid_email() {
    //  Arrange
    let app = spawn_app().await;
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect();
    let body = serde_json::json!({
        "user_name": "test",
        "email": "test",
        "password": password
    });

    // Act
    let response = app.post_create_user(&body).await;

    // Assert
    assert_is_bad_request(&response);
    assert!(response
        .json::<BadRequestJson>()
        .await
        .expect("Failed to parse json into object.")
        .error_message
        .contains("Email is invalid"));
}
