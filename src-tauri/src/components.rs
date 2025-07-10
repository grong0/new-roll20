use std::fs::read_to_string;

pub fn skill_status_not_proficient() -> String {
	let file = read_to_string("../components/skill-status-not-proficient.html").unwrap_or("".to_string());
	return file;
}

pub fn skill_status_proficient() -> String {
	let file = read_to_string("../components/skill-status-proficient.html").unwrap_or("".to_string());
	return file;
}

pub fn skill_status_expert() -> String {
	let file = read_to_string("../components/skill-status-expert.html").unwrap_or("".to_string());
	return file;
}

pub fn workspace_actions_attack(
	name: &String,
	action_type: &String,
	range: &String,
	hitdc: &String,
	damage: &String,
	damage_type: &String,
	notes: &String,
) -> String {
	let file = read_to_string("../components/workspace-actions-attack.html")
		.unwrap_or("".to_string())
		.replace("{name}", &name.as_str())
		.replace("{type}", &action_type.as_str())
		.replace("{range}", &range.as_str())
		.replace("{hitdc}", &hitdc.as_str())
		.replace("{damage}", &damage.as_str())
		.replace("{damage_type}", &damage_type.as_str())
		.replace("{notes}", &notes.as_str());
	return file;
}

pub fn workspace_actions(
	actions: &String,
	bonus_actions: &String,
	reactions: &String,
	other_actions: &String,
	limited_use_actions: &String,
	actions_in_combat: &String,
	bonus_actions_in_combat: &String,
	reactions_in_combat: &String,
	other_actions_in_combat: &String,
	limited_use_actions_in_combat: &String,
) -> String {
	let file = read_to_string("../components/workspace-actions.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{bonus_actions}", bonus_actions)
		.replace("{reactions}", reactions)
		.replace("{other_actions}", other_actions)
		.replace("{limited_use_actions}", limited_use_actions)
		.replace("{actions_in_combat}", actions_in_combat)
		.replace("{bonus_actions_in_combat}", bonus_actions_in_combat)
		.replace("{reactions_in_combat}", reactions_in_combat)
		.replace("{other_actions_in_combat}", other_actions_in_combat)
		.replace("{limited_use_actions_in_combat}", limited_use_actions_in_combat);
	return file;
}

pub fn workspace_actions_action(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("../components/workspace-actions-action.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn workspace_actions_bonusaction(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("../components/workspace-actions-bonusaction.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn workspace_actions_reaction(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("../components/workspace-actions-reaction.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn workspace_actions_other(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("../components/workspace-actions-other.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn workspace_actions_limiteduse(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("../components/workspace-actions-limiteduse.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn class_badge(class_higher: &String, class_lower: &String) -> String {
	let file = read_to_string("../components/class-badge.html")
		.unwrap_or("".to_string())
		.replace("{class}", class_higher)
		.replace("{class_lower}", class_lower);
	return file;
}
