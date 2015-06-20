use super::base_item::*;
use std::collections::BTreeMap;

pub struct Actor {
    pub class: usize,
    pub initial_level: i32,
    pub max_level: i32,
    pub character_sprite_filename: String,
    pub face_sprite_filename: String,
    pub equipment: BTreeMap<EquipSlot, usize>,
}
