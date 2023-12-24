use teloxide::utils::command::BotCommands;

#[derive(Clone, BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum LoginCommand {
	Login(String),
}
