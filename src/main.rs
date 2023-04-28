mod commands;
mod utils;

use chrono::prelude::*;
use dotenv::dotenv;
use once_cell::sync::{Lazy, OnceCell};
use poise::serenity_prelude::{self as serenity};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

use crate::commands::log;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

static VNDB_NAME_CACHE: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug)]
struct NukiLog {
    discord_uid: String,
    count: i64,
    timestamp: NaiveDateTime,
    comment: Option<String>,
}

#[derive(Debug)]
pub struct Data {
    db: SqlitePool,
}

#[tokio::main]
async fn main() {
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

    let bot = Data { db };

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
