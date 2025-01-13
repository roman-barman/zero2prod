use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn login_form(flash_message: IncomingFlashMessages) -> HttpResponse {
    let mut message_html = String::new();
    for m in flash_message.iter() {
        writeln!(message_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
<html lang="en">
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        <title>Login</title>
    </head>
    <body>
        {message_html}
        <form action="/login" method="post">
            <label>
                Username
                <input type="text" name="username" placeholder="Enter username">
            </label>
            <label>
                Password
                <input type="password" name="password" placeholder="Enter password">
            </label>
            <input type="submit" value="Login">
        </form>
    </body>
</html>
            "#
        ))
}
