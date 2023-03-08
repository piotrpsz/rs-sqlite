#![allow(dead_code)]

extern crate sqlite3_sys;

use std::ffi::{c_void, CStr, CString};
use std::intrinsics::copy;
use std::mem::transmute;

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
use crate::types::{Row, Type, Value};

include!("macros.inc");

/// Stmt object to handle prepared statement
pub(crate) struct Stmt(*mut sqlite3_stmt);

impl Stmt {
    pub(crate) fn new() -> Stmt {
        Stmt(std::ptr::null_mut())
    }

    /**** prepare **************************************************/

    /// Prepare query
    pub(crate) fn prepare(&mut self, db: *mut sqlite3, query: &str) -> bool {
        unsafe {
            SQLITE_OK == sqlite3_prepare_v2(
                db,
                str2ptr!(query),
                -1,
                &mut self.0,
                std::ptr::null_mut(),
            )
        }
    }

    /**** reset ****************************************************/

    /// Resets prepared query
    pub(crate) fn reset(&mut self) -> bool {
        unsafe {
            SQLITE_OK == (sqlite3_reset(self.0) & sqlite3_clear_bindings(self.0))
        }
    }

    /**** finalize *************************************************/

    /// Finalize prepared query
    pub(crate) fn finalize(&mut self) -> bool {
        unsafe {
            SQLITE_OK == sqlite3_finalize(self.0)
        }
    }

    /**** step *****************************************************/

    /// Step to next row in result
    pub(crate) fn step(&mut self) -> c_int {
        unsafe { sqlite3_step(self.0) }
    }

    /**** column_count *********************************************/

    /// Columns number in row in result
    pub(crate) fn column_count(&self) -> usize {
        unsafe { sqlite3_column_count(self.0) as usize }
    }

    /**** column_type **********************************************/

    /// Returns value type in column
    pub(crate) fn column_type(&self, idx: usize) -> Type {
        unsafe {
            let sqlite_type = sqlite3_column_type(self.0, idx as c_int);
            Type::vtype(sqlite_type as usize)
        }
    }

    /**** column_index *********************************************/

    /// Zwraca indeks kolumny o wskazanej nazwie
    pub(crate) fn column_index(&self, column_name: &str) -> i32 {
        unsafe {
            sqlite3_bind_parameter_index(self.0, str2ptr!(column_name)) as i32
        }
    }

    /**** column_name **********************************************/

    /// Zwraca nazwę kolumny o podanym indeksie
    pub(crate) fn column_name(&self, idx: usize) -> String {
        unsafe {
            let ptr = sqlite3_column_name(self.0, idx as c_int);
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
    pub(crate) fn fetch_result(&mut self) -> Vec<Row> {
        let column_count = self.column_count();
        let mut result = Vec::new();

        while SQLITE_ROW == self.step() {
            let row = self.fetch_row(column_count);
            if !row.is_empty() {
                result.push(row);
            }
        }

        result
    }

    /**** fetch_row ************************************************/

    /// Zwraca wiersz z wyniku
    pub(crate) fn fetch_row(&self, n: usize) -> Row {
        let mut row = Row::with_capacity(n as usize);

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

    #[inline]
    fn bind_i64(&self, idx: usize, v: i64) -> bool {
        unsafe {
            SQLITE_OK == sqlite3_bind_int64(self.0, idx as c_int, v as sqlite3_int64)
        }
    }
    #[inline]
    fn bind_f64(&self, idx: usize, v: f64) -> bool {
        unsafe {
            SQLITE_OK == sqlite3_bind_double(self.0, idx as c_int, v as c_double)
        }
    }
    #[inline]
    fn bind_str(&self, idx: usize, v: &str) -> bool {
        unsafe {
            let idx = idx as c_int;
            let ptr = v.as_ptr() as *const c_char;
            let nbytes = v.len() as c_int;
            SQLITE_OK == sqlite3_bind_text(self.0, idx, ptr, nbytes, transmute(!0 as *const c_void))
        }
    }
    #[inline]
    fn bind_blob(&self, idx: usize, v: &Vec<u8>) -> bool {
        unsafe {
            let idx = idx as c_int;
            let ptr = v.as_ptr() as *const c_void;
            let nbytes = v.len() as c_int;
            SQLITE_OK == sqlite3_bind_blob(self.0, idx, ptr, nbytes, transmute(!0 as *const c_void))
        }
    }
    #[inline]
    fn bind_null(&self, idx: usize) -> bool {
        unsafe {
            SQLITE_OK == sqlite3_bind_null(self.0, idx as c_int)
        }
    }

    /*                       G E T T E R S                             */

    #[inline]
    fn fetch_i64(&self, idx: usize) -> i64 {
        unsafe {
            sqlite3_column_int64(self.0, idx as c_int) as i64
        }
    }
    #[inline]
    fn fetch_int(&self, idx: usize) -> isize {
        unsafe {
            sqlite3_column_int(self.0, idx as c_int) as isize
        }
    }
    #[inline]
    fn fetch_f64(&self, idx: usize) -> f64 {
        unsafe {
            sqlite3_column_double(self.0, idx as c_int) as f64
        }
    }
    #[inline]
    fn fetch_str(&self, idx: usize) -> String {
        unsafe {
            let ptr = sqlite3_column_text(self.0, idx as c_int);
            String::from_utf8_lossy(CStr::from_ptr(ptr as *const c_char).to_bytes()).into_owned()
        }
    }
    #[inline]
    fn fetch_blob(&self, idx: usize) -> Vec<u8> {
        unsafe {
            let nbytes = sqlite3_column_bytes(self.0, idx as c_int) as usize;
            let ptr = sqlite3_column_blob(self.0, idx as c_int);
            let mut data: Vec<u8> = vec![0; nbytes];
            copy(ptr as *mut c_void, data.as_mut_ptr() as *mut c_void, nbytes);
            data
        }
    }
}
