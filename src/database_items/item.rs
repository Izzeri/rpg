use super::usable_item::*;

pub enum ItemType {
    Regular,
    Key,
}

/// Defines a usable item that can be stored in an inventory
pub struct Item {
    pub base: UsableItem,
    pub item_type: ItemType,
    pub price: i32,
    pub consumable: bool,
}
