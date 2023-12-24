use serde::{
	Deserialize,
	Serialize,
};

use rand::Rng;

use std::borrow::Cow;

use crate::{
	Flashcard,
	ClasterInfo,
};

#[derive(Hash, Clone, Debug, Deserialize, Serialize)]
pub struct Claster {
	info: ClasterInfo,
	cards: Vec<Flashcard>,
}

impl Claster {
	pub fn new(info: ClasterInfo, cards: Vec<Flashcard>) -> Claster {
		Claster {
			info,
			cards,
		}
	}
	pub fn title(&self) -> String {
		self.info.title()
	}
	pub fn info(&self) -> String {
		self.info.detail_info()
	}
	pub fn flashcards(&self) -> Option<Vec<Flashcard>> {
		if self.cards.len() > 0 {
			Some(self.cards.clone())
		}
		else {
			None
		}
	}
	pub fn random_number() -> usize {
		let mut rng = rand::thread_rng();
		let number: usize = rng.gen();
		number
	}
	pub fn random_flashcard(&self) -> Flashcard {
		self.cards[Self::random_number() % self.cards.len()].clone()
	}
}
