// cargo run --example main
use rs_sqlite::{
    db::SQLite,
    store::Store,
};

fn main() {
    let mut db = SQLite::new()
        .in_memory();

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
        let insert_person = "INSERT INTO person (first_name, last_name, age, cof, data) VALUES (?,?,?,?,?)";
        let rowid = db.insert(insert_person,
                              Store::with_capacity(6)
                                  .add("Ahsoka")
                                  .add("Tao")
                                  .add(102)
                                  .add(3.1415)
                                  .add(vec![1u8, 2, 255, 5, 170]));
        println!("rowid of added row: {:?}", rowid);
        print_content("after Ahsoka added", &mut db);

        let update_person = "UPDATE person SET first_name=?, last_name=?, age=?, cof=?, data=? WHERE id=?";
        let stat =  db.update(update_person,
                              Store::with_capacity(6)
                                  .add("Luke")
                                  .add("Skywalker")
                                  .add(102)
                                  .add(3.1415)
                                  .add(vec![4u8, 5, 6])
                                  .add(rowid));
        print_content("after change Ahsoka => Luke", &mut db);

        let rowid = db.insert(insert_person,
                              Store::with_capacity(6)
                                  .add("Dart")
                                  .add("Vader")
                                  .add(102)
                                  .add(3.1415)
                                  .add(vec![0x1u8, 0x2, 0xff, 0x5, 0xaa]));
        print_content("after Dart Vader added", &mut db);
    }
}

fn print_content(title: &str, db: &mut SQLite) {
    println!("\nContent {:?}", title);
    println!("---------------------------------------------");

    let retv = db.select("SELECT * FROM person", Store::new());
    if let Some(retv) = retv {
        for (id, row) in retv.iter().enumerate() {
            println!("[Row {}]", id + 1);
            println!("{:?}", row);
        }
    }
}
