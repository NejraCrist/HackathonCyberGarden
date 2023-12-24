// use rand::prelude::SliceRandom;
use conception::{
	Claster,
	Flashcard,
};

use std::borrow::Cow;
use std::collections::{
	HashMap,
	HashSet,
};

use crate::structs::ClasterId;

#[derive(Debug)]
pub struct ClasterSpace {
	map: HashMap<ClasterId, Claster>,
	titles: HashMap<String, ClasterId>,
	hash_set: HashSet<ClasterId>,
}

impl ClasterSpace {
	pub fn new() -> ClasterSpace {
		ClasterSpace {
			map: HashMap::new(),
			titles: HashMap::new(),
			hash_set: HashSet::new(),
		}
	}
	pub fn insert(&mut self, data: Claster) {
		let id = ClasterId::new(&data);
		self.titles.insert(data.title(), id.clone());
		self.hash_set.insert(id.clone());
		self.map.insert(id, data);
	}
	pub fn get_id(&self, title: &String) -> Option<ClasterId> {
		self.titles.get(title).cloned()
	}
	pub fn get_claster(&self, id: &ClasterId) -> Option<&Claster> {
		self.map.get(id)
	}
	pub fn get_string(&self, id: &ClasterId) -> Option<String> {
		Some(self.get_claster(id)?.title())
	}
	pub fn title_list(&self) -> Vec<String> {
		self.map.clone().into_values().map(|data| {data.title()}).collect()
	}
	pub fn info(&self, id: &String) -> Option<String> {
		Some(self.get_claster(&self.get_id(id)?)?.info())
	}
	pub fn random_flashcard(&self, id: &HashSet<ClasterId>) -> Option<Flashcard> {
		let data = self.hash_set.intersection(id).map(|__q| __q).collect::<Vec<&ClasterId>>();
		if data.len() == 0 {
			return None;
		}
		let number = Claster::random_number();
		let flashcard = self.map.get(data[number % data.len()])?.random_flashcard();
		
		Some(flashcard)
	}
}
