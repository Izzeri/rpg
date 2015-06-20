use super::base_item::*;
use std::collections::BTreeMap;

/// Base for equippable items (i.e. armor and weapons)
pub struct EquipItem {
    pub base: BaseItem,
    pub slot: EquipSlot,
    pub parameters: BTreeMap<Parameter, i32>,
}
