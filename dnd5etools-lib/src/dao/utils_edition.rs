use std::fmt;

use serde::Deserialize;


/**
 * A context-sensitive behavior hint. Generally, entities marked with `"edition": "one"`
 * will not be modified (as they are assumed to be up-to-date) prior to display/use, and
 * entities lacking an `"edition"` or using `"edition": "classic"` may be modified to
 * better suit modern rules. For example:\n- Classes/subclasses: an edition mismatch
 * between class and subclass will prompt the renderer to add a note that the subclass is
 * from a different game edition, and that feature levels may have to be adjusted
 * accordingly (notably, when rendering synthetic subclass copies)\n- Plutonium, when
 * using the "Modern (2024)" rules version: non-"one" species will be stripped of their
 * ability scores; non-"one" backgrounds will gain extra ability scores; etc.
 */
#[derive(Debug, Default, Deserialize)]
pub enum Edition {
	CLASSIC,
	ONE,
	#[default]
	NONE
}



// impl<'de> Visitor for Edition {
// 	type Value = Edition;

// 	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
// 		return formatter.write_str("expecting either \"classic\" or \"one\"");
// 	}

// 	fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
// 	where
// 		E: serde::de::Error,
// 	{
// 		match v {
// 			String::from("classic") => Ok(Edition::CLASSIC),
// 			String::from("one") => Ok(Edition::ONE),
// 			_ => Err(E::from("not a valid enum")),
// 		}
// 	}
// }
