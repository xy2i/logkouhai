use sqlx::query;

use crate::sheets::get_token;
use crate::{Context, Error};

/// Sync any uses of the /log command with your Google sheets.
/// The google sheets must be Xelieu template
#[poise::command(slash_command)]
pub async fn sheets_register(
    ctx: Context<'_>,
    #[description = "The spreadsheet ID to log to"] spreadsheet_id: String,
) -> Result<(), Error> {
    let uid = ctx.author().id.0.to_string();
    ctx.say("先輩のために頑張ります！！").await?;

    get_token(ctx).await?;

    query!(
        "insert into sheets_id values($1, $2)
on conflict do update set spreadsheet_id=$2",
        uid,
        spreadsheet_id
    )
    .execute(ctx.data().db.as_ref())
    .await?;

    ctx.say(&format!("Set spreadsheet ID to `{}`.", spreadsheet_id))
        .await?;

    Ok(())
}
