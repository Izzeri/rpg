
pub enum Element {
    Fire,
    Ice,
    Wind,
    Earth,
    Thunder,
    Water,
    Hate,
    Love,
}

pub enum Parameter {
    Str,
    Vit,
    Int,
    Wis,
    Dex,
    Agi,
    Mhp,
    Mmp,
    Map,

    Atk,
    Mat,
    Def,
    Mdf,
    Hrg,
    Mrg,
    Arg,
    Ssr,
    Sdr,
}

pub enum ExParameter {
    Eva,
    Cri,
    Cnt,
    Mrf,
}

pub enum SpParameter {
    Hit,
    Tgr,
    Rec,
    Mcr,
    Acr,
    Pdr,
    Mdr,
    Dmr,
}

pub enum SkillType {
    Common,
    Auramancy,
    Animancy,
    Astromancy,
}

pub enum WeaponType {
    Axe,
    Sword,
    Spear,
}

pub enum ArmorType {
    Helmet,
    Body,
    Legs,
    Boots,
}

pub enum EquipSlot {
    Head,
    Body,
    Legs,
    Feet,
    LeftHand,
    RightHand,
    Ring,
    Necklace,
}

pub enum Feature {
    ElementRate(Element, f32),
    DebuffRate(f32),
    StateRate(f32),
    StateResist(usize),

    Parameter(Parameter, f32),
    ExParameter(ExParameter, f32),
    SpParameter(SpParameter, f32),

    AttackElement(Element),
    AttackState(usize),

    AddSkillType(SkillType),
    DisableSkillType(SkillType),
    AddSkill(usize),
    DisableSkill(usize),

    EquipWeapon(WeaponType),
    EquipArmor(ArmorType),
    LockEquip(EquipSlot),
    SealEquip(EquipSlot),
}

/// Base for all database items
pub struct BaseItem {
    pub id: usize,
    pub name: String,
    pub icon_index: usize,
    pub description: String,
    pub features: Vec<Feature>,
}
