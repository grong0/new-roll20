use std::collections::HashMap;

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
}

#[derive(Debug)]
pub struct SpellSlots {
    pub level_1: SpellSlot,
    pub level_1_mod: i64,
    pub level_2: SpellSlot,
    pub level_2_mod: i64,
    pub level_3: SpellSlot,
    pub level_3_mod: i64,
    pub level_4: SpellSlot,
    pub level_4_mod: i64,
    pub level_5: SpellSlot,
    pub level_5_mod: i64,
    pub level_6: SpellSlot,
    pub level_6_mod: i64,
    pub level_7: SpellSlot,
    pub level_7_mod: i64,
    pub level_8: SpellSlot,
    pub level_8_mod: i64,
    pub level_9: SpellSlot,
    pub level_9_mod: i64,
}

/// Speed could be gotten as a function that takes in from different places because you could have items that give you more speed
/// Verify all fields to make sure they work in all circumstances
#[derive(Debug)]
pub struct Character {
    name: String,
    key: String,
    classes: HashMap<String, u64>,
    xp: u64,
    level: u64,
    background: Background,
    race: Race,
    details: Details,
    inspiration: bool,
    ability_scores: AbilityScores,
    proficiency_bonus: i64,
    armor_class: u64,
    initiative: i64,
    speed: u64,
    hit_point: HitPoint,
    hit_die: Vec<Die>,
    death_save: DeathSave,
    exhastion_level: u64,
    attacks: Vec<Attack>,
    currency: Currency,
    equipment: Vec<String>,           // TODO: actually implement
    tool_proficiencies: Vec<String>,  // TODO: actually implement
    other_proficiencies: Vec<String>, // TODO: actually implement
    feats_and_traits: Vec<String>,    // a list of feat and traits' keys
    spell_casting_ability: i64,
    spell_save_dc: i64,
    spell_attack_bonus: i64,
    spells: Vec<String>, // a list of spell keys
    spell_slots: SpellSlots,
    carrying_capacity_modifer: i64,
    global_magic_attack_modifier: i64,
    magic_caster_level: u64,
    spell_save_dc_modifier: i64,
    initiative_modifier: i64,
    death_save_modifier: i64,
    global_saving_throw_modifier: i64,
    jack_of_all_trades: bool,
    reliable_talent: bool,
    passive_perception_modifier: i64,

    extra_weapon_proficiencies: Vec<String>,
    extra_armor_proficiencies: Vec<String>,
    items: Vec<CharacterItem>,
    skills: HashMap<Skill, SkillExperience>,
    senses: Senses,
    conditions: Vec<Condition>,
    diseases: Vec<Disease>,
    status: Vec<Status>,
}

impl Character {
    pub fn compile_hit_die(self, dao: &DAO) -> Vec<Die> {
        let mut hit_dice: Vec<Die> = vec![];
        for (key, value) in self.classes {
            if !dao.classes.contains_key(&key) {
                println!("key of '{}' not in dao classes", key);
                continue;
            }
            hit_dice.push(Die {
                number: value.clone(),
                faces: dao.classes.get(&key).unwrap().hit_die.faces.clone(),
            });
        }
        return hit_dice;
    }
}
