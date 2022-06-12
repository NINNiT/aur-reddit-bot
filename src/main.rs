use crate::{client::get_client, parse::stream_latest_comments};

mod aur;
mod client;
mod parse;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // let client = get_client().await.unwrap();

    // client
    //     .submit_text("YOYO", "TEST", "krangler_bot_test")
    //     .await
    //     .unwrap();

    stream_latest_comments().await;
}
