use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use crate::Pool;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

use revant::{Result, Status};

fn handle_request(id: usize, request_message: String) -> Result
{
    let workerdir = format!("Workers/Workerdir{}/",id);
    let compile = format!("Workers/compile.rb");
    let run = format!("Workers/run.rb");

    Result{ runtime: 0.3, status:Status::Success, output: workerdir }
}

#[command]
pub async fn run(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{
    // println!("\n############################\n{:?}\n############################\n{:?}\n############################\n",msg,args);

    let data = ctx.data.read().await;
    let pool = data.get::<Pool>().unwrap();

    let request_message = args.message().to_string();

    pool.execute(move |id: usize, resSender: &mpsc::Sender<Result>|
        {
            // println!("-------------\nYOLO from {}\n------------",id);
            // thread::sleep(Duration::from_secs(60));

            let result: Result = handle_request(id,request_message);
            resSender.send(result).unwrap();
        }
    );

    //Not optimal - Change to a proper listener
    let result = pool.resReceiver.recv().unwrap();
    let response = format!("{}",result.output);
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}
