use anyhow::Result;
use axum::{
    async_trait,
    extract::{Extension, FromRequestParts},
    http::{request::Parts, StatusCode},
    middleware::from_extractor,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, get_service, post},
    Router,
};
use axum_extra::extract::cookie::CookieJar;
use context::Context;
use db::models::user::User;
use handlers::*;
use redis::{Commands, RedisError};
use serde_json::Value;
use settings::Settings;
use std::sync::Arc;
use tera::Tera;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{debug, info};

mod context;
mod db;
mod handlers;
mod logic;
mod settings;

#[derive(Debug)]
pub struct AppError {
    inner: anyhow::Error,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Html::from(format!("Oh, something bad happened: {}", &self.inner)).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        AppError { inner: e.into() }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "warn,petclinic=debug")
    }

    tracing_subscriber::fmt::init();

    let settings = Arc::new(Settings::new()?);

    info!("Env: {settings:?}");
    let state = Context::new(Arc::clone(&settings))?;

    let app = get_public_routes()
        .merge(get_protected_routes())
        .fallback(|| async { "fallback route?" })
        .layer(TraceLayer::new_for_http())
        .route_layer(Extension(Arc::new(state)))
        .route_layer(Extension(settings.clone()))
        .route_layer(Extension(get_tera_instance(
            settings.tera_templates.as_str(),
        )));

    info!("Server started at: {:?}", settings.service_port);
    axum::Server::bind(&format!("0.0.0.0:{}", settings.service_port).parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn get_public_routes() -> Router {
    Router::new()
        .route("/", get(home::home))
        .route("/logout", get(auth::logout))
        .route("/login", get(auth::login).post(auth::post_login))
        .nest_service(
            "/static",
            get_service(ServeDir::new("static")).handle_error(|_| async move {}),
        )
}

fn get_protected_routes() -> Router {
    Router::new()
        .route("/vets", get(vets::list))
        .route("/vets/save", post(vets::save))
        .route("/vets/:id", get(vets::get))
        .route("/pets", get(pets::list))
        .route("/pets/save", post(pets::save))
        .route("/pets/:id", get(pets::get))
        .route("/vets/delete/:id", get(vets::delete))
        .route("/pets/delete/:id", get(pets::delete))
        .route_layer(from_extractor::<User>())
}

struct Principal {
    user: Option<User>,
}

impl tera::Function for Principal {
    fn call(
        &self,
        _args: &std::collections::HashMap<String, serde_json::Value>,
    ) -> tera::Result<Value> {
        debug!("User in tera: {:?}", &self.user);
        if let Some(username) = &self.user {
            tera::Result::Ok(Value::String(username.username.clone()))
        } else {
            tera::Result::Ok(Value::String(String::from("Not Logged in")))
        }
    }
}

fn get_tera_instance<'a>(tera_templates: impl Into<&'a str>) -> Tera {
    debug!("Creating Tera instance");
    let mut tera = match Tera::new(tera_templates.into()) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![".html", ".sql"]);
    tera
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Redirect);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let context = Arc::clone(parts.extensions.get::<Arc<Context>>().ok_or_else(|| {
            tracing::debug!("Failed to get mut Context");
            (StatusCode::TEMPORARY_REDIRECT, Redirect::to("/login"))
        })?);

        let cookiejar = CookieJar::from_headers(&parts.headers);

        let tera = parts.extensions.get_mut::<Tera>().ok_or_else(|| {
            tracing::debug!("Failed to get mut Tera");
            (StatusCode::TEMPORARY_REDIRECT, Redirect::to("/login"))
        })?;

        // check if the session cookie is valid against  redis
        let mut connection = context.redis_connection.lock().await;
        let cookie = cookiejar.get("axum_session").ok_or_else(|| {
            debug!("Session cookie not found, redirecting to login url");
            (StatusCode::TEMPORARY_REDIRECT, Redirect::to("/login"))
        })?;

        let valid_session: Result<User, RedisError> = connection.get(cookie.value());

        match valid_session {
            Ok(user) => {
                // refresh the key ttl
                connection
                    .expire(cookie.value(), context.settings.session.timeout)
                    .map_err(|e| {
                        tracing::debug!("{e:?}");
                        (StatusCode::TEMPORARY_REDIRECT, Redirect::to("/login"))
                    })?;

                tera.register_function(
                    "principal",
                    Principal {
                        user: Some(user.clone()),
                    },
                );

                if context.settings.config_name == "development" {
                    tera.full_reload().map_err(|e| {
                        tracing::debug!("{e:?}");
                        (StatusCode::TEMPORARY_REDIRECT, Redirect::to("/login"))
                    })?;
                }

                Ok(user)
            }
            Err(_error) => {
                // redis query failed
                Err((StatusCode::TEMPORARY_REDIRECT, Redirect::to("/login")))
            }
        }
    }
}
