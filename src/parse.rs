use futures_util::StreamExt;
use linkify::{LinkFinder, LinkKind};
use log::{info, warn};
use roux::{subreddit::responses::SubredditCommentsData, Me, Subreddit};
use roux_stream::stream_comments;
use sqlx::{Pool, Sqlite};
use std::time::Duration;
use tokio_retry::strategy::ExponentialBackoff;
use url::Url;

use crate::{
    aur::fetch_pkgdata_from_aur,
    client::{get_client, reply_to_comment},
    db::{db_check_comment_exists, db_establish_connection},
};

pub async fn stream_latest_comments(subreddit: &Subreddit) {
    info!("Streaming comments for {:?}", subreddit.name);

    let db_con = db_establish_connection().await;
    let client = get_client().await.unwrap();
    let retry_strategy = ExponentialBackoff::from_millis(5).factor(100).take(3);

    let (mut stream, join_handle) = stream_comments(
        &subreddit,
        Duration::from_secs(10),
        retry_strategy,
        Some(Duration::from_secs(10)),
    );

    while let Some(comment) = stream.next().await {
        let comment = comment.unwrap();
        handle_single_comment(&client, &comment, &db_con).await
    }

    join_handle.await.unwrap().unwrap();
}

async fn handle_single_comment(
    client: &Me,
    comment: &SubredditCommentsData,
    db_con: &Pool<Sqlite>,
) {
    if check_is_valid_comment(comment, db_con).await {
        let aur_urls = get_aur_urls_from_comment(&comment).await;

        if aur_urls.is_some() {
            let packages = get_pkg_names_from_aur_urls(&aur_urls.unwrap()).await;

            if packages.is_some() {
                for p in packages.unwrap() {
                    let response = fetch_pkgdata_from_aur(p).await;

                    if response.is_ok() {
                        let comment_id = &comment.name.as_ref().unwrap().as_str();

                        reply_to_comment(client, comment_id, response.unwrap(), &db_con).await;
                    }
                }
            }
        }
    }
}

async fn get_aur_urls_from_comment(comment: &SubredditCommentsData) -> Option<Vec<Url>> {
    for c in comment.body.iter() {
        let found_aur_urls: Vec<Url> = LinkFinder::new()
            .url_must_have_scheme(false)
            .kinds(&[LinkKind::Url])
            .links(&c)
            .filter(|l| l.as_str().contains("aur.archlinux.org/packages"))
            .flat_map(|l| Url::parse(l.as_str()))
            .collect();

        if !found_aur_urls.is_empty() {
            info!(
                "Found AUR-Urls in comment {}",
                comment.name.as_ref().unwrap().to_string()
            );
            return Some(found_aur_urls);
        }
    }
    warn!(
        "Did not find AUR-Urls in comment {}",
        comment.name.as_ref().unwrap().to_string()
    );
    None
}

async fn get_pkg_names_from_aur_urls(aur_urls: &Vec<Url>) -> Option<Vec<String>> {
    let mut packages: Vec<String> = Vec::new();
    for url in aur_urls {
        packages.push(get_pkg_name_from_aur_url(url).await)
    }
    if !packages.is_empty() {
        return Some(packages);
    }

    None
}

async fn get_pkg_name_from_aur_url(url: &Url) -> String {
    let package_name = url.to_string().split('/').last().unwrap().to_string();
    info!("Extracted package name is {}", package_name);
    package_name
}

async fn check_is_valid_comment(comment: &SubredditCommentsData, db_con: &Pool<Sqlite>) -> bool {
    let author = comment.author.as_ref().unwrap();
    let comment_id = comment.name.as_ref().unwrap();

    if author.eq(dotenv::var("BOT_USERNAME").unwrap().as_str()) {
        return false;
    }

    if db_check_comment_exists(db_con, &comment_id).await {
        return false;
    }

    info!(
        "Comment {} is valid",
        comment.name.as_ref().unwrap().to_string()
    );

    true
}
