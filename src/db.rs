use log::info;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqliteConnection};
use std::str::FromStr;

pub async fn db_establish_connection() -> SqliteConnection {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("Connecting to sqlite database at {}", database_url);

    SqliteConnectOptions::from_str(&database_url)
        .unwrap()
        .create_if_missing(true)
        .connect()
        .await
        .unwrap()
}

pub async fn db_upsert_table(con: &mut SqliteConnection) {
    info!("Creating comments table");

    sqlx::query("CREATE TABLE IF NOT EXISTS comments ( id STRING PRIMARY KEY );")
        .execute(con)
        .await
        .unwrap();
}

pub async fn db_insert_comment(con: &mut SqliteConnection, comment_id: &str) {
    info!("Inserting id for comment {}", comment_id);

    let query = "INSERT INTO comments values (?)";

    sqlx::query(query)
        .bind(comment_id)
        .execute(con)
        .await
        .unwrap();
}

pub async fn db_check_comment_exists(con: &mut SqliteConnection, comment_id: &str) -> bool {
    info!(
        "Checking if comment already handled for comment {}",
        comment_id
    );

    let query = "SELECT COUNT(*) FROM comments where id = ?";

    let count: (i32,) = sqlx::query_as(query)
        .bind(comment_id)
        .fetch_one(con)
        .await
        .unwrap();

    if count.0 > 0 {
        info!("Already handled comment {}", comment_id);
        return true;
    } else {
        info!("New comment {}", comment_id);
        return false;
    }
}
