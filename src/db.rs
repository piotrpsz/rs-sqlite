#![allow(clippy::not_unsafe_ptr_arg_deref)]
/*
 * Copyright (C) 2023 Piotr Pszczółkowski
 * Licence: GNU v2
 *
 * E-mail: piotr@beesoft.pl
 *
 * Project: rs-sqlite
 * File: db.rs
 */
extern crate sqlite3_sys;

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs;
use std::ptr::null_mut;

use fxhash::hash32;
use sqlite3_sys::{sqlite3,
                  sqlite3_stmt,
                  sqlite3_close_v2,
                  sqlite3_errcode,
                  sqlite3_errmsg,
                  sqlite3_exec,
                  sqlite3_initialize,
                  sqlite3_last_insert_rowid,
                  sqlite3_libversion,
                  sqlite3_libversion_number,
                  sqlite3_open_v2,
                  sqlite3_shutdown,
                  SQLITE_DONE,
                  SQLITE_OK,
                  SQLITE_OPEN_CREATE,
                  SQLITE_OPEN_READONLY,
                  SQLITE_OPEN_READWRITE};

use crate::stmt::Statement;
use crate::store::Store;
use crate::types::*;

static IN_MEMORY: &str = ":memory:";
const DB_NULL: *mut sqlite3 = null_mut();

include!("macros.inc");


/// Object handled connection with SQLite database
/// (via c-library sqlite3).

pub struct SQLite {
    db: *mut sqlite3,
    fpath: String,
    prepared: HashMap<u32, *mut sqlite3_stmt>,
    use_prepared: bool,
}

impl SQLite {
    /// Inits handler object.
    pub fn new() -> SQLite {
        println!("sqlite3: {}", SQLite::version());
        unsafe {
            match sqlite3_initialize() {
                SQLITE_OK => SQLite::default(),
                _ => panic!("can't init sqlite database engine")
            }
        }
    }

    /**** reause_prepared ******************************************/

    /// Force prepared statemt reuse.
    pub fn reuse_prepared(mut self) -> Self {
        self.use_prepared = true;
        self
    }

    /**** file *****************************************************/

    /// Sets path to database file.
    pub fn file(mut self, fpath: &str) -> Self {
        self.fpath = fpath.into();
        self
    }

    /**** in_memory ************************************************/

    /// Sets database in memory.
    pub fn in_memory(mut self) -> Self {
        self.fpath = IN_MEMORY.into();
        self
    }

    /**** close ****************************************************/

    /// Closes database.
    pub fn close(&mut self) -> bool {
        match self.db {
            DB_NULL => true,
            _ => {
                match unsafe { sqlite3_close_v2(self.db) } {
                    SQLITE_OK => {
                        self.db = null_mut();
                        true
                    }
                    _ => {
                        sql_error!(self.db);
                        false
                    }
                }
            }
        }
    }

    /**** err_string ***********************************************/

    /// Returns last error description.
    /// @ Safety
    /// here row pointer is passed
    pub fn err_string(db: *mut sqlite3) -> String {
        unsafe {
            let cptr = sqlite3_errmsg(db);
            String::from_utf8_lossy(CStr::from_ptr(cptr).to_bytes()).into_owned()
        }
    }
    pub fn error_string(&self) -> String {
        unsafe {
            let cptr = sqlite3_errmsg(self.db);
            String::from_utf8_lossy(CStr::from_ptr(cptr).to_bytes()).into_owned()
        }
    }


    /**** err_code *************************************************/

    /// Returns last error code.
    /// # Safety
    /// here raw pointer is passed
    pub fn err_code(db: *mut sqlite3) -> i32 {
        unsafe { sqlite3_errcode(db) }
    }
    pub fn error_code(&self) -> i32 {
        unsafe { sqlite3_errcode(self.db) }
    }


    /**** open *****************************************************/

    /// Opens a database existed already on disk.
    pub fn open(&mut self, read_only: bool) -> bool {
        // can't open a database when is alreadey opened
        if self.db != DB_NULL {
            eprintln!("database already opened");
            return false;
        }

        let flags = match read_only {
            false => SQLITE_OPEN_READONLY,
            true => SQLITE_OPEN_READWRITE,
        };

        unsafe {
            let stat = sqlite3_open_v2(
                str2ptr!(self.fpath.clone()),
                &mut self.db,
                flags,
                std::ptr::null());
            match stat {
                SQLITE_OK => true,
                _ => {
                    sql_error!(self.db);
                    false
                }
            }
        }
    }

    /**** create ***************************************************/

