use teloxide::utils::command::BotCommands;

#[derive(Clone, BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum StudentCommand {
	Get,
	Add(String),
	Remove(String),
	List(String),
	Info(String),
	Flash,
	Answer,
	Next,
}
