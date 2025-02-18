use std::{fs, io::Error};

use serde_json::json;

const REPO_URL: &str = "https://github.com/5etools-mirror-3/5etools-2014-src";

// fn compare_versions(ver1: String, ver2: String) -> i8 {
// 	let split_ver1 = ver1.split(".").map(|num|);
// 	let split_ver2 = ver2.split(".");

// 	for i in 0..3 {

// 	}
// }

// fn current_version() -> String {
// 	let file = fs::read("data/changelog.json");

// 	if file.is_err() {
// 		return "0.0.0".to_string();
// 	}

// 	let json = json!(file.unwrap());
// 	let array = json.as_array().unwrap();

// 	let ver = json!(array.get(array.len()-1)).get("ver").unwrap();
// 	return ver.as_str().unwrap().to_string();
// }

// fn is_newer_version() -> bool {
// 	// get local version
// 	let local_version = current_version();

// 	// get remote version
// 	let downloader = git_download::repo(REPO_URL).branch_name("main").add_file("data", ".temp/").exec();
// 	if downloader.is_err() {
// 		return false;
// 	}
// }

pub fn update_data() -> i32 {
	git_download::repo(REPO_URL).branch_name("main").add_file("data", "./data").exec();

	return 0;
}
