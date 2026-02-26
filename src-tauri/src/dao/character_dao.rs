use std::collections::HashMap;

use crate::dao::common::{ArmorProficiencies, WeaponProficiencies};

use super::{
	common::{Abilities, CharacterItem, Currency, Details, Die},
	DAO,
};

#[derive(Debug)]
pub enum SkillExperience {
	UNTRAINED,
	TRAINED,
	EXPERTISE,
}

#[derive(Debug)]
pub struct Senses {
	pub perception: u64,
	pub investigation: u64,
	pub insight: u64,
}

#[derive(Debug)]
pub struct AbilityScores {
	pub strength: i64,
	pub dexterity: i64,
	pub constitution: i64,
	pub intelligence: i64,
	pub wisdom: i64,
	pub charisma: i64,
}

#[derive(Debug)]
pub struct DeathSave {
	pub success: u64,
	pub fail: u64,
}

#[derive(Debug)]
pub enum DamageType {
	ACID,
	BLUDGEONING,
	COLD,
	FIRE,
	FORCE,
	LIGHTNING,
	NECROTIC,
	PIERCING,
	POISON,
	PSYCHIC,
	RADIANT,
	SLASHING,
	THUNDER,
}

#[derive(Debug)]
pub struct AttackDamage {
	pub die: Vec<Die>,
	pub ability_bonus: Abilities,
	pub flat_bonus: u64,
	pub crit_die: Vec<Die>,
	pub damage_type: DamageType,
}

#[derive(Debug)]
pub struct Attack {
	pub name: String,
	pub attack_modifier: Abilities,
	pub attack_proficiency_bonus: u64,
	pub proficient: bool,
	pub range: String,
	pub crit_range: u64,
	pub damages: Vec<AttackDamage>,
	pub saving_throw: Abilities,
	pub saving_throw_dc: Abilities,
	pub save_effect: String,
	pub description: String,
}

#[derive(Debug)]
pub struct SpellSlot {
	pub total: u64,
	pub current: u64,
	pub modifier: i64,
}

#[derive(Debug)]
pub struct SpellSlots {
	pub level_1: SpellSlot,
	pub level_2: SpellSlot,
	pub level_3: SpellSlot,
	pub level_4: SpellSlot,
	pub level_5: SpellSlot,
	pub level_6: SpellSlot,
	pub level_7: SpellSlot,
	pub level_8: SpellSlot,
	pub level_9: SpellSlot,
}

#[derive(Debug)]
pub enum AbilityScoreImprovement {
	ATTRIBUTE,
	FEAT
}

#[derive(Debug)]
pub enum HitPointChoice {
	AVERAGE,
	ROLL
}

// TODO: maybe make levelchoice a list of choices that was made per level or is going to make
#[derive(Debug)]
pub struct LevelChoice {
	pub class: String,
	/// 1: ability
	/// 2: feat
	/// 0b10 -> new feat
	pub choice: u8,
	pub ability_score_improvement: Vec<Abilities>,
	pub feat_acquired: String, // feat key
	pub took_average: bool,
	pub roll_result: u8,
	pub new_spells: Vec<String>
}

impl LevelChoice {
	pub fn improved_ability_score(&self) -> bool {
		return self.choice & 0b1 == 0b1;
	}

	pub fn gained_feat(&self) -> bool {
		return (self.choice >> 1) & 0b1 == 0b1;
	}

	pub fn gained_spells(&self) -> bool {
		return (self.choice >> 2) & 0b1 == 0b1;
	}
}

