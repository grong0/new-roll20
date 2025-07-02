use crate::{
    components::{class_badge, skill_status_expert, skill_status_not_proficient, skill_status_proficient, workspace_actions, workspace_actions_action, workspace_actions_attack, workspace_actions_bonusaction, workspace_actions_limiteduse, workspace_actions_other, workspace_actions_reaction},
    frontend_functions::{ability_score_to_modifier, get_class_color},
};

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
pub fn player_class_level() -> String {
    return "13".to_string();
}

#[tauri::command]
pub fn player_classes() -> String {
    let classes = vec!["artificer", "barbarian", "bard", "cleric", "druid", "fighter", "monk", "paladin", "ranger", "rogue", "sorcerer", "warlock", "wizard"];

    let mut content = String::new();
    for player_class in classes {
        content += class_badge(player_class.to_ascii_uppercase().to_string(), player_class.to_string(), get_class_color(player_class.to_string())).as_str();
    }
    return content;
}

#[tauri::command]
pub fn player_abilities_str_score() -> String {
    return 9.to_string();
}
#[tauri::command]
pub fn player_abilities_dex_score() -> String {
    return 16.to_string();
}
#[tauri::command]
pub fn player_abilities_con_score() -> String {
    return 11.to_string();
}
#[tauri::command]
pub fn player_abilities_int_score() -> String {
    return 20.to_string();
}
#[tauri::command]
pub fn player_abilities_wis_score() -> String {
    return 10.to_string();
}
#[tauri::command]
pub fn player_abilities_cha_score() -> String {
    return 8.to_string();
}

#[tauri::command]
pub fn player_abilities_str_modifier() -> String {
    return ability_score_to_modifier(9).to_string();
}
#[tauri::command]
pub fn player_abilities_dex_modifier() -> String {
    return ability_score_to_modifier(16).to_string();
}
#[tauri::command]
pub fn player_abilities_con_modifier() -> String {
    return ability_score_to_modifier(11).to_string();
}
#[tauri::command]
pub fn player_abilities_int_modifier() -> String {
    return ability_score_to_modifier(20).to_string();
}
#[tauri::command]
pub fn player_abilities_wis_modifier() -> String {
    return ability_score_to_modifier(10).to_string();
}
#[tauri::command]
pub fn player_abilities_cha_modifier() -> String {
    return ability_score_to_modifier(8).to_string();
}

#[tauri::command]
pub fn player_skills_acrobatics_status() -> String {
    return skill_status_proficient();
}
#[tauri::command]
pub fn player_skills_animalhandling_status() -> String {
    return skill_status_proficient();
}
#[tauri::command]
pub fn player_skills_arcana_status() -> String {
    return skill_status_proficient();
}
#[tauri::command]
pub fn player_skills_athletics_status() -> String {
    return skill_status_expert();
}
#[tauri::command]
pub fn player_skills_deception_status() -> String {
    return skill_status_proficient();
}
#[tauri::command]
pub fn player_skills_history_status() -> String {
    return skill_status_expert();
}
#[tauri::command]
pub fn player_skills_insight_status() -> String {
    return skill_status_proficient();
}
#[tauri::command]
pub fn player_skills_intimidation_status() -> String {
    return skill_status_proficient();
}
#[tauri::command]
pub fn player_skills_investigation_status() -> String {
    return skill_status_proficient();
}
#[tauri::command]
pub fn player_skills_medicine_status() -> String {
    return skill_status_proficient();
}
#[tauri::command]
pub fn player_skills_nature_status() -> String {
    return skill_status_expert();
}
#[tauri::command]
pub fn player_skills_perception_status() -> String {
    return skill_status_not_proficient();
}
#[tauri::command]
pub fn player_skills_performance_status() -> String {
    return skill_status_not_proficient();
}
#[tauri::command]
pub fn player_skills_persuasion_status() -> String {
    return skill_status_proficient();
}
#[tauri::command]
pub fn player_skills_religion_status() -> String {
    return skill_status_not_proficient();
}
#[tauri::command]
pub fn player_skills_slightofhand_status() -> String {
    return skill_status_not_proficient();
}
#[tauri::command]
pub fn player_skills_stealth_status() -> String {
    return skill_status_not_proficient();
}
#[tauri::command]
pub fn player_skills_survival_status() -> String {
    return skill_status_proficient();
}

