use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}

#[command]
async fn h(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, help()).await?;
    Ok(())
}

fn help() -> &'static str {
    "Welcome, I am your Guide to Salty Dave's Shady Secret Sever! \n\
    !homie will make you a Homie \n\
    !notahomie will take away your homie status \n\
    !raid This will tag @theHomies that there is a raid happening \n\
    !brag Tag the Homies to brag to them! \n\
    !jump Tag that we are jumping into the Raid \n\
    "
}