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
        //remove the command from the text
        let text = msg.content.clone();
        let text = text.replace("!saypogo","");
        let mut message = MessageBuilder::new();
        for part in text.split_ascii_whitespace(){
            message.push(format!("{} ", part));
        }

        let pogo_channel_id: u64 = 593992232885813279;

        for (channel_id, _channel) in msg.guild(&ctx.cache).expect("No guild in the cache").channels{
            if channel_id.eq(&pogo_channel_id){
                channel_id.say(&ctx.http, message.build()).await?;
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
        //remove the command from the text
        let text = msg.content.clone();
        let text = text.replace("!sayfriday","");
        let mut message = MessageBuilder::new();
        for part in text.split_ascii_whitespace(){
            message.push(format!("{} ", part));
        }

        let friday_channel_id = 759498740234584086;

        for (channel_id, _channel) in msg.guild(&ctx.cache).expect("No guild in the cache").channels{
            if channel_id.eq(&friday_channel_id){
                channel_id.say(&ctx.http, message.build()).await?;
            }
        }
    }
    Ok(())
}

// #[command]
// async fn say(ctx: &Context, msg: &Message) -> CommandResult {
//     if msg.content.is_empty(){
//         //do Nothing
//     }
//     else {
//         let text = msg.content.clone();
//         let text = text.replace("!say", "");
//         let channel_ids = &msg.mention_channels;
//         for test in channel_ids.clone().unwrap(){
//             println!("mentions: {}", test.name);
//         }
//         let mut channel_id = None;
//         if channel_ids.is_none() {
//             channel_id = Some(msg.channel_id);
//         } else {
//             let guild = msg.guild(&ctx.cache).await.expect("No guild in the cache");
//             let channels = &guild.channels;
//             let mut message = MessageBuilder::new();
//             for (channelid, guild_channel) in channels.into_iter() {
//                 for channel_mention in channel_ids.clone().unwrap() {
//                     if channel_mention.id.eq(channelid) {
//                         let channel = guild_channel;
//                         let txt = text.split_ascii_whitespace();
//                         for part in txt {
//                             if part.starts_with('@') {
//                                 let mention = guild.member_named(part.trim_start()).expect("No member by that name found");
//                                 message.mention(mention);
//                             } else {
//                                 message.push(format!("{} ", part));
//                             }
//                         }
//                         println!("Text = {}", text);
//                         channel.say(ctx, message.build()).await?;
//                     }
//                 }
//             }
//         }
//     }
//     Ok(())
// }