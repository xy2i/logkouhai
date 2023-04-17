use chrono::{Duration, NaiveDateTime, Utc};
use poise::serenity_prelude::{self as serenity};
use sqlx::query_as;

use crate::{
    utils::{fmt_duration, week_start},
    Context, Error,
};

struct Log {
    uid: String,
    timestamp: NaiveDateTime,
    count: i64,
    name: Option<String>,
    time: Option<i64>,
    comment: Option<String>,
}

/// Get your last logs and total read, or someone else's.
#[poise::command(slash_command)]
pub async fn info(
    ctx: Context<'_>,
    #[description = "Show stats for this VN"] vn: Option<String>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let this_week = week_start(Utc::now().naive_utc());

    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let uid = u.id.0.to_string();

    let logs = if let Some(vn) = vn.clone() {
        query_as!(
        Log,
        r#"select uid, timestamp, count, name, time, comment from log where uid=? and name=? order by timestamp desc"#,
        uid, vn)
    .fetch_all(&ctx.data().db)
    .await?
    } else {
        query_as!(
        Log,
        r#"select uid, timestamp, count, name, time, comment from log where uid=? order by timestamp desc"#,
        uid)
    .fetch_all(&ctx.data().db)
    .await?
    };

    let count: i64 = logs.iter().map(|l| l.count).sum();
    let time: i64 = logs
        .iter()
        .map(|l| match l.time {
            None => 0,
            Some(mins) => mins,
        })
        .sum();
    let time = Duration::minutes(time);

    let weekly_count: i64 = logs
        .iter()
        .filter(|l| l.timestamp > this_week)
        .map(|l| l.count)
        .sum();
    let weekly_time: i64 = logs
        .iter()
        .filter(|l| l.timestamp > this_week)
        .map(|l| match l.time {
            None => 0,
            Some(mins) => mins,
        })
        .sum();
    let weekly_time = Duration::minutes(weekly_time);

    let mut s = String::new();
    s += &format!("**Stats for <@{uid}>**");
    if let Some(vn) = vn.clone() {
        s += &format!(" for vn **{vn}**");
    }
    s += &format!(
        "
Logged **{count}** characters in **{}** hours.
**Weekly stats**:
Logged **{weekly_count}** characters in **{}** hours.",
        fmt_duration(time),
        fmt_duration(weekly_time),
    );

    ctx.send(|f| f.embed(|f: &mut serenity::CreateEmbed| f.title("").description(s)))
        .await?;
    Ok(())
}
