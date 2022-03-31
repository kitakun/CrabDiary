use mobc::Connection;
use warp::Rejection;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::NoTls;
use mobc::{Pool};

pub type Result<T> = std::result::Result<T, Rejection>;
pub type DBCon = Connection<PgConnectionManager<NoTls>>;
pub type DBPool = Pool<PgConnectionManager<NoTls>>;