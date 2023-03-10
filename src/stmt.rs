/*
 * Copyright (C) 2023 Piotr Pszczółkowski
 * Licence: GNU v2
 *
 * E-mail: piotr@beesoft.pl
 *
 * Project: rs-sqlite
 * File: stmt.rs
 */
#![allow(dead_code)]

extern crate sqlite3_sys;

use std::ffi::{c_void, CStr, CString};
use std::intrinsics::copy;
use std::mem::transmute;
use std::ptr::null_mut;

use libc::{c_char, c_double, c_int};
use sqlite3_sys::{sqlite3,
                  sqlite3_bind_blob,
                  sqlite3_bind_double,
                  sqlite3_bind_int64,
                  sqlite3_bind_null,
                  sqlite3_bind_parameter_index,
                  sqlite3_bind_text,
                  sqlite3_clear_bindings,
                  sqlite3_column_blob,
                  sqlite3_column_bytes,
                  sqlite3_column_count,
                  sqlite3_column_double,
                  sqlite3_column_int,
                  sqlite3_column_int64,
                  sqlite3_column_name,
                  sqlite3_column_text,
                  sqlite3_column_type,
                  sqlite3_finalize,
                  sqlite3_int64,
                  sqlite3_prepare_v2,
                  sqlite3_reset,
                  sqlite3_step,
                  sqlite3_stmt,
                  SQLITE_OK,
                  SQLITE_ROW};

use crate::store::Store;
use crate::types::{Row, Type};
use crate::value::Value;

use super::db::SQLite;

include!("macros.inc");

/// Stmt object to handle prepared statement
pub(crate) struct Statement {
    pub(crate) stmt: *mut sqlite3_stmt,
    db: *mut sqlite3,
}

impl Statement {
    /// Creates Statement object for passed 'query'.
    /// Function will prepare a stmt.
    pub(crate) fn for_query(db: *mut sqlite3, query: &str) -> Option<Statement> {
        let mut stmt = Statement { stmt: null_mut(), db };
        match stmt.prepare(query) {
            true => Some(stmt),
            _ => None,
        }
    }

    /// Create Statement object with 'db' and 'stmt'.
    /// It is nothing to do, 'stmt' is already prepared.
    pub(crate) fn for_stmt(db: *mut sqlite3, stmt: *mut sqlite3_stmt) -> Statement {
        Statement { stmt, db }
    }

    /**** prepare **************************************************/

    /// Prepare query
    fn prepare(&mut self, query: &str) -> bool {
        unsafe {
            SQLITE_OK == sqlite3_prepare_v2(
                self.db,
                str2ptr!(query),
                -1,
                &mut self.stmt,
                std::ptr::null_mut(),
            )
        }
    }

    /**** reset ****************************************************/

    /// Resets prepared query
    pub(crate) fn reset(&mut self) -> bool {
        unsafe {
            SQLITE_OK == (sqlite3_reset(self.stmt) & sqlite3_clear_bindings(self.stmt))
        }
    }

    /**** finalize *************************************************/

    /// Finalize prepared query
    pub(crate) fn finalize(&self) -> bool {
        unsafe {
            SQLITE_OK == sqlite3_finalize(self.stmt)
        }
    }

    /**** step *****************************************************/

    /// Step to next row in result
    pub(crate) fn step(&self) -> c_int {
        unsafe { sqlite3_step(self.stmt) }
    }

    /**** column_count *********************************************/

    /// Columns number in row in result
    pub(crate) fn column_count(&self) -> usize {
        unsafe { sqlite3_column_count(self.stmt) as usize }
    }

    /**** column_type **********************************************/

    /// Returns value type in column
    pub(crate) fn column_type(&self, idx: usize) -> Type {
        unsafe {
            let sqlite_type = sqlite3_column_type(self.stmt, idx as c_int);
            Type::vtype(sqlite_type as usize)
        }
    }

    /**** column_index *********************************************/

    /// Zwraca indeks kolumny o wskazanej nazwie
    pub(crate) fn column_index(&self, column_name: &str) -> i32 {
        unsafe {
            sqlite3_bind_parameter_index(self.stmt, str2ptr!(column_name))
        }
    }

    /**** column_name **********************************************/

    /// Zwraca nazwę kolumny o podanym indeksie
    pub(crate) fn column_name(&self, idx: usize) -> String {
        unsafe {
            let ptr = sqlite3_column_name(self.stmt, idx as c_int);
            String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned()
        }
    }

    /**** bind *****************************************************/

    /// Bindowanie przysłanych argumentów do spreparowanego 'stmt'
    pub(crate) fn bind(&mut self, args: Store) -> bool {
        for it in args.iter().enumerate() {
            if !self.bind_at_index(it.0 + 1, it.1) {
                return false;
            }
        }
        true
    }

    /**** bind_at_index ********************************************/

