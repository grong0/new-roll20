// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod serde_utils;
mod version_checking;
mod dao;

use std::collections::HashMap;

use version_checking::{is_newer_version, update_data};

use crate::dao::{character_dao::{AbilityScoreImprovement, AbilityScores, Character, DeathSave, HitPointChoice, LevelChoice, Senses}, common::{Abilities, Currency, Details}, DAO};

const UPDATE_DATA: bool = false;

fn main() {
	if UPDATE_DATA && is_newer_version() {
		let result = update_data();
		if result.is_err() {
			println!("{:?}", result.err());
		};
	}
	println!();

	let mut dao = DAO::new();

	dao.characters.insert(String::from("test_character"), Character {
		name: String::from("Test Character"),
		key: String::from("test_character"),
		classes: HashMap::from([(String::from("wizard"), 8), (String::from("fighter"), 5)]),
		xp: 0,
		level: 14,
		level_choices: vec![
			LevelChoice{
				class: String::from("wizard"),
				choice: 0b000,
				ability_score_improvement: vec![],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
			LevelChoice{
				class: String::from("wizard"),
				choice: 0b000,
				ability_score_improvement: vec![],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
			LevelChoice{
				class: String::from("wizard"),
				choice: 0b000,
				ability_score_improvement: vec![],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
			LevelChoice{
				class: String::from("wizard"),
				choice: 0b010,
				ability_score_improvement: vec![Abilities::INTELLIGENCE, Abilities::INTELLIGENCE],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
			LevelChoice{
				class: String::from("wizard"),
				choice: 0b000,
				ability_score_improvement: vec![],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
			LevelChoice{
				class: String::from("wizard"),
				choice: 0b000,
				ability_score_improvement: vec![],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
			LevelChoice{
				class: String::from("wizard"),
				choice: 0b000,
				ability_score_improvement: vec![],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
			LevelChoice{
				class: String::from("wizard"),
				choice: 0b010,
				ability_score_improvement: vec![Abilities::INTELLIGENCE, Abilities::INTELLIGENCE],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
			LevelChoice{
				class: String::from("fighter"),
				choice: 0b000,
				ability_score_improvement: vec![],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
			LevelChoice{
				class: String::from("fighter"),
				choice: 0b000,
				ability_score_improvement: vec![],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
			LevelChoice{
				class: String::from("fighter"),
				choice: 0b000,
				ability_score_improvement: vec![],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
			LevelChoice{
				class: String::from("fighter"),
				choice: 0b010,
				ability_score_improvement: vec![Abilities::INTELLIGENCE, Abilities::DEXTERITY],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
			LevelChoice{
				class: String::from("fighter"),
				choice: 0b000,
				ability_score_improvement: vec![],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
			LevelChoice{
				class: String::from("fighter"),
				choice: 0b010,
				ability_score_improvement: vec![Abilities::DEXTERITY, Abilities::DEXTERITY],
				feat_aquired: String::from(""),
				took_average: true,
				roll_result: 0
			},
		],
		background: String::from("acolyte|phb"),
		race: String::from("human|phb"),
		details: Details {
			age: 0,
			eyes: String::from(""),
			hair: String::from(""),
			skin: String::from(""),
			weight: 0,
			height: String::from(""),
			appearance: String::from(""),
			personality: String::from(""),
			ideal: String::from(""),
			bond: String::from(""),
			flaw: String::from(""),
			backstory: String::from(""),
			physical: String::from(""),
			alignment: String::from(""),
			allies_and_organizations: String::from(""),
			additional_features_and_traits: String::from(""),
			treasure: String::from("")
		},
		inspiration: true,
		ability_scores: AbilityScores {
			strength: 8,
			dexterity: 15,
			constitution: 13,
			intelligence: 15,
			wisdom: 10,
			charisma: 10
		},
		current_hit_points: 10,
		current_temp_hit_points: 12,
		death_save: DeathSave{ success: 0, fail: 0 },
		exhastion_level: 0,
		attacks: vec![],
		currency: Currency { platinum_piece: 12, gold_piece: 43, electrum_piece: 10, silver_piece: 14, copper_piece: 89 },
		inventory: HashMap::new(),
		other_possessions: vec![],
		tool_proficiencies: vec![],
		other_proficiencies: vec![],
		feats_and_traits: vec![],
		spells: vec![],
		jack_of_all_trades: true,
		reliable_talent: true,
		extra_armor_proficiencies: vec![],
		extra_weapon_proficiencies: vec![],
		items: vec![],
		skills: HashMap::new(),
		senses: Senses{
			perception: 0,
			investigation: 0,
			insight: 0
		},
		conditions: vec![],
		diseases: vec![],
		status: vec![]
	});

	println!("{:#?}", dao.characters);

	// new_roll20_lib::run()
}
