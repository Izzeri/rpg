use super::base_item::*;
use super::usable_item::*;

pub enum Cost {
    Flat(i32),
    Percentage(f32),
    None,
}

pub struct Skill {
    pub base: UsableItem,
    pub skill_type: SkillType,
    pub hp_cost: Cost,
    pub mp_cost: Cost,
    pub ap_cost: Cost,
    pub required_weapon: WeaponType,
}
