use teloxide::{
	dispatching::dialogue::{
		serializer::{Json},
		ErasedStorage, SqliteStorage, Storage,
	},
	prelude::*,
};

use telegram_bot::{
	types::SqlStorage,
	state::State,
	functions,
	command::{
		LoginCommand,
		StudentCommand,
		HelpCommand,
		HELP
	},
	init::{
		CONFIG,
		CLASTER_SPACE,
	},
};

#[tokio::main]
async fn start_bot() -> Result<(), Box<dyn std::error::Error>> {
	lazy_static::initialize(&CLASTER_SPACE);
	
	
	let bot = if let Some(token) = &CONFIG.tg_token {
		Bot::new(token)
	} 
	else {
		Bot::from_env()
	};

	let storage: SqlStorage = SqliteStorage::open(&CONFIG.db_name, Json).await?.erase(); //db_name

	let handler = Update::filter_message()
		.enter_dialogue::<Message, ErasedStorage<State>, State>()
		.branch(dptree::case![State::Start].endpoint(functions::access))
		.branch(dptree::case![State::Access].filter_command::<LoginCommand>().endpoint(functions::login_user))
		.branch(dptree::case![State::NotAccess].endpoint(functions::not_access))
		.branch(dptree::entry().filter_command::<HelpCommand>().endpoint(functions::command_list))
		.branch(
			dptree::case![State::StudentState { claster_id, answer}]
				.branch(dptree::entry().filter_command::<StudentCommand>().endpoint(functions::student_command))
		);
	
	Dispatcher::builder(bot, handler)
		.dependencies(dptree::deps![storage])
		.enable_ctrlc_handler()
		.build()
		.dispatch()
		.await;
	Ok(())
}

fn main() {
	lazy_static::initialize(&CONFIG);
	match start_bot() {
		Ok(_) => {},
		Err(error) => {
			eprintln!("{:#?}", error);
		},
	}
}
