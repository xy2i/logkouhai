use sqlx::query;

use crate::{Context, Error};

/// Undo your last log.
#[poise::command(slash_command)]
pub async fn undo(ctx: Context<'_>) -> Result<(), Error> {
    let uid = ctx.author().id.0.to_string();

    query!(
        r#"delete from log where uid=? and id=
(select max(id) from log where uid=?)"#,
        uid,
        uid
    )
    .execute(ctx.data().db.as_ref())
    .await?;

    ctx.say("Undo sucessful.").await?;
    Ok(())
}
