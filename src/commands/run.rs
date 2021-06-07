use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use crate::Pool;

use std::thread;
use std::time::Duration;

struct Result
{
    runtime: f32,
    status: Status,
    output: String,
}

enum Status
{
    Success,
    Failure
}

fn handle_request(id: usize, request_message: String)
{
    let workerdir = format!("Workers/Workerdir{}/",id);
    let compile = format!("Workers/compile.rb");
    let run = format!("Workers/run.rb");


}

#[command]
pub async fn run(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{
    println!("\n############################\n{:?}\n############################\n{:?}\n############################\n",msg,args);

    let data = ctx.data.read().await;
    let pool = data.get::<Pool>().unwrap();

    let request_message = args.message().to_string();

    pool.execute(move |id: usize|
        {
            println!("-------------\nYOLO from {}\n------------",id);
            // thread::sleep(Duration::from_secs(60));
            
            handle_request(id,request_message);
        }
    );

    msg.channel_id.say(&ctx.http, "Roger Doger").await?;
    Ok(())
}
