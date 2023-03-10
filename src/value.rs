/*
 * Copyright (C) 2023 Piotr Pszczółkowski
 * Licence: GNU v2
 *
 * E-mail: piotr@beesoft.pl
 *
 * Project: rs-sqlite
 * File: value.rs
 */
use crate::types::{Timestamp, Type};
use chrono::{Local, NaiveDateTime, NaiveDate};

pub struct NullValue;

#[derive(Debug)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    Text(String),
    Blob(Vec<u8>),
}


impl Value {
    pub fn kind(&self) -> Type {
        match self {
            Value::Null => Type::Null,
            Value::Int(_) => Type::Int64,
            Value::Float(_) => Type::Float64,
            Value::Text(_) => Type::Text,
            Value::Blob(_) => Type::Blob
        }
    }
}

impl From<Timestamp> for Value {
    fn from(v: Timestamp) -> Self {
        Value::Int(v.value())
    }
}

impl From<NaiveDateTime> for Value {
    fn from(v: NaiveDateTime) -> Self {
        Value::Text(v.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}

impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Value::Int(v as i64)
    }
}

impl From<i16> for Value {
    fn from(v: i16) -> Self {
        Value::Int(v as i64)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Int(v as i64)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Int(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Float(v as f64)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(v as f64)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Text(s.clone().to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Text(s)
    }
}

impl From<&Vec<u8>> for Value {
    fn from(s: &Vec<u8>) -> Self {
        Value::Blob(s.clone())
    }
}

impl From<Vec<u8>> for Value {
    fn from(s: Vec<u8>) -> Self {
        Value::Blob(s)
    }
}

impl From<NullValue> for Value {
    fn from(_: NullValue) -> Self {
        Value::Null
    }
}

impl From<&Value> for i64 {
    fn from(value: &Value) -> i64 {
        match value {
            Value::Int(v) => *v,
            _ => panic!("it is not i64")
        }
    }
}

impl From<&Value> for f64 {
    fn from(value: &Value) -> f64 {
        match value {
            Value::Float(v) => *v,
            _ => panic!("it is not f64")
        }
    }
}

impl From<&Value> for String {
    fn from(value: &Value) -> String {
        match value {
            Value::Text(text) => text.clone(),
            _ => panic!("it is not a string")
        }
    }
}

impl From<&Value> for Vec<u8> {
    fn from(value: &Value) -> Vec<u8> {
        match value {
            Value::Blob(value) => value.clone(),
            _ => panic!("it is not a vec<u8>")
        }
    }
}

impl From<&Value> for Timestamp {
    fn from(value: &Value) -> Timestamp {
        match value {
            Value::Int(value) => Timestamp::tm(*value),
            _ => panic!("it is not a Timestamp(i64)")
        }
    }
}

impl From<&Value> for NaiveDateTime {
    fn from(value: &Value) -> NaiveDateTime {
        match value {
            Value::Text(value) => NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap(),
            _ => panic!("it is not a NaiveDateTime"),
        }
    }

}
