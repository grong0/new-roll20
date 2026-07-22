use serde::{Deserialize, Deserializer, de::Visitor};

use crate::dao::utils::Source;

pub struct CopyModifier {}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct CopyBlockCopyGenericTemplate {
	name: String,
	source: Source,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CopyBlockCopyGeneric {
	_mod: ModObject,
	_templates: Vec<CopyBlockCopyGenericTemplate>,
	_preserve: PreserveObject,
	name: String,
	source: Source,
	/** Used in deity data */
	pantheon: String,
	/** Used in subclass data */
	short_name: String,
	/** Used in subclass data */
	class_name: String,
	/** Used in subclass data */
	class_source: Source,
	/** Used in subclass feature data */
	subclass_source: Source,
	/** Used in subclass feature data */
	subclass_short_name: String,
	/** Used in subclass feature data */
	level: String,
	/** Used in race feature data */
	race_source: Source,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Version {
	name: String,
	source: Source,
	_mod: ModObject,
	/**
	 * Note that, by default, all properties are preserved for a version. To avoid preserving properties, pass an empty object `{}`.
	 * Originally an object that has keys that always have the value 'true'.
	 */
	#[serde(deserialize_with = "deserialize_preserve_object")]
	_preserve: PreserveObject,
}

/**
 * Fields can either be
 * - String
 * - `CopyModifier`
 * - `Vec<CopyModifier>`
 */
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct ModObject {}

/**
 * Might have to implement serialize
 */
type PreserveObject = Vec<String>;

pub fn deserialize_preserve_object<'de, D>(deserializer: D) -> Result<PreserveObject, D::Error>
where
	D: Deserializer<'de>,
{
	struct PreserveObjectVisitor;

	impl<'de> Visitor<'de> for PreserveObjectVisitor {
		type Value = PreserveObject;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}

		fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::MapAccess<'de>,
		{
			let mut preserve_object: PreserveObject = vec![];
			let current_entry = map.next_entry()?;
			while current_entry.is_some() {
				let (current_key, _): (&str, bool) = current_entry.unwrap();
				preserve_object.push(current_key.to_string());
			}
			Ok(preserve_object)
		}
	}

	return deserializer.deserialize_any(PreserveObjectVisitor);
}
