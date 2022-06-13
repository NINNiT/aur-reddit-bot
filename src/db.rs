use log::info;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite, SqlitePool};

pub async fn db_establish_connection() -> Pool<Sqlite> {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("Connecting to sqlite database at {}", database_url);

    SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap()
}

pub async fn db_upsert_table(con: &Pool<Sqlite>) {
    info!("Creating comments table");

    sqlx::query("CREATE TABLE IF NOT EXISTS comments ( id STRING PRIMARY KEY );")
        .execute(con)
        .await
        .unwrap();
}

pub async fn db_insert_comment(con: &Pool<Sqlite>, comment_id: &str) {
    info!("Inserting id for comment {}", comment_id);

    let query = "INSERT INTO comments values (?)";

    sqlx::query(query)
        .bind(comment_id)
        .execute(con)
        .await
        .unwrap();
}

pub async fn db_check_comment_exists(con: &Pool<Sqlite>, comment_id: &str) -> bool {
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
