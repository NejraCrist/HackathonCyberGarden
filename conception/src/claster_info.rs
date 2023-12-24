use serde::{
	Deserialize,
	Serialize,
};

use std::borrow::Cow;

#[derive(Hash, Clone, Debug, Deserialize, Serialize)]
pub struct ClasterInfo {
	title: String,
	detail_info: String,
	source: Vec<String>,
}

impl ClasterInfo {
	pub fn new(title: String, detail_info: String, source: Vec<String>) -> ClasterInfo {
		ClasterInfo {
			title,
			detail_info,
			source,
		}
	}
	pub fn title(&self) -> String {
		self.title.clone()
	}
	pub fn detail_info(&self) -> String {
		self.detail_info.clone()
	}
}
