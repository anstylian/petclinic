use crate::{
    db::models::vet::{NewVet, Vet},
    AppError, Context,
};
use axum::{
    extract::{Extension, Path, Query},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;
use tera::Tera;
use tracing::log::trace;

use std::{collections::HashMap, sync::Arc};

#[derive(Deserialize)]
pub struct VetForm {
    id: i32,
    name: String,
}
pub async fn save(
    Extension(ctx): Extension<Arc<Context>>,
    vet: Form<VetForm>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = ctx.db_connection_pool.get().await?;

    db_conn
        .interact(move |conn| -> anyhow::Result<()> {
            if let Some(mut v) = Vet::select_by_id(conn, vet.id)? {
                v.name = vet.name.clone();
                v.update(conn)?;
            } else {
                // Adding a new one
                let vet = NewVet {
                    name: vet.name.clone(),
                };
                vet.save(conn)?;
            }

            Ok(())
        })
        .await
        .map_err(|e| AppError {
            inner: anyhow::Error::msg(e.to_string()),
        })??;

    Ok(Redirect::to("/vets"))
}

pub async fn list(
    Extension(tera): Extension<Tera>,
    Extension(ctx): Extension<Arc<Context>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Html<String>, AppError> {
    trace!("list reuested");
    let db_conn = ctx.db_connection_pool.get().await?;

    let mut c = tera::Context::new();

    let vets = db_conn
        .interact(move |conn| {
            let name = params.get("name");
            match name {
                Some(n) => Vet::select_by_name(conn, n),
                None => Ok(Vet::vets(conn)?),
            }
        })
        .await
        .map_err(|e| AppError {
            inner: anyhow::Error::msg(e.to_string()),
        })??;

    c.insert("vets", &vets);
    let r = tera.render("vet/list.html", &c)?;

    Ok(Html::from(r))
}
pub async fn get(
    Extension(tera): Extension<Tera>,
    Extension(ctx): Extension<Arc<Context>>,
    Path(id): Path<i32>,
) -> Result<Html<String>, AppError> {
    let db_conn = ctx.db_connection_pool.get().await?;

    let mut c = tera::Context::new();

    let mut vet = db_conn
        .interact(move |conn| Vet::select_by_id(conn, id))
        .await
        .map_err(|e| AppError {
            inner: anyhow::Error::msg(e.to_string()),
        })??;

    if id == 0 {
        vet = Some(Vet::default());
    }
    if vet.is_none() {
        return Ok(Html::from("Vet not found".to_string()));
    }

    c.insert("vet", &vet);

    let r = tera.render("vet/edit.html", &c)?;

    Ok(Html::from(r))
}

pub async fn delete(
    Extension(ctx): Extension<Arc<Context>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = ctx.db_connection_pool.get().await?;
    db_conn
        .interact(move |conn| Vet::delete_by_id(conn, id))
        .await
        .map_err(|e| AppError {
            inner: anyhow::Error::msg(e.to_string()),
        })??;
    Ok(Redirect::to("/vets"))
}
