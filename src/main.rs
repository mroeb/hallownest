
use std::env;

use dotenv::dotenv;
use serenity::{all::{Context, EventHandler, GatewayIntents, Ready}, async_trait, Client};
use tokio::{join, task};


struct Handler;


#[async_trait]
impl EventHandler for Handler {



    async fn ready(&self, _: Context, ready: Ready) {

        println!("Logged in as: {}", ready.user.name);


        println!("GUILDS - ");
        for g in ready.guilds.iter() {
            println!("{}", g.id);
        }

    }

}



#[tokio::main]
async fn main() {

    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected discord token in the enviroment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    
    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");


    let hallow_task = task::spawn(async move {
        if let Err(why) = client.start().await {
            println!("Client error: {why:?}");
        }
    });


    let _ = join!(hallow_task);


}