/*
 * Copyright (c) Facebook, Inc. and its affiliates.
*
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{collections::HashMap, env};

use serenity::{
    all::{Channel, ChannelType, PermissionOverwrite, PrivateChannel},
    async_trait,
    builder::CreateChannel,
    model::{channel::Message, gateway::Ready, guild::Guild},
    prelude::*,
};

use serenity::model::channel::PermissionOverwriteType;
use serenity::model::id::UserId;
use serenity::model::permissions::Permissions;

const HELP_MESSAGE: &str = "
}ello there, Human!

TicketMaster is a ticket manager bot that can handle
All of your tickets, for more information, please
Join to our discord server: <LINK>

❓ Need technical help?
➡️ Post in the <#CHANNEL_ID> channel and other humans will assist you.

❓ Looking for the Code of Conduct?
➡️ Here it is: <https://opensource.facebook.com/code-of-conduct>

❓ Something wrong?
➡️ You can flag an admin with @admin

I hope that resolves your issue!

— TicketMasterBot 🤖
";

struct Cfg {
    main_guild_channel: u64,
    name: String,
    first_channel_msg: String,
}

impl Cfg {
    fn new(main_guild_channel: u64, name: String, first_channel_msg: String) -> Self {
        return Self {
            main_guild_channel,
            name,
            first_channel_msg,
        };
    }
}

struct Handler;

impl Handler {
    async fn HandleUserMessage(&self, ctx: Context, msg: &Message, cmd: String, cfg: Cfg) {
        if &cmd == &String::from("help") {
            if let Err(why) = msg.channel_id.say(&ctx.http, &cmd).await {
                println!("Error sending message: {:?}", why);
            }
        }

        if &cmd == &String::from("new") {
            let asd = msg.channel(&ctx).await.unwrap();
            if asd.id() != CONFIG_MAIN_CHANNEL {
                // Only works on configed channel
                return;
            }
            // random number for ticket name
            let rnd: i32 = 300;
            /*
            let mut sample: HashMap<&str, Value> = HashMap::new();
            sample.insert("name", Value::String(String::from("SLAM")));
            let mut channel = CreateChannel(sample);
            */

            // get the user's guild
            let guild = ctx.http.get_guild(msg.guild_id.unwrap()).await.unwrap();
            // get the guild's everyone default role
            let everyone = guild.role_by_name("@everyone").unwrap();
            let permissions = vec![
                PermissionOverwrite {
                    allow: Permissions::default(),
                    deny: Permissions::VIEW_CHANNEL,
                    kind: PermissionOverwriteType::Role(serenity::all::RoleId::new(u64::from(
                        everyone.id,
                    ))),
                },
                PermissionOverwrite {
                    allow: Permissions::SEND_MESSAGES,
                    deny: Permissions::MANAGE_CHANNELS,
                    kind: PermissionOverwriteType::Member(serenity::all::UserId::new(u64::from(
                        msg.author.id,
                    ))),
                },
            ];

            let channel = CreateChannel::new(format!("Ticket-{}", rnd))
                .kind(serenity::all::ChannelType::Text)
                .permissions(permissions);

            // Assuming a guild has already been bound.

            // guild
            let created_channel_sample: serenity::model::channel::GuildChannel = guild
                .create_channel(ctx.http, channel.clone())
                .await
                .unwrap();

            //guild.create_channel(channel).await;
            //channel.execute(&ctx, msg.guild_id.unwrap()).await;
        }
    }

    async fn SendMessage(&self, ctx: &Context, msg: &Message, resp: String) {
        if let Err(why) = msg.channel_id.say(&ctx.http, resp).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        //println!(
        //    "Coming Message from user {} with value of {}",
        //    msg.author.name, &msg.content
        //);
        //println!("{:?}", msg);
        let msg_st: Option<char> = msg.content.chars().nth(0);
        if msg_st.is_some() && msg_st.unwrap() == '!' {
            let actual_cmd: String = msg.content.chars().skip(1).collect();
            self.HandleUserMessage(ctx, &msg, actual_cmd).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    println!("This is your Toket: {}", &token);
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_INVITES
        | GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
