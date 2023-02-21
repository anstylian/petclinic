use crate::{
    db::models::pet::{self, Pet, NewPet},
    db::models::{user::User, vet::Vet},
    AppError, Context,
};
use axum::{
    extract::{Extension, Path, Query},
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use chrono::Utc;
use diesel::RunQueryDsl;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use tera::Tera;

#[derive(Deserialize)]
pub struct PetForm {
    pub id: i32,
    pub name: String,
    pub owner_name: String,
    pub owner_phone: String,
    pub age: i32,
    pub current_vet: i32,
    pub pet_type: i32,
}

impl From<Form<PetForm>> for Pet {
    fn from(form: Form<PetForm>) -> Pet {
        Pet {
            id: form.id,
            name: form.name.clone(),
            owner_name: form.owner_name.clone(),
            owner_phone: form.owner_phone.clone(),
            age: form.age,
            vet_id: Some(form.current_vet),
            pet_type: form.pet_type,
            created_by: 0,
            created_at: Utc::now().naive_utc(),
        }
    }
}

impl From<Form<PetForm>> for NewPet {
    fn from(form: Form<PetForm>) -> NewPet {
        NewPet {
            name: form.name.clone(),
            owner_name: form.owner_name.clone(),
            owner_phone: form.owner_phone.clone(),
            age: form.age,
            vet_id: Some(form.current_vet),
            pet_type: form.pet_type,
            created_by: 0,
            created_at: Utc::now().naive_utc(),
        }
    }
}

pub async fn save(
    Extension(ctx): Extension<Arc<Context>>,
    user: User,
    pet_form: Form<PetForm>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = ctx.db_connection_pool.get().await?;

    let pet_id = pet_form.id;
    if pet_id == 0 {
        let mut pet: NewPet = pet_form.into();
        pet.created_by = user.id;
        db_conn
            .interact(move |conn| pet.save(conn))
            .await
            .map_err(|e| AppError {
                inner: anyhow::Error::msg(e.to_string()),
            })??;
        // pet.save(conn)?;
    } else {
        let pet = db_conn
            .interact(move |conn| Pet::select_by_id(conn, pet_id))
            .await
            .map_err(|e| AppError {
                inner: anyhow::Error::msg(e.to_string()),
            })?;

        if pet.is_err() {
            return Ok(Redirect::to("/pets"));
        }

        let mut pet = pet?;

        pet.name = pet_form.name.clone();
        pet.owner_name = pet_form.owner_name.clone();
        pet.owner_phone = pet_form.owner_phone.clone();
        pet.age = pet_form.age;
        pet.pet_type = pet_form.pet_type;

        if pet_form.current_vet > 0 {
            pet.vet_id = Some(pet_form.current_vet);
        } else {
            pet.vet_id = Some(0)
        }

        db_conn
            .interact(move |conn| pet.update(conn))
            .await
            .map_err(|e| AppError {
                inner: anyhow::Error::msg(e.to_string()),
            })??;
    }

    Ok(Redirect::to("/pets"))
}

pub async fn list(
    Extension(tera): Extension<Tera>,
    Extension(ctx): Extension<Arc<Context>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Html<String>, AppError> {
    let db_conn = ctx.db_connection_pool.get().await?;
    let mut c = tera::Context::new();

    let pets = db_conn
        .interact(move |conn| {
            let name = params.get("name");
            match name {
                Some(n) => Pet::select_by_name(conn, n),
                None => Ok(Pet::pets(conn)?),
            }
        })
        .await
        .map_err(|e| AppError {
            inner: anyhow::Error::msg(e.to_string()),
        })??;

    let types = pet::types();

    c.insert("pets", &pets);
    c.insert("pet_types", &types);

    let r = tera.render("pet/list.html", &c)?;

    Ok(Html::from(r))
}

pub async fn delete(
    Extension(ctx): Extension<Arc<Context>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = ctx.db_connection_pool.get().await?;
    db_conn
        .interact(move |conn| Pet::delete_by_id(conn, id))
        .await
        .map_err(|e| AppError {
            inner: anyhow::Error::msg(e.to_string()),
        })??;

    Ok(Redirect::to("/pets"))
}

pub async fn get(
    Extension(tera): Extension<Tera>,
    Extension(ctx): Extension<Arc<Context>>,
    Path(id): Path<i32>,
) -> Result<Response, AppError> {
    let db_conn = ctx.db_connection_pool.get().await?;

    // TODO handle this correctly
    let mut c = tera::Context::new();

    let pet = if id == 0 {
        Pet::default()
    } else {
        db_conn
            .interact(move |conn| Pet::select_by_id(conn, id))
            .await
            .map_err(|e| AppError {
                inner: anyhow::Error::msg(e.to_string()),
            })??
    };

    let vets = db_conn
        .interact(move |conn| Vet::all().load(conn))
        .await
        .map_err(|e| AppError {
            inner: anyhow::Error::msg(e.to_string()),
        })??;

    let types = pet::types();

    c.insert("pet_types", &types);
    let vet = db_conn
        .interact(move |conn| {
            pet.vet_id
                .map(|vet_id| Vet::select_by_id(conn, vet_id))
                .transpose()
        })
        .await
        .map_err(|e| AppError {
            inner: anyhow::Error::msg(e.to_string()),
        })??;

    if let Some(current_vet) = vet {
        c.insert("current_vet", &current_vet);
    }
    c.insert("pet", &pet);
    c.insert("vets", &vets);

    let r = tera.render("pet/edit.html", &c)?;

    Ok(Html::from(r).into_response())
}
