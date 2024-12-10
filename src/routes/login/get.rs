use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let error_html = match query.0.error {
        None => "".to_string(),
        Some(error_message) => format!("<p><i>{error_message}</i></p>"),
    };
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
        {error_html}
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
