use serenity::all::PermissionOverwrite;
use serenity::builder::{CreateChannel, CreateCommand};
use serenity::client::Context;
use serenity::model::application::{
    CommandInteraction, CommandOptionType, ResolvedOption, ResolvedValue,
};
use serenity::model::channel::PermissionOverwriteType;
use serenity::model::permissions::Permissions;

pub async fn run(
    options: &[ResolvedOption<'_>],
    ctx: &Context,
    cmd: &CommandInteraction,
) -> String {
    // random number for ticket name
    let rnd: i32 = 300;
    /*
    let mut sample: HashMap<&str, Value> = HashMap::new();
    sample.insert("name", Value::String(String::from("SLAM")));
    let mut channel = CreateChannel(sample);
    */

    // get the user's guild

    let guild = ctx.http.get_guild(cmd.guild_id.unwrap()).await.unwrap();
    // get the guild's everyone default role
    let everyone = guild.role_by_name("@everyone").unwrap();
    // set the want permission for user
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::default(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(serenity::all::RoleId::new(u64::from(everyone.id))),
        },
        PermissionOverwrite {
            allow: Permissions::SEND_MESSAGES,
            deny: Permissions::MANAGE_CHANNELS,
            kind: PermissionOverwriteType::Member(serenity::all::UserId::new(u64::from(
                cmd.user.id,
            ))),
        },
    ];

    let channel = CreateChannel::new(format!("Ticket-{}", rnd))
        .kind(serenity::all::ChannelType::Text)
        .permissions(permissions);

    // Assuming a guild has already been bound.

    // guild
    let created_channel_sample: serenity::model::channel::GuildChannel = guild
        .create_channel(&ctx.http, channel.clone())
        .await
        .unwrap();

    return String::from("Created Channel with for you");
}

pub fn register() -> CreateCommand {
    CreateCommand::new("new").description("Create New Ticket")
    /*
    *
       .add_option(
           CreateCommandOption::new(CommandOptionType::User, "id", "The user to lookup")
               .required(true),
       )
    */
}
