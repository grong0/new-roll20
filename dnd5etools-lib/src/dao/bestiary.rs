use serde::{Deserialize, Deserializer, de::Visitor};

use crate::dao::{
	entry::{Entry, EntryDataData, EntrySpellcasting, MediaHref},
	utils::{
		AdditionalSources, Alias, Alignment, BasicRules, ConditionImmunityArray, CreatureType, DamageImmunityArray, DamageResistArray,
		DamageVulnerabilityArray, DataDamageTags, Group, IsReprinted, Legacy, OtherSources, Page, ProficiencyLevel, ReferenceSources,
		ReprintedAs, SRD, Size, Source, Speed, TagsConditions, TagsSavingThrow,
	},
	utils_copy::{CopyBlockCopyGeneric, Version},
	utils_foundry::FoundryTokenScale,
	utils_token::{AltArt, Token},
};

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Align {
	alignment: Vec<Alignment>,
	chance: i64,
	note: String,
	special: String,
}

fn deserialize_align<'de, D>(deserializer: D) -> Result<Align, D::Error>
where
	D: Deserializer<'de>,
{
	struct AlignVisitor;

	impl<'de> Visitor<'de> for AlignVisitor {
		type Value = Align;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}

		fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			// TODO: this feels super nasty, please rewrite
			let mut alignment: Vec<Alignment> = vec![];
			match v {
				"L" => alignment.push(Alignment::L),
				"N" => alignment.push(Alignment::N),
				"NX" => alignment.push(Alignment::NX),
				"NY" => alignment.push(Alignment::NY),
				"C" => alignment.push(Alignment::C),
				"G" => alignment.push(Alignment::G),
				"E" => alignment.push(Alignment::E),
				"U" => alignment.push(Alignment::U),
				"A" => alignment.push(Alignment::A),
				_ => return Err(E::custom(format!("Encountered unaccounted for alignment: {}", v))),
			}
			Ok(Align { alignment, ..Default::default() })
		}
	}

	return deserializer.deserialize_any(AlignVisitor);
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct AcItem {
	// Object 1
	ac: i64,
	from: Vec<String>,
	condition: String,
	braces: bool,

	// Object 2
	special: String,
}

fn deserialize_ac_item<'de, D>(deserializer: D) -> Result<AcItem, D::Error>
where
	D: Deserializer<'de>,
{
	struct AcItemVisitor;

	impl<'de> Visitor<'de> for AcItemVisitor {
		type Value = AcItem;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}

		fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok(AcItem { ac: v, ..Default::default() })
		}
	}

	return deserializer.deserialize_any(AcItemVisitor);
}

#[derive(Debug, Default, Deserialize)]
pub struct AbilityScore {
	#[serde(default)]
	value: i64,
	#[serde(default)]
	special: String,
}

fn deserialize_ability_score<'de, D>(deserializer: D) -> Result<AbilityScore, D::Error>
where
	D: Deserializer<'de>,
{
	struct AbilityScoreVisitor;

	impl<'de> Visitor<'de> for AbilityScoreVisitor {
		type Value = AbilityScore;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}

		fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok(AbilityScore { value: v, ..Default::default() })
		}
	}

	return deserializer.deserialize_any(AbilityScoreVisitor);
}

/**
 * minimum: 1
 */
