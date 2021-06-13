#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use crate::{Pool, LANGCMP, LANGRUN};

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::process::Command;
use std::fs;
use std::env;

use revant::{ResResult, Result, Status};

use regex::Regex;

fn handle_request(id: usize, request_message: String) -> ResResult
{
    let workerdir   = format!("Workers/Workerdir{}",id);
    let compile     = format!("Workers/jobcmp.rb");
    let run         = format!("Workers/jobrun.rb");
    let clean       = format!("Workers/clean.rb");

    let re = Regex::new(r"([^```]+)").unwrap();
    let mut v: Vec<String> = Vec::with_capacity(2 as usize);
    for it in re.captures_iter(&request_message)
    {
        if &it[0] == ""
        {
            return ResResult::InvalidCommand;
        }

        v.push(it[0].to_string());
    }

    if v.len() != 2
    {
        return ResResult::InvalidCommand;
    }

    let call = v.get(0).unwrap().replace("\n", "");
    let re = Regex::new(r"\s+").unwrap();
    let call = re.replace_all(call.as_str(), " ");
    let mut call_args: Vec<String> = Vec::new();
    for it in call.split(" ")
    {
        call_args.push(String::from(it));
    }

    let key = call_args.get(0).unwrap();

    if      LANGCMP.contains_key(key.as_str())
    {

        let (util, filename, flag, output) = LANGCMP.get(key.as_str()).unwrap();

        let filename = format!("{}/{}",workerdir,filename);
        let output   = if output != &"" {format!("{}/{}",workerdir,output)} else { "".to_string() };
        let call_args = call_args[1..].join(" ");
        let command = format!("{} {} {} {} {}", util,filename, flag, output, call_args);

        println!("Compiling - {}",command);

        fs::write(&filename, v.get(1).unwrap()).expect("Error writing code to file");

        let exec = Command::new("ruby")
                            .arg(compile)
                            .arg(&command)
                            .output()
                            .unwrap_or_else(|e| {
                                panic!("{}",e);
                            });

        if exec.stderr.len() > 0 as usize
        {
            let s = String::from_utf8_lossy(&exec.stderr);
            println!("Cleaning - {}",workerdir);
            fs::remove_dir_all(&workerdir).unwrap();
            fs::create_dir(&workerdir).unwrap();
            return ResResult::Result(Result{ status:Status::Failure, output: s.to_string() });
        }

        let (util, filename) = LANGRUN.get(key.as_str()).unwrap();

        let filename = format!("{}/{}",workerdir,filename);

        let command = if util == &"bin"
        {
            filename
        }
        else
        {
            format!("{} {}",util,filename)
        };

        println!("Executing - {}",command);

        let exec = Command::new("ruby")
                            .arg(run)
                            .arg(&command)
                            .output()
                            .unwrap_or_else(|e| {
                                panic!("{}",e);
                            });

        if exec.stderr.len() > 0 as usize
        {
            let s = String::from_utf8_lossy(&exec.stderr);
            println!("Cleaning - {}",workerdir);
            fs::remove_dir_all(&workerdir).unwrap();
            fs::create_dir(&workerdir).unwrap();
            return ResResult::Result(Result{ status:Status::Failure, output: s.to_string() });
        }
        else
        {
            let s = String::from_utf8_lossy(&exec.stdout);
            println!("Cleaning - {}",workerdir);
            fs::remove_dir_all(&workerdir).unwrap();
            fs::create_dir(&workerdir).unwrap();
            return ResResult::Result(Result{ status:Status::Success, output: s.to_string() });
        }
    }
    else if LANGRUN.contains_key(key.as_str())
    {
        let (util, filename) = LANGRUN.get(key.as_str()).unwrap();

        let filename = format!("{}/{}",workerdir,filename);

        fs::write(&filename, v.get(1).unwrap()).expect("Error writing code to file");

        let command = if util == &"bin"
        {
            filename
        }
        else
        {
            format!("{} {}",util,filename)
        };

        println!("Executing - {}",command);

        let exec = Command::new("ruby")
                            .arg(run)
                            .arg(&command)
                            .output()
                            .unwrap_or_else(|e| {
                                panic!("{}",e);
                            });

        if exec.stderr.len() > 0 as usize
        {
            let s = String::from_utf8_lossy(&exec.stderr);
            println!("Cleaning - {}",workerdir);
            fs::remove_dir_all(&workerdir).unwrap();
            fs::create_dir(&workerdir).unwrap();
            return ResResult::Result(Result{ status:Status::Failure, output: s.to_string() });
        }
        else
        {
            let s = String::from_utf8_lossy(&exec.stdout);
            println!("Cleaning - {}",workerdir);
            fs::remove_dir_all(&workerdir).unwrap();
            fs::create_dir(&workerdir).unwrap();
            return ResResult::Result(Result{ status:Status::Success, output: s.to_string() });
        }
    }
    else
    {
        ResResult::InvalidCommand
    }
}

#[command]
pub async fn run(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{
    let data = ctx.data.read().await;
    let pool = data.get::<Pool>().unwrap();

    let request_message = args.message().to_string();

    pool.execute(move |id: usize, resSender: &mpsc::Sender<ResResult>|
        {
            let result = handle_request(id,request_message);
            resSender.send(result).unwrap();
        }
    );

    //Not optimal - Change to a proper listener
    let result = pool.resReceiver.recv().unwrap();
    match result
    {
        ResResult::Result(result) => msg.channel_id.say(&ctx.http, format!("{}",result.output)).await?,
        ResResult::InvalidCommand => msg.channel_id.say(&ctx.http, "Invalid command, use ```!help``` to view available commands").await?,
    };

    Ok(())
}
