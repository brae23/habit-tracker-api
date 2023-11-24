use habit_tracker::domain::List;

use crate::helpers::{assert_is_success, assert_is_unauthorized, spawn_app};

#[tokio::test]
async fn user_must_be_logged_in_to_get_daily_task_list() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_daily_task_list().await;

    // Assert
    assert_is_unauthorized(&response);
}

#[tokio::test]
async fn get_daily_task_list_returns_new_list_when_list_does_not_already_exist() {
    // Arrange
    let app = spawn_app().await;
    app.login_as_test_user().await;
    let expected_list_name = "New Daily Task List";
    let expected_list_description = format!("New Daily Task List for {}", app.test_user.user_name);

    // Act
    let response = app.get_daily_task_list().await;

    // Assert
    assert_is_success(&response);
    let response_list: List = response.json().await.expect("Failed to parse json body");
    assert_eq!(response_list.name, expected_list_name);
    assert_eq!(
        response_list.description.unwrap(),
        expected_list_description
    );
    assert_eq!(response_list.list_items.unwrap().len(), 0);
}