#[tauri::command]
pub fn player_skills_acrobatics_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_animalhandling_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_arcana_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_athletics_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_deception_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_history_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_insight_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_intimidation_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_investigation_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_medicine_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_nature_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_perception_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_performance_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_persuasion_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_religion_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_slightofhand_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_stealth_bonus() -> String {
    return 5.to_string();
}
#[tauri::command]
pub fn player_skills_survival_bonus() -> String {
    return 5.to_string();
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

struct Damage {
    dice: String,
    damage_type: String,
}

struct Action {
    name: String,
    action_type: String,
    range: String,
    hitdc: String,
    damage: Damage,
    notes: String,
}

#[tauri::command]
pub fn sheet_workspace_actions(workspace_actions_filter: String) -> String {
    println!("{}", &workspace_actions_filter);
    return match workspace_actions_filter.as_str() {
        "actions-all" => workspace_actions(),
        "actions-actions" => workspace_actions_action(),
        "actions-bonus" => workspace_actions_bonusaction(),
        "actions-reactions" => workspace_actions_reaction(),
        "actions-other" => workspace_actions_other(),
        "actions-limited" => workspace_actions_limiteduse(),
        _ => workspace_actions(),
    };
}

#[tauri::command]
pub fn player_actions_action() -> String {
    let actions = vec![
        Action {
            name: "Ray of Frost".to_string(),
            action_type: "Cantrip - Wizard".to_string(),
            range: 60.to_string(),
            hitdc: 11.to_string(),
            damage: Damage {
                dice: "4d8".to_string(),
                damage_type: "cold".to_string(),
            },
            notes: "V/S".to_string(),
        },
        Action {
            name: "Unarmed Strike".to_string(),
            action_type: "Melee Attack".to_string(),
            range: 5.to_string(),
            hitdc: 5.to_string(),
            damage: Damage {
                dice: "0".to_string(),
                damage_type: "bludgeoning".to_string(),
            },
            notes: "".to_string(),
        },
        Action {
            name: "Melf's Minute Meteors".to_string(),
            action_type: "3rd Level - Wizard".to_string(),
            range: "Self".to_string(),
            hitdc: "DEX 14".to_string(),
            damage: Damage {
                dice: "2d6".to_string(),
                damage_type: "fire".to_string(),
            },
            notes: "Concentration, V/S/M".to_string(),
        },
    ];

    let mut content = String::new();
    for action in actions {
        content += workspace_actions_attack(action.name, action.action_type, action.range, action.hitdc, action.damage.dice, action.damage.damage_type, action.notes).as_str();
    }
    return content;
}

#[tauri::command]
pub fn player_actions_bonusaction() -> String {
    let actions = vec![Action {
        name: "Melf's Minute Meteors".to_string(),
        action_type: "3rd Level - Wizard".to_string(),
        range: "Self".to_string(),
        hitdc: "DEX 14".to_string(),
        damage: Damage {
            dice: "2d6".to_string(),
            damage_type: "fire".to_string(),
        },
        notes: "Concentration, V/S/M".to_string(),
    }];

    let mut content = String::new();
    for action in actions {
        content += workspace_actions_attack(action.name, action.action_type, action.range, action.hitdc, action.damage.dice, action.damage.damage_type, action.notes).as_str();
    }
    return content;
}

#[tauri::command]
pub fn player_actions_reaction() -> String {
    let actions = vec![Action {
        name: "".to_string(),
        action_type: "".to_string(),
        range: "".to_string(),
        hitdc: "".to_string(),
        damage: Damage { dice: "".to_string(), damage_type: "".to_string() },
        notes: "".to_string(),
    }];

    let mut content = String::new();
    for action in actions {
        content += workspace_actions_attack(action.name, action.action_type, action.range, action.hitdc, action.damage.dice, action.damage.damage_type, action.notes).as_str();
    }
    return content;
}

#[tauri::command]
pub fn player_actions_other() -> String {
    let actions = vec![Action {
        name: "".to_string(),
        action_type: "".to_string(),
        range: "".to_string(),
        hitdc: "".to_string(),
        damage: Damage { dice: "".to_string(), damage_type: "".to_string() },
        notes: "".to_string(),
    }];

    let mut content = String::new();
    for action in actions {
        content += workspace_actions_attack(action.name, action.action_type, action.range, action.hitdc, action.damage.dice, action.damage.damage_type, action.notes).as_str();
    }
    return content;
}

#[tauri::command]
pub fn player_actions_limiteduse() -> String {
    let actions = vec![Action {
        name: "".to_string(),
        action_type: "".to_string(),
        range: "".to_string(),
        hitdc: "".to_string(),
        damage: Damage { dice: "".to_string(), damage_type: "".to_string() },
        notes: "".to_string(),
    }];

    let mut content = String::new();
    for action in actions {
        content += workspace_actions_attack(action.name, action.action_type, action.range, action.hitdc, action.damage.dice, action.damage.damage_type, action.notes).as_str();
    }
    return content;
}

#[tauri::command]
pub fn player_actionsincombat_action() -> String {
    return "Attack, Cast a Spell, Dash, Disengage, Dodge, Grapple, Help, Hide, Improvise, Ready, Search, Shove, Use an Object".to_string();
}

#[tauri::command]
pub fn player_actionsincombat_bonusaction() -> String {
    return "Nothing".to_string();
}

#[tauri::command]
pub fn player_actionsincombat_reaction() -> String {
    return "Nothing".to_string();
}

#[tauri::command]
pub fn player_actionsincombat_other() -> String {
    return "Nothing".to_string();
}

#[tauri::command]
pub fn player_actionsincombat_limiteduse() -> String {
    return "Nothing".to_string();
}
