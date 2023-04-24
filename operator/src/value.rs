use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,

    Boolean(bool),

    Number(Number),

    Array(Vec<Value>),

    String(String),

    Object(HashMap<String, Value>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    PosInfinity,
    NegInfinity,
    Double(f64),
    Byte(u8),
    Int(i64),
    UInt(u64),
    Float(f64),
}
