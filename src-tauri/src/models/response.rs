use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Player {
    pub nickname: Option<String>,
    pub level: Option<u32>,
    pub gender: Option<String>,
    pub class: Option<String>,
    pub runes: Option<u32>,
    pub total_runes_obtained: Option<u32>,
    pub last_grace: Option<String>,
    pub great_runes: Option<GreatRune>,
    pub base_attributes: Option<BaseAttribute>,
    pub attributes: Option<Attribute>,
    pub resistances: Option<Resistance>,
    pub coordinates: Option<Coordinate>,
    pub statistics: Option<Statistics>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GreatRune {
    pub name: Option<String>,
    pub activated: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BaseAttribute {
    pub hp: Option<u32>,
    pub max_hp: Option<u32>,
    pub fp: Option<u32>,
    pub max_fp: Option<u32>,
    pub stamina: Option<u32>,
    pub max_stamina: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Attribute {
    pub vigor: Option<u32>,
    pub mind: Option<u32>,
    pub endurance: Option<u32>,
    pub strength: Option<u32>,
    pub dexterity: Option<u32>,
    pub intelligence: Option<u32>,
    pub faith: Option<u32>,
    pub arcane: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Resistance {
    // Immunity: Poison
    pub immunity_poison: Option<u32>,
    pub max_immunity_poison: Option<u32>,

    // Immunity: Scarlet Rot
    pub immunity_scarlet_rot: Option<u32>,
    pub max_immunity_scarlet_rot: Option<u32>,

    // Robustness: Hemorrhage
    pub robustness_hemorrhage: Option<u32>,
    pub max_robustness_hemorrhage: Option<u32>,

    // Robustness: Frostbite
    pub robustness_frostbite: Option<u32>,
    pub max_robustness_frostbite: Option<u32>,

    // Vitality: Deathblight
    pub vitality_deathblight: Option<u32>,
    pub max_vitality_deathblight: Option<u32>,

    // Focus: Sleep
    pub focus_sleep: Option<u32>,
    pub max_focus_sleep: Option<u32>,

    // Focus: Madness
    pub focus_madness: Option<u32>,
    pub max_focus_madness: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Coordinate {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Statistics {
    pub total_deaths: Option<u32>,
    pub total_playtime: Option<u32>,
}
