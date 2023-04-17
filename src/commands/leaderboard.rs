use chrono::{Duration, NaiveDateTime, Utc};
use poise::serenity_prelude::{self as serenity};
use sqlx::{query, query_as};

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

/// Leaderboard, for all or for a specific vn.
#[poise::command(slash_command)]
pub async fn leaderboard(
    ctx: Context<'_>,
    #[description = "Show leaderboard for a specific vn"] vn: Option<String>,
) -> Result<(), Error> {
    fn ordinal(i: usize) -> String {
        match i {
            1 => "1st".into(),
            2 => "2nd".into(),
            3 => "3rd".into(),
            _ => format!("{i}th"),
        }
    }

    // struct Log {
    //     uid: String,
    //     count: i64,
    //     time: i64,
    // }

    // let mut logs = if let Some(vn) = vn {
    //     query_as!(
    //         Log,
    //         r#"select uid, coalesce(sum(count), 0) as `count:_`, coalesce(sum(time), 0) as `time:_` from log where name=? group by uid order by timestamp desc limit 20 "#,
    //         vn
    //     )
    // .fetch_all(&ctx.data().db)
    // .await?;
    // } else {
    //     query_as!(
    //         Log,
    //         r#"select uid, coalesce(sum(count), 0) as `count:_`, coalesce(sum(time), 0) as `time:_` from log group by uid order by timestamp desc limit 20 "#,
    //     )
    // .fetch_all(&ctx.data().db)
    // .await?;
    // };

    // let mut s = String::new();
    // for (i, data) in logs {
    //     s += &format!(
    //         "**{} <@{}>**: **{}** chars, **{}** hours",
    //         ordinal(i + 1),
    //         data.uid,
    //         data.count,
    //         fmt_duration(Duration::minutes(data.time))
    //     )
    // }

    // let title = if let Some(vn) = vn {
    //     &format!("Leaderboard for {vn}")
    // } else {
    //     "Leaderboard"
    // };

    // ctx.send(|f| f.embed(|f| f.title(title).description(s)))
    //     .await?;
    Ok(())
}
