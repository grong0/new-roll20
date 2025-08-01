use serde_json::{Map, Value};

use super::common::{form_key, Components, Duration, Entry, Range, ScalingLevelDice, Source, Time};

use crate::serde_utils::{
	serde_as_array, serde_as_array_mapping, serde_as_bool, serde_as_object, serde_as_object_from_option, serde_as_string, serde_as_u64,
};

#[derive(Debug)]
pub struct Spell {
	pub name: String,
	pub source: Source,
	pub key: String,
	pub srd: bool,
	pub level: u64,
	pub school: String,
	pub time: Time,
	pub range: Range,
	pub components: Components,
	pub duration: Duration,
	pub entries: Vec<Entry>,
	pub scaling_level_dice: Vec<ScalingLevelDice>,
	pub damage_inflict: Vec<String>,
	pub saving_throw: String,
	pub misc_tags: Vec<String>,
	pub area_tags: Vec<String>,
	pub other_sources: Vec<String>,
	pub entries_higher_level: Vec<String>,
	pub ritual: bool,
	pub condition_inflict: Vec<String>,
	pub affects_creature_type: Vec<String>,
	pub damage_resist: Vec<String>,
}

impl Spell {
	pub fn new(value: Value) -> Spell {
		let object = serde_as_object(&value, Map::new());

		let name = serde_as_string(object.get("name"), "N/A".to_string());
		let source = Source::new(object.get("source"), object.get("page"));

		let mut scaling_level_dice: Vec<ScalingLevelDice> = vec![];
		let scaling_level_dice_option = object.get("scalingLevelDice");
		if scaling_level_dice_option.is_some() {
			let scaling_level_dice_value = scaling_level_dice_option.unwrap();
			if scaling_level_dice_value.is_object() {
				scaling_level_dice = vec![ScalingLevelDice::new(object.get("scalingLevelDice"))];
			} else if scaling_level_dice_value.is_array() {
				scaling_level_dice =
					serde_as_array(object.get("scalingLevelDice")).iter().map(|i| ScalingLevelDice::new(Some(i))).collect();
			} else {
				println!("scalingLevelDice was something other than an object or array")
			}
		}

		let entries_higher_level_array = serde_as_array_mapping(object.get("entriesHigherLevel"), serde_as_object_from_option, Map::new());
		let entries_higher_level = serde_as_array_mapping(
			entries_higher_level_array.get(0).unwrap_or(&Map::new()).get("entries"),
			serde_as_string,
			"N/A".to_string(),
		);
		if entries_higher_level.len() > 1 {
			println!("entriesHigherLevel had a size bigger than 1: {:?}", entries_higher_level.len());
		}

		return Spell {
			key: form_key(&name, &source.name),
			name,
			source,
			srd: serde_as_bool(object.get("srd"), false),
			level: serde_as_u64(object.get("level"), 0),
			school: serde_as_string(object.get("school"), "N/A".to_string()),
			time: Time::new(object.get("time")),
			range: Range::new(object.get("range")),
			components: Components::new(object.get("components")),
			duration: Duration::new(object.get("duration")),
			entries: serde_as_array(object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
			scaling_level_dice,
			damage_inflict: serde_as_array_mapping(object.get("damageInflict"), serde_as_string, "N/A".to_string()),
			saving_throw: serde_as_string(object.get("savingThrow"), "N/A".to_string()),
			misc_tags: serde_as_array_mapping(object.get("miscTags"), serde_as_string, "N/A".to_string()),
			area_tags: serde_as_array_mapping(object.get("areaTags"), serde_as_string, "N/A".to_string()),
			other_sources: serde_as_array_mapping(object.get("otherSources"), serde_as_string, "N/A".to_string()),
			entries_higher_level,
			ritual: serde_as_bool(object.get("ritual"), false),
			condition_inflict: serde_as_array_mapping(object.get("conditionInflict"), serde_as_string, "N/A".to_string()),
			affects_creature_type: serde_as_array_mapping(object.get("affectCreatureType"), serde_as_string, "N/A".to_string()),
			damage_resist: serde_as_array_mapping(object.get("damageResist"), serde_as_string, "N/A".to_string()),
		};
	}
}
