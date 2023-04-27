use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn homie(ctx: &Context, msg: &Message) -> CommandResult {
    //todo Make check if they are already a homie
    let guild_id = u64::from(msg.guild_id.expect("No Guild ID found"));
    let user_id= u64::from(msg.author.id);
    let role_id =  741418770039308356;
    let saying;
    let test = msg.author.has_role(&ctx.http,guild_id, role_id).await;
    match test {
        Ok(has_role) => {
            if has_role {
                saying = format!("{} is already homie and wasting my time", msg.author);
            }
            else {
                saying = format!("{} is now a homie and ready to raid", msg.author);
            }
        },
        Err(..) => {saying = format!("Shit borked Yo")}
    }
    msg.channel_id.say(&ctx.http, saying).await?;
    ctx.http.add_member_role(guild_id, user_id, role_id, Some("making user a homie")).await?;
    Ok(())
}

#[command]
async fn notahomie(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = u64::from(msg.guild_id.expect("No Guild ID found"));
    let user_id= u64::from(msg.author.id);
    let role_id =  741418770039308356;
    let saying ;
    let has_role =  msg.author.has_role(&ctx, guild_id, role_id).await?;
    if has_role {
        saying = format!("{} is no longer a homie and totally Lame ", msg.author);
        ctx.http.remove_member_role(guild_id, user_id, role_id, Some("remove users Homie status")).await?;
    }
    else {
        saying = format!("{} was not a homie to begin with... what a loser... ", msg.author);
    }
    msg.channel_id.say(&ctx.http, saying).await?;
    Ok(())
}

#[command]
async fn raid(ctx: &Context, msg: &Message) -> CommandResult {
    let role_id =  "741418770039308356";
        let saying = format!("{} has found a Raid! \n Lets hit it <@&{}>", msg.author, role_id );
        msg.channel_id.say(&ctx.http, saying).await?;
    Ok(())
}

#[command]
async fn brag(ctx: &Context, msg: &Message) -> CommandResult {
    let role_id =  "741418770039308356";
    let saying = format!("<@&{}> {} got some cool shit to brag about! \n Damn that is some cool shit...", role_id , msg.author);
    msg.channel_id.say(&ctx.http, saying).await?;
    Ok(())
}

#[command]
async fn jump(ctx: &Context, msg: &Message) -> CommandResult {
    //todo
    // make this read the raid post if it has reacts tag them
    let role_id =  "741418770039308356";
    let gif_url = "https://tenor.com/view/jump-water-fail-deep-gif-4858815";
    let saying = format!("{}'s raid is jumping in <@&{}> \n {}", msg.author, role_id, gif_url);
    msg.channel_id.say(&ctx.http, saying).await?;
    Ok(())
}

#[command]
async fn jumping(ctx: &Context, msg: &Message) -> CommandResult {
    //todo
    // make this read the raid post if it has reacts tag them
    let role_id =  "741418770039308356";
    let gif_url = "https://tenor.com/view/trampoline-jump-gomez-addamsfamily-gif-6004087";
    let saying = format!("{}'s raid is jumping in <@&{}> \n {}", msg.author, role_id, gif_url);
    msg.channel_id.say(&ctx.http, saying).await?;
    Ok(())
}

#[command]
async fn hump(ctx: &Context, msg: &Message) -> CommandResult {
    let gif_url = "https://tenor.com/view/funny-dance-gif-5144812";
    let saying = format!("{}", gif_url);
    msg.channel_id.say(&ctx.http, saying).await?;
    Ok(())
}

#[command]
async fn lucky(ctx: &Context, msg: &Message) -> CommandResult {
    let gif_url = "https://tenor.com/view/lucky-napoleondynamite-luck-gif-7694851";
    let saying = format!("{}", gif_url);
    msg.channel_id.say(&ctx.http, saying).await?;
    Ok(())
}

#[command]
async fn shadyquestions(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = u64::from(msg.guild_id.expect("No Guild ID found"));
    let user_id= u64::from(msg.author.id);



    msg.channel_id.say(&ctx.http, saying).await?;
    Ok(())
}