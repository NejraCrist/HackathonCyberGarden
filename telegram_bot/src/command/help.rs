use teloxide::utils::command::BotCommands;

pub const HELP: &'static str = include_str!("HELP.md");

#[derive(Clone, BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum HelpCommand {
	Command_List
}
