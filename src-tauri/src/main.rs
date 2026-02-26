// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dao;
mod serde_utils;
mod version_checking;

use std::{
	any::{type_name, type_name_of_val},
	collections::HashMap,
};

use version_checking::{is_newer_version, update_data};

use crate::dao::{
	character_dao::{AbilityScoreImprovement, AbilityScores, Character, DeathSave, HitPointChoice, LevelChoice, Senses},
	common::{Abilities, Currency, Details},
	DAO,
};

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

	dao.characters.insert(
		String::from("test_character"),
		Character {
			name: String::from("Test Character"),
			key: String::from("test_character"),
			xp: 0,
			level: 14,
			level_choices: vec![
				// Level 1
				LevelChoice {
					class: String::from("wizard|phb"),
					choice: 0b00,
					ability_score_improvement: vec![],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![
						String::from("ray_of_frost|phb"),
						String::from("prestidigitation|phb"),
						String::from("fire_bolt|phb"),
						String::from("thunderwave|phb"),
						String::from("magic_missile|phb"),
						String::from("detect_magic|phb"),
						String::from("mage_armor|phb"),
						String::from("feather_fall|phb"),
						String::from("sleep|phb"),
					],
				},
				// Level 2
				LevelChoice {
					class: String::from("wizard|phb"),
					choice: 0b00,
					ability_score_improvement: vec![],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![String::from("grease|phb"), String::from("chromatic_orb|phb")],
				},
				// Level 3
				LevelChoice {
					class: String::from("wizard|phb"),
					choice: 0b00,
					ability_score_improvement: vec![],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![String::from("web|phb"), String::from("shatter|phb")],
				},
				// Level 4
				LevelChoice {
					class: String::from("wizard|phb"),
					choice: 0b01,
					ability_score_improvement: vec![Abilities::INTELLIGENCE, Abilities::INTELLIGENCE],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![String::from("mage_hand|phb"), String::from("mirror_image|phb"), String::from("misty_step|phb")],
				},
				// Level 5
				LevelChoice {
					class: String::from("wizard|phb"),
					choice: 0b00,
					ability_score_improvement: vec![],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![String::from("fireball|phb"), String::from("fear|phb")],
				},
				// Level 6
				LevelChoice {
					class: String::from("wizard|phb"),
					choice: 0b00,
					ability_score_improvement: vec![],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![String::from("slow|phb"), String::from("haste|phb")],
				},
				// Level 7
				LevelChoice {
					class: String::from("wizard|phb"),
					choice: 0b00,
					ability_score_improvement: vec![],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![String::from("dimension_door|phb"), String::from("polymorph|phb")],
				},
				// Level 8
				LevelChoice {
					class: String::from("wizard|phb"),
					choice: 0b01,
					ability_score_improvement: vec![Abilities::INTELLIGENCE, Abilities::INTELLIGENCE],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![String::from("locate_creature|phb"), String::from("banishment|phb")],
				},
				// Level 9
				LevelChoice {
					class: String::from("fighter|phb"),
					choice: 0b00,
					ability_score_improvement: vec![],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![],
				},
				// Level 10
				LevelChoice {
					class: String::from("fighter|phb"),
					choice: 0b00,
					ability_score_improvement: vec![],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![],
				},
				// Level 11
				LevelChoice {
					class: String::from("fighter|phb"),
					choice: 0b00,
					ability_score_improvement: vec![],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![],
				},
				// Level 12
				LevelChoice {
					class: String::from("fighter|phb"),
					choice: 0b01,
					ability_score_improvement: vec![Abilities::INTELLIGENCE, Abilities::DEXTERITY],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![],
				},
				// Level 13
				LevelChoice {
					class: String::from("fighter|phb"),
					choice: 0b00,
					ability_score_improvement: vec![],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![],
				},
				// Level 14
				LevelChoice {
					class: String::from("fighter|phb"),
					choice: 0b01,
					ability_score_improvement: vec![Abilities::DEXTERITY, Abilities::DEXTERITY],
					feat_acquired: String::from(""),
					took_average: true,
					roll_result: 0,
					new_spells: vec![],
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
				treasure: String::from(""),
			},
			inspiration: true,
			ability_scores: AbilityScores { strength: 8, dexterity: 15, constitution: 13, intelligence: 15, wisdom: 10, charisma: 10 },
			current_hit_points: 10,
			current_temp_hit_points: 12,
			death_save: DeathSave { success: 0, fail: 0 },
			exhaustion_level: 0,
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
			senses: Senses { perception: 0, investigation: 0, insight: 0 },
			conditions: vec![],
			diseases: vec![],
			status: vec![],
		},
	);

	let character = dao.characters.get("test_character").unwrap();

	println!("{:#?}", character.get_classes_and_levels(&dao));
	println!("{:#?}", character.get_first_class(&dao));
	println!("{:#?}", character.get_multiclassed_classes(&dao));
	println!("{:#?}", character.get_total_level(&dao));
	println!("{:#?}", character.get_hit_die(&dao));
	println!("{:#?}", character.is_multiclass_spellcaster(&dao));
	println!("{:#?}", character.get_magic_caster_level(&dao));
	println!("{:#?}", character.get_proficiency_bonus(&dao));
	println!("{:#?}", character.get_all_spells(&dao));
	println!("{:#?}", character.get_all_spells(&dao).len());
	println!("{:#?}", character.get_spells_from_level(&dao, 0));
	println!("{:#?}", character.get_spells_from_level(&dao, 1));
	println!("{:#?}", character.get_spells_from_level(&dao, 2));
	println!("{:#?}", character.get_spells_from_level(&dao, 3));

	// new_roll20_lib::run()
}
