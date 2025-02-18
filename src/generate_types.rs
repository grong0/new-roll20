use typify::import_types;
use serde_json::Value;

fn main() {
	import_types!(schema = "schema/site-fast/life.json");

	let life = Life {
		life_background: Value::String("red".to_string()),
		life_class: Value::String("the life gamer".to_string()),
		life_trinket: Value::String("locket".to_string())
	};

	life.life_background;
}
