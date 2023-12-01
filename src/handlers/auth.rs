use crate::{
    logic::users::{self},
    AppError, Context,
};
use axum::{
    extract::{Extension, Form, Query},
    response::{Html, IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use redis::{Commands, RedisError};
use serde::Deserialize;
use std::sync::Arc;
use tera::Tera;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginParams {
    error: Option<String>,
}

pub async fn logout(jar: CookieJar) -> Result<impl IntoResponse, AppError> {
    let session_cookie = Cookie::new("axum_session", "");
    let updated_jar = jar.remove(session_cookie);

    Ok((updated_jar, Redirect::to("/")))
}

pub async fn login(
    Extension(tera): Extension<Tera>,
    params: Query<LoginParams>,
) -> Result<Html<String>, AppError> {
    let mut c = tera::Context::new();

    if let Some(_error) = &params.error {
        c.insert("error", "Invalid credentials (try with admin/admin)");
    }
    let r = tera.render("auth/login.html", &c)?;

    Ok(Html::from(r))
}

pub async fn post_login(
    Extension(ctx): Extension<Arc<Context>>,
    jar: CookieJar,
    Form(login): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
    let session_cookie = Cookie::new("axum_session", users::session_key());

    let user = users::authenticate(&login.username, &login.password, &ctx).await?;

    if let Some(user) = user {
        // Add a session to redis, where the key is the cookie value
        // And the redis value is the user info
        let mut conn = ctx.redis_connection.lock().await;

        let redis_key = session_cookie.name_value().1;
        let redis_value = serde_json::to_string(&user)?;
        info!("Redis key {} Value: {}", &redis_key, &redis_value);
        let redis_response: Result<(), RedisError> = conn.set_ex(redis_key, redis_value, 10);
        if let Err(e) = redis_response {
            tracing::error!("Cannot write into redis: {:?}", e);
        }
        let updated_jar = jar.add(session_cookie);
        Ok((updated_jar, Redirect::to("/pets")))
    } else {
        Ok((jar, Redirect::to("/login?error")))
    }
}
