pub fn ability_score_to_modifier(ability_score: u64) -> u64 {
    return ((ability_score as f64 - 10_f64) / 2_f64).round() as u64;
}

pub fn get_class_color(class: String) -> String {
    match class.to_ascii_lowercase().as_str() {
        "artificer" => "sky".to_string(),
        "barbarian" => "red".to_string(),
        "bard" => "amber".to_string(),
        "blood hunter" => "rose".to_string(),
        "cleric" => "yellow".to_string(),
        "druid" => "lime".to_string(),
        "fighter" => "green".to_string(),
        "monk" => "orange".to_string(),
        "paladin" => "violet".to_string(),
        "ranger" => "emerald".to_string(),
        "sorcerer" => "blue".to_string(),
        "warlock" => "purple".to_string(),
        "wizard" => "indigo".to_string(),
        _ => "primary".to_string(),
    }
}
