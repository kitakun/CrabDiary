use crate::types::types::DBPool;
use std::convert::Infallible;

use warp::{
    http::{header, Method},
    Filter,
};

mod db;
mod types;
mod controllers;

#[tokio::main]
async fn main() {
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    // controllers definitions goes here:
    let world_path = warp::path("world");

    let world_routes = world_path
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(controllers::world_controller::list_worlds_handler);

    // handle errors+cors from all routes
    let routes = world_routes
        .recover(types::error::handle_rejection)
        .with(
            warp::cors()
                .allow_credentials(true)
                .allow_methods(&[
                    Method::OPTIONS,
                    Method::GET,
                    Method::POST,
                    Method::DELETE,
                    Method::PUT,
                ])
                .allow_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
                .expose_headers(vec![header::LINK])
                .max_age(300)
                .allow_any_origin(),
        );

    println!("Start listening for 127.0.0.1:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

    
fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}