use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Redirect;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use cookie::time::Duration;
use uri_encode::encode_uri_component;

pub mod db;
pub mod claims;
pub mod mail;

pub async fn api_request_login(Path((id,email)): Path<(String,String)>) -> Result<String,StatusCode> {
    if id.trim().is_empty() || email.trim().is_empty() {
        Err(StatusCode::BAD_REQUEST)
    } else {
        match db::register_login(&id, &email).await {
            Ok(valid) => Ok(format!("{valid}")),
            Err(_) => Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn api_attempt_login(Path((meeting, email, token)): Path<(String, String, String)>) -> Result<(CookieJar, Redirect), StatusCode> {
    if let Ok(token) = db::attempt_login(&meeting, &email, &token).await {

        let meeting = encode_uri_component(meeting);

        let path = format!("/api/meeting/{meeting}");
        let redirect_path = format!("/meet/{meeting}");
        let redirect = Redirect::to(&redirect_path);

        let jwt_cookie = Cookie::build(("login", token))
            .max_age(Duration::days(30))
            .path(path.to_owned())
            .http_only(true);

        let meeting_cookie = Cookie::build(("meeting", meeting))
            .max_age(Duration::days(30))
            .path(redirect_path.to_owned());

        let email_cookie = Cookie::build(("email", email))
            .max_age(Duration::days(30))
            .path(redirect_path.to_owned());

        let jar = CookieJar::new()
            .add(jwt_cookie)
            .add(meeting_cookie)
            .add(email_cookie);

        Ok((jar, redirect))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn api_logout(Path(meeting): Path<String>) -> Result<(CookieJar, Redirect), StatusCode> {
    let meeting = encode_uri_component(meeting);

    let path = format!("/api/meeting/{meeting}");
    let redirect_path = format!("/meet/{meeting}");
    let redirect = Redirect::to(&redirect_path);

    let jwt_cookie = Cookie::build(("login", "")).path(path.to_owned()).http_only(true);
    let meeting_cookie = Cookie::build(("meeting", "")).path(redirect_path.to_owned());
    let email_cookie = Cookie::build(("email", "")).path(redirect_path.to_owned());

    let jar = CookieJar::new()
        .add(jwt_cookie)
        .add(meeting_cookie)
        .add(email_cookie);

    Ok((jar, redirect))
}
