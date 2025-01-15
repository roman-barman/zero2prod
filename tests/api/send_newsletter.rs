use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_send_newsletters_form() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_send_newsletters().await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}
