use std::env;

use serenity::all::GuildId;
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("DEV_GUILD_ID")
                .expect("Expected DEV_GUILD_ID in env")
                .parse()
                .expect("DEV_GUILD_ID must be an integer"),
        );

        println!("Guild ID: {:?}", guild_id);
    }
}
