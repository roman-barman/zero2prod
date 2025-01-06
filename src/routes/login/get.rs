use actix_web::cookie::{time::Duration, Cookie};
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};

pub async fn login_form(request: HttpRequest) -> HttpResponse {
    let error_html = match request.cookie("_flash") {
        None => "".into(),
        Some(cookie) => {
            format!("<p><i>{}</i></p>", cookie.value())
        }
    };
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(Cookie::build("_flash", "").max_age(Duration::ZERO).finish())
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
