use serde_json::{to_value, Map, Value};

pub fn serde_as_string(value: Option<&Value>, default: String) -> String {
	return value.unwrap_or(&to_value(&default).unwrap()).as_str().unwrap_or(&default.to_string()).to_string();
}

pub fn serde_as_u64(value: Option<&Value>, default: u64) -> u64 {
	return value.unwrap_or(&to_value(default).unwrap()).as_u64().unwrap_or(default);
}

pub fn serde_as_i64(value: Option<&Value>, default: i64) -> i64 {
	return value.unwrap_or(&to_value(default).unwrap()).as_i64().unwrap_or(default);
}

pub fn serde_as_bool(value: Option<&Value>, default: bool) -> bool {
	return value.unwrap_or(&to_value(default).unwrap()).as_bool().unwrap_or(default);
}

pub fn serde_as_object(value: &Value, default: Map<String, Value>) -> Map<String, Value> {
	return value.as_object().unwrap_or(&default.clone()).to_owned();
}

pub fn serde_as_object_from_option(value: Option<&Value>, default: Map<String, Value>) -> Map<String, Value> {
	return value.unwrap_or(&to_value::<Map<String, Value>>(default.clone()).unwrap()).as_object().unwrap_or(&default.clone()).to_owned();
}

pub fn serde_as_array(value: Option<&Value>) -> Vec<Value> {
	return value.unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).to_owned();
}

pub fn serde_as_array_mapping<T: Clone>(value: Option<&Value>, mapping_func: fn(Option<&Value>, T) -> T, default: T) -> Vec<T> {
	return value
		.unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap())
		.as_array()
		.unwrap_or(&vec![])
		.iter()
		.map(|i| mapping_func(Some(i), default.clone()))
		.collect();
}
