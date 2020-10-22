mod commands;

use log::{error, info};
use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use std::collections::HashSet;

use commands::{meta::*, owner::*, homies::*};
use serenity::model::channel::{Message, Reaction};
use serenity::model::gateway::Activity;

use serde::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, new_message: Message) {
        if new_message.author.bot {
            //todo save the bots message ids to be compared to reactions
        }
    }

    async fn reaction_add(&self, _ctx: Context, add_reaction: Reaction) {
        let _test = add_reaction.user_id.expect("No user ID");
        //todo Save reacts into a table: UserID, React type, Message ID
        //todo make sure it is not the bot reacting to itself
    }

    async fn ready(&self, context: Context, ready: Ready) {
        context
            .set_activity(Activity::playing("Shady Shit: !h for help "))
            .await;
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(h, ping, pong, saypogo, sayfriday, homie, notahomie, raid, brag, jump)]
struct General;

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    // kankyo::load().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    // env_logger::init();

    let token = start_up().config.dis_token;

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::new(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

#[derive(Deserialize)]
struct Properties {
    dis_token: String,
}

#[derive(Deserialize)]
pub struct Config {
    config: Properties,
}

fn start_up() -> Config {
    let path = "C:\\Users\\David\\CLionProjects\\shadybot\\config.toml";
    let config: Config =
        toml::from_str(&std::fs::read_to_string(path).expect("Reading to string from path failed"))
            .expect("Failed to parse string to toml");
    return config;
}