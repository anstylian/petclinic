use crate::db::schema::user;
use anyhow::Result;
use diesel::{
    backend::Backend,
    dsl::{AsSelect, Eq},
    AsChangeset, ExpressionMethods, Identifiable, QueryDsl, QueryResult, Queryable,
    QueryableByName, RunQueryDsl, Selectable, SelectableHelper, SqliteConnection,
};
use redis::{ErrorKind, FromRedisValue};
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
)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

type All<DB> = diesel::dsl::Select<user::table, AsSelect<User, DB>>;
type WithName<'a> = Eq<user::username, &'a str>;

impl User {
    pub fn all<DB>() -> All<DB>
    where
        DB: Backend,
    {
        user::table.select(User::as_select())
    }

    pub fn with_name(name: &str) -> WithName {
        user::username.eq(name)
    }

    pub fn users(conn: &mut SqliteConnection) -> QueryResult<Vec<Self>> {
        Self::all().load(conn)
    }

    pub fn select_by_name(conn: &mut SqliteConnection, name: &str) -> Result<Self> {
        Ok(crate::db::schema::user::table
            .filter(Self::with_name(name))
            .get_result::<Self>(conn)?)
    }
}

impl FromRedisValue for User {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        if let redis::Value::Data(u) = v {
            let s = String::from_utf8_lossy(u);
            match serde_json::from_str(&s) {
                Ok(u) => Ok(u),
                Err(e) => {
                    tracing::error!("Failed to deserialize: {e:?}");
                    Err((ErrorKind::NoScriptError, "Failed to deserialize error").into())
                }
            }
        } else {
            Err((ErrorKind::TypeError, "Parse to JSON Failed").into())
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn get_users() {
//         let conn = &mut crate::db::sqlite::oneoff_connection().expect("Failed to connect at db");
//         let t = User::users(conn);
//         println!("{t:?}");
//
//         let t = crate::db::schema::user::table
//             .filter(User::with_name("admin"))
//             .get_result::<User>(conn);
//         println!("{t:?}");
//
//         let conn =
//             &mut crate::db::sqlite::oneoff_connection().expect("Failed to connect at sqlite");
//         let t = crate::db::schema::user::table
//             .filter(User::with_name("admin"))
//             .get_result::<User>(conn);
//         println!("{t:?}");
//
//         let t = User::all::<diesel::pg::Pg>();
//         println!("{t:?}");
//     }
// }
