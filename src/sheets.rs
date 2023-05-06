use crate::utils::{fmt_duration, get_xelieu_tab_name};
use crate::{Context, Error};
use anyhow::anyhow;
use chrono::{Duration, NaiveDateTime};
use google_sheets4::api::ValueRange;

use google_sheets4::oauth2::authenticator_delegate::InstalledFlowDelegate;
use google_sheets4::oauth2::storage::{TokenInfo, TokenStorage};
use google_sheets4::oauth2::{self, InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use google_sheets4::{self, hyper, hyper_rustls, Sheets};
use hyper::Client;
use poise::serenity_prelude::{self as serenity, ChannelId};
use serde_json::json;
use sqlx::types::time::OffsetDateTime;
use sqlx::{query, query_as, SqlitePool};
use std::sync::Arc;

//const REDIRECT_URL: &str = "https://tolocalhost.com";
const REDIRECT_URL: &str = "https://mx.xy2.dev";

struct MyFlowDelegate(Arc<serenity::Context>);
async fn present_user_url(
    url: &str,
    _need_code: bool,
    ctx: &serenity::Context,
) -> Result<String, String> {
    ChannelId(1090757758699192430)
        //ChannelId(1097207605744631901)
        .send_message(ctx, |m| {
            m.content(format!(
                "Please go to {url} and follow the instructions displayed there."
            ))
        })
        .await
        .unwrap();
    Ok(String::new())
}

impl InstalledFlowDelegate for MyFlowDelegate {
    fn redirect_uri(&self) -> Option<&str> {
        Some(REDIRECT_URL)
    }
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        need_code: bool,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send + 'a>>
    {
        Box::pin(present_user_url(url, need_code, &self.0))
    }
}

struct MyTokenStorage {
    uid: String,
    db: Arc<SqlitePool>,
}

#[poise::async_trait]
impl TokenStorage for MyTokenStorage {
    async fn set(&self, _scopes: &[&str], token: TokenInfo) -> anyhow::Result<()> {
        let Some(refresh_token) = token.refresh_token else {
           return Err(anyhow!("could not get refresh token"))
        };

        if refresh_token.is_empty() {
            return Err(anyhow!("refresh token was empty"));
        }

        query!(
            r#"insert into sheets_auth values($1, $2, $3, $4)
            on conflict do update set token=$3, expires_at=$4"#,
            self.uid,
            refresh_token,
            token.access_token,
            token.expires_at,
        )
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }

    /// Retrieve a token stored by set for the given set of scopes
    async fn get(&self, _scopes: &[&str]) -> Option<TokenInfo> {
        struct StoredToken {
            refresh_token: String,
            access_token: Option<String>,
            expires_at: Option<OffsetDateTime>,
        }

        let row = query_as!(
            StoredToken,
            "select refresh_token, token as access_token, expires_at as `expires_at: OffsetDateTime` from sheets_auth where uid=?",
            self.uid
        )
        .fetch_one(self.db.as_ref())
        .await
        .ok()?;

        Some(TokenInfo {
            access_token: row.access_token,
            refresh_token: Some(row.refresh_token),
            expires_at: row.expires_at,
            id_token: None,
        })
    }
}

pub async fn get_token(ctx: Context<'_>) -> Result<(), Error> {
    let secret = oauth2::read_application_secret("client_secret.json")
        .await
        .unwrap();

    let auth = InstalledFlowAuthenticator::builder(
        secret,
        InstalledFlowReturnMethod::HTTPPortRedirect(8080),
    )
    .flow_delegate(Box::new(MyFlowDelegate(Arc::new(
        ctx.serenity_context().clone(),
    ))))
    .with_storage(Box::new(MyTokenStorage {
        uid: ctx.author().id.0.to_string(),
        db: ctx.data().db.clone(),
    }))
    .build()
    .await
    .unwrap();

    auth.token(&["https://www.googleapis.com/auth/spreadsheets"])
        .await?;

    Ok(())
}

pub struct SheetsLog {
    pub date: NaiveDateTime,
    pub name: Option<String>,
    pub chars: u32,
    pub time: Option<i64>,
}

pub async fn log_to_sheets(
    ctx: Context<'_>,
    spreadsheet_id: String,
    log: SheetsLog,
) -> Result<(), Error> {
    let secret = oauth2::read_application_secret("client_secret.json")
        .await
        .unwrap();

    let auth = InstalledFlowAuthenticator::builder(
        secret,
        InstalledFlowReturnMethod::HTTPPortRedirect(8080),
    )
    .flow_delegate(Box::new(MyFlowDelegate(Arc::new(
        ctx.serenity_context().clone(),
    ))))
    .with_storage(Box::new(MyTokenStorage {
        uid: ctx.author().id.0.to_string(),
        db: ctx.data().db.clone(),
    }))
    .build()
    .await
    .unwrap();

    let tab_name = get_xelieu_tab_name(log.date);
    let range = format!("'{tab_name}'!A1:G1");

    println!("range: {range}");

    let req = ValueRange {
        major_dimension: None,
        range: Some(range.clone()),
        values: Some(vec![vec![
            json!(format!("{}", log.date.format("%Y-%m-%d"))),
            json!(log.name),
            json!("Visual Novel"),
            json!(log.chars),
            json!(""),
            json!(""),
            json!(log.time.map(|v| {
                let d = Duration::minutes(v);
                format!("{}:{}:00", d.num_hours(), d.num_minutes() % 60)
            })),
        ]]),
    };

    println!("{req:?}");

    let hub = Sheets::new(
        Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        auth,
    );

    hub.spreadsheets()
        .values_append(req, &spreadsheet_id, &range)
        .value_input_option("USER_ENTERED")
        .doit()
        .await?;

    Ok(())
}
