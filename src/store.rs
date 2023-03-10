/*
 * Copyright (C) 2023 Piotr Pszczółkowski
 * Licence: GNU v2
 *
 * E-mail: piotr@beesoft.pl
 *
 * Project: rs-sqlite
 * File: store.rs
 */
use core::slice::Iter;
use chrono::NaiveDateTime;
use crate::types::Timestamp;

use crate::value::{NullValue, Value};

#[derive(Debug)]
pub struct Store(Vec<Value>);


impl Store {
    pub fn new() -> Store {
        Store(Vec::new())
    }
    pub fn with_capacity(capacity: usize) -> Store {
        Store(Vec::with_capacity(capacity))
    }

    /// Dodanie Value dla parametru przysłanego przez wartość.
    pub fn add<T>(mut self, data: T) -> Self
        where T: ValueConvertible
    {
        let v = data.to_value();
        self.0.push(v);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn iter(&self) -> Iter<'_, Value> {
        self.0.iter()
    }
}

/********************************************************************
*                                                                   *
*             T r a i t - V a l u e C o n v e r t i b l e           *
*                                                                   *
********************************************************************/

pub trait ValueConvertible {
    fn to_value(&self) -> Value;
}

impl ValueConvertible for NaiveDateTime {
    fn to_value(&self) -> Value {
        (*self).into()
    }
}
impl ValueConvertible for Timestamp {
    fn to_value(&self) -> Value {
        (*self).into()
    }
}
impl ValueConvertible for i8 {
    fn to_value(&self) -> Value {
        (*self as i64).into()
    }
}
impl ValueConvertible for u8 {
    fn to_value(&self) -> Value {
        (*self as i64).into()
    }
}
impl ValueConvertible for i16 {
    fn to_value(&self) -> Value {
        (*self as i64).into()
    }
}
impl ValueConvertible for u16 {
    fn to_value(&self) -> Value {
        (*self as i64).into()
    }
}
impl ValueConvertible for i32 {
    fn to_value(&self) -> Value {
        (*self as i64).into()
    }
}
impl ValueConvertible for u32 {
    fn to_value(&self) -> Value {
        (*self as i64).into()
    }
}
impl ValueConvertible for i64 {
    fn to_value(&self) -> Value {
        (*self).into()
    }
}
impl ValueConvertible for f32 {
    fn to_value(&self) -> Value {
        (*self as f64).into()
    }
}
impl ValueConvertible for f64 {
    fn to_value(&self) -> Value {
        (*self).into()
    }
}
impl<'a> ValueConvertible for &'a str {
    fn to_value(&self) -> Value {
        (*self).into()
    }
}
impl<'a> ValueConvertible for &'a Vec<u8> {
    fn to_value(&self) -> Value {
        (*self).into()
    }
}
impl ValueConvertible for Vec<u8> {
    fn to_value(&self) -> Value {
        self.into()
    }
}
impl ValueConvertible for NullValue {
    fn to_value(&self) -> Value {
        Value::Null
    }
}
