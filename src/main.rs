use futures_util::future::join_all;
use log::info;
use roux::Subreddit;

use crate::parse::stream_latest_comments;

mod aur;
mod client;
mod parse;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Started aur-reddit-bot");

    let subreddits: Vec<Subreddit> = vec![Subreddit::new("krangler_bot_test")];
    let functions: Vec<_> = subreddits
        .iter()
        .map(|sr| stream_latest_comments(&sr))
        .collect();

    // stream_latest_comments(&Subreddit::new("krangler_bot_test")).await;

    join_all(functions).await;
}
