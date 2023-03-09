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
        let desc = format!("rowid of added row: {:?}", rowid);
        print_content("after Ahsoka added", &desc,  &mut db);

        let update_person = "UPDATE person SET first_name=?, last_name=?, age=?, cof=?, data=? WHERE id=?";
        let stat =  db.update(update_person,
                              Store::with_capacity(6)
                                  .add("Luke")
                                  .add("Skywalker")
                                  .add(102)
                                  .add(3.1415)
                                  .add(vec![4u8, 5, 6])
                                  .add(rowid));
        let desc = format!("update status: {:?}", stat);
        print_content("after change Ahsoka => Luke", &desc,  &mut db);

        let rowid = db.insert(insert_person,
                              Store::new()
                                  .add("Dart")
                                  .add("Vader")
                                  .add(102)
                                  .add(6.625)
                                  .add(vec![100u8, 200]));
        let desc = format!("rowid of added row: {:?}", rowid);
        print_content("after Dart Vader added", &desc,  &mut db);

        let select_query = "SELECT * FROM person WHERE id=?";
        let retv = db.select(select_query,
                            Store::with_capacity(1)
                                .add(1)).unwrap();
        println!("Select one\n{:?}", retv);
        let row = &retv[0];

        let id = row["id"].as_ref().unwrap();
        let tid: i64 = id.into();

        let cof = row["cof"].as_ref().unwrap();
        let tcof: f64 = cof.into();
        println!("id => {}, cof => {}", tid, tcof);
    }
}

fn print_content(title: &str, desc: &str, db: &mut SQLite) {
    println!("---------------------------------------------");
    println!("Content {}\n{}\n", title, desc);

    let retv = db.select("SELECT * FROM person", Store::new());
    if let Some(retv) = retv {
        for (id, row) in retv.iter().enumerate() {
            println!("[Row {}]", id + 1);
            println!("{:?}\n", row);
        }
    }
}
