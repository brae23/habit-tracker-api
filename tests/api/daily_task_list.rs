use crate::helpers::spawn_app;

#[tokio::test]
async fn user_must_be_logged_in_to_get_daily_task_list() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_daily_task_list().await;

    // Assert
    assert_eq!(response.status().as_u16(), 401);
}
