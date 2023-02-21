use crate::{context::Context, db::models::user::User, AppError};
use anyhow::Result;
use rand::{distributions::Alphanumeric, Rng};
use sha1::{Digest, Sha1};
use std::sync::Arc;

pub async fn authenticate(
    username: &str,
    password: &str,
    ctx: &Arc<Context>,
) -> Result<Option<User>> {
    let db_conn = ctx.db_connection_pool.get().await?;
    let u = username.to_owned();
    let user = db_conn
        .interact(move |conn| User::select_by_name(conn, &u))
        .await
        .map_err(|e| AppError {
            inner: anyhow::Error::msg(e.to_string()),
        });

    match user {
        Ok(Ok(user)) => {
            // Password verification
            let mut hasher = Sha1::new();
            hasher.update(password);
            let encrypted: String = format!("{:x}", hasher.finalize());
            if encrypted == user.password || user.password.is_empty() {
                Ok(Some(user))
            } else {
                tracing::error!("Wrong password for user: {username}");
                Ok(None)
            }
        }
        Ok(Err(e)) => {
            tracing::error!("User not found: {username}, '{e:?}'");
            Ok(None)
        }
        Err(e) => {
            tracing::error!("User not found: {username}, '{e:?}'");
            Ok(None)
        }
    }
}

pub fn session_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
