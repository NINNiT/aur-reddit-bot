use futures_util::future::join_all;
use log::info;
use roux::Subreddit;

use crate::{
    db::{db_establish_connection, db_upsert_table},
    parse::stream_latest_comments,
};

// use crate::{db2::db_upsert_table, parse::stream_latest_comments};

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
    db_upsert_table(&db_establish_connection().await).await;

    // list of subreddits that comments should be streamed from
    let subreddits: Vec<Subreddit> = vec![Subreddit::new("krangler_bot_test")];

    // map subreddits to async function tasks
    let functions: Vec<_> = subreddits
        .iter()
        .map(|sr| stream_latest_comments(&sr))
        .collect();

    // wait for all (in this case, indefintely)
    join_all(functions).await;
}
