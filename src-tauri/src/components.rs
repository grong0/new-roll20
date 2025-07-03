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
	name: String,
	action_type: String,
	range: String,
	hitdc: String,
	damage: String,
	damage_type: String,
	notes: String,
) -> String {
	let file = read_to_string("../components/actions-workspace-attack.html")
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

pub fn workspace_actions() -> String {
	let file = read_to_string("../components/workspace-actions.html").unwrap_or("".to_string());
	return file;
}

pub fn workspace_actions_action() -> String {
	let file = read_to_string("../components/workspace-actions-action.html").unwrap_or("".to_string());
	return file;
}

pub fn workspace_actions_bonusaction() -> String {
	let file = read_to_string("../components/workspace-actions-bonusaction.html").unwrap_or("".to_string());
	return file;
}

pub fn workspace_actions_reaction() -> String {
	let file = read_to_string("../components/workspace-actions-reaction.html").unwrap_or("".to_string());
	return file;
}

pub fn workspace_actions_other() -> String {
	let file = read_to_string("../components/workspace-actions-other.html").unwrap_or("".to_string());
	return file;
}

pub fn workspace_actions_limiteduse() -> String {
	let file = read_to_string("../components/workspace-actions-limiteduse.html").unwrap_or("".to_string());
	return file;
}

pub fn class_badge(class_higher: String, class_lower: String, class_color: String) -> String {
	let file = read_to_string("../components/class-badge.html")
		.unwrap_or("".to_string())
		.replace("{class}", &class_higher)
		.replace("{class_lower}", &class_lower)
		.replace("{color}", &class_color);
	return file;
}
