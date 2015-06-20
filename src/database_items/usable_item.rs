use super::base_item::*;

pub enum Scope {
    Noone,
    User,
    OnePerson,
    WholeParty,
    OneOrWholeParty,
    Everyone,
}

pub enum Occasion {
    Always,
    Battle,
    Menu,
    Never,
}

pub enum HitType {
    CertainHit,
    PhysicalAttack,
    MagicalAttack,
}

pub enum DamageType {
    None,
    HpDamage,
    MpDamage,
    ApDamage,
    HpRecovery,
    MpRecovery,
    ApRecovery,
    HpDrain,
    MpDrain,
    ApDrain,
}

pub struct Damage {
    pub damage_type: DamageType,
    pub element: Element,
    pub formula: String,
    pub variance: f32,
    pub can_crit: bool,
}

pub enum Effect {
    RecoverHp(f32, i32),
    RecoverMp(f32, i32),
    RecoverAp(f32, i32),
    AddState(usize, f32),
    RemoveState(usize, f32),
    AddBuff(Parameter, i32),
    AddDebuff(Parameter, i32),
    RemoveBuff(Parameter),
    RemoveDebuff(Parameter),
    RaiseParameter(Parameter, i32),
    Escape,
}

/// Defines the base for a usable item (i.e. skills and items)
pub struct UsableItem {
    pub base: BaseItem,
    pub scope: Scope,
    pub occasion: Occasion,
    pub hit_type: HitType,
    pub damage: Damage,
    pub effects: Vec<Effect>,
}
