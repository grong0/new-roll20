use std::collections::HashMap;

use crate::dao::common::{ArmorProficiencies, WeaponProficiencies};

use super::{
	backgrounds_dao::Background,
	common::{Abilities, CharacterItem, Currency, Details, Die},
	conditionsdiseases_dao::{Condition, Disease, Status},
	races_dao::Race,
	skills_dao::Skill,
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
pub struct HitPoint {
	pub max: u64,
	pub current: i64,
	pub current_temp_point: u64,
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

/// Speed could be gotten as a function that takes in from different places because you could have items that give you more speed
/// Verify all fields to make sure they work in all circumstances
#[derive(Debug)]
pub struct Character {
	name: String,
	key: String,
	classes: HashMap<String, u64>,
	xp: u64,
	background: String, // a background key
	race: String,       // a race key
	details: Details,
	inspiration: bool,
	ability_scores: AbilityScores,
	hit_point: HitPoint,
	death_save: DeathSave,
	exhastion_level: u64,
	attacks: Vec<Attack>,
	currency: Currency,
	inventory: Vec<String>,           // TODO: actually implement
	tool_proficiencies: Vec<String>,  // list of keys
	other_proficiencies: Vec<String>, // list of keys
	feats_and_traits: Vec<String>,    // a list of feat and traits' keys
	spells: Vec<String>,              // a list of spell keys
	jack_of_all_trades: bool,
	reliable_talent: bool,
	extra_weapon_proficiencies: Vec<String>, // list of keys
	extra_armor_proficiencies: Vec<String>,  // list of keys
	items: Vec<CharacterItem>,
	skills: HashMap<String, SkillExperience>, // skill_key key to struct value
	senses: Senses,
	conditions: Vec<String>, // list of keys
	diseases: Vec<String>,   // list of keys
	status: Vec<String>,     // list of keys

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
	pub fn get_hit_die(self, dao: &DAO) -> Vec<Die> {
		let mut hit_dice: Vec<Die> = vec![];
		for (key, value) in self.classes {
			if !dao.classes.contains_key(&key) {
				println!("key of '{}' not in dao classes", key);
				continue;
			}
			hit_dice.push(Die { number: value.clone(), faces: dao.classes.get(&key).unwrap().hit_die.faces.clone() });
		}
		return hit_dice;
	}

	pub fn get_level(&self) -> u8 {
		return 0;
	}

	pub fn get_magic_caster_level(&self) -> u8 {
		return 0;
	}

	pub fn get_proficiency_bonus(&self) -> u8 {
		return 0;
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

	pub fn get_weapon_proficiencies(&self) -> Vec<WeaponProficiencies> {
		return vec![];
	}

	pub fn get_armor_proficiencies(&self) -> Vec<ArmorProficiencies> {
		return vec![];
	}
}
