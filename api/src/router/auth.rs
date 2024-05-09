use std::sync::{Arc, Mutex};

use cookie::Cookie;
use edgedb_protocol::model::Uuid;
use rspc::{
    integrations::httpz::{CookieJar, Request},
    Error, ErrorCode, Router,
};
use serde::{Deserialize, Serialize};
use specta::Type;
use woothee::parser::Parser;

use crate::{
    core::context::{query, Context},
    middleware::{auth, cookies},
    service::{
        auth::{Auth, Session, SessionWithMetadata},
        users::Users,
    },
};

use super::R;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
struct LoginArgs {
    email: String,
    password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
struct RegisterArgs {
    email: String,
    first_name: String,
    last_name: String,
    password: String,
}

#[derive(Debug)]
struct DeviceDetails {
    os_name: Option<String>,
    os_version: Option<String>,
    browser_name: Option<String>,
    browser_version: Option<String>,
}

fn get_device_details(request: Arc<Mutex<Request>>) -> DeviceDetails {
    let request = request.lock().unwrap();
    let user_agent = request
        .headers()
        .get("user-agent")
        .and_then(|ua| ua.to_str().ok());

    let user_agent = match user_agent {
        Some(ua) => ua,
        None => {
            return DeviceDetails {
                os_name: None,
                os_version: None,
                browser_name: None,
                browser_version: None,
            }
        }
    };

    let parser = Parser::new();

    match parser.parse(user_agent) {
        Some(result) => DeviceDetails {
            os_name: Some(result.os.to_string()),
            os_version: Some(result.os_version.to_string()),
            browser_name: Some(result.name.to_string()),
            browser_version: Some(result.version.to_string()),
        },
        None => DeviceDetails {
            os_name: None,
            os_version: None,
            browser_name: None,
            browser_version: None,
        },
    }
}

async fn verify(ctx: Context, id: Uuid) -> Result<Option<Session>, Error> {
    let auth = query!(ctx, Auth);
    let session = match auth.validate_session(id).await {
        Ok(session) => session,
        Err(_) => return Ok(None),
    };
    Ok(Some(session))
}

async fn get_sessions(ctx: Context, _: ()) -> Result<Vec<SessionWithMetadata>, Error> {
    let (auth, session) = query!(ctx, Auth, Session);
    let sessions = auth
        .get_sessions(session.user.id, session.id)
        .await
        .map_err(|e| Error::new(ErrorCode::BadRequest, e.to_string()))?;
    Ok(sessions)
}

async fn login(ctx: Context, args: LoginArgs) -> Result<(), Error> {
    let LoginArgs { email, password } = args;
    let (auth, users, cookies, request) = query!(ctx, Auth, Users, CookieJar, Mutex<Request>);

    let user = users
        .get_user_by_email(email.as_str())
        .await
        .map_err(|_| Error::new(ErrorCode::BadRequest, "Unable to find user.".to_string()))?;

    tokio::task::spawn_blocking(move || {
        bcrypt::verify(password, &user.hashed_password).unwrap_or(false)
    })
    .await
    .map_err(|e| Error::new(ErrorCode::InternalServerError, e.to_string()))?
    .then(|| ())
    .ok_or_else(|| {
        Error::new(
            ErrorCode::BadRequest,
            "Invalid email or password".to_string(),
        )
    })?;
    let device_details = get_device_details(request.clone());
    let session = auth
        .create_session(
            user.id,
            device_details.os_name,
            device_details.os_version,
            device_details.browser_name,
            device_details.browser_version,
        )
        .await
        .map_err(|e| Error::new(ErrorCode::BadRequest, e.to_string()))?;

    let mut cookie = Cookie::new("auth_session", session.to_string());

    cookie.set_http_only(true);
    cookie.set_domain("localtest.me");
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

async fn register(ctx: Context, args: RegisterArgs) -> Result<(), Error> {
    let RegisterArgs {
        email,
        password,
        first_name,
        last_name,
    } = args;
    let (auth, users, cookies, request) = query!(ctx, Auth, Users, CookieJar, Mutex<Request>);

    let password =
        tokio::task::spawn_blocking(move || bcrypt::hash(password, bcrypt::DEFAULT_COST))
            .await
            .map_err(|e| Error::new(ErrorCode::InternalServerError, e.to_string()))?
            .map_err(|e| Error::new(ErrorCode::InternalServerError, e.to_string()))?;

    let user = users
        .create_user(
            first_name.as_str(),
            last_name.as_str(),
            email.as_str(),
            password.as_str(),
        )
        .await
        .map_err(|_| Error::new(ErrorCode::BadRequest, "Error creating user".to_string()))?;

    let device_details = get_device_details(request.clone());
    let session = auth
        .create_session(
            user.id,
            device_details.os_name,
            device_details.os_version,
            device_details.browser_name,
            device_details.browser_version,
        )
        .await
        .map_err(|e| Error::new(ErrorCode::BadRequest, e.to_string()))?;

    let mut cookie = Cookie::new("auth_session", session.to_string());

    cookie.set_http_only(true);
    cookie.set_domain("localtest.me");
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

async fn logout(ctx: Context, _: ()) -> Result<(), Error> {
    let (cookies, auth, session) = query!(ctx, CookieJar, Auth, Session);

    auth.invalidate_session(session.id)
        .await
        .map_err(|e| Error::new(ErrorCode::BadRequest, e.to_string()))?;

    let mut cookie = Cookie::new("auth_session", "");

    cookie.set_http_only(true);
    cookie.set_domain("localtest.me");
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}

pub fn mount() -> Router<Context> {
    R.router()
        .procedure("verify", R.query(verify))
        .procedure("login", R.with(cookies()).mutation(login))
        .procedure("register", R.with(cookies()).mutation(register))
        .procedure("logout", R.with(cookies()).with(auth()).query(logout))
        .procedure(
            "getSessions",
            R.with(cookies()).with(auth()).query(get_sessions),
        )
}
