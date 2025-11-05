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
pub struct LevelChoice {
	pub class: String,
	/// 0: neither
	/// 1: attribute
	/// 2: feat
	/// 4: average
	/// 8: roll
	pub choice: u8,
	pub ability: Abilities,
	pub feat: String, // feat key
	pub hit_point_result: u8,
}

/// TODO: Verify all fields to make sure they work in all circumstances.
/// TODO: Figure out a way to represent the timeline of choices that a
/// player can make to have that timeline functionality.
#[derive(Debug)]
pub struct Character {
	pub name: String,
	pub key: String,
	pub classes: HashMap<String, u64>,
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
	pub exhastion_level: u64,
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

	pub fn get_total_level(&self, dao: &DAO) -> u8 {
		let mut total_level = 0;
		for (_, value) in self.get_classes_and_levels(dao) {
			total_level += value;
		}
		return total_level;
	}

	pub fn get_hit_die(&self, dao: &DAO) -> Vec<Die> {
		let mut hit_dice: Vec<Die> = vec![];
		for (key, value) in &self.classes {
			match dao.classes.get(key) {
				Some(_) => {
					hit_dice.push(Die { number: value.clone(), faces: dao.classes.get(key).unwrap().hit_die.faces.clone() });
				}
				None => {
					println!("key of '{}' not in dao classes", key);
					continue;
				}
			}
		}
		return hit_dice;
	}

	pub fn is_multiclass_spellcaster(&self, dao: &DAO) -> bool {
		let mut num_of_spellcasting_classes: f32 = 0f32;
		for (key, _) in &self.classes {
			match dao.classes.get(key) {
				Some(class) => {
					if vec!["full", "1/2", "artificer"].contains(&class.caster_progression.as_str()) {
						num_of_spellcasting_classes += 1f32;
					}
					num_of_spellcasting_classes += class.get_caster_progression_to_value();
				}
				None => {
					println!("key of '{}' not in dao classes", key);
					continue;
				}
			}
		}
		return num_of_spellcasting_classes > 1f32;
	}

	pub fn get_magic_caster_level(&self, dao: &DAO) -> u8 {
		let mut level: u8 = 0;
		for (key, value) in &self.classes {
			match dao.classes.get(key) {
				Some(class) => {
					level += match class.caster_progression.as_str() {
						"full" => value.clone(),
						"1/2" => ((value / 2) as f32).floor() as u64,
						"artificer" => ((value / 2) as f32).ceil() as u64,
						_ => 0,
					} as u8;
				}
				None => {
					println!("key of '{}' not in dao classes", key);
					continue;
				}
			}
		}
		return level;
	}

	pub fn get_proficiency_bonus(&self, dao: &DAO) -> u8 {
		return (((self.get_total_level(dao) - 1) / 4) as f32).floor() as u8 + 2;
	}

	pub fn get_armor_class(&self) -> u64 {
		return 0;
	}

	pub fn get_speed(&self) -> u64 {
		return 0;
	}

	pub fn get_initiative(&self) -> u64 {
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

	pub fn get_spells(&self, dao: &DAO, level: u8) -> Vec<String> {
		// TODO: maybe convert this to a filter
		let mut spells: Vec<String> = vec![];
		for spell in &self.spells {
			if dao.spells.contains_key(spell) && dao.spells.get(spell).unwrap().level == level as u64 {
				spells.push(spell.clone());
			}
		}
		return spells;
	}

	pub fn get_weapon_proficiencies(&self) -> Vec<WeaponProficiencies> {
		return vec![];
	}

	pub fn get_armor_proficiencies(&self) -> Vec<ArmorProficiencies> {
		return vec![];
	}
}
