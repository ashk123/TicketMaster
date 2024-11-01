use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

fn getInfo(guild_name: &str) -> String {
    format!(
        "
    TicketMaster Discord Bot - {}

    For using TicketMaster bot, follow these commands:
    
    `/info`: For reading this message
    `/new`: For making new ticket
    `/close`: For clsing a existing ticket
    `/config`: For confing the TicketMaster bot (discord's admins only)

    for more information please visit the <https://github.com/ashk123/TicketMaster>

    ",
        guild_name
    )
}

pub fn run(options: &[ResolvedOption]) -> String {
    getInfo("[Dynamic]")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("info").description("Get infomation about this bot")
    /*
    *
       .add_option(
           CreateCommandOption::new(CommandOptionType::User, "id", "The user to lookup")
               .required(true),
       )
    */
}
