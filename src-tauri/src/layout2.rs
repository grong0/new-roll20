use crate::{components::title_card_component, frontend_functions::ability_score_to_modifier};

#[tauri::command]
pub fn player_name() -> String {
    return "George".to_string();
}

#[tauri::command]
pub fn player_race() -> String {
    return "Human".to_string();
}

#[tauri::command]
pub fn player_level() -> String {
    return "13".to_string();
}

#[tauri::command]
pub fn player_hitpoints_max() -> String {
    return "304".to_string();
}

#[tauri::command]
pub fn player_hitpoints_current() -> String {
    return "250".to_string();
}

#[tauri::command]
pub fn player_hitpoints_percent() -> String {
	return format!("{}%", (250.0 as f32 / 304.0 as f32).round() * 100 as f32);
}

#[tauri::command]
pub fn player_hitpoints_temp() -> String {
    return "12".to_string();
}

#[tauri::command]
pub fn player_armorclass() -> String {
    return "20".to_string();
}

#[tauri::command]
pub fn player_initiative() -> String {
    return "4".to_string();
}

#[tauri::command]
pub fn player_speed_walking() -> String {
    return "30".to_string();
}

#[tauri::command]
pub fn player_class_level(class_name: &str) -> String {
    return "13".to_string();
}

// #[tauri::command]
// pub fn player_classes() -> String {
//     // classes = [
//     //     "artificer",
//     //     "barbarian",
//     //     "bard",
//     //     "cleric",
//     //     "druid",
//     //     "fighter",
//     //     "monk",
//     //     "paladin",
//     //     "ranger",
//     //     "rogue",
//     //     "sorcerer",
//     //     "warlock",
//     //     "wizard",
//     //]

//     // content = ""
//     // for player_class in classes:
//     //     content += compile_component(
//     //         "class-badge",
//     //         ("{color}", get_class_color(player_class)),
//     //         ("{class}", player_class.capitalize()),
//     //         ("{class_lower}", player_class),
//     //     )
//     // return content
// 	return "".to_string();
// }

// TODO: need to create a function for every instance of this in `index.html` or add a input on the html side to input the specific ability/skill it is
#[tauri::command]
pub fn player_attribute_score(ability: &str) -> String {
    return 20.to_string();
}

// TODO: need to create a function for every instance of this in `index.html` or add a input on the html side to input the specific ability/skill it is
#[tauri::command]
pub fn player_attribute_modifier(ability: &str) -> String {
    return ability_score_to_modifier(20).to_string();
}

// TODO: need to create a function for every instance of this in `index.html` or add a input on the html side to input the specific ability/skill it is
// #[tauri::command]
// pub fn player_skill_status(skill: &str) -> String {
//     return compile_component("skill-status-proficient");
// }

// TODO: need to create a function for every instance of this in `index.html` or add a input on the html side to input the specific ability/skill it is
#[tauri::command]
pub fn player_skill_bonus(skill: &str) -> String {
    return "5".to_string();
}

#[tauri::command]
pub fn player_senses_perception() -> String {
    return "18".to_string();
}

#[tauri::command]
pub fn player_senses_investigation() -> String {
    return "18".to_string();
}

#[tauri::command]
pub fn player_senses_insight() -> String {
    return "18".to_string();
}

#[tauri::command]
pub fn player_proficiencies_armor() -> String {
    return "None".to_string();
}

#[tauri::command]
pub fn player_proficiencies_weapons() -> String {
    return "Crossbow, Light, Dagger, Dart, Quarterstaff, Sling".to_string();
}

#[tauri::command]
pub fn player_proficiencies_tools() -> String {
    return "Tinkerers' Tools, Artisan Tools".to_string();
}

#[tauri::command]
pub fn player_proficiencies_languages() -> String {
    return "Abyssal, Celestial, Draconic".to_string();
}
