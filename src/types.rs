/*
 * Copyright (C) 2023 Piotr Pszczółkowski
 * Licence: GNU v2
 *
 * E-mail: piotr@beesoft.pl
 *
 * Project: rs-sqlite
 * File: types.rs
 */
use std::collections::HashMap;
use std::convert::From;
use chrono::{DateTime, Local, Utc};
use crate::value::Value;
pub type Row = HashMap<String, Option<Value>>;


#[derive(PartialEq, Copy, Clone)]
pub enum Type {
    Int64 = 1,
    Float64,
    Text,
    Blob,
    Null,
}

impl Type {
    pub(crate) fn vtype(v: usize) -> Type {
        match v {
            1 => Type::Int64,
            2 => Type::Float64,
            3 => Type::Text,
            4 => Type::Blob,
            5 => Type::Null,
            _ => todo!()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Timestamp(i64);

impl Timestamp {
    pub fn now() -> Timestamp {
        let dt_utc = DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc);
        Timestamp(dt_utc.timestamp())
    }
    pub fn tm(v: i64) -> Timestamp {
        Timestamp(v)
    }
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl From<i64> for Timestamp {
    fn from(v: i64) -> Self {
        Timestamp(v)
    }
}