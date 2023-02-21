use crate::db::schema::visit;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Eq)]
#[diesel(table_name = visit)]
pub struct Visit {
    pub id: Option<i32>,
    pub pet_id: i32,
    pub vet_id: i32,
    pub visit_date: NaiveDateTime,
    pub notes: Option<String>,
}
