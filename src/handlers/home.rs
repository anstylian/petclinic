use crate::{AppError, Context};
use axum::{extract::Extension, response::Html};
use std::sync::Arc;
use tera::Tera;

pub async fn home(
    Extension(tera): Extension<Tera>,
) -> Result<Html<String>, AppError> {
    let c = tera::Context::new();

    tracing::debug!("Main request");

    let r = tera.render("home.html", &c)?;

    Ok(Html::from(r))
}
