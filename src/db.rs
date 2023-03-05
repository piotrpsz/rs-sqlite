extern crate sqlite3_sys;

use std::ffi::{CStr, CString};
use std::fs;
use std::ptr::{null_mut};


use sqlite3_sys::{sqlite3,
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
                  SQLITE_OPEN_READWRITE};
use crate::hash;

use crate::store::Store;
use crate::stmt::Stmt;
use crate::types::*;

static IN_MEMORY: &str = ":memory:";
const DB_NULL: *mut sqlite3 = null_mut();

include!("macros.inc");


/// Object handled connection with SQLite database
/// (via c-library sqlite3).

pub struct SQLite {
    db: *mut sqlite3,
    fpath: String,
}

impl SQLite {
    /// Inits database.
    pub fn new() -> SQLite {
        println!("sqlite3: {}", SQLite::version());
        unsafe {
            match sqlite3_initialize() {
                SQLITE_OK => SQLite::default(),
                _ => panic!("can't init sqlite database engine")
            }
        }
    }
    /// Sets path to database file.
    pub fn file(mut self, fpath: &str) -> Self {
        self.fpath = fpath.clone().into();
        self
    }
    /// Sets database in memory.
    pub fn in_memory(mut self) -> Self {
        self.fpath = IN_MEMORY.into();
        self
    }
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
                        sql_error!(self);
                        false
                    }
                }
            }
        }
    }
    /// Returns last error description.
    pub fn err_string(&self) -> String {
        unsafe {
            let cptr = sqlite3_errmsg(self.db);
            String::from_utf8_lossy(CStr::from_ptr(cptr).to_bytes()).into_owned()
        }
    }
    /// Returns last error code.
    pub fn err_code(&self) -> i32 {
        unsafe { sqlite3_errcode(self.db) as i32 }
    }
    /// Creates and inits a database.
    pub fn create(&mut self, cmd: Vec<&str>) -> bool {
        if self.db != DB_NULL {
            eprintln!("database already opened");
            return false;
        }

        // Remove database file if on disk.
        if self.fpath != IN_MEMORY {
            match fs::remove_file(&self.fpath) {
                Err(err) => {
                    match err.kind() {
                        std::io::ErrorKind::NotFound => (),
                        _ => {
                            eprintln!("{}", err.to_string());
                            return false;
                        }
                    }
                }
                _ => ()
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
                            sql_error!(self);
                            return false;
                        }
                    }
                    true
                }
                _ => {
                    sql_error!(self);
                    false
                }
            }
        }
    }
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
                    sql_error!(self);
                    false
                }
            }
        }
    }
    /// Executes a query with passed arguments.
    pub fn exec_query(&mut self, query: &str, args: Store) -> bool {
        if self.db == DB_NULL {
            eprintln!("database is not opened");
            return false;
        }

        let mut stmt = Stmt::new();
        if stmt.prepare(self.db, query) && stmt.bind(args) {
            if SQLITE_DONE == stmt.step() {
                stmt.finalize();
                return true;
            }
        }
        sql_error!(self);
        stmt.finalize();
        false
    }
    /// Executes INSERT command with arguments
    /// and returns 'rowid' of inserted row.
    pub fn insert(&mut self, query: &str, args: Store) -> i64 {
        match self.exec_query(query, args) {
            true => self.last_inserted_id(),
            _ => 0
        }
    }
    /// Executes SELECT command with argumets
    /// and returns fetched rows.
    pub fn select(&mut self, query: &str, args: Store) -> Option<Vec<Row>> {
        if self.db == DB_NULL {
            eprintln!("database is not opened");
            return None;
        }

        let mut stmt = Stmt::new();
        if stmt.prepare(self.db, query) && stmt.bind(args) {
            let result = stmt.fetch_result();
            if !result.is_empty() {
                return Some(result);
            }
            return None;
        }
        sql_error!(self);
        stmt.finalize();
        None
    }
    /// Executes UPDATE command with arguments.
    pub fn update(&mut self, query: &str, args: Store) -> bool {
        self.exec_query(query, args)
    }
    /// Returns last inserted 'rowid'
    fn last_inserted_id(&self) -> i64 {
        unsafe { sqlite3_last_insert_rowid(self.db) as i64 }
    }
    /// Returns library version as a number.
    pub fn version_number() -> i32 {
        unsafe { sqlite3_libversion_number() as i32 }
    }
    /// Returns library version as text.
    pub fn version() -> String {
        unsafe {
            CStr::from_ptr(sqlite3_libversion()).to_string_lossy().into_owned()
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
        SQLite { db: null_mut(), fpath: "".into() }
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
            if self.close() {
                if SQLITE_OK == sqlite3_shutdown() {
                    return;
                }
            }
            sql_error!(self);
        }
    }
}
