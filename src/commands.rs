mod export;
mod info;
mod leaderboard;
mod undo;
mod vn;

use crate::{Context, Error};
use export::export;
use info::info;
use leaderboard::leaderboard;
use undo::undo;
use vn::vn;

/// Log your nukis
#[poise::command(slash_command, subcommands("export", "info", "undo", "vn"))]
pub async fn log(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
