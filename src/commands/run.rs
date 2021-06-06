use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use crate::Pool;
use std::thread;
use std::time::Duration;

#[command]
pub async fn run(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{
    println!("\n############################\n{:?}\n############################\n{:?}\n############################\n",msg,args);

    let mut data = ctx.data.read().await;
    let pool = data.get::<Pool>().unwrap();

    pool.execute(|id: usize|
        {
            println!("-------------\nYOLO from {}\n------------",id);
            // thread::sleep(Duration::from_secs(5));
        }
    );

    msg.channel_id.say(&ctx.http, "Roger Doger").await?;
    Ok(())
}
