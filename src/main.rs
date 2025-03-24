use anyhow::Result;
use config::Config;
use serde::Deserialize;
use serenity::all::{ClientBuilder, HttpBuilder};
use serenity::model::gateway::GatewayIntents;

mod core;
use core::discord::Handler;
mod utils;
use utils::log::{LogFormat, init_logger};

#[tracing::instrument]
async fn shutdown_signal() {
    tracing::event!(tracing::Level::INFO, "Listening for abort signal...");

    if let Err(why) = tokio::signal::ctrl_c().await {
        tracing::error!("Failed to listen for abort signal: {}", why);
    } else {
        tracing::event!(
            tracing::Level::INFO,
            "Abort signal received. Shutting down..."
        );
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Settings {
    debug: bool,
    token: String,
}

fn load_config() -> Result<Settings> {
    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().ok();
    }

    let settings = Config::builder()
        .add_source(config::File::with_name("config").required(false))
        .add_source(config::Environment::with_prefix("DISCORD"))
        .set_default(
            "debug",
            match cfg!(debug_assertions) {
                true => true,
                false => false,
            },
        )?
        .build()?
        .try_deserialize()?;

    Ok(settings)
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logger(LogFormat::Full);

    let settings = load_config().expect("Failed to load settings");

    tracing::info!("Settings: {:?}", settings);
    let http = HttpBuilder::new(settings.token).build();

    let user = match http.get_current_user().await {
        Ok(user) => user,
        Err(why) => {
            tracing::error!("Failed to validate token: {}", why);
            return Ok(());
        }
    };

    tracing::info!("Logged in as: {}", user.name);

    let mut client = match ClientBuilder::new_with_http(http, GatewayIntents::default())
        .event_handler(Handler)
        .await
    {
        Ok(client) => client,
        Err(why) => {
            tracing::error!("Failed to create client: {}", why);
            return Ok(());
        }
    };

    tracing::info!("Client created");

    if let Err(why) = client.start().await {
        tracing::error!("Failed to start client: {}", why);
    }

    Ok(())
}