/// TODO: Verify all fields to make sure they work in all circumstances.
/// TODO: Figure out a way to represent the timeline of choices that a
/// player can make to have that timeline functionality.
#[derive(Debug)]
pub struct Character {
	pub name: String,
	pub key: String,
	pub xp: u64,
	pub level: u8,
	pub level_choices: Vec<LevelChoice>,
	pub background: String, // a background key
	pub race: String,       // a race key
	pub details: Details,
	pub inspiration: bool,
	pub ability_scores: AbilityScores,
	pub current_hit_points: u64,
	pub current_temp_hit_points: u64,
	pub death_save: DeathSave,
	pub exhaustion_level: u64,
	pub attacks: Vec<Attack>,
	pub currency: Currency,
	pub inventory: HashMap<String, CharacterItem>, // key item_key to value struct
	pub other_possessions: Vec<String>,
	pub tool_proficiencies: Vec<String>,  // list of keys
	pub other_proficiencies: Vec<String>, // list of keys
	pub feats_and_traits: Vec<String>,    // a list of feat and traits' keys
	pub spells: Vec<String>,              // a list of spell keys
	pub jack_of_all_trades: bool,
	pub reliable_talent: bool,
	pub extra_weapon_proficiencies: Vec<String>, // list of keys
	pub extra_armor_proficiencies: Vec<String>,  // list of keys
	pub items: Vec<CharacterItem>,
	pub skills: HashMap<String, SkillExperience>, // key skill_key to value struct
	pub senses: Senses,
	pub conditions: Vec<String>, // list of keys
	pub diseases: Vec<String>,   // list of keys
	pub status: Vec<String>,     // list of keys

	// TODO: create modifiers and overrides
	// for more customization and to keep the original data
	// global_magic_attack_modifier: i64,
	// carrying_capacity_modifer: i64,
	// global_saving_throw_modifier: i64,
	// passive_perception_modifier: i64,
	// proficiency_bonus_modifier: i64,
	// armor_class_modifier: i64,
	// speed_modifier: i64,
	// initiative_modifier: i64,
	// spell_casting_ability_modifier: i64,
	// spell_save_dc_modifier: i64,
	// spell_attack_bnus_modifier: i64,
}

impl Character {
	pub fn get_hit_points(&self) -> u64 {
		return 0;
	}

	// pub fn get_unplanned_levels(&self) -> Vec<u8> {
	// 	let discrepancy = self.level_choices.len() - self.level;
	// 	println!("test: {}", self.level_choices.len()..self.level as usize);
	// 	if discrepancy <= 0 {
	// 		return vec![];
	// 	}
	// 	return vec![](self.level_choices.len()..self.level as usize).into_iter();
	// }

	pub fn get_classes_and_levels(&self, dao: &DAO) -> HashMap<String, u8> {
		let mut classes = HashMap::<String, u8>::new();
		for choice in &self.level_choices {
			match dao.classes.get(&choice.class) {
				Some(_) => {
					// TODO: redo this mess
					if !classes.contains_key(&choice.class) {
						classes.insert(choice.class.clone(), 1);
					} else {
						let previous_value = classes.get(&choice.class).unwrap();
						classes.insert(choice.class.clone(), *previous_value + 1);
					}
				}
				None => {
					println!("key of '{}' not in dao classes", choice.class);
					continue;
				}
			}
		}
		return classes;
	}

	pub fn get_first_class(&self, dao: &DAO) -> Option<String> {
		if self.level_choices.len() == 0 {
			return None;
		}
		let class = self.level_choices.get(0).unwrap().class.clone();
		if !dao.classes.contains_key(&class) {
			return None;
		}
		return Some(class);
	}

	pub fn get_multiclassed_classes(&self, dao: &DAO) -> Vec<String> {
		let mut multiclassed_classes = vec![];
		match self.get_first_class(dao) {
			Some(first_class) => {
				for (class_key, level) in self.get_classes_and_levels(dao) {
					if class_key != first_class {
						multiclassed_classes.push(class_key);
					}
				}
			}
			None => {}
		}
		return multiclassed_classes;
	}

	pub fn get_total_level(&self, dao: &DAO) -> u8 {
		let mut total_level = 0;
		for (_, level) in self.get_classes_and_levels(dao) {
			total_level += level;
		}
		return total_level;
	}

	pub fn get_hit_die(&self, dao: &DAO) -> Vec<Die> {
		let mut hit_dice: Vec<Die> = vec![];
		for (class_key, level) in &self.get_classes_and_levels(dao) {
			match dao.classes.get(class_key) {
				Some(class) => {
					hit_dice.push(Die { number: level.clone() as u64, faces: class.hit_die.faces.clone() });
				}
				None => {
					println!("key of '{}' not in dao classes", class_key);
					continue;
				}
			}
		}
		return hit_dice;
	}

