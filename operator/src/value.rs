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
    Short(i32),
    UShort(u32),
    Int(i64),
    UInt(u64),
    Float(f32),
}


// Conversions traits

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        if let Ok(parsed) = value.trim().parse::<f64>() {
            return parsed.into();
        }

        Self::String(value)
    }
}

impl<T> From<HashMap<&str, T>> for Value
where
    Value: From<T>,
{
    fn from(value: HashMap<&str, T>) -> Self {
        let val_map: HashMap<String, Value> = value
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        Self::Object(val_map)
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
from_num_val_impl!(i32, Short);
from_num_val_impl!(u32, UShort);
from_num_val_impl!(u64, UInt);
from_num_val_impl!(i64, Int);
from_num_val_impl!(u8, Byte);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let num_string: Value = String::from("Foobar").into();
        let val = Value::String("Foobar".into());

        assert!(num_string == val);
    }

    #[test]
    fn test_array() {
        let arr = vec!["A", "B", "C"];
        let intoed_val: Value = arr.into();

        let val_arr = Value::Array(vec!["A".into(), "B".into(), "C".into()]);
        assert!(intoed_val == val_arr);
    }

    #[test]
    fn test_object() {
        let map = HashMap::from([("name", "foobar"), ("age", "23")]);
        let intoed_map: Value = map.into();
        let map_val = Value::Object(HashMap::from([
            ("name".to_string(), "foobar".into()),
            ("age".to_string(), (23 as f64).into()),
        ]));

        assert!(map_val == intoed_map);
    }


    #[test]
    fn test_number_from_string() {
        let num_str: Value = "23".into();
        let f64_val: Value = (23 as f64).into();

        assert!(num_str == f64_val); 
    }
}