    /// Bindowanie podanej wartości na wskazanej pozycji
    fn bind_at_index(&self, idx: usize, v: &Value) -> bool {
        match v {
            Value::Null => self.bind_null(idx),
            Value::Int(x) => self.bind_i64(idx, *x),
            Value::Float(x) => self.bind_f64(idx, *x),
            Value::Text(x) => self.bind_str(idx, x),
            Value::Blob(x) => self.bind_blob(idx, x),
        }
    }

    /**** fetch_result *********************************************/

    /// Odczyt wszystkich wierszy z ostatnio wyknanego zapytania.
    ///
    /// # Returns
    /// wektor wierszy (być może pusty)
    pub(crate) fn fetch_result(&self) -> Option<Vec<Row>> {
        let column_count = self.column_count();
        let mut result = Vec::new();

        while SQLITE_ROW == self.step() {
            let row = self.fetch_row(column_count);
            if !row.is_empty() {
                result.push(row);
            }
        }

        match !result.is_empty() {
            true => Some(result),
            _ => None
        }
    }

    /**** fetch_row ************************************************/

    /// Zwraca wiersz z wyniku
    pub(crate) fn fetch_row(&self, n: usize) -> Row {
        let mut row = Row::with_capacity(n);

        for i in 0..n {
            let name = self.column_name(i);
            match self.column_type(i) {
                Type::Null => {
                    row.insert(name, None);
                }
                Type::Int64 => {
                    row.insert(name, Some(Value::from(self.fetch_i64(i))));
                }
                Type::Float64 => {
                    row.insert(name, Some(Value::from(self.fetch_f64(i))));
                }
                Type::Text => {
                    row.insert(name, Some(Value::from(self.fetch_str(i))));
                }
                Type::Blob => {
                    row.insert(name, Some(Value::from(self.fetch_blob(i))));
                }
            };
        }
        row
    }

    /*                       S E T T E R S                             */

    fn bind_i64(&self, idx: usize, v: i64) -> bool {
        unsafe {
            match sqlite3_bind_int64(self.stmt, idx as c_int, v as sqlite3_int64) {
                SQLITE_OK => true,
                _ => {
                    sql_error!(self.db);
                    false
                }
            }
        }
    }
    fn bind_f64(&self, idx: usize, v: f64) -> bool {
        unsafe {
            match sqlite3_bind_double(self.stmt, idx as c_int, v as c_double) {
                SQLITE_OK => true,
                _ => {
                    sql_error!(self.db);
                    false
                }
            }
        }
    }
    fn bind_str(&self, idx: usize, v: &str) -> bool {
        unsafe {
            let idx = idx as c_int;
            let ptr = v.as_ptr() as *const c_char;
            let nbytes = v.len() as c_int;

            match sqlite3_bind_text(self.stmt, idx, ptr, nbytes, transmute(!0 as *const c_void)) {
                SQLITE_OK => true,
                _ => {
                    sql_error!(self.db);
                    false
                }
            }
        }
    }
    fn bind_blob(&self, idx: usize, v: &Vec<u8>) -> bool {
        unsafe {
            let idx = idx as c_int;
            let ptr = v.as_ptr() as *const c_void;
            let nbytes = v.len() as c_int;

            match sqlite3_bind_blob(self.stmt, idx, ptr, nbytes, transmute(!0 as *const c_void)) {
                SQLITE_OK => true,
                _ => {
                    sql_error!(self.db);
                    false
                }
            }
        }
    }
    fn bind_null(&self, idx: usize) -> bool {
        unsafe {
            match sqlite3_bind_null(self.stmt, idx as c_int) {
                SQLITE_OK => true,
                _ => {
                    sql_error!(self.db);
                    false
                }
            }
        }
    }

    /*                       G E T T E R S                             */

    fn fetch_i64(&self, idx: usize) -> i64 {
        unsafe {
            sqlite3_column_int64(self.stmt, idx as c_int)
        }
    }
    #[inline]
    fn fetch_int(&self, idx: usize) -> isize {
        unsafe {
            sqlite3_column_int(self.stmt, idx as c_int) as isize
        }
    }
    #[inline]
    fn fetch_f64(&self, idx: usize) -> f64 {
        unsafe {
            sqlite3_column_double(self.stmt, idx as c_int)
        }
    }
    #[inline]
    fn fetch_str(&self, idx: usize) -> String {
        unsafe {
            // sql_error!(self.db);
            let ptr = sqlite3_column_text(self.stmt, idx as c_int);
            String::from_utf8_lossy(CStr::from_ptr(ptr as *const c_char).to_bytes()).into_owned()
        }
    }
    #[inline]
    fn fetch_blob(&self, idx: usize) -> Vec<u8> {
        unsafe {
            let nbytes = sqlite3_column_bytes(self.stmt, idx as c_int) as usize;
            let ptr = sqlite3_column_blob(self.stmt, idx as c_int);
            let mut data: Vec<u8> = vec![0; nbytes];
            copy(ptr as *mut c_void, data.as_mut_ptr() as *mut c_void, nbytes);
            data
        }
    }
}

impl Drop for Statement {
    fn drop(&mut self) {
        if !self.stmt.is_null() {
            self.reset();
            // self.finalize();
            self.stmt = null_mut();
        }
    }
}