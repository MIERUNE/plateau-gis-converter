use crate::vector_tile::tile;
use ahash::RandomState;
use indexmap::IndexSet;

#[derive(Default)]
pub struct TagsEncoder {
    keys: IndexSet<String, RandomState>,
    values: IndexSet<Value, RandomState>,
}

impl TagsEncoder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, key: &str, value: Value) -> [u32; 2] {
        let key_idx = match self.keys.get_index_of(key) {
            None => self.keys.insert_full(key.to_string()).0,
            Some(idx) => idx,
        };
        let value_idx = match self.values.get_index_of(&value) {
            None => self.values.insert_full(value).0,
            Some(idx) => idx,
        };
        [key_idx as u32, value_idx as u32]
    }

    pub fn into_keys_and_values(self) -> (Vec<String>, Vec<tile::Value>) {
        let keys = self.keys.into_iter().collect();
        let values = self
            .values
            .into_iter()
            .map(|v| v.into_tile_value())
            .collect();
        (keys, values)
    }
}

/// Wrapper for tile::Value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    String(String),
    Float([u8; 4]),
    Double([u8; 8]),
    Int(i64),
    UInt(u64),
    SInt(i64),
    Bool(bool),
}

impl Value {
    pub fn into_tile_value(self) -> tile::Value {
        use Value::*;
        match self {
            String(v) => tile::Value {
                string_value: Some(v),
                ..Default::default()
            },
            Float(v) => tile::Value {
                float_value: Some(f32::from_ne_bytes(v)),
                ..Default::default()
            },
            Double(v) => tile::Value {
                double_value: Some(f64::from_ne_bytes(v)),
                ..Default::default()
            },
            Int(v) => tile::Value {
                int_value: Some(v),
                ..Default::default()
            },
            UInt(v) => tile::Value {
                uint_value: Some(v),
                ..Default::default()
            },
            SInt(v) => tile::Value {
                sint_value: Some(v),
                ..Default::default()
            },
            Bool(v) => tile::Value {
                bool_value: Some(v),
                ..Default::default()
            },
        }
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}
impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}
impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::UInt(v)
    }
}
impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::UInt(v as u64)
    }
}
impl From<i64> for Value {
    fn from(v: i64) -> Self {
        if v >= 0 {
            Value::UInt(v as u64)
        } else {
            Value::SInt(v)
        }
    }
}
impl From<i32> for Value {
    fn from(v: i32) -> Self {
        if v >= 0 {
            Value::UInt(v as u64)
        } else {
            Value::SInt(v as i64)
        }
    }
}
impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Float(v.to_ne_bytes())
    }
}
impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Double(v.to_ne_bytes())
    }
}
impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}
