mod export;
mod info;
mod sheets_register;
mod undo;
mod vn;

use crate::{Context, Error};
use export::export;
use info::info;

use sheets_register::sheets_register;
use undo::undo;
use vn::vn;

/// Log your nukis
#[poise::command(
    slash_command,
    subcommands("export", "info", "undo", "vn", "sheets_register")
)]
pub async fn log(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
