use serde_json::{Map, Value};

use super::common::{form_key, Entry, PackItem, Source};

use crate::serde_utils::{serde_as_array, serde_as_array_mapping, serde_as_bool, serde_as_object, serde_as_string, serde_as_u64};

#[derive(Debug)]
pub struct Item {
	pub name: String,
	pub source: Source,
	pub key: String,
	pub type_var: String,
	pub rarity: String,
	pub weight: u64,
	pub weapon_category: String,
	pub age: String,
	pub property: Vec<String>,
	pub range: String,
	pub reload: u64,
	pub dmg1: String,
	pub dmg_type: String,
	pub firearm: bool,
	pub weapon: bool,
	pub ammo_type: String,
	pub srd: bool,
	pub basic_rules: bool,
	pub value: u64,
	pub arrow: bool,
	pub pack_contents: Vec<PackItem>,
	pub dmg2: String,
	pub axe: bool,
	pub entries: Vec<Entry>,
	pub ac: u64,
	pub armor: bool,
	pub strength: u64,
	pub stealth: bool,
	pub club: bool,
	pub bolt: bool,
	pub scf_type: String,
	pub dagger: bool,
	pub sword: bool,
	pub polearm: bool,
	pub crossbow: bool,
	pub spear: bool,
	pub hammer: bool,
	pub bow: bool,
	pub mace: bool,
	pub net: bool,
	pub staff: bool,
	pub bullet_sling: bool,
}

impl Item {
	pub fn new(value: Value) -> Item {
		let object = serde_as_object(&value, Map::new());

		let name = serde_as_string(object.get("name"), "N/A".to_string());
		let source = Source::new(object.get("source"), object.get("page"));

		return Item {
			key: form_key(&name, &source.name),
			name,
			source,
			type_var: serde_as_string(object.get("type"), "N/A".to_string()),
			rarity: serde_as_string(object.get("rarity"), "N/A".to_string()),
			weight: serde_as_u64(object.get("weight"), 0),
			weapon_category: serde_as_string(object.get("weaponCategory"), "N/A".to_string()),
			age: serde_as_string(object.get("age"), "N/A".to_string()),
			property: serde_as_array_mapping(object.get("property"), serde_as_string, "N/A".to_string()),
			range: serde_as_string(object.get("range"), "N/A".to_string()),
			reload: serde_as_u64(object.get("reload"), 0),
			dmg1: serde_as_string(object.get("dmg1"), "N/A".to_string()),
			dmg_type: serde_as_string(object.get("dmgType"), "N/A".to_string()),
			firearm: serde_as_bool(object.get("firearm"), false),
			weapon: serde_as_bool(object.get("weapon"), false),
			ammo_type: serde_as_string(object.get("ammoType"), "N/A".to_string()),
			srd: serde_as_bool(object.get("srd"), false),
			basic_rules: serde_as_bool(object.get("basicRules"), false),
			value: serde_as_u64(object.get("value"), 0),
			arrow: serde_as_bool(object.get("arrow"), false),
			pack_contents: serde_as_array(object.get("packContents")).iter().map(|i| PackItem::new(i)).collect(),
			dmg2: serde_as_string(object.get("dmg2"), "N/A".to_string()),
			axe: serde_as_bool(object.get("axe"), false),
			entries: serde_as_array(object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
			ac: serde_as_u64(object.get("ac"), 0),
			armor: serde_as_bool(object.get("armor"), false),
			strength: serde_as_u64(object.get("strength"), 0),
			stealth: serde_as_bool(object.get("stealth"), false),
			club: serde_as_bool(object.get("club"), false),
			bolt: serde_as_bool(object.get("bolt"), false),
			scf_type: serde_as_string(object.get("scfType"), "N/A".to_string()),
			dagger: serde_as_bool(object.get("dagger"), false),
			sword: serde_as_bool(object.get("sword"), false),
			polearm: serde_as_bool(object.get("polearm"), false),
			crossbow: serde_as_bool(object.get("crossbow"), false),
			spear: serde_as_bool(object.get("spear"), false),
			hammer: serde_as_bool(object.get("hammer"), false),
			bow: serde_as_bool(object.get("bow"), false),
			mace: serde_as_bool(object.get("mace"), false),
			net: serde_as_bool(object.get("net"), false),
			staff: serde_as_bool(object.get("staff"), false),
			bullet_sling: serde_as_bool(object.get("bulletSling"), false),
		};
	}
}
