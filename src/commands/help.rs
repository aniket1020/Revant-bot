#![allow(non_snake_case)]
#![allow(unused_imports)]

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use serenity::builder::CreateEmbed;
use serenity::utils::Colour;

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult
{
    msg.channel_id.send_message(&ctx.http, |m|
        {
            m.embed(|emb|
                {
                    emb.title("Result");
                    emb.description("Available programming languages are:\n
                    1] C++ => (use `!run cpp`)
                    2] C => (use `!run c`)
                    3] Python3 => (use `!run python`)
                    4] Java => (use `!run java`)
                    ");
                    emb
                }
            )
        }
    ).await?;
    Ok(())
}
