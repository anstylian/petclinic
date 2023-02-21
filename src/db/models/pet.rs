use crate::db::schema::pet;
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::{
    backend::Backend,
    dsl::{AsSelect, Eq},
    ExpressionMethods, Identifiable, Insertable, QueryDsl, QueryResult, Queryable, RunQueryDsl,
    Selectable, SelectableHelper, SqliteConnection, AsChangeset,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Eq,
    Identifiable,
    PartialEq,
    Queryable,
    Selectable,
    Serialize,
    Insertable,
    Default,
)]
#[diesel(table_name = pet)]
pub struct Pet {
    pub id: i32,
    pub name: String,
    pub owner_name: String,
    pub owner_phone: String,
    pub age: i32,
    pub pet_type: i32,
    pub vet_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub created_by: i32,
}

#[derive(
    Serialize,
    Insertable,
)]
#[diesel(table_name = pet)]
pub struct NewPet {
    pub name: String,
    pub owner_name: String,
    pub owner_phone: String,
    pub age: i32,
    pub pet_type: i32,
    pub vet_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub created_by: i32,
}

type All<DB> = diesel::dsl::Select<pet::table, AsSelect<Pet, DB>>;
type WithId = Eq<pet::id, i32>;
type WithName<'a> = Eq<pet::name, &'a str>;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub enum PetType {
    Cat,
    Dog,
    Lizard,
    Horse,
}

pub fn types() -> HashMap<u32, PetType> {
    HashMap::from([
        (1, PetType::Cat),
        (2, PetType::Dog),
        (3, PetType::Lizard),
        (4, PetType::Horse),
    ])
}

impl Pet {
    pub fn all<DB>() -> All<DB>
    where
        DB: Backend,
    {
        pet::table.select(Pet::as_select())
    }

    fn with_id(id: i32) -> WithId {
        pet::id.eq(id)
    }

    fn with_name(name: &str) -> WithName {
        pet::name.eq(name)
    }

    pub fn pets(conn: &mut SqliteConnection) -> QueryResult<Vec<Self>> {
        Self::all().load(conn)
    }

    pub fn select_by_id(conn: &mut SqliteConnection, id: i32) -> Result<Self> {
        Ok(crate::db::schema::pet::table
            .filter(Self::with_id(id))
            .get_result::<Self>(conn)?)
    }

    pub fn select_by_name(conn: &mut SqliteConnection, name: &str) -> Result<Vec<Pet>> {
        Ok(crate::db::schema::pet::table
            .filter(Self::with_name(name))
            .get_results::<Self>(conn)?)
    }

    pub fn delete_by_id(conn: &mut SqliteConnection, id: i32) -> Result<usize> {
        Ok(
            diesel::delete(crate::db::schema::pet::table.filter(Self::with_id(id)))
                .execute(conn)?,
        )
    }

    pub fn update(self, conn: &mut SqliteConnection) -> Result<usize> {
        Ok(diesel::update(crate::db::schema::pet::table)
           .filter(Self::with_id(self.id))
            .set(self)
            .execute(conn)?)
    }
}

impl NewPet {
    pub fn save(self, conn: &mut SqliteConnection) -> Result<usize> {
        Ok(diesel::insert_into(crate::db::schema::pet::table)
            .values(&self)
            .execute(conn)?)
    }
}

