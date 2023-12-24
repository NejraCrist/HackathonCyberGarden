use conception::Flashcard;

use std::collections::HashSet;

use crate::structs::ClasterId;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
	#[default]
	Start,
	NotAccess,
	Access,
	StudentState {
		claster_id: HashSet<ClasterId>,
		answer: String,
	},
} 