type LegendaryActions = i64;

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CreatureData {
	name: String,
	/**
	 * Used anywhere a shortened form of the creatures name is required (e.g. in legendary action headers).
	 * If not supplied, a shortened name will be automatically generated from the creature's full name.
	 * Alternatively use "true" if the "shortName" should be an exact copy of the creature's "name".
	 */
	#[serde(deserialize_with = "deserialize_bool_into_string")]
	short_name: String,
	alias: Alias,
	group: Group,
	// Used in sidekicks, which can have levels (and generally do not have alignment)
	level: i64,
	size: Vec<Size>,
	size_note: String,
	#[serde(rename = "type")]
	// creature_type: CreatureType1 | CreatureType,
	source: Source,
	// Sub-source text that is shown when hovered.
	source_sub: String,
	other_sources: OtherSources,
	reference_sources: ReferenceSources,
	reprinted_as: ReprintedAs,
	is_reprinted: IsReprinted,
	// TODO: fix custom deserializer (struct -> Vec<struct>)
	// #[serde(deserialize_with = "deserialize_align")]
	// alignment: Vec<Align>,
	alignment_prefix: String,
	ac: Vec<AcItem>,
	hp: CreatureHp,
	speed: Speed,
	// initiative: CreatureInitiative | f64,
	#[serde(deserialize_with = "deserialize_ability_score")]
	str: AbilityScore,
	#[serde(deserialize_with = "deserialize_ability_score")]
	dex: AbilityScore,
	#[serde(deserialize_with = "deserialize_ability_score")]
	con: AbilityScore,
	#[serde(deserialize_with = "deserialize_ability_score")]
	int: AbilityScore,
	#[serde(deserialize_with = "deserialize_ability_score")]
	wis: AbilityScore,
	#[serde(deserialize_with = "deserialize_ability_score")]
	cha: AbilityScore,
	save: CreatureSave,
	skill: CreatureSkill,
	tool: CreatureTool,
	// TODO: fix custom deserializer (struct > Vec<struct>)
	// #[serde(deserialize_with = "deserialize_creature_gear")]
	// gear: Vec<CreatureGear>,
	sense: Vec<String>,
	// passive: String | i64,
	languages: Vec<String>,
	pb_note: String,
	cr: CreatureCr,
	vulnerable: DamageVulnerabilityArray,
	resist: DamageResistArray,
	immune: DamageImmunityArray,
	condition_immune: ConditionImmunityArray,
	spellcasting: EntrySpellcasting,
	#[serde(rename = "trait")]
	creature_trait: Vec<CreatureTrait>,
	action_note: String,
	action_header: Vec<Entry>,
	action: Vec<CreatureAction>,
	bonus_note: String,
	bonus_header: Vec<Entry>,
	bonus: Vec<CreatureBonus>,
	reaction_note: String,
	reaction_header: Vec<Entry>,
	reaction: Vec<CreatureReaction>,
	legendary_group: CreatureLegendaryGroup,
	legendary_actions: LegendaryActions,
	legendary_actions_lair: LegendaryActions,
	legendary_header: Vec<Entry>,
	legendary: Vec<CreatureLegendary>,
	mythic_header: Vec<Entry>,
	mythic: CreatureMythic,
	variant: Vec<EntryVariantBestiary>,
	page: Page,
	/**
	 * "enum": [
	 *		true,
	 *		null
	 *	]
	 */
	familiar: bool,
	additional_sources: AdditionalSources,
	has_token: bool,
	token_credit: String,
	token_custom: bool,
	foundry_token_scale: FoundryTokenScale,
	alt_art: AltArt,
	token: Token,
	/**
	 * "enum": [
	 *		true,
	 *		null
	 *	]
	 */
	is_named_creature: bool,
	/**
	 * Used to flag adventure NPCs
	 * "enum": [
	 *		true,
	 *		null
	 *	]
	 */
	is_npc: bool,
	environment: Vec<CreatureEnvironment>,
	treasure: Vec<CreatureTreasure>,
	sound_clip: MediaHref,
	dragon_casting_color: CreatureDragonCastingColor,
	dragon_age: CreatureDragonAge,
	trait_tags: Vec<CreatureTraitTags>,
	action_tags: Vec<CreatureActionTags>,
	// "description": "- X: Any (Choose)\n- XX: All\n- CS: Can't Speak Known Languages\n- LF: Languages Known in Life\n- TP: Telepathy\n- OTH: Other\n- -\n- AB: Abyssal\n- AQ: Aquan\n- AU: Auran\n- C: Common\n- CE: Celestial\n- CSL: Common Sign Language\n- D: Dwarvish\n- DR: Draconic\n- DS: Deep Speech\n- DU: Druidic\n- E: Elvish\n- G: Gnomish\n- GI: Giant\n- GO: Goblin\n- GTH: Gith\n- H: Halfling\n- I: Infernal\n- IG: Ignan\n- O: Orc\n- P: Primordial\n- S: Sylvan\n- T: Terran\n- TC: Thieves' cant\n- U: Undercommon",
	language_tags: Vec<CreatureLanguageTags>,
	// "description": "- B: Blindsight\n- D: Darkvision\n- SD: Superior Darkvision\n- T: Tremorsense\n- U: Truesight",
	sense_tags: Vec<CreatureSenseTags>,
	// "description": "- P: Psionics\n- I: Innate\n- F: Form Only\n- S: Shared\n- O: Other\n- CA: Class, Artificer\n- CB: Class, Bard\n- CC: Class, Cleric\n- CD: Class, Druid\n- CP: Class, Paladin\n- CR: Class, Ranger\n- CS: Class, Sorcerer\n- CL: Class, Warlock\n- CW: Class, Wizard",
	spellcasting_tags: CreatureSpellcastingTags,
	damage_tags: Vec<DataDamageTags>,
	damage_tags_spell: Vec<DataDamageTags>,
	damage_tags_legendary: Vec<DataDamageTags>,
	// "description": "- AOE: Has Areas of Effect\n- CUR: Inflicts Curse\n- DIS: Inflicts Disease\n- HPR: Has HP Reduction\n- MW: Has Weapon Attacks, Melee\n- RW: Has Weapon Attacks, Ranged\n- MA: Has Attacks, Melee\n- RA: Has Attacks, Ranged\n- RCH: Has Reach Attacks\n- MLW: Has Melee Weapons\n- RNG: Has Ranged Weapons\n- THW: Has Thrown Weapons",
	misc_tags: CreatureMiscTags,
	// A collection of UID, e.g. \"longsword|phb\"
	attached_items: Vec<String>,
	condition_inflict: TagsConditions,
	condition_inflict_legendary: TagsConditions,
	condition_inflict_spell: TagsConditions,
	saving_throw_forced: TagsSavingThrow,
	saving_throw_forced_legendary: TagsSavingThrow,
	saving_throw_forced_spell: TagsSavingThrow,
	// Intended for homebrew use only.
	footer: Vec<Entry>,
	legacy: Legacy,
	// The spell used to summon this creature; specifically for TCE-esque summon spells.
	summoned_by_spell: String,
	// The level of the spell used to summon this creature; specifically for TCE-esque summon spells.
	summoned_by_spell_level: i64,
	// The class which can summon this creature; e.g. for those granted by some TCE class features.
	summoned_by_class: String,
	// If this creature should be scalable by summoning/owning player level.
	summoned_scale_by_player_level: bool,
	// An internal flag indicating this creature is a copy of another, and is a temporary/placeholder entry which will be factored out using the \"_copy\" format at a later date.
	_is_copy: bool,
	_versions: Vec<CreatureVersion>,
	has_fluff: bool,
	has_fluff_images: bool,
	srd: SRD,
	srd52: SRD,
	basic_rules: BasicRules,
	basic_rules_2024: BasicRules,
}