    /// Creates and inits a database.
    pub fn create(&mut self, cmd: Vec<&str>) -> bool {
        if self.db != DB_NULL {
            eprintln!("database already opened");
            return false;
        }

        // Remove database file if on disk.
        if self.fpath != IN_MEMORY {
            if let Err(err) = fs::remove_file(&self.fpath) {
                match err.kind() {
                    std::io::ErrorKind::NotFound => (),
                    _ => {
                        eprintln!("{}", err);
                        return false;
                    }
                }
            };
        }
        unsafe {
            let stat = sqlite3_open_v2(
                str2ptr!(self.fpath.clone()),
                &mut self.db,
                SQLITE_OPEN_CREATE | SQLITE_OPEN_READWRITE,
                std::ptr::null());
            match stat {
                SQLITE_OK => {
                    // Execute initial queries (create tables for example)
                    for query in cmd {
                        if !self.exec(query) {
                            return false;
                        }
                    }
                    true
                }
                _ => {
                    sql_error!(self.db);
                    false
                }
            }
        }
    }

    /**** exec *****************************************************/

    /// Executes a query without parameters
    pub fn exec(&mut self, query: &str) -> bool {
        if self.db == DB_NULL {
            eprintln!("database is not opened");
            return false;
        }
        unsafe {
            let stat = sqlite3_exec(
                self.db,
                str2ptr!(query),
                None,
                std::ptr::null_mut(),
                std::ptr::null_mut());
            match stat {
                SQLITE_OK => true,
                _ => {
                    sql_error!(self.db);
                    false
                }
            }
        }
    }

    /**** exec_query ***********************************************/

    /// Executes a query with passed arguments.
    pub fn exec_query(&mut self, query: &str, args: Store) -> bool {
        if self.db == DB_NULL {
            eprintln!("database is not opened");
            return false;
        }

        if let Some(mut stmt) = self.stmt_for_query(query) {
            if stmt.bind(args) && SQLITE_DONE == stmt.step() {
                return true;
            }
        }
        sql_error!(self.db);
        false
    }

    /**** insert ***************************************************/

    /// Executes INSERT command with arguments
    /// and returns 'rowid' of inserted row.
    pub fn insert(&mut self, query: &str, args: Store) -> i64 {
        match self.exec_query(query, args) {
            true => self.last_inserted_id(),
            _ => 0
        }
    }

    /**** select ***************************************************/

    /// Executes SELECT command with argumets
    /// and returns fetched rows.
    pub fn select(&mut self, query: &str, args: Store) -> Option<Vec<Row>> {
        if DB_NULL == self.db {
            eprintln!("database is not opened");
            return None;
        }

        if let Some(mut stmt) = self.stmt_for_query(query) {
            if stmt.bind(args) {
                return stmt.fetch_result();
            }
        }
        sql_error!(self.db);
        None
    }

    /**** update ***************************************************/

    /// Executes UPDATE command with arguments.
    pub fn update(&mut self, query: &str, args: Store) -> bool {
        self.exec_query(query, args)
    }

    /**** last_inserted_id *****************************************/

    /// Returns last inserted 'rowid'
    fn last_inserted_id(&self) -> i64 {
        unsafe { sqlite3_last_insert_rowid(self.db) }
    }

    /**** version_number *******************************************/

    /// Returns library version as a number.
    pub fn version_number() -> i32 {
        unsafe {
            sqlite3_libversion_number()
        }
    }

    /**** version **************************************************/

    /// Returns library version as text.
    pub fn version() -> String {
        unsafe {
            CStr::from_ptr(sqlite3_libversion()).to_string_lossy().into_owned()
        }
    }

    /**** stmt_for_query *******************************************/

    /// Creates or looking for statement.
    fn stmt_for_query(&mut self, query: &str) -> Option<Statement> {
        match self.use_prepared {
            true => {
                let query_hash = hash32(query);
                match self.prepared.get(&query_hash) {
                    Some(stmt) => {
                        println!("found previously prepared statement [{} - {}]", query_hash, query);
                        Some(Statement::for_stmt(self.db, *stmt))
                    },
                    _ => {
                        eprintln!("new statement preparation [{}]", query);
                        if let Some(statement) = Statement::for_query(self.db, query) {
                            println!("hash of prepared statemnt {}", query_hash);
                            self.prepared.insert(query_hash, statement.stmt);
                            return Some(statement);
                        }
                        None
                    }
                }
            }
            // every time statements should be prepared
            _ => Statement::for_query(self.db, query),
        }
    }
}

/********************************************************************
*                                                                   *
*                   D e f a u l t   T r a i t                       *
*                                                                   *
********************************************************************/

impl Default for SQLite {
    fn default() -> Self {
        SQLite {
            db: null_mut(),
            fpath: "".into(),
            prepared: HashMap::new(),
            use_prepared: false,
        }
    }
}

/********************************************************************
*                                                                   *
*                       D r o p   T r a i t                         *
*                                                                   *
********************************************************************/

impl Drop for SQLite {
    fn drop(&mut self) {
        unsafe {
            if self.close() && SQLITE_OK == sqlite3_shutdown() {
                return;
            }
            sql_error!(self.db);
        }
    }
}
