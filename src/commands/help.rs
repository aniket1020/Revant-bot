use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult
{
    msg.channel_id.say(&ctx.http, "hello").await?;
    Ok(())
}
