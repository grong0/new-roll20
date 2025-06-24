pub fn ability_score_to_modifier(ability_score: u64) -> u64 {
	return ((ability_score as f64 - 10_f64) / 2_f64).round() as u64;
}
