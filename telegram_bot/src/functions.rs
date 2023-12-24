// use rand::Rng;
use std::collections::HashSet;

use teloxide::Bot;
use teloxide::types::{
	Message,
	ParseMode,
};
use teloxide::prelude::Requester;



use crate::types::{
	HandlerResult,
	UserDialogue,
};
use crate::command::{
	LoginCommand,
	StudentCommand,
	HelpCommand,
	HELP
};

use crate::state::State;
use crate::structs::ClasterId;
use crate::init::CLASTER_SPACE;


pub async fn send_formating(bot: Bot, msg: Message, text: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	let mut send_message = bot.send_message(msg.chat.id, text);
	send_message.parse_mode = Some(ParseMode::MarkdownV2);
	send_message.await?;
	Ok(())
}

pub async fn not_access(bot: Bot, dialogue: UserDialogue, msg: Message) -> HandlerResult {
	bot.send_message(msg.chat.id, "Not access").await?;
	dialogue.update(State::Start).await?;
	Ok(())
}

pub async fn login_user(bot: Bot, dialogue: UserDialogue, msg: Message, cmd: LoginCommand) -> HandlerResult {
	match cmd {
		LoginCommand::Login(data) => {
			match data.as_str() {
				"" | "student" | "Student" | "stud" | "Stud" => {
					bot.send_message(msg.chat.id, "Student").await?;
					dialogue.update(State::StudentState { claster_id: HashSet::new(), answer: "".into() }).await?;
					Ok(())
				},
				_ => Ok(()),
			}
		},
	}
}

pub async fn command_list(bot: Bot, dialogue: UserDialogue, msg: Message) -> HandlerResult {
	send_formating(bot, msg, format!("{}", HELP)).await?;
	Ok(())
}

pub async fn access(bot: Bot, dialogue: UserDialogue, msg: Message) -> HandlerResult {
	bot.send_message(msg.chat.id, "Your profile gets access").await?;
	
	dialogue.update(State::Access).await?;
	Ok(())
}

pub async fn student_command
(
	bot: Bot, 
	dialogue: UserDialogue, 
	msg: Message, 
	(mut claster_id, mut answer): (HashSet<ClasterId>, String), 
	command: StudentCommand 
) -> HandlerResult {
	match command {
		StudentCommand::Get => {
			
			let mut result = String::new();
			let data = claster_id.iter().collect::<Vec<&ClasterId>>();
			for link in &data {
				if let Some(data) = CLASTER_SPACE.get().await.get_string(link) {
					result += &format!("`{}`\n", data);
				}
			}
			send_formating(bot, msg, format!("{}", result)).await?;
			Ok(())
			
		},
		StudentCommand::Add(name_claster) => {
			let opt_id = CLASTER_SPACE.get().await.get_id(&name_claster);
			match opt_id {
				Some(id) => {
					claster_id.insert(id.clone());
					dialogue.update(
						State::StudentState {
							claster_id: claster_id, 
							answer: answer
						}
					).await?;
					bot.send_message(msg.chat.id, format!("Add \'{}\'", name_claster)).await?;
				},
				None => {
					bot.send_message(msg.chat.id, format!("\'{}\' not found", name_claster)).await?;
				},
			}
			Ok(())
		},
		StudentCommand::Remove(name_claster) => {
			let opt_id = CLASTER_SPACE.get().await.get_id(&name_claster);
			match opt_id {
				Some(id) => {
					claster_id.remove(&id);
					dialogue.update(
						State::StudentState {
							claster_id: claster_id, 
							answer: answer
						}
					).await?;
					bot.send_message(msg.chat.id, format!("Remove \'{}\'", name_claster)).await?;
				},
				None => {
					send_formating(bot, msg, format!("\'`{}`\' not found", name_claster)).await?;
				},
			}
			Ok(())
		},
		StudentCommand::List(data) => {
			let list = CLASTER_SPACE.get().await.title_list();
			let mut string: String = "Tab for copy:\n".into();
			if data.len() != 0 {
				for element in &list {
					string += format!("`{} {}`\n", data, element).as_str();
				}
			}
			else {
				for element in &list {
					string += format!("`{}`\n", element).as_str();
				}
			}
			send_formating(bot, msg, string).await?;
			Ok(())
		},
		StudentCommand::Info(name_claster) => {
			let opt_info = CLASTER_SPACE.get().await.info(&name_claster);
			if let Some(info) = opt_info {
				if info.len() != 0 {
					bot.send_message(msg.chat.id, format!("{}", info)).await?;
					return Ok(());
				}
			}
			bot.send_message(msg.chat.id, "Information not found").await?;
			Ok(())
		},
		StudentCommand::Flash | StudentCommand::Next => {
			let opt_flashcard = CLASTER_SPACE.get().await.random_flashcard(&claster_id);
			if let Some(flashcard) = opt_flashcard {
				answer = flashcard.answer();
				dialogue.update(
					State::StudentState {
						claster_id: claster_id, 
						answer: answer,
					}
				).await?;
				bot.send_message(msg.chat.id, format!("{}\n\n/answer\n\n/next", flashcard.question())).await?;
				Ok(())
			}
			else {
				bot.send_message(msg.chat.id, "Error: not found clasters").await?;
				Ok(())
			}
		},
		StudentCommand::Answer => {
			bot.send_message(msg.chat.id, format!("{}", answer)).await?;
			Ok(())
		},
	}
}
