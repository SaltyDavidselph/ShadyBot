use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
async fn pong(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Ping!").await?;
    Ok(())
}

#[command]
async fn saypogo(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.content.is_empty(){
    }
    else {
        let text = msg.content.clone();
        //todo
        // add something to remove the "!say"
        // string.chars().skip(len).collect()
        let text = text.replace("!saypogo","");
        let pogo_channel_id = 593992232885813279;
        let guild = msg.guild(&ctx.cache).await.expect("No guild in the cache");
        let channels = &guild.channels;
        let mut message = MessageBuilder::new();
        for (channelid, guild_channel) in channels.into_iter() {
            if (channelid.0).eq(&pogo_channel_id) {
                let channel = guild_channel;
                let txt = text.split_ascii_whitespace();
                for part in txt{
                    if part.starts_with('@'){
                        let mention = guild.member_named(part.trim_start()).expect("No member by that name found");
                        message.mention(mention);
                    }
                    else{
                        message.push(format!("{} ", part));
                    }
                }
                channel.say(ctx, message.build()).await?;
            }
        }
    }
    Ok(())
}

#[command]
async fn sayfriday(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.content.is_empty(){
    }
    else {
        let text = msg.content.clone();
        let text = text.replace("!sayfriday","");
        let friday_channel_id = 759498740234584086;
        let guild = msg.guild(&ctx.cache).await.expect("No guild in the cache");
        let channels = &guild.channels;
        let mut message = MessageBuilder::new();
        for (channelid, guild_channel) in channels.into_iter() {
            if (channelid.0).eq(&friday_channel_id) {
                let channel = guild_channel;
                let txt = text.split_ascii_whitespace();
                for part in txt{
                    if part.starts_with('@'){
                        let mention = guild.member_named(part.trim_start()).expect("No member by that name found");
                        message.mention(mention);
                    }
                    else{
                        message.push(format!("{} ", part));
                    }
                }
                channel.say(ctx, message.build()).await?;
            }
        }
    }
    Ok(())
}