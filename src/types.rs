use::std::collections::HashMap;
use std::convert::From;

pub type Row = HashMap<String, Option<Value>>;
pub struct NullValue;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl Value {
    pub fn kind(&self) -> Type{
        match self {
            Value::Null => Type::Null,
            Value::Int(_) => Type::Int64,
            Value::Float(_) => Type::Float64,
            Value::Text(_) => Type::Text,
            Value::Blob(_) => Type::Blob
        }
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
    fn from(s: NullValue) -> Self {
        Value::Null
    }
}
impl From<&Value> for i64 {
    fn from(value: &Value) -> i64 {
        if let Value::Int(v) = value {
            return *v;
        }
        0
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

