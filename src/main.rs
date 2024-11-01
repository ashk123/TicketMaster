/*
=======================================
Project: TicketMaster
Author: EikoAkiba
Email: eikoakiba@proton.me
Repository: github.com/ashk123/TicketMaster
Twitter: EikoAkiba__
=======================================
*/
mod commands;

use std::env;

use dotenv::dotenv;
use log::{info, trace};

use chrono;
use lazy_static::lazy_static; // 1.4.0
use serenity::all::ChannelId;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::{
    all::{Channel, ChannelType, PermissionOverwrite, PrivateChannel},
    async_trait,
    builder::CreateChannel,
    model::{channel::Message, gateway::Ready, guild::Guild},
    prelude::*,
};
use std::sync::Mutex;

use serenity::model::application::{Command, Interaction};

use serenity::model::channel::{self, PermissionOverwriteType};
use serenity::model::id::UserId;
use serenity::model::permissions::Permissions;

lazy_static! {
    static ref TempChannels: Mutex<Cfg> = Mutex::new(Cfg::default());
}

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

I hope that resolves your issue 

— TicketMasterBot 🤖
";

#[derive(PartialEq)]
enum LogKindModel {
    Info,
    Warning,
    Error,
}

#[derive(Default)]
struct Cfg {
    main_guild_channel: ChannelId,
    temp_channels: Vec<ChannelId>,
}

impl Cfg {
    fn new(main_guild_channel: ChannelId) -> Self {
        Self {
            main_guild_channel,
            temp_channels: vec![],
        }
    }
    // TODO: Check the input (created_ticket_channel)
    fn add_temp_channel(&mut self, created_ticket_channel: ChannelId) {
        self.temp_channels.push(created_ticket_channel);
    }

    fn IsAvailable(&self, cha: ChannelId) -> bool {
        self.temp_channels.contains(&cha)
    }

    fn set_main_channel(&mut self, data: ChannelId) {
        self.main_guild_channel = data;
    }

    fn get_main_channel(&self) -> ChannelId {
        self.main_guild_channel
    }
}

struct Handler;

impl Handler {
    // }    cfg.set_main_channel(conf_channel);

    async fn SendMessage(&self, ctx: &Context, msg: &Message, resp: String) {
        if let Err(why) = msg.channel_id.say(&ctx.http, resp).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");
            let content: Option<String> = match command.data.name.as_str() {
                "info" => Some(commands::info::run(&command.data.options())),
                "new" => Some(commands::new::run(&command.data.options(), &ctx, &command).await),
                "list" => Some(commands::info::run(&command.data.options())),
                "close" => {
                    Some(commands::close::run(&command.data.options(), &ctx, &command).await)
                }
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        TempChannels
            .lock()
            .unwrap()
            .set_main_channel(ChannelId::new(0)); // Temp code block
                                                  // 1234: example user guild's id
        let guild_id_sample: Vec<serenity::model::guild::UnavailableGuild> = ready.guilds;

        for i in guild_id_sample {
            let guild = ctx.http.get_guild(i.id).await.unwrap();
            let _ = guild
                .set_commands(
                    &ctx.http,
                    vec![
                        commands::info::register(),
                        commands::new::register(),
                        commands::close::register(),
                    ],
                )
                .await;
        }

        /*
        guild_id_sample.into_iter().for_each(|i| {
            let value = ctx.http.clone();
            tokio::spawn(async move {
                let guild = value.get_guild(i.id).await.unwrap();
            });
        });
        */
        println!("{} is connected!", ready.user.name);
    }
}

fn logger(msg: &str, mode: LogKindModel) {
    let mut first: &str = "[I]";
    if mode == LogKindModel::Warning {
        first = "[W]";
    } else if mode == LogKindModel::Error {
        first = "[E]";
    }

    let date = chrono::Local::now();
    println!("{}{} {}", first, date.format("[%Y-%m-%d][%H:%M:%S]"), msg);
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // loads the .env file

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    logger("[+] Discord Token Loaded", LogKindModel::Info);

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
        logger(
            format!("Client error: {:?}", why).as_str(),
            LogKindModel::Error,
        )
    }
}