/**
 * TODO: similar deserialize in entry I think
 */
fn deserialize_bool_into_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
	D: Deserializer<'de>,
{
	struct BoolIntoStringVisitor;

	impl<'de> Visitor<'de> for BoolIntoStringVisitor {
		type Value = String;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}

		fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok(v.to_string())
		}
	}

	return deserializer.deserialize_any(BoolIntoStringVisitor);
}

#[derive(Debug, Default, Deserialize)]
pub struct CreatureTypeChoose {
	choose: Vec<CreatureType>,
}

#[derive(Debug, Default, Deserialize)]
pub struct CreatureTypeTags {
	#[serde(default)]
	tag: String,
	#[serde(default)]
	prefix: String,
	#[serde(default, rename = "prefixHidden")]
	prefix_hidden: bool,
}

fn deserialize_creature_type_tags<'de, D>(deserializer: D) -> Result<CreatureTypeTags, D::Error>
where
	D: Deserializer<'de>,
{
	struct CreatureTypeTagsVisitor;

	impl<'de> Visitor<'de> for CreatureTypeTagsVisitor {
		type Value = CreatureTypeTags;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}

		fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok(CreatureTypeTags { tag: v.to_string(), ..Default::default() })
		}
	}

	return deserializer.deserialize_any(CreatureTypeTagsVisitor);
}

