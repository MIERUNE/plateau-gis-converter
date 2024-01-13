//! Attribute encoder for MVT.

use crate::vector_tile::tile;
use ahash::RandomState;
use indexmap::IndexSet;

#[derive(Default)]
pub struct TagsEncoder {
    keys: IndexSet<String, RandomState>,
    values: IndexSet<Value, RandomState>,
}

/// Utility for encoding MVT attributes (tags).
impl TagsEncoder {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
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

    #[inline]
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

/// Wrapper for MVT Values
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tags_encoder() {
        let mut encoder = TagsEncoder::new();
        assert_eq!(encoder.add("k0", "v0".into()), [0, 0]);
        assert_eq!(encoder.add("k0", "v0".into()), [0, 0]);
        assert_eq!(encoder.add("k1", "v0".into()), [1, 0]);
        assert_eq!(encoder.add("k1", "v1".into()), [1, 1]);
        assert_eq!(encoder.add("k0", "v0".into()), [0, 0]);
        assert_eq!(encoder.add("k0", "v2".into()), [0, 2]);
        assert_eq!(encoder.add("k1", "v2".into()), [1, 2]);
        assert_eq!(encoder.add("k2", "v0".to_string().into()), [2, 0]);
        assert_eq!(encoder.add("k1", "v1".into()), [1, 1]);
        assert_eq!(encoder.add("k1", "v1".to_string().into()), [1, 1]);
        assert_eq!(encoder.add("k1", 10i32.into()), [1, 3]);
        assert_eq!(encoder.add("k2", 10.5f64.into()), [2, 4]);
        assert_eq!(encoder.add("k3", 10u32.into()), [3, 3]);
        assert_eq!(encoder.add("k3", (-10i32).into()), [3, 5]);
        assert_eq!(encoder.add("k3", true.into()), [3, 6]);
        assert_eq!(encoder.add("k3", 1.into()), [3, 7]);
        assert_eq!(encoder.add("k2", 10.5f32.into()), [2, 8]);
        assert_eq!(encoder.add("k4", 10.5f64.into()), [4, 4]);
        assert_eq!(encoder.add("k3", (-10i64).into()), [3, 5]);
        assert_eq!(encoder.add("k3", 10u64.into()), [3, 3]);
        assert_eq!(encoder.add("k5", Value::Int(11)), [5, 9]);
        assert_eq!(encoder.add("k5", 12i64.into()), [5, 10]);

        let (keys, values) = encoder.into_keys_and_values();
        assert_eq!(keys, vec!["k0", "k1", "k2", "k3", "k4", "k5"]);
        assert_eq!(
            values,
            vec![
                tile::Value {
                    string_value: Some("v0".to_string()),
                    ..Default::default()
                },
                tile::Value {
                    string_value: Some("v1".to_string()),
                    ..Default::default()
                },
                tile::Value {
                    string_value: Some("v2".to_string()),
                    ..Default::default()
                },
                tile::Value {
                    uint_value: Some(10),
                    ..Default::default()
                },
                tile::Value {
                    double_value: Some(10.5),
                    ..Default::default()
                },
                tile::Value {
                    sint_value: Some(-10),
                    ..Default::default()
                },
                tile::Value {
                    bool_value: Some(true),
                    ..Default::default()
                },
                tile::Value {
                    uint_value: Some(1),
                    ..Default::default()
                },
                tile::Value {
                    float_value: Some(10.5),
                    ..Default::default()
                },
                tile::Value {
                    int_value: Some(11),
                    ..Default::default()
                },
                tile::Value {
                    uint_value: Some(12),
                    ..Default::default()
                },
            ]
        );
    }
}
