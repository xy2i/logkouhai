use chrono::{NaiveDate, NaiveDateTime};
use sqlx::query_as;
use std::str;

use crate::{Context, Error};

#[derive(Debug, serde::Serialize)]
struct Log {
    timestamp: NaiveDateTime,
    count: i64,
    name: Option<String>,
    time: Option<i64>,
    comment: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct LogDate {
    timestamp: NaiveDate,
    count: i64,
    name: Option<String>,
    time: Option<i64>,
    comment: Option<String>,
}

/// Export your logs to Excel.
#[poise::command(slash_command)]
pub async fn export(ctx: Context<'_>) -> Result<(), Error> {
    let uid = ctx.author().id.0.to_string();

    let logs = query_as!(
        Log,
        r#"select timestamp, count, name, time, comment from log where uid=? order by timestamp desc"#,
        uid
    )
    .fetch_all(ctx.data().db.as_ref())
    .await?
    .into_iter()
    .map(|v| LogDate {
        timestamp: v.timestamp.date(),
        comment:v.comment,
        count:v.count,
        name:v.name,
        time:v.time,
    }).collect::<Vec<_>>();

    let mut v = vec![];
    {
        let mut wtr = csv::Writer::from_writer(&mut v);

        for log in logs {
            wtr.serialize(log)?;
        }

        wtr.flush()?;
    }

    let mut s = String::new();
    s += "```csv\n";
    s += str::from_utf8(&v).unwrap();
    s += "```\n";

    ctx.say(s).await?;
    Ok(())
}
