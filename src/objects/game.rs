use super::object::Object;
use crate::render::{messages::Messages, *};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub map: Map,
    pub messages: Messages,
    pub inventory: Vec<Object>,
    pub dungeon_level: u32,
}
