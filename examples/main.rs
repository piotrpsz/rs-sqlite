// cargo run --example main
use rs_sqlite::db::SQLite;

fn main() {
    let mut db = SQLite::new().in_memory();
    println!("sqlite3: {}", SQLite::version());

    let create_person = r#"
    CREATE TABLE person (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        first_name TEXT COLLATE NOCASE,
        second_name TEXT COLLATE NOCASE,
        last_name TEXT  COLLATE NOCASE,
        age INTEGER,
        cof DOUBLE,
        data BLOB
    )"#;

    if db.create(vec![create_person]) {

    }
}
