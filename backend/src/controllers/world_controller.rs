use crate::{
    db::repo::world_repo,
    types::types::DBPool,
    types::types::Result
};

use warp::{
    reject,
    reply::json,
    Reply
};

pub async fn list_worlds_handler(db_pool: DBPool) -> Result<impl Reply> {
    let worlds_list = world_repo::fetch(&db_pool).await.map_err(reject::custom)?;

    println!("Request world_controller.list_worlds_handler world_list.count = {}", worlds_list.len());
    // as is for now
    Ok(json::<Vec<_>>(&worlds_list))
}
