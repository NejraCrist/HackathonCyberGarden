use conception::Claster;

use std::hash::{
	Hasher,
	Hash,
};
use std::collections::hash_map::DefaultHasher;


#[derive(Debug, PartialEq, Eq, Hash, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClasterId(u64);

impl ClasterId {
	pub fn new(claster: &Claster) -> ClasterId {
		let mut hasher = DefaultHasher::new();
		claster.hash(&mut hasher);
		ClasterId(hasher.finish())
	}
}
