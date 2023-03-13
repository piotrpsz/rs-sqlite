# rs-sqlite
Crate rs-sqlite allows you to use the sqlite3 library in rust.<br>
First of all, you must have the sqlite3 library available on your hard drive.<br>
For greater efficiency, the crate implements the reuse of prepared already statements.

# How to use
## Add crate to project
Add this line in Cargo.toml to dependecis:
```asciidoc
rs-sqlite = { git = "https://github.com/piotrpsz/rs-sqlite", version="*" }
```
or add new section:
```asciidoc
[dependencies.rs-sqlite]
git = "https://github.com/piotrpsz/rs-sqlite"
version="*"
```
in source file add:
```asciidoc
use rs_sqlite::{
    db::SQLite,
    store::Store,
};
```

## Implementation
We create a handle to a database object by calling <b>SQLite::new()</b>.<br>

### Database in-memory
You can create an SQLite object which the database will store in your computer's memory:<br>
```asciidoc
let mut db = SQLite::new()
    .in_memory();
```

### Database on disk
You can create an SQLite object which the database will store on  your computer's hard drive:
```asciidoc
let mut db = SQLite::new()
    .file("/Users/piotr/example.sqlite");
```

### Defining the table and its structure
Suppose we want to create a table named <b>person</b>.<br>
We create a string containing the command to create the table with information about its columns.
```asciidoc
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
```

### Create a database with table(s)
The moment has come when we want to create a database.<br>
We create the database and table(s) in the following way:
```asciidoc
db.create(vec![create_person])
```
Note that the parameter passed to <b>create(..)</b> method is a vector of strings.<br>
That is, by passing more proper strings, you can create more tables when the database is created.<br>

### Insert a row

```asciidoc
let insert_person = "INSERT INTO person (first_name, last_name, age, cof, data) VALUES (?,?,?,?,?)";
let id = db.insert(insert_person,
                   Store::with_capacity(6)
                   .add("Ahsoka")
                   .add("Tao")
                   .add(102)
                   .add(3.1415)
                   .add(vec![1u8, 2, 255, 5, 170]));
```

### Update the row
```asciidoc
let update_person = "UPDATE person SET first_name=?, last_name=?, age=?, cof=?, data=? WHERE id=?";
let stat =  db.update(update_person,
                      Store::new()
                      .add("Luke")
                      .add("Skywalker")
                      .add(102)
                      .add(3.1415)
                      .add(vec![4u8, 5, 6])
                      .add(id));
```

### Displaying data from the table

```asciidoc
let retv = db.select("SELECT * FROM person", Store::new());

if let Some(retv) = retv {
    for (id, row) in retv.iter().enumerate() {
        println!("[Row {}]", id + 1);
        println!("{:?}\n", row);
    }
}
```

### Complete example
A more complete example can be found in the <b>example</b> directory in GtiHub<br>
project's repository:
#### https://github.com/piotrpsz/rs-sqlite/tree/main/examples