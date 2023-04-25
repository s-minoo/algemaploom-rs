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



impl From<bool> for Value{
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}


impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl<T> From<HashMap<String, T>> for Value
where
    Value: From<T>,
{
    fn from(value: HashMap<String, T>) -> Self {
        let val_map: HashMap<String, Value> =
            value.into_iter().map(|(k, v)| (k, v.into())).collect();
        Self::Object(val_map)
    }
}

impl<T> From<Vec<T>> for Value
where
    Value: From<T>,
{
    fn from(value: Vec<T>) -> Self {
        let val_vec: Vec<Value> = value.into_iter().map(|f| f.into()).collect();
        Self::Array(val_vec)
    }
}

macro_rules! from_num_val_impl {
    ($t:ident, $number:ident) => {
        impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Self::Number(Number::$number(value))
            }
        }
    };
}

from_num_val_impl!(f64, Double);
from_num_val_impl!(f32, Float);
from_num_val_impl!(u64, UInt);
from_num_val_impl!(i64, Int);
from_num_val_impl!(u8, Byte);

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    PosInfinity,
    NegInfinity,
    Double(f64),
    Byte(u8),
    Int(i64),
    UInt(u64),
    Float(f32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let arr = vec!["A", "B", "C"];
    }

    #[test]
    fn test_object() {}

    #[test]
    fn test_number() {}
}
