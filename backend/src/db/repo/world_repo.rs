use shared::domain::world::World;
use crate::types::error::Error::DBQueryError;
use super::{ super::get_db_con, super::DBResult};
use crate::{ types::types::DBPool};
use chrono::{DateTime, Utc};
// use common::*;
use mobc_postgres::tokio_postgres::Row;

pub const TABLE: &str = "world";
const SELECT_FIELDS: &str = "id, name, created_at, updated_at";

pub async fn fetch(db_pool: &DBPool) -> DBResult<Vec<World>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {}", SELECT_FIELDS, TABLE);
    let rows = con.query(query.as_str(), &[]).await.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_world(&r)).collect())
}

fn row_to_world(row: &Row) -> World {
    let id: i32 = row.get(0);
    let name: String = row.get(1);

    let created_at_str: String = row.get(2);
    let updated_at_str: String = row.get(3);

    let created_at = DateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S %z UTC").unwrap().with_timezone(&Utc);
    let updated_at = DateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S %z UTC").unwrap().with_timezone(&Utc);

    World { 
        id,
        name,
        created_at,
        updated_at
     }
}