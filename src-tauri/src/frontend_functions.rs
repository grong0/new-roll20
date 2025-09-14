pub fn ability_score_to_modifier(ability_score: u64) -> u64 {
	return ((ability_score as f64 - 10_f64) / 2_f64).round() as u64;
}

pub fn get_class_color(class: &String) -> String {
	match class.to_ascii_lowercase().as_str() {
		"artificer" => String::from("sky"),
		"barbarian" => String::from("red"),
		"bard" => String::from("amber"),
		"blood hunter" => String::from("rose"),
		"cleric" => String::from("yellow"),
		"druid" => String::from("lime"),
		"fighter" => String::from("green"),
		"monk" => String::from("orange"),
		"paladin" => String::from("violet"),
		"ranger" => String::from("emerald"),
		"sorcerer" => String::from("blue"),
		"warlock" => String::from("purple"),
		"wizard" => String::from("indigo"),
		_ => String::from("primary"),
	}
}

pub fn abbreviated_time_unit(casting_time: &String) -> String {
	// TODO: double check that this covers all units
	match casting_time.as_str() {
		"action" => String::from("A"),
		"bonus_action" => String::from("B"),
		"reaction" => String::from("R"),
		"hour" => String::from("h"),
		"minute" => String::from("m"),
		// TODO: check if this even exists
		"seconds" => String::from("s"),
		_ => String::from("?"),
	}
}
