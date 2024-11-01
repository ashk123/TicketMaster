use std::sync::Arc;

use serenity::{
    all::{Cache, ChannelId, CommandInteraction, Context, CreateCommand},
    futures::TryFutureExt,
    model::application::ResolvedOption,
};

use crate::TempChannels;

pub async fn run(
    options: &[ResolvedOption<'_>],
    ctx: &Context,
    cmd: &CommandInteraction,
) -> String {
    // Get the channel and check if it is the configed channel and if true return it
    if !TempChannels.lock().unwrap().IsAvailable(cmd.channel_id) {
        return String::from("Please use this command from created ticket channel");
    }

    let channel = ctx.http.get_channel(cmd.channel_id).await.unwrap();
    let channel_name_sample = channel.id().name(&ctx).await.unwrap();

    // Remove the Channel
    channel.delete(&ctx).await.unwrap();

    // Print the succsessfull message in the configed channel
    //ctx.http.delete_channel(channel_id, audit_log_reason)
    let main_channel_obj: ChannelId = TempChannels.lock().unwrap().get_main_channel();
    println!("{:?}", main_channel_obj);
    main_channel_obj
        .say(
            &ctx,
            format!("{} deleted succsessfully", channel_name_sample),
        )
        .await
        .unwrap();

    String::from("") // This is not important cause channeld will get deleted
}

pub fn register() -> CreateCommand {
    CreateCommand::new("close").description("Close the current ticket")
}
