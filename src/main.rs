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
use serenity::model::*;
use serde::*;
use serenity::model::channel::ReactionType::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, _new_message: Message) {

    }

    // async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
    //     let reactor = ctx.cache.user(add_reaction.user_id.expect("no User Id Found")).await.expect("no User found");
    //     let guild = &ctx.cache.guild(add_reaction.guild_id.unwrap()).await.expect("Guild not found");
    //     let brave_cap =  guild.role_by_name("Brave and Mighty Captian").expect("Brave and mighty Captian Not found");
    //     //check if the reator is the brave Captian
    //     let check = reactor.has_role(&ctx, u64::from(guild.id),u64::from(brave_cap.id)).await;
    //     if check.expect("")
    //     //reactor.has_role(ctx, add_reaction.guild_id,brave_cap).await?
    //     {
    //         //check if the emoji is salt
    //         if add_reaction.emoji.as_data().eq(":salt:"){
    //
    //             // message.react(&ctx, Unicode(String::from(":salt:")));
    //         }
    //     }
    // }

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
#[commands(h, ping, pong, say, saypogo, sayfriday, homie, notahomie, raid, brag, jump, jumping, hump)]
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