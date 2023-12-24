use serde::{
	Deserialize,
	Serialize,
};

use std::borrow::Cow;

#[derive(Hash, Clone, Debug, Deserialize, Serialize)]
pub struct Flashcard {
	question: String,
	answer: String,
}

impl Flashcard {
	pub fn new(question: String, answer: String) -> Flashcard {
		Flashcard {
			question,
			answer,
		}
	}
	pub fn question(&self) -> String {
		self.question.clone()
	}
	pub fn answer(&self) -> String {
		self.answer.clone()
	}
}