	pub fn is_multiclass_spellcaster(&self, dao: &DAO) -> bool {
		let mut num_of_spellcasting_classes: f32 = 0f32;
		for (class_key, _) in &self.get_classes_and_levels(dao) {
			match dao.classes.get(class_key) {
				Some(class) => {
					if vec!["full", "1/2", "artificer"].contains(&class.caster_progression.as_str()) {
						num_of_spellcasting_classes += 1f32;
					}
					num_of_spellcasting_classes += class.get_caster_progression_to_value();
				}
				None => {
					println!("key of '{}' not in dao classes", class_key);
					continue;
				}
			}
		}
		return num_of_spellcasting_classes > 1f32;
	}

	pub fn get_magic_caster_level(&self, dao: &DAO) -> u8 {
		let mut magic_caster_level: u8 = 0;
		for (class_key, level) in &self.get_classes_and_levels(dao) {
			match dao.classes.get(class_key) {
				Some(class) => {
					magic_caster_level += match class.caster_progression.as_str() {
						"full" => level.clone(),
						"1/2" => ((level / 2) as f32).floor() as u8,
						"artificer" => ((level / 2) as f32).ceil() as u8,
						_ => 0,
					} as u8;
				}
				None => {
					println!("key of '{}' not in dao classes", class_key);
					continue;
				}
			}
		}
		return magic_caster_level;
	}

	pub fn get_proficiency_bonus(&self, dao: &DAO) -> u8 {
		return (((self.get_total_level(dao) - 1) / 4) as f32).floor() as u8 + 2;
	}

	pub fn get_armor_class(&self) -> u64 {
		/**
		 * Is affected by
		 * conditions?
		 * items?
		 * race?
		 * class?
		 * feats?
		 */
		return 0;
	}

	pub fn get_speed(&self) -> u64 {
		/**
		 * Is affected by
		 * conditions?
		 * items?
		 * race?
		 * class?
		 * feats?
		 */
		return 0;
	}

	pub fn get_initiative(&self) -> u64 {
		/**
		 * Is affected by
		 * conditions?
		 * items?
		 * race?
		 * class?
		 * feats?
		 */
		return 0;
	}

	pub fn get_spell_casting_ability(&self) -> u64 {
		return 0;
	}

	pub fn get_spell_save_dc(&self) -> u64 {
		return 0;
	}

	pub fn get_spell_attack_bonus(&self) -> u64 {
		return 0;
	}

	pub fn get_spell_slots(&self) -> SpellSlots {
		return SpellSlots {
			level_1: SpellSlot { total: 1, current: 1, modifier: 0 },
			level_2: SpellSlot { total: 1, current: 1, modifier: 0 },
			level_3: SpellSlot { total: 1, current: 1, modifier: 0 },
			level_4: SpellSlot { total: 1, current: 1, modifier: 0 },
			level_5: SpellSlot { total: 1, current: 1, modifier: 0 },
			level_6: SpellSlot { total: 1, current: 1, modifier: 0 },
			level_7: SpellSlot { total: 1, current: 1, modifier: 0 },
			level_8: SpellSlot { total: 1, current: 1, modifier: 0 },
			level_9: SpellSlot { total: 1, current: 1, modifier: 0 },
		};
	}

	pub fn get_all_spells(&self, dao: &DAO) -> Vec<&String> {
		// TODO: maybe convert this to a filter
		let mut spells: Vec<&String> = vec![];
		for choice in &self.level_choices {
			spells.extend(choice.new_spells.iter().filter(|spell_key| dao.spells.contains_key(spell_key.to_owned())));
		}
		return spells;
	}

	pub fn get_spells_from_level(&self, dao: &DAO, level: u8) -> Vec<&String> {
		// TODO: maybe convert this to a filter
		let mut spells: Vec<&String> = vec![];
		for spell_key in self.get_all_spells(dao) {
			if dao.spells.contains_key(spell_key) && dao.spells.get(spell_key).unwrap().level == level as u64 {
				spells.push(spell_key);
			}
		}
		return spells;
		// return self.get_all_spells(dao).iter().filter(|spell_key| dao.spells.get(spell_key.to_owned()).unwrap().level == level as u64).collect();
	}

	pub fn get_weapon_proficiencies(&self) -> Vec<WeaponProficiencies> {
		return vec![];
	}

	pub fn get_armor_proficiencies(&self) -> Vec<ArmorProficiencies> {
		return vec![];
	}
}
