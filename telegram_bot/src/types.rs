use teloxide::prelude::Dialogue;
use teloxide::dispatching::dialogue::ErasedStorage;

use crate::state::State;

pub type UserDialogue = Dialogue<State, ErasedStorage<State>>;
pub type SqlStorage = std::sync::Arc<ErasedStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
