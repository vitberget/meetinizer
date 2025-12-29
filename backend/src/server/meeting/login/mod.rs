use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Redirect;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;

pub mod db;
pub mod claims;
pub mod mail;

pub async fn api_request_login(Path((id,email)): Path<(String,String)>) -> String {
    let valid = db::register_login(&id, &email).await.unwrap();
    format!("{valid}")
}

pub async fn api_attempt_login(Path((meeting, email, token)): Path<(String, String, String)>) -> Result<(CookieJar, Redirect), StatusCode> {
    if let Ok(token) = db::attempt_login(&meeting, &email, &token).await {

        let path = format!("/api/meeting/{meeting}");
        let redirect_path = format!("/meet/{meeting}");
        let redirect = Redirect::to(&redirect_path);

        let jwt_cookie = Cookie::build(("login", token)).path(path.to_owned()).http_only(true);
        let meeting_cookie = Cookie::build(("meeting", meeting)).path(redirect_path.to_owned());
        let email_cookie = Cookie::build(("email", email)).path(redirect_path.to_owned());

        let jar = CookieJar::new()
            .add(jwt_cookie)
            .add(meeting_cookie)
            .add(email_cookie);

        Ok((jar, redirect))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
