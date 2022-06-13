use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use dotenv;
use log::info;
use roux::{util::RouxError, Me, Reddit};
use sqlx::{Pool, Sqlite};

use crate::{
    aur::AurApiResRoot,
    db::{self, db_insert_comment},
};

pub async fn get_client() -> Result<Me, RouxError> {
    let user_agent = "rust:aur-reddit-bot:v1 (/u/masterninni)";
    let client_id = dotenv::var("BOT_CLIENT_ID").unwrap();
    let client_secret = dotenv::var("BOT_CLIENT_SECRET").unwrap();
    let username = dotenv::var("BOT_USERNAME").unwrap();
    let password = dotenv::var("BOT_PASSWORD").unwrap();

    Reddit::new(user_agent, client_id.as_str(), client_secret.as_str())
        .username(username.as_str())
        .password(password.as_str())
        .login()
        .await
}

pub async fn reply_to_comment(
    client: &Me,
    parent_id: &str,
    response: AurApiResRoot,
    db_con: &Pool<Sqlite>,
) {
    let message = generate_comment_string(response).await;
    let res = client.comment(message.as_str(), parent_id).await;

    if res.as_ref().unwrap().status().is_success() {
        db_insert_comment(db_con, &parent_id).await;
    }

    info!(
        "Replied to comment {} with status {}",
        parent_id,
        res.as_ref().unwrap().status()
    );
}

pub async fn generate_comment_string(response: AurApiResRoot) -> String {
    let pkgdata = response.results.first().unwrap();

    let aur_link = format!(
        "[{}](https://aur.archlinux.org/packages/{})",
        pkgdata.name, pkgdata.name
    );

    let out_of_date = if pkgdata.out_of_date.is_some() {
        "__out-of-date__".to_string()
    } else {
        "".to_string()
    };

    let description = format!("{}", pkgdata.description);

    let pkgdata_table = [
        format!("Field|Value"),
        format!(":--|:--"),
        format!("Version|{}", pkgdata.version),
        format!("Maintainer|{}", pkgdata.maintainer),
        format!("Votes|{}", pkgdata.num_votes),
        format!("Popularity|{}", pkgdata.popularity),
        format!("License|{}", pkgdata.license.join(" ").to_string()),
        format!(
            "Last Modified|{}",
            unix_time_to_datetime(pkgdata.last_modified).await
        )
        .to_string(),
    ]
    .join("\n");

    let footer_line = format!("---");
    let footer_content = format!("^(This bot has been written by masterninni using Rust, primarily as an exercise, and is licensed under GPL-3. Contributions are welcome and the git repo can be found at [GitHub](https://github.com/NINNiT/aur-reddit-bot). Please DM me if there are any problems!)");

    let footer = [footer_line, footer_content].join("\n");

    let final_string = [aur_link, out_of_date, description, pkgdata_table, footer].join("\n\n");

    return final_string;
}

pub async fn unix_time_to_datetime(unix_time: i64) -> String {
    // Creates a new SystemTime from the specified number of whole seconds
    let d = UNIX_EPOCH + Duration::from_secs(unix_time.try_into().unwrap());
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M").to_string();
    timestamp_str
}
