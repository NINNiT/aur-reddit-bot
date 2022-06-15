use crate::{
    db::{db_establish_connection, db_upsert_table},
    parse::stream_latest_comments,
};
use futures_util::future::join_all;
use log::info;
use roux::Subreddit;

mod aur;
mod client;
mod db;
mod parse;

#[tokio::main]
async fn main() {
    // initialize logging
    pretty_env_logger::init();
    info!("Started aur-reddit-bot");

    // create db table if not already existing
    db_upsert_table(&mut db_establish_connection().await).await;

    let subreddits: Vec<Subreddit> = dotenv::var("SUBREDDITS")
        .unwrap()
        .split(",")
        .map(|s| Subreddit::new(s))
        .collect();

    // map subreddits to async function tasks
    let functions: Vec<_> = subreddits
        .iter()
        .map(|sr| stream_latest_comments(&sr))
        .collect();

    // wait for all (in this case, indefintely)
    join_all(functions).await;
}
