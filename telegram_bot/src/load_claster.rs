use tokio::fs::File;
use tokio::io::AsyncReadExt;

use regex::Regex;

use conception::Claster;

use std::path::Path;
use std::fs::{
	read_dir,
	DirEntry,
};

use crate::structs::ClasterSpace;

const REGEX: &str = ".json$";

async fn read_file(name_file: &str) -> Result<String, Box<dyn std::error::Error>> {
	let mut buf = String::new();
	let mut file = File::open(name_file).await?;
	file.read_to_string(&mut buf).await?;
	Ok(buf)
}

async fn check_condition(regex: &Regex, file: DirEntry) -> Result<Claster, Box<dyn std::error::Error>> {
	
	let path_file: String = file.path().as_path().to_string_lossy().into();
	if !regex.is_match(&path_file) {
		return Err(format!("\"{}\" is not .json file", path_file).into())
	}
	let data = read_file(&path_file).await?;
	Ok(serde_json::from_str::<Claster>(&data)?)
}

pub async fn load_claster(path: &Path) -> Result<ClasterSpace, Box<dyn std::error::Error>> {
	let mut result = ClasterSpace::new();
	let regex = Regex::new(REGEX)?;
	
	let dir_data = read_dir(path)?;
	for file in dir_data {
		if let Ok(dir_entry) = file {
			if let Ok(claster) = check_condition(&regex, dir_entry).await {
				result.insert(claster);
			}
		}
	}
	
	Ok(result)
}
