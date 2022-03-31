use serde::{Deserialize};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct WorldOwner {
    pub id: i32,
    pub user_id: i32,
    pub world_id: i32,
}