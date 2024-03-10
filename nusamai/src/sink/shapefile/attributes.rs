use chrono::Datelike;
use hashbrown::HashMap;
use shapefile::dbase::{self, Date, FieldValue, Record};

use nusamai_citygml::object::Map;
use nusamai_citygml::object::Value;
use nusamai_citygml::schema::TypeRef;
use shapefile::Shape;

pub struct FieldInfo {
    field_type: TypeRef,
    size: u8,
}

pub type FieldInfoMap = HashMap<String, FieldInfo>;
pub type Features = Vec<(Shape, Map)>;

pub fn make_table_builder(fields: &FieldInfoMap) -> dbase::TableWriterBuilder {
    let mut builder = dbase::TableWriterBuilder::new();

    for (field_name, field_info) in fields {
        let name = field_name.as_str().try_into().unwrap(); // FIXME: handle errors

        match field_info.field_type {
            TypeRef::String | TypeRef::Code | TypeRef::URI => {
                builder = builder.add_character_field(name, field_info.size);
            }
            TypeRef::Integer | TypeRef::NonNegativeInteger => {
                builder = builder.add_integer_field(name);
            }
            TypeRef::Double | TypeRef::Measure => {
                builder = builder.add_float_field(name, 50, 10);
            }
            TypeRef::Boolean => {
                builder = builder.add_logical_field(name);
            }
            TypeRef::Date => {
                builder = builder.add_date_field(name);
            }
            TypeRef::Point => {
                // todo
            }
            TypeRef::Unknown => {
                // todo
            }
            TypeRef::Named(_) => {
                // todo
            }
            TypeRef::JsonString(_) => {
                // todo
            }
            TypeRef::DateTime => {
                // todo
            }
        }
    }

    builder
}

pub fn fill_missing_fields(attributes: &mut Map, field_info: &FieldInfoMap) {
    for (field_name, field_info) in field_info {
        if !attributes.contains_key(field_name.as_str()) {
            match field_info.field_type {
                TypeRef::String | TypeRef::Code | TypeRef::URI => {
                    attributes.insert(field_name.clone(), Value::String("".to_string()));
                }
                TypeRef::Integer | TypeRef::NonNegativeInteger => {
                    attributes.insert(field_name.clone(), Value::Integer(0));
                }
                TypeRef::Double | TypeRef::Measure => {
                    attributes.insert(field_name.clone(), Value::Double(0.0));
                }
                TypeRef::Boolean => {
                    attributes.insert(field_name.clone(), Value::String("".to_string()));
                }
                TypeRef::Date => {
                    attributes.insert(field_name.clone(), Value::String("".to_string()));
                }
                TypeRef::Point => {
                    // todo
                }
                TypeRef::Unknown => {
                    // todo
                }
                TypeRef::Named(_) => {
                    // todo
                }
                TypeRef::JsonString(_) => {
                    // todo
                }
                TypeRef::DateTime => {
                    // todo
                }
            }
        }
    }
}

pub fn make_field_list(features: &Features) -> FieldInfoMap {
    let mut fields: FieldInfoMap = Default::default();

    for (_, attributes) in features {
        for (field_name, field_value) in attributes {
            match field_value {
                Value::String(_) | Value::Code(_) | Value::Uri(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::String,
                            size: 255,
                        },
                    );
                }
                Value::Integer(_) | Value::NonNegativeInteger(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Integer,
                            size: 4,
                        },
                    );
                }
                Value::Double(_) | Value::Measure(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Double,
                            size: 8,
                        },
                    );
                }
                Value::Boolean(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Boolean,
                            size: 1,
                        },
                    );
                }
                Value::Date(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Date,
                            size: 8,
                        },
                    );
                }
                Value::Point(_) => {
                    // todo
                }
                Value::Array(_) => {
                    // todo
                }
                Value::Object(_) => {
                    // todo
                }
            }
        }
    }
    fields
}

pub fn attributes_to_record(attributes: Map) -> Record {
    let mut record = dbase::Record::default();

    for (attr_name, attr_value) in attributes {
        match attr_value {
            Value::String(s) => {
                // Shapefile string type can only store up to 255 characters.
                if s.len() > 255 {
                    log::warn!("{} value too long, truncating to 255 characters", attr_name);
                    record.insert(attr_name, FieldValue::Character(Some(s[0..255].to_owned())));
                } else {
                    record.insert(attr_name, FieldValue::Character(Some(s.to_owned())));
                }
            }
            Value::Code(c) => {
                // value of the code
                record.insert(attr_name, FieldValue::Character(Some(c.value().to_owned())));
            }
            Value::Integer(i) => {
                record.insert(attr_name, FieldValue::Integer(i.to_owned() as i32));
            }
            Value::NonNegativeInteger(i) => {
                record.insert(attr_name, FieldValue::Integer(i.to_owned() as i32));
            }
            // Handle as Float
            Value::Double(d) => {
                record.insert(attr_name, FieldValue::Float(Some(d.to_owned() as f32)));
            }
            // Handle as Float
            Value::Measure(m) => {
                record.insert(
                    attr_name,
                    FieldValue::Float(Some(m.value().to_owned() as f32)),
                );
            }
            Value::Boolean(b) => {
                record.insert(attr_name, FieldValue::Logical(Some(b.to_owned())));
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
