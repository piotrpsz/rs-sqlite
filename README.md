# rs-sqlite
Crate rs-sqlite allows you to use the sqlite3 library in rust.<br>
First of all, you must have the sqlite3 library available on your hard drive

# How to use
We create a handle to a database object by calling <b>SQLite::new()</b>.<br>

### Create database in-memory
You can create a database in your computer's memory:<br>
```asciidoc
    let mut db = SQLite::new()
        .in_memory();
```

### Create database on disk
You can also create it on your computer's hard drive:
```asciidoc
    let mut db = SQLite::new()
        .file("/Users/piotr/example.sqlite");
```

### Defining the table and its structure
The database created in this way is completely empty.<br>
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

### Create a table in the database

And now we can create the table defined in this way:
```asciidoc
    db.create(vec![create_person])
```
Note that the parameter to the <b>create(..)</b> function is a vector of strings.<br>
That is, by passing more proper strings, you can create more tables with one function call.<br>

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
                              Store::with_capacity(6)
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
A more complete example can be found in the <b>example</b> directory<br>
in GitHub project's repository https://github.com/piotrpsz/rs-sqlite/tree/main/examples