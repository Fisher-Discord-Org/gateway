use std::env;

use serenity::all::GuildId;
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use tracing::instrument;

#[derive(Debug)]
pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    #[instrument(skip(self, _ctx, ready))]
    async fn ready(&self, _ctx: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("DEV_GUILD_ID")
                .expect("Expected DEV_GUILD_ID in env")
                .parse()
                .expect("DEV_GUILD_ID must be an integer"),
        );

        println!("Guild ID: {:?}", guild_id);
    }
}
