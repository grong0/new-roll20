use std::collections::HashMap;

use serde_json::Value;

use super::{
    backgrounds_dao::Background,
    common::{Ability, CharacterItem, CharacterTreasure, Details},
    feats_dao::Feat,
    races_dao::Race,
    skills_dao::Skill,
    spells_dao::Spell,
};

pub enum SkillStatus {
    UNTRAINED,
    TRAINED,
    EXPERTISE,
}

pub struct Senses {
    perception: u8,
    investigation: u8,
    insight: u8,
}

/// Speed could be gotten as a function that takes in from different places because you could have items that give you more speed
pub struct Character {
    name: String,
    xp: u64,
    race: Race,
    classes: HashMap<String, u8>, // TODO: refine and confirm validity
    background: Background,       // TODO: refine and confirm validity
    details: Details,
    weapon_proficiencies: Vec<String>, // TODO: refine and confirm validity
    armor_proficiencies: Vec<String>,  // TODO: refine and confirm validity
    tool_proficiencies: Vec<String>,   // TODO: refine and confirm validity
    feats: Vec<Feat>,                  // TODO: refine and confirm validity
    spells: Vec<Spell>,                // TODO: refine and confirm validity
    items: Vec<CharacterItem>,
    treasure: CharacterTreasure,
    ability: Ability,                    // TODO: refine and confirm validity
    skills: HashMap<Skill, SkillStatus>, // TODO: refine and confirm validity
    current_hp: u64,
    max_hp: u64,
    armor_class: u32,
    initiative: u8,
    senses: Senses, // TODO: refine and confirm validity because it could be a function
}
