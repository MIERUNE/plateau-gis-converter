use chrono::Datelike;
use hashbrown::HashMap;
use nusamai_citygml::{
    object::{Map, Value},
    schema::{DataTypeDef, FeatureTypeDef, TypeDef, TypeRef},
};
use shapefile::dbase::{self, Date, FieldValue, Record};

pub fn make_table_builder(
    typedef: &TypeDef,
) -> (dbase::TableWriterBuilder, HashMap<String, FieldValue>) {
    let mut builder = dbase::TableWriterBuilder::new();
    let mut defaults = HashMap::new();

    let attributes = match typedef {
        TypeDef::Feature(FeatureTypeDef { attributes, .. }) => {
            let key = "id";
            builder = builder.add_character_field(key.try_into().unwrap(), 255);
            defaults.insert(key.into(), FieldValue::Character(None));
            attributes
        }
        TypeDef::Data(DataTypeDef { attributes, .. }) => attributes,
        TypeDef::Property(_) => unreachable!(),
    };

    for (field_name, attr) in attributes {
        let Ok(name) = field_name.as_str().try_into() else {
            log::error!("Field name '{}' cannot be used in Shapefile", field_name);
            continue;
        };
        let key = field_name.to_string();

        match attr.type_ref {
            TypeRef::String | TypeRef::Code | TypeRef::URI | TypeRef::JsonString(_) => {
                builder = builder.add_character_field(name, 255);
                defaults.insert(key, FieldValue::Character(None));
            }
            TypeRef::Integer | TypeRef::NonNegativeInteger => {
                builder = builder.add_numeric_field(name, 11, 0);
                defaults.insert(key, FieldValue::Numeric(None));
            }
            TypeRef::Double | TypeRef::Measure => {
                builder = builder.add_numeric_field(name, 18, 6);
                defaults.insert(key, FieldValue::Numeric(None));
            }
            TypeRef::Boolean => {
                builder = builder.add_character_field(name, 6);
                defaults.insert(key, FieldValue::Character(None));
            }
            TypeRef::Date => {
                builder = builder.add_date_field(name);
                defaults.insert(key, FieldValue::Date(None));
            }
            TypeRef::DateTime => {
                // todo
            }
            TypeRef::Point => {
                // todo
            }
            TypeRef::Unknown => {
                unreachable!();
            }
            TypeRef::Named(_) => {
                unreachable!();
            }
        }
    }

    (builder, defaults)
}

pub fn attributes_to_record(
    attributes: Map,
    fields_default: &HashMap<String, FieldValue>,
) -> Record {
    let mut record = dbase::Record::default();

    // Fill in with default values for attributes that are not present
    for (name, default) in fields_default {
        if !attributes.contains_key(name) {
            record.insert(name.to_string(), default.clone());
        }
    }

    for (attr_name, attr_value) in attributes {
        match attr_value {
            Value::String(s) => {
                // Shapefile cannot store string longer than 254 bytes
                let s = trim_string_bytes(s, 254);
                record.insert(attr_name, FieldValue::Character(Some(s)));
            }
            Value::Code(c) => {
                // value of the code
                record.insert(attr_name, FieldValue::Character(Some(c.value().to_owned())));
            }
            Value::Integer(i) => {
                record.insert(attr_name, FieldValue::Numeric(Some(i as f64)));
            }
            Value::NonNegativeInteger(i) => {
                record.insert(attr_name, FieldValue::Numeric(Some(i as f64)));
            }
            Value::Double(d) => {
                record.insert(
                    attr_name,
                    FieldValue::Numeric(match d.is_nan() {
                        true => None,
                        false => Some(d),
                    }),
                );
            }
            Value::Measure(m) => {
                record.insert(attr_name, FieldValue::Numeric(Some(m.value())));
            }
            Value::Boolean(b) => {
                record.insert(
                    attr_name,
                    FieldValue::Character(Some(match b {
                        true => "true".to_string(),
                        false => "false".to_string(),
                    })),
                );
            }
            Value::Uri(u) => {
                record.insert(
                    attr_name,
                    FieldValue::Character(Some(u.value().to_string())),
                );
            }
            Value::Date(d) => {
                // Date represented as an ISO8601 string
                record.insert(
                    attr_name,
                    FieldValue::Date(Some(Date::new(d.day(), d.month(), d.year() as u32))),
                );
            }
            Value::Point(_p) => {
                // TODO: implement
            }
            Value::Array(_arr) => {
                // TODO: handle multiple values
            }
            Value::Object(_obj) => {
                // TODO: handle nested objects
            }
        };
    }

    record
}

fn trim_string_bytes(s: String, n: usize) -> String {
    let bytes = s.as_bytes();
    if bytes.len() <= n {
        return s;
    }
    log::warn!("string is too long, truncating to {} characters", n);
    match std::str::from_utf8(&bytes[..n]) {
        Ok(valid_str) => valid_str.to_string(),
        Err(e) => {
            let valid_up_to = e.valid_up_to();
            let valid_str = std::str::from_utf8(&bytes[..valid_up_to]).unwrap();
            valid_str.to_string()
        }
    }
}
