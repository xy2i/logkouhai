mod commands;
mod sheets;
mod utils;

use dotenv::dotenv;
use once_cell::sync::Lazy;
use poise::serenity_prelude::{self as serenity};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use crate::commands::log;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

static VNDB_NAME_CACHE: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug)]
pub struct Data {
    db: Arc<SqlitePool>,
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    println!("debug mode");

    dotenv().ok();

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(
            env::var("DATABASE_URL")
                .expect("missing DATABASE_URL")
                .as_str(),
        )
        .await
        .expect("Could not connect to database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Couldn't run database migrations");

    let bot = Data { db: Arc::new(db) };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![log()],
            ..Default::default()
        })
        .token(env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(Default::default())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    serenity::GuildId(
                        env::var("GUILD_ID")
                            .expect("missing GUILD_ID")
                            .parse()
                            .unwrap(),
                    ),
                )
                .await?;
                println!("registered");
                Ok(bot)
            })
        });

    framework.run().await.unwrap();
}
