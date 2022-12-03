use clap::{Parser, ValueEnum};
use tokio::time::sleep;
use std::time::Duration;
use serenity::model::user::OnlineStatus;
use serenity::model::gateway::{Activity, GatewayIntents};
use serenity::prelude::*;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    token: String,

    #[arg(short, long)]
    status_message: String,

    #[arg(value_enum)]
    status_type: Status,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Status {
    /// Show online status
    Online,

    /// Show Do Not Disturb status
    DoNotDisturb,

    /// Show Idle status
    Idle,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut client = Client::builder(&args.token, GatewayIntents::empty()).await.unwrap();
    let manager = client.shard_manager.clone();
    tokio::spawn(async move {
        let status = match args.status_type {
            Status::Online => OnlineStatus::Online,
            Status::DoNotDisturb => OnlineStatus::DoNotDisturb,
            Status::Idle => OnlineStatus::Idle
        };
        sleep(Duration::from_secs(15)).await;

        for shard in manager.lock().await.runners.lock().await.iter_mut() {
            shard.1.runner_tx.set_presence(Some(Activity::playing(args.status_message.clone())), status);
        }
    });
    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}
