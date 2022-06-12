use dotenv;
use log::info;
use roux::{util::RouxError, Me, Reddit};

use crate::{aur::AurApiResRoot, db::db_insert_comment};

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

pub async fn reply_to_comment(client: &Me, parent_id: &str, response: AurApiResRoot) {
    let message = generate_comment_string(response).await;
    let res = client.comment(message.as_str(), parent_id).await;

    if res.as_ref().unwrap().status().is_success() {
        db_insert_comment(&parent_id)
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
        "_out-of-date_".to_string()
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
        format!("Last Modified|{}", pkgdata.last_modified).to_string(),
    ]
    .join("\n");

    let final_string = [aur_link, out_of_date, description, pkgdata_table].join("\n\n");

    return final_string;
}
