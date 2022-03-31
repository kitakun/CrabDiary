// pub mod owner;
// pub mod pet;
pub mod repo;

use crate::{types::error, types::error::Error::*, types::types::DBCon, types::types::DBPool};
use mobc::Pool;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::fs;

use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls};

pub type DBResult<T> = std::result::Result<T, error::Error>;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;
const INIT_SQL: &str = "scripts";

pub async fn init_db(db_pool: &DBPool) -> DBResult<()> {
    let con = get_db_con(db_pool).await?;

    let current_dir = std::env::current_dir().unwrap();

    let path = current_dir
        .join("backend")
        .join("src")
        .join("db")
        .join(INIT_SQL);

    let script_pathes = fs::read_dir(path).unwrap();

    for entry in script_pathes {
        let entry_path = entry?.path();
        let file_name = entry_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        println!("Try to load SQL on path=({})", file_name);

        let init_file = fs::read_to_string(entry_path)?;

        con.batch_execute(init_file.as_str())
            .await
            .map_err(DBInitError)?;

        println!("Done with ({})", file_name);
    }

    Ok(())
}

pub async fn get_db_con(db_pool: &DBPool) -> DBResult<DBCon> {
    db_pool.get().await.map_err(DBPoolError)
}

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    // TODO get config from file or env
    let mut config = Config::new();
    config
        .host("localhost")
        .user("tester")
        .port(5432)
        .password("tester")
        .dbname("rust_diary");

    // let config = Config::from_str("postgres://postgres@127.0.0.1:7878/pet_owner")?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}