#[derive(Debug, Default, Deserialize)]
pub enum CreatureTypeSideKickType {
	#[serde(rename = "expert")]
	Expert,
	#[serde(rename = "spellcaster")]
	Spellcaster,
	#[serde(rename = "warrior")]
	Warrior,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CreatureType1 {
	// creature_type: CreatureType | CreatureTypeChoose,
	swarm_size: Size,
	// TODO: fix custom deserializer (struct -> Vec<struct>)
	// #[serde(deserialize_with = "deserialize_creature_type_tags")]
	// tags: Vec<CreatureTypeTags>,
	// TODO: fix custom deserializer (struct -> Vec<struct>)
	// side_kick_type: CreatureTypeSideKickType,
	// #[serde(deserialize_with = "deserialize_creature_type_tags")]
	side_kick_tags: Vec<CreatureTypeTags>,
	side_kick_hidden: bool,
	note: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct CreatureHp {
	// object 1
	#[serde(default)]
	average: i64,
	#[serde(default)]
	formula: String,

	// object 2
	#[serde(default)]
	special: String,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum CreatureInitiativeAdvantageMode {
	#[serde(rename = "adv")]
	Advantage,
	#[serde(rename = "dis")]
	Disadvantage,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
pub struct CreatureInitiative {
	#[serde(default)]
	initiative: f64,
	#[serde(default)]
	proficiency: ProficiencyLevel,
	#[serde(default)]
	advantage_mode: CreatureInitiativeAdvantageMode,
}

#[derive(Debug, Default, Deserialize)]
pub struct CreatureSave {
	#[serde(default)]
	str: String,
	#[serde(default)]
	dex: String,
	#[serde(default)]
	con: String,
	#[serde(default)]
	int: String,
	#[serde(default)]
	wis: String,
	#[serde(default)]
	cha: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct CreatureSkillOther {
	#[serde(default)]
	acrobatics: String,
	#[serde(default, rename = "animal handling")]
	animal_handling: String,
	#[serde(default)]
	arcana: String,
	#[serde(default)]
	athletics: String,
	#[serde(default)]
	deception: String,
	#[serde(default)]
	history: String,
	#[serde(default)]
	insight: String,
	#[serde(default)]
	intimidation: String,
	#[serde(default)]
	investigation: String,
	#[serde(default)]
	medicine: String,
	#[serde(default)]
	nature: String,
	#[serde(default)]
	perception: String,
	#[serde(default)]
	performance: String,
	#[serde(default)]
	persuasion: String,
	#[serde(default)]
	religion: String,
	#[serde(default, rename = "sleight of hand")]
	sleight_of_hand: String,
	#[serde(default)]
	stealth: String,
	#[serde(default)]
	survival: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct CreatureSkill {
	#[serde(default)]
	acrobatics: String,
	#[serde(default, rename = "animal handling")]
	animal_handling: String,
	#[serde(default)]
	arcana: String,
	#[serde(default)]
	athletics: String,
	#[serde(default)]
	deception: String,
	#[serde(default)]
	history: String,
	#[serde(default)]
	insight: String,
	#[serde(default)]
	intimidation: String,
	#[serde(default)]
	investigation: String,
	#[serde(default)]
	medicine: String,
	#[serde(default)]
	nature: String,
	#[serde(default)]
	perception: String,
	#[serde(default)]
	performance: String,
	#[serde(default)]
	persuasion: String,
	#[serde(default)]
	religion: String,
	#[serde(default, rename = "sleight of hand")]
	sleight_of_hand: String,
	#[serde(default)]
	stealth: String,
	#[serde(default)]
	survival: String,
	#[serde(default)]
	other: Vec<CreatureSkillOther>,
}

#[derive(Debug, Default, Deserialize)]
pub struct CreatureTool {
	#[serde(rename = "artisan's tools")]
	artisans_tools: String,
	#[serde(rename = "alchemist's supplies")]
	alchemists_supplies: String,
	#[serde(rename = "brewer's supplies")]
	brewers_supplies: String,
	#[serde(rename = "calligrapher's supplies")]
	calligraphers_supplies: String,
	#[serde(rename = "carpenter's tools")]
	carpenters_tools: String,
	#[serde(rename = "cartographer's tools")]
	cartographers_tools: String,
	#[serde(rename = "cobbler's tools")]
	cobblers_tools: String,
	#[serde(rename = "cook's utensils")]
	cooks_utensils: String,
	#[serde(rename = "glassblower's tools")]
	glassblowers_tools: String,
	#[serde(rename = "jeweler's tools")]
	jewelers_tools: String,
	#[serde(rename = "leatherworker's tools")]
	leatherworkers_tools: String,
	#[serde(rename = "mason's tools")]
	masons_tools: String,
	#[serde(rename = "painter's supplies")]
	painters_supplies: String,
	#[serde(rename = "potter's tools")]
	potters_tools: String,
	#[serde(rename = "smith's tools")]
	smiths_tools: String,
	#[serde(rename = "tinker's tools")]
	tinkers_tools: String,
	#[serde(rename = "weaver's tools")]
	weavers_tools: String,
	#[serde(rename = "woodcarver's tools")]
	woodcarvers_tools: String,
	#[serde(rename = "disguise kit")]
	disguise_kit: String,
	#[serde(rename = "forgery kit")]
	forgery_kit: String,
	#[serde(rename = "gaming set")]
	gaming_set: String,
	#[serde(rename = "dragonchess set")]
	dragonchess_set: String,
	#[serde(rename = "dice set")]
	dice_set: String,
	#[serde(rename = "three-dragon ante set")]
	three_dragon_ante_set: String,
	#[serde(rename = "playing card set")]
	playing_card_set: String,
	#[serde(rename = "herbalism kit")]
	herbalism_kit: String,
	#[serde(rename = "musical instrument")]
	musical_instrument: String,
	bagpipes: String,
	drum: String,
	#[serde(rename = "dulcimer")]
	dulcimer: String,
	flute: String,
	horn: String,
	lute: String,
	lyre: String,
	#[serde(rename = "pan flute")]
	pan_flute: String,
	shawm: String,
	viol: String,
	#[serde(rename = "navigator's tools")]
	navigators_tools: String,
	#[serde(rename = "thieves' tools")]
	thieves_tools: String,
	#[serde(rename = "poisoner's kit")]
	poisoners_kit: String,
	vehicles: String,
	#[serde(rename = "vehicles (air)")]
	vehicles_air: String,
	#[serde(rename = "vehicles (land)")]
	vehicles_land: String,
	#[serde(rename = "vehicles (water)")]
	vehicles_water: String,
	#[serde(rename = "vehicles (space)")]
	vehicles_space: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct CreatureGear {
	// An item UID, e.g. "longsword|phb"
	#[serde(default)]
	item: String,
	#[serde(default)]
	quantity: i64,
	#[serde(default)]
	display_name: String,
}

fn deserialize_creature_gear<'de, D>(deserializer: D) -> Result<CreatureGear, D::Error>
where
	D: Deserializer<'de>,
{
	struct CreatureGearVisitor;

	impl<'de> Visitor<'de> for CreatureGearVisitor {
		type Value = CreatureGear;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}

		fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok(CreatureGear { item: v.to_string(), quantity: 1, ..Default::default() })
		}
	}

	return deserializer.deserialize_any(CreatureGearVisitor);
}

#[derive(Default, Debug, Deserialize)]
pub struct CreatureCr {
	#[serde(default)]
	cr: String,
	#[serde(default)]
	lair: String,
	#[serde(default)]
	coven: String,
	#[serde(default)]
	xp: i64,
	#[serde(default, rename = "xpLair")]
	xp_lair: i64,
}

fn deserialize_creature_cr<'de, D>(deserializer: D) -> Result<CreatureCr, D::Error>
where
	D: Deserializer<'de>,
{
	struct CreatureCrVisitor;

	impl<'de> Visitor<'de> for CreatureCrVisitor {
		type Value = CreatureCr;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}

		fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok(CreatureCr { cr: v.to_string(), ..Default::default() })
		}
	}

	return deserializer.deserialize_any(CreatureCrVisitor);
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreatureTraitType {
	Entries,
	Inset,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CreatureTrait {
	name: String,
	entries: Vec<Entry>,
	#[serde(rename = "type")]
	trait_type: CreatureTraitType,
	// Forces a sort order. Traits with sort orders will always be arranged before those without.
	sort: i64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CreatureAction {
	name: String,
	entries: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CreatureBonus {
	name: String,
	entries: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CreatureReaction {
	name: String,
	entries: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CreatureLegendaryGroup {
	name: String,
	source: Source,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CreatureLegendary {
	name: String,
	entries: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CreatureMythic {
	name: String,
	entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreatureEnvironment {
	Any,
	Underwater,
	Coastal,
	Mountain,
	Grassland,
	Hill,
	Arctic,
	Urban,
	Forest,
	Swamp,
	Underdark,
	Desert,
	Badlands,
	Farmland,
	Planar,
	#[serde(rename = "planar, transitive")]
	PlanarTransitive,
	#[serde(rename = "planar, elemental")]
	PlanarElemental,
	#[serde(rename = "planar, inner")]
	PlanarInner,
	#[serde(rename = "planar, upper")]
	PlanarUpper,
	#[serde(rename = "planar, lower")]
	PlanarLower,
	#[serde(rename = "planar, feywild")]
	PlanarFeywild,
	#[serde(rename = "planar, shadowfell")]
	PlanarShadowfell,
	#[serde(rename = "planar, water")]
	PlanarWater,
	#[serde(rename = "planar, earth")]
	PlanarEarth,
	#[serde(rename = "planar, fire")]
	PlanarFire,
	#[serde(rename = "planar, air")]
	PlanarAir,
	#[serde(rename = "planar, ooze")]
	PlanarOoze,
	#[serde(rename = "planar, magma")]
	PlanarMagma,
	#[serde(rename = "planar, ash")]
	PlanarAsh,
	#[serde(rename = "planar, ice")]
	PlanarIce,
	#[serde(rename = "planar, elemental chaos")]
	PlanarElementalChaos,
	#[serde(rename = "planar, ethereal")]
	PlanarEthereal,
	#[serde(rename = "planar, astral")]
	PlanarAstral,
	#[serde(rename = "planar, arborea")]
	PlanarArborea,
	#[serde(rename = "planar, arcadia")]
	PlanarArcadia,
	#[serde(rename = "planar, beastlands")]
	PlanarBeastlands,
	#[serde(rename = "planar, bytopia")]
	PlanarBytopia,
	#[serde(rename = "planar, elysium")]
	PlanarElysium,
	#[serde(rename = "planar, mount celestia")]
	PlanarMount_Celestia,
	#[serde(rename = "planar, ysgard")]
	PlanarYsgard,
	#[serde(rename = "planar, abyss")]
	PlanarAbyss,
	#[serde(rename = "planar, acheron")]
	PlanarAcheron,
	#[serde(rename = "planar, carceri")]
	PlanarCarceri,
	#[serde(rename = "planar, gehenna")]
	PlanarGehenna,
	#[serde(rename = "planar, hades")]
	PlanarHades,
	#[serde(rename = "planar, nine hells")]
	PlanarNineHells,
	#[serde(rename = "planar, pandemonium")]
	PlanarPandemonium,
	#[serde(rename = "planar, limbo")]
	PlanarLimbo,
	#[serde(rename = "planar, mechanus")]
	PlanarMechanus,
	#[serde(rename = "planar, outlands")]
	PlanarOutlands,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreatureTreasure {
	Any,
	Individual,
	Arcana,
	Armaments,
	Implements,
	Relics,
	#[default]
	None,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreatureDragonCastingColor {
	Black,
	Blue,
	Green,
	Red,
	White,
	Brass,
	Bronze,
	Copper,
	Gold,
	Silver,
	Deep,
	Spirit,
	#[default]
	None,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreatureDragonAge {
	Young,
	Adult,
	Wyrmling,
	Greatwyrm,
	Ancient,
	Aspect,
	#[default]
	None,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum CreatureTraitTags {
	Aggressive,
	Ambusher,
	Amorphous,
	Amphibious,
	#[serde(rename = "Antimagic Susceptibility")]
	AntimagicSusceptibility,
	#[serde(rename = "Beast of Burden")]
	BeastOfBurden,
	Brute,
	Camouflage,
	Charge,
	#[serde(rename = "Damage Absorption")]
	DamageAbsorption,
	#[serde(rename = "Death Burst")]
	DeathBurst,
	#[serde(rename = "Devil's Sight")]
	DevilsSight,
	#[serde(rename = "False Appearance")]
	FalseAppearance,
	#[serde(rename = "Fey Ancestry")]
	FeyAncestry,
	Flyby,
	#[serde(rename = "Hold Breath")]
	HoldBreath,
	Illumination,
	#[serde(rename = "Immutable Form")]
	ImmutableForm,
	#[serde(rename = "Incorporeal Movement")]
	IncorporealMovement,
	#[serde(rename = "Keen Senses")]
	KeenSenses,
	#[serde(rename = "Legendary Resistances")]
	LegendaryResistances,
	#[serde(rename = "Light Sensitivity")]
	LightSensitivity,
	#[serde(rename = "Magic Resistance")]
	MagicResistance,
	#[serde(rename = "Magic Weapons")]
	MagicWeapons,
	Mimicry,
	#[serde(rename = "Pack Tactics")]
	PackTactics,
	Pounce,
	Rampage,
	Reckless,
	Regeneration,
	Rejuvenation,
	Shapechanger,
	#[serde(rename = "Siege Monster")]
	SiegeMonster,
	#[serde(rename = "Sneak Attack")]
	SneakAttack,
	#[serde(rename = "Spell Immunity")]
	SpellImmunity,
	#[serde(rename = "Spider Climb")]
	SpiderClimb,
	#[serde(rename = "Sunlight Sensitivity")]
	SunlightSensitivity,
	#[serde(rename = "Tree Stride")]
	TreeStride,
	Tunneler,
	#[serde(rename = "Turn Immunity")]
	TurnImmunity,
	#[serde(rename = "Turn Resistance")]
	TurnResistance,
	#[serde(rename = "Undead Fortitude")]
	UndeadFortitude,
	#[serde(rename = "Unusual Nature")]
	UnusualNature,
	#[serde(rename = "Water Breathing")]
	WaterBreathing,
	#[serde(rename = "Web Sense")]
	WebSense,
	#[serde(rename = "Web Walker")]
	WebWalker,
	#[default]
	None,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum CreatureActionTags {
	#[serde(rename = "Breath Weapon")]
	BreathWeapon,
	#[serde(rename = "Frightful Presence")]
	FrightfulPresence,
	Multiattack,
	Parry,
	Shapechanger,
	Swallow,
	Teleport,
	Tentacles,
	#[default]
	None,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum CreatureLanguageTags {
	X,
	XX,
	CS,
	LF,
	TP,
	OTH,
	AB,
	AQ,
	AU,
	C,
	CE,
	CSL,
	D,
	DR,
	DS,
	DU,
	E,
	G,
	GI,
	GO,
	GTH,
	H,
	I,
	IG,
	O,
	P,
	S,
	T,
	TC,
	U,
	#[default]
	None,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum CreatureSenseTags {
	B,
	D,
	SD,
	T,
	U,
	#[default]
	None,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum CreatureSpellcastingTags {
	P,
	I,
	F,
	S,
	O,
	CA,
	CB,
	CC,
	CD,
	CP,
	CR,
	CS,
	CL,
	CW,
	#[default]
	None,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum CreatureMiscTags {
	AOE,
	CUR,
	DIS,
	HPR,
	MW,
	RW,
	MA,
	RA,
	RCH,
	MLW,
	RNG,
	THW,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Creature {
	name: String,
	/**
	 * Used anywhere a shortened form of the creatures name is required (e.g. in legendary action headers).
	 * If not supplied, a shortened name will be automatically generated from the creature's full name.
	 * Alternatively use "true" if the "shortName" should be an exact copy of the creature's "name".
	 */
	#[serde(deserialize_with = "deserialize_bool_into_string")]
	short_name: String,
	alias: Alias,
	group: Group,
	// Used in sidekicks, which can have levels (and generally do not have alignment)
	level: i64,
	size: Vec<Size>,
	size_note: String,
	#[serde(rename = "type")]
	// creature_type: CreatureType1 | CreatureType,
	source: Source,
	// Sub-source text that is shown when hovered.
	source_sub: String,
	other_sources: OtherSources,
	reference_sources: ReferenceSources,
	reprinted_as: ReprintedAs,
	is_reprinted: IsReprinted,
	// TODO: fix custom deserializer (struct -> Vec<struct>)
	// #[serde(deserialize_with = "deserialize_align")]
	// alignment: Vec<Align>,
	alignment_prefix: String,
	ac: Vec<AcItem>,
	hp: CreatureHp,
	speed: Speed,
	// initiative: CreatureInitiative | f64,
	#[serde(deserialize_with = "deserialize_ability_score")]
	str: AbilityScore,
	#[serde(deserialize_with = "deserialize_ability_score")]
	dex: AbilityScore,
	#[serde(deserialize_with = "deserialize_ability_score")]
	con: AbilityScore,
	#[serde(deserialize_with = "deserialize_ability_score")]
	int: AbilityScore,
	#[serde(deserialize_with = "deserialize_ability_score")]
	wis: AbilityScore,
	#[serde(deserialize_with = "deserialize_ability_score")]
	cha: AbilityScore,
	save: CreatureSave,
	skill: CreatureSkill,
	tool: CreatureTool,
	// TODO: fix custom deserializer (struct > Vec<struct>)
	// #[serde(deserialize_with = "deserialize_creature_gear")]
	// gear: Vec<CreatureGear>,
	sense: Vec<String>,
	// passive: String | i64,
	languages: Vec<String>,
	pb_note: String,
	cr: CreatureCr,
	vulnerable: DamageVulnerabilityArray,
	resist: DamageResistArray,
	immune: DamageImmunityArray,
	condition_immune: ConditionImmunityArray,
	spellcasting: EntrySpellcasting,
	#[serde(rename = "trait")]
	creature_trait: Vec<CreatureTrait>,
	action_note: String,
	action_header: Vec<Entry>,
	action: Vec<CreatureAction>,
	bonus_note: String,
	bonus_header: Vec<Entry>,
	bonus: Vec<CreatureBonus>,
	reaction_note: String,
	reaction_header: Vec<Entry>,
	reaction: Vec<CreatureReaction>,
	legendary_group: CreatureLegendaryGroup,
	legendary_actions: LegendaryActions,
	legendary_actions_lair: LegendaryActions,
	legendary_header: Vec<Entry>,
	legendary: Vec<CreatureLegendary>,
	mythic_header: Vec<Entry>,
	mythic: CreatureMythic,
	variant: Vec<EntryVariantBestiary>,
	page: Page,
	/**
	 * "enum": [
	 *		true,
	 *		null
	 *	]
	 */
	familiar: bool,
	additional_sources: AdditionalSources,
	has_token: bool,
	token_credit: String,
	token_custom: bool,
	foundry_token_scale: FoundryTokenScale,
	alt_art: AltArt,
	token: Token,
	/**
	 * "enum": [
	 *		true,
	 *		null
	 *	]
	 */
	is_named_creature: bool,
	/**
	 * Used to flag adventure NPCs
	 * "enum": [
	 *		true,
	 *		null
	 *	]
	 */
	is_npc: bool,
	environment: Vec<CreatureEnvironment>,
	treasure: Vec<CreatureTreasure>,
	sound_clip: MediaHref,
	dragon_casting_color: CreatureDragonCastingColor,
	dragon_age: CreatureDragonAge,
	trait_tags: Vec<CreatureTraitTags>,
	action_tags: Vec<CreatureActionTags>,
	// "description": "- X: Any (Choose)\n- XX: All\n- CS: Can't Speak Known Languages\n- LF: Languages Known in Life\n- TP: Telepathy\n- OTH: Other\n- -\n- AB: Abyssal\n- AQ: Aquan\n- AU: Auran\n- C: Common\n- CE: Celestial\n- CSL: Common Sign Language\n- D: Dwarvish\n- DR: Draconic\n- DS: Deep Speech\n- DU: Druidic\n- E: Elvish\n- G: Gnomish\n- GI: Giant\n- GO: Goblin\n- GTH: Gith\n- H: Halfling\n- I: Infernal\n- IG: Ignan\n- O: Orc\n- P: Primordial\n- S: Sylvan\n- T: Terran\n- TC: Thieves' cant\n- U: Undercommon",
	language_tags: Vec<CreatureLanguageTags>,
	// "description": "- B: Blindsight\n- D: Darkvision\n- SD: Superior Darkvision\n- T: Tremorsense\n- U: Truesight",
	sense_tags: Vec<CreatureSenseTags>,
	// "description": "- P: Psionics\n- I: Innate\n- F: Form Only\n- S: Shared\n- O: Other\n- CA: Class, Artificer\n- CB: Class, Bard\n- CC: Class, Cleric\n- CD: Class, Druid\n- CP: Class, Paladin\n- CR: Class, Ranger\n- CS: Class, Sorcerer\n- CL: Class, Warlock\n- CW: Class, Wizard",
	spellcasting_tags: CreatureSpellcastingTags,
	damage_tags: Vec<DataDamageTags>,
	damage_tags_spell: Vec<DataDamageTags>,
	damage_tags_legendary: Vec<DataDamageTags>,
	// "description": "- AOE: Has Areas of Effect\n- CUR: Inflicts Curse\n- DIS: Inflicts Disease\n- HPR: Has HP Reduction\n- MW: Has Weapon Attacks, Melee\n- RW: Has Weapon Attacks, Ranged\n- MA: Has Attacks, Melee\n- RA: Has Attacks, Ranged\n- RCH: Has Reach Attacks\n- MLW: Has Melee Weapons\n- RNG: Has Ranged Weapons\n- THW: Has Thrown Weapons",
	misc_tags: CreatureMiscTags,
	// A collection of UID, e.g. \"longsword|phb\"
	attached_items: Vec<String>,
	condition_inflict: TagsConditions,
	condition_inflict_legendary: TagsConditions,
	condition_inflict_spell: TagsConditions,
	saving_throw_forced: TagsSavingThrow,
	saving_throw_forced_legendary: TagsSavingThrow,
	saving_throw_forced_spell: TagsSavingThrow,
	// Intended for homebrew use only.
	footer: Vec<Entry>,
	legacy: Legacy,
	// The spell used to summon this creature; specifically for TCE-esque summon spells.
	summoned_by_spell: String,
	// The level of the spell used to summon this creature; specifically for TCE-esque summon spells.
	summoned_by_spell_level: i64,
	// The class which can summon this creature; e.g. for those granted by some TCE class features.
	summoned_by_class: String,
	// If this creature should be scalable by summoning/owning player level.
	summoned_scale_by_player_level: bool,
	// An internal flag indicating this creature is a copy of another, and is a temporary/placeholder entry which will be factored out using the \"_copy\" format at a later date.
	_is_copy: bool,
	_versions: Vec<CreatureVersion>,
	has_fluff: bool,
	has_fluff_images: bool,
	srd: SRD,
	srd52: SRD,
	basic_rules: BasicRules,
	basic_rules_2024: BasicRules,
	_copy: CopyBlockCopyGeneric,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct EntryVariantBestiaryToken {
	name: String,
	source: Source,
	page: Page,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct EntryVariantBestiaryVersion {
	// Shared
	name: String,
	source: Source,

	// Object 1
	add_as: String,

	// Object 2
	add_headers_as: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct EntryVariantBestiary {
	name: String,
	// Always "variant"
	entry_type: String,
	source: Source,
	page: Page,
	data: EntryDataData,
	id: String,
	srd: SRD,
	srd52: SRD,
	basic_rules: BasicRules,
	basic_rules_2024: BasicRules,
	entries: Vec<Entry>,
	token: EntryVariantBestiaryToken,
	_version: EntryVariantBestiaryVersion,
}

// TODO: resolve dual type
// pub type CreatureVersion = Version | CreatureData;
pub type CreatureVersion = Version;
