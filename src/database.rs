use crate::global::DATA_PATH;
use anyhow::Result;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use std::str::FromStr as _;

const DB_NAME: &str = "db.sqlite";

pub(crate) async fn create_sqlite_pool() -> Result<SqlitePool> {
    let database_dir = DATA_PATH.to_string_lossy().replace("\\", "/");
    let database_url = format!("sqlite://{database_dir}/{DB_NAME}");

    let connection_options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let sqlite_pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;

    sqlx::migrate!().run(&sqlite_pool).await?;

    Ok(sqlite_pool)
}
