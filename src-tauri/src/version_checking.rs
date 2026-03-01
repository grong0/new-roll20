use std::{
	fs::{copy, create_dir, create_dir_all, read_dir, remove_dir_all, File},
	io::Read,
	iter::zip,
	path::Path,
};

use serde_json::{from_str, Map, Value};
use zip::{ZipArchive, read::ZipFile};

use crate::serde_utils::{serde_as_object_from_option, serde_as_string};

const REPO_URL_ZIP: &str = "https://github.com/5etools-mirror-3/5etools-2014-src/archive/refs/heads/main.zip";

const TEMP_DIRECTORY: &str = "../data/.temp";
const ZIP_NAME: &str = "5etools-2014-src-main";

const DATA_DIRECTORY: &str = "../data/raw";

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
	create_dir_all(&dst)?;
	for entry in read_dir(src)? {
		let entry = entry?;
		let ty = entry.file_type()?;
		if ty.is_dir() {
			copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
		} else {
			copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
		}
	}
	Ok(())
}

fn create_dir_with_debug(path: &str) -> Result<(), String> {
	let create_result = create_dir(path);
	if create_result.is_err() {
		return Err(format!("Failed to create directory at {}: {}", path, create_result.err().unwrap()));
	}
	println!("Created new data directory at {}", path);
	Ok(())
}

fn remove_dir_with_debug_without_error(path: &str) -> String {
	let remove_result = remove_dir_all(path);
	if remove_result.is_err() {
		return format!("Failed to remove dir at {}: {}", path, remove_result.err().unwrap());
	}
	return format!("Removed dir at {}", path);
}

/// returns 1 if version 1 is newer, 2 if version 2 is newer, and 0 if they're the same
fn compare_versions(ver1: String, ver2: String) -> u8 {
	let split_ver1 = ver1.split(".").map(|num| num.parse::<u8>().unwrap());
	let split_ver2 = ver2.split(".").map(|num| num.parse::<u8>().unwrap());

	for (sub_ver_1, sub_ver_2) in zip(split_ver1, split_ver2) {
		if sub_ver_1 > sub_ver_2 {
			return 1;
		} else if sub_ver_2 > sub_ver_1 {
			return 2;
		}
		continue;
	}

	return 0;
}

fn get_version_from_file<R: Read>(file: &mut R) -> String {
	let mut buf = String::new();
	let result = file.read_to_string(&mut buf);
	if result.is_err() {
		println!("file failed to fill buffer: {}", result.err().unwrap());
	}

	let json: Value = from_str(&buf).unwrap();
	let array = json.as_array().unwrap();

	return serde_as_string(serde_as_object_from_option(array.get(array.len() - 1), Map::new()).get("ver"), "0.0.0".to_string());
}

fn download_zip_into_temp() -> Result<ZipArchive<File>, String> {
	// remove any existing temp folder and create a new one
	remove_dir_with_debug_without_error(TEMP_DIRECTORY);
	create_dir_with_debug(TEMP_DIRECTORY)?;

	// create download builder
	let downloader = downloader::Downloader::builder().download_folder(Path::new(TEMP_DIRECTORY)).build();
	if downloader.is_err() {
		return Err(format!("Failed to build the downloader: {}", downloader.err().unwrap()));
	}
	println!("Created downloader");

	// download the repo
	let download = downloader::Download::new(REPO_URL_ZIP).file_name(Path::new(&format!("{}.zip", ZIP_NAME)));
	println!("Downloading...");
	let download_result = downloader.unwrap().download(&[download]);
	println!("Finished downloading");
	if download_result.is_err() {
		return Err(format!("Downloader failed to download: {}", download_result.err().unwrap()));
	}
	let download_list = download_result.unwrap();
	if download_list.get(0).is_none() {
		return Err(String::from("Download list doesn't have download result"));
	}
	if download_list[0].as_ref().is_err() {
		return Err(format!("Specific Download failed to download: {}", download_list[0].as_ref().err().unwrap()));
	}
	let download_final = download_list[0].as_ref().unwrap();
	if download_final.status[0].1 != 200 {
		return Err(format!("Download had a non success status: {}", download_final.status[0].1));
	}
	let file_path = format!("{}/{}.zip", TEMP_DIRECTORY, ZIP_NAME);
	let file = File::open(&file_path);
	if file.is_err() {
		return Err(format!("Failed to create zip file at {}: {}", &file_path, file.err().unwrap()));
	}
	println!("Created zip file struct");

	// read zip file
	let zip = ZipArchive::new(file.unwrap());
	if zip.is_err() {
		return Err(format!("Failed to create zip archive from file: {}", zip.err().unwrap()));
	}
	println!("Created zip archive from file");

	return Ok(zip.unwrap());
}

pub fn is_newer_version_from_zip(zip: &mut ZipArchive<File>) -> Result<bool, String> {
	// get local version
	let local_changelog = File::open(format!("{}/changelog.json", DATA_DIRECTORY));
	if local_changelog.is_err() {
		println!("Failed to open changelog in data directory: {}", local_changelog.as_ref().err().unwrap());
		return Ok(true);
	}
	let local_version = get_version_from_file(local_changelog.unwrap().by_ref());
	println!("local version: {}", local_version);

	// get remote version
	let zip_changelog = zip.by_name(format!("{}/data/changelog.json", ZIP_NAME).as_str());
	if zip_changelog.is_err() {
		return Err(format!("Failed to find changelog in zip archive: {}", zip_changelog.err().unwrap()));
	}
	let remote_version = get_version_from_file(zip_changelog.unwrap().by_ref());
	println!("remote version: {}", remote_version);

	return Ok(compare_versions(local_version, remote_version) == 2);
}

pub fn update_data_from_zip() -> Result<bool, String> {
	let mut zip = download_zip_into_temp()?;

	if !is_newer_version_from_zip(&mut zip)? {
		println!("Remote version was either outdated or the same version as the local version");
		remove_dir_with_debug_without_error(TEMP_DIRECTORY);
		return Ok(false);
	}

	// extract zip contents
	let extraction_result = zip.extract(TEMP_DIRECTORY);
	if extraction_result.is_err() {
		return Err(format!("Failed to extract the zip archive into the temp directory: {}", extraction_result.err().unwrap()));
	}
	println!("Extracted the zip archive data directory into the local data directory");
	let extracted_zip_data_path = format!("{}/{}/data/", TEMP_DIRECTORY, ZIP_NAME);
	let extracted_zip = File::open(&extracted_zip_data_path);
	if extracted_zip.is_err() {
		return Err(format!("Failed to create a file struct from the extracted zip: {}", extracted_zip.err().unwrap()));
	}
	println!("Created file struct from extracted zip");

	// empty current data directory
	remove_dir_with_debug_without_error(DATA_DIRECTORY);
	create_dir_with_debug(DATA_DIRECTORY)?;
	let local_data_directory = File::open(DATA_DIRECTORY);
	if local_data_directory.is_err() {
		return Err(format!("Failed to find the local data directory using path using: {}", DATA_DIRECTORY));
	}
	println!("Created new local data directory File struct");

	// copy contents from extracted data directory to local data directory
	let copy_result = copy_dir_all(&extracted_zip_data_path, DATA_DIRECTORY);
	if copy_result.is_err() {
		return Err(String::from("Failed to copy from data directory from zip to the local data directory"));
	}
	println!("Copied contents from the zip data directory to the local data directory");

	// remove temp directory
	remove_dir_with_debug_without_error(TEMP_DIRECTORY);

	return Ok(true);
}
