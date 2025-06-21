use std::collections::HashMap;

use serde_json::Value;

use super::{backgrounds_dao::Background, common::{CharacterItem, CharacterTreasure, Details}, feats_dao::Feat, races_dao::Race, spells_dao::Spell};

pub struct Character {
	name: String,
	xp: u64,
	race: Race,
	classes: HashMap<String, u8>,
	background: Background,
	details: Details,
	weapon_proficiencies: Vec<String>,
	armor_proficiencies: Vec<String>,
	tool_proficiencies: Vec<String>,
	feats: Vec<Feat>,
	spells: Vec<Spell>,
	items: Vec<CharacterItem>,
	treasure: CharacterTreasure
}
