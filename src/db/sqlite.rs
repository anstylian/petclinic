use anyhow::Result;
use diesel::{sqlite::SqliteConnection, Connection};

/*
TODO use this signiture
pub fn oneoff_connection(sqlite: &str) -> Result<SqliteConnection> {
*/
pub fn oneoff_connection() -> Result<SqliteConnection> {
    const SQLITE: &str = "db.sqlite";
    Ok(SqliteConnection::establish(SQLITE)?)
}
