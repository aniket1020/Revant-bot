extern crate dotenv;

mod commands;

use std::env;
use dotenv::dotenv;

use std::{
    collections::HashSet,
    sync::Arc,
};

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    http::Http,
    model::{gateway::Ready, channel::Message,},
    prelude::*,
};

use tracing::{error, info};
use tracing_subscriber::{
    FmtSubscriber,
    EnvFilter,
};

use commands::{
    help::*,
    owners::*,
    run::*,
};

use revant::ThreadPool;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}


struct Handler;

#[async_trait]
impl EventHandler for Handler
{
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
}

#[group]
#[commands(help,quit,run)]
struct General;

pub struct Pool;

impl TypeMapKey for Pool
{
    type Value = ThreadPool;
}

#[tokio::main]
async fn main()
{
    dotenv().expect(".env not found");

    let subscriber = FmtSubscriber::builder()
                        .with_env_filter(EnvFilter::from_default_env())
                        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await
    {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
                        .configure(|c| c
                                    .owners(owners)
                                    .prefix("!"))
                        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
                        .framework(framework)
                        .event_handler(Handler)
                        .await
                        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move
        {
            tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
            shard_manager.lock().await.shutdown_all().await;
        }
    );

    println!("Starting Workers");
    let pool = ThreadPool::new(8);

    {
        let mut data = client.data.write().await;
        data.insert::<Pool>(pool);
    }

    println!("Starting Revant Bot");

    if let Err(error) = client.start().await
    {
        println!("Client error : {}",error);
    }
}
