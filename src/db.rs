use sqlite::{self, Connection};

pub fn db_establish_connection() -> Connection {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Connection::open(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn db_upsert_table() {
    let con = db_establish_connection();
    con.execute("CREATE TABLE IF NOT EXISTS comments ( id STRING PRIMARY KEY );")
        .unwrap();
}

pub fn db_insert_comment(comment_id: &str) {
    let con = db_establish_connection();

    con.execute(format!("insert into comments values {};", comment_id))
        .unwrap();
}

pub fn db_check_comment_exists(comment_id: &str) -> bool {
    let con = db_establish_connection();
    let res = con.iterate(
        format!("select count(*) from comments where id = {}", comment_id),
        |pairs| {
            for &(column, value) in pairs.iter() {
                if value.unwrap().as_ > 0 {
                    return true;
                }
            }
            if result.firsunwrap() > 0 {
                return true;
            } else {
                return false;
            }
        },
    );

    false
}
