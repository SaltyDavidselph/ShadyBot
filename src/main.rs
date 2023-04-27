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
use serenity::model::channel::Message ;
use serenity::model::gateway::Activity;
use serde::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

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
#[commands(h, ping, pong, saypogo, sayfriday, homie, notahomie, raid, brag, jump, jumping, hump)]
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

    let http = Http::new(&token);

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

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token,intents )
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