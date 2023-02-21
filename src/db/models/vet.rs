use crate::db::schema::vet;
use anyhow::Result;
use diesel::{
    backend::Backend,
    dsl::{AsSelect, Eq},
    prelude::*,
    AsChangeset, Identifiable, Insertable, QueryDsl, QueryResult, Queryable, QueryableByName,
    RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};

#[derive(
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Eq,
    Identifiable,
    PartialEq,
    Queryable,
    QueryableByName,
    Selectable,
    Serialize,
    Insertable,
    Default,
)]
#[diesel(table_name = vet)]
pub struct Vet {
    pub id: i32,
    pub name: String,
}

#[derive(
    Serialize,
    Insertable,
)]
#[diesel(table_name = vet)]
pub struct NewVet {
    pub name: String,
}

type All<DB> = diesel::dsl::Select<vet::table, AsSelect<Vet, DB>>;
type WithId = Eq<vet::id, i32>;
type WithName<'a> = Eq<vet::name, &'a str>;

impl Vet {
    pub fn all<DB>() -> All<DB>
    where
        DB: Backend,
    {
        vet::table.select(Vet::as_select())
    }

    pub fn with_id(id: i32) -> WithId {
        vet::id.eq(id)
    }

    fn with_name(name: &str) -> WithName {
        vet::name.eq(name)
    }

    pub fn vets(conn: &mut SqliteConnection) -> QueryResult<Vec<Self>> {
        Self::all().load(conn)
    }

    pub fn select_by_id(conn: &mut SqliteConnection, id: i32) -> Result<Option<Self>> {
        Ok(crate::db::schema::vet::table
            .filter(Self::with_id(id))
            .get_result::<Self>(conn)
            .optional()?)
    }

    pub fn select_by_name(conn: &mut SqliteConnection, name: &str) -> Result<Vec<Self>> {
        Ok(crate::db::schema::vet::table
            .filter(Self::with_name(name))
            .get_results::<Self>(conn)?)
    }

    pub fn delete_by_id(conn: &mut SqliteConnection, id: i32) -> Result<usize> {
        Ok(
            diesel::delete(crate::db::schema::vet::table.filter(Self::with_id(id)))
                .execute(conn)?,
        )
    }

    pub fn update(self, conn: &mut SqliteConnection) -> Result<usize> {
        Ok(diesel::update(crate::db::schema::vet::table)
           .filter(Self::with_id(self.id))
            .set(self)
            .execute(conn)?)
    }
}

impl NewVet {
    pub fn save(self, conn: &mut SqliteConnection) -> Result<usize> {
        Ok(diesel::insert_into(crate::db::schema::vet::table)
            .values(&self)
            .execute(conn)?)
    }

}
