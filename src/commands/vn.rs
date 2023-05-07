use chrono::{Duration, NaiveDate, NaiveTime, Utc};
use sqlx::query;

use crate::{
    sheets::{log_to_sheets, SheetsLog},
    utils::{fmt_duration, get_vn_name, parse_date},
    Context, Error,
};

/// Log your VN chars and readtime.
#[poise::command(slash_command)]
pub async fn vn(
    ctx: Context<'_>,
    #[description = "Amount of characters read"] chars: u32,
    #[description = "Time you've read for, in `[hr:min]` or `[min]` format. Example: `1:28`, `54`"]
    time: Option<String>,
    #[description = "Name of the VN. You can also use a vndb ID, like v17, which you can find in the vndb URL"]
    name: Option<String>,
    #[description = "Comment"] comment: Option<String>,
    #[description = "Backlog to this date: format year-month-day. Example: `2023-01-14`"]
    date: Option<NaiveDate>,
) -> Result<(), Error> {
    let id = ctx.author().id.to_string();
    let logged_date = match date {
        Some(date) => date.and_time(NaiveTime::MIN),
        None => Utc::now().naive_utc(),
    };

    let time = match time {
        Some(time) => match parse_date(&time) {
            Ok(date) => Some(date),
            Err(e) => {
                let _ = ctx.say(e).await;
                return Ok(());
            }
        },
        None => None,
    }
    .map(|duration| duration.num_minutes());

    query!(
        r#"insert into log(uid, timestamp, type, count, name, time, comment)
        values(?,?,?,?,?,?,?)"#,
        id,
        logged_date,
        "vn",
        chars,
        name,
        time,
        comment,
    )
    .execute(ctx.data().db.as_ref())
    .await?;

    // Try logging to google sheets
    let spreadsheet_id = query!("select spreadsheet_id from sheets_id where uid=?", id,)
        .fetch_optional(ctx.data().db.as_ref())
        .await?;
    let logged_to_sheet = if let Some(row) = spreadsheet_id {
        log_to_sheets(
            ctx,
            row.spreadsheet_id,
            SheetsLog {
                date: logged_date,
                name: name.clone(),
                chars,
                time,
            },
        )
        .await?;
        true
    } else {
        false
    };

    let mut res = vec![];
    res.push(format!("<@{id}>"));
    if date.is_some() {
        res.push(format!(
            " back-logged (at <t:{}:R>)",
            logged_date.timestamp()
        ))
    } else {
        res.push(format!(" logged"))
    }

    res.push(format!(" **{chars}** chars"));
    if let Some(time) = time {
        res.push(format!(
            " for **{}**",
            fmt_duration(Duration::minutes(time))
        ))
    }
    if let Some(name) = name {
        res.push(format!(
            " on **[{}]({})**",
            get_vn_name(name.clone()),
            name.clone()
        ));
    }
    res.push(format!("."));

    if logged_to_sheet {
        res.push(" *sheets OK*".to_string())
    }

    if let Some(comment) = comment {
        res.push(format!("\n> {comment}"));
    }

    ctx.say(res.join("")).await?;
    Ok(())
}
