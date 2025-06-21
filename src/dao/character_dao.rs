use std::collections::HashMap;

use serde_json::Value;

use super::{
    backgrounds_dao::Background, common::{Ability, CharacterItem, CharacterTreasure, Details}, conditionsdiseases_dao::{Condition, Disease, Status}, feats_dao::Feat, races_dao::Race, skills_dao::Skill, spells_dao::Spell
};

#[derive(Debug)]
pub enum SkillExperience {
    UNTRAINED,
    TRAINED,
    EXPERTISE,
}

#[derive(Debug)]
pub struct Senses {
    perception: u8,
    investigation: u8,
    insight: u8,
}

/// Speed could be gotten as a function that takes in from different places because you could have items that give you more speed
#[derive(Debug)]
pub struct Character {
    name: String,
	key: String,
    xp: u64,
    race: Race,
    classes: HashMap<String, u8>, // TODO: refine and confirm validity
    background: Background,       // TODO: refine and confirm validity
    details: Details,
    extra_weapon_proficiencies: Vec<String>, // TODO: refine and confirm validity
    extra_armor_proficiencies: Vec<String>,  // TODO: refine and confirm validity
    extra_tool_proficiencies: Vec<String>,   // TODO: refine and confirm validity
    feats: Vec<Feat>,                  // TODO: refine and confirm validity
    spells: Vec<Spell>,                // TODO: refine and confirm validity
    items: Vec<CharacterItem>,
    treasure: CharacterTreasure,
    ability: Ability,                    // TODO: refine and confirm validity
    skills: HashMap<Skill, SkillExperience>, // TODO: refine and confirm validity
    current_hp: u64,
    senses: Senses, // TODO: refine and confirm validity because it could be a function
	conditions: Vec<Condition>,
	diseases: Vec<Disease>,
	status: Vec<Status>
}

impl Character {
	pub fn get_max_hp() -> u64 {
		return 10;
	}

	pub fn get_current_ac() -> u32 {
		return 10;
	}

	pub fn get_initiative() -> u8 {
		return 0;
	}

	pub fn get_weapon_proficiencies(self) -> Vec<String> {
		return self.extra_weapon_proficiencies;
	}

	pub fn get_armor_proficiencies(self) -> Vec<String> {
		return self.extra_armor_proficiencies;
	}

	pub fn get_tool_proficiencies(self) -> Vec<String> {
		return self.extra_tool_proficiencies;
	}

}
