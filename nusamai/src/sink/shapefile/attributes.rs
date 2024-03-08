use chrono::Datelike;
use hashbrown::HashMap;
use indexmap::IndexMap;
use shapefile::dbase::{self, Date, FieldValue, Record};

use nusamai_citygml::object::Value;
use nusamai_citygml::schema::TypeRef;
use shapefile::Shape;

pub struct FieldInfo {
    field_type: TypeRef,
    size: u8,
}

pub type FieldInfoList = HashMap<String, FieldInfo>;

pub type Features = Vec<(Shape, IndexMap<String, Value, ahash::RandomState>)>;

pub struct TableBuilder {
    pub fields: FieldInfoList,
    pub builder: dbase::TableWriterBuilder,
}

impl TableBuilder {
    pub fn new(fields: FieldInfoList) -> Self {
        Self {
            fields,
            builder: dbase::TableWriterBuilder::new(),
        }
    }
    pub fn build(mut self) -> dbase::TableWriterBuilder {
        for (field_name, field_info) in self.fields {
            match field_info.field_type {
                TypeRef::String | TypeRef::Code | TypeRef::URI => {
                    self.builder = self.builder.add_character_field(
                        field_name.as_str().try_into().unwrap(),
                        field_info.size,
                    );
                }
                TypeRef::Integer | TypeRef::NonNegativeInteger => {
                    self.builder = self
                        .builder
                        .add_integer_field(field_name.as_str().try_into().unwrap());
                }
                // Handle as Float
                TypeRef::Double | TypeRef::Measure => {
                    self.builder = self.builder.add_float_field(
                        field_name.as_str().try_into().unwrap(),
                        50,
                        10,
                    );
                }
                TypeRef::Boolean => {
                    self.builder = self
                        .builder
                        .add_logical_field(field_name.as_str().try_into().unwrap());
                }
                TypeRef::Date => {
                    self.builder = self
                        .builder
                        .add_date_field(field_name.as_str().try_into().unwrap());
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
        self.builder
    }
}

pub fn fill_missing_fields(features: &mut Features, field_info: &FieldInfoList) {
    for (_, attributes) in features.iter_mut() {
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
}

pub fn make_field_list(features: &Features) -> FieldInfoList {
    let mut fields: FieldInfoList = Default::default();

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

pub fn prepare_shapefile_attributes(features: &Features) -> Vec<Record> {
    let mut records = Vec::new();
    for (_, feature_attributes) in features.iter() {
        let mut record = dbase::Record::default();

        for (attr_name, attr_value) in feature_attributes {
            match attr_value {
                Value::String(s) => {
                    record.insert(attr_name.into(), FieldValue::Character(Some(s.to_owned())));
                }
                Value::Code(c) => {
                    // value of the code
                    record.insert(
                        attr_name.into(),
                        FieldValue::Character(Some(c.value().to_owned())),
                    );
                }
                Value::Integer(i) => {
                    record.insert(attr_name.into(), FieldValue::Integer(i.to_owned() as i32));
                }
                Value::NonNegativeInteger(i) => {
                    record.insert(attr_name.into(), FieldValue::Integer(i.to_owned() as i32));
                }
                // Handle as Float
                Value::Double(d) => {
                    record.insert(
                        attr_name.into(),
                        FieldValue::Float(Some(d.to_owned() as f32)),
                    );
                }
                // Handle as Float
                Value::Measure(m) => {
                    record.insert(
                        attr_name.into(),
                        FieldValue::Float(Some(m.value().to_owned() as f32)),
                    );
                }
                Value::Boolean(b) => {
                    record.insert(attr_name.into(), FieldValue::Logical(Some(b.to_owned())));
                }
                Value::Uri(u) => {
                    record.insert(
                        attr_name.into(),
                        FieldValue::Character(Some(u.value().to_string())),
                    );
                }
                Value::Date(d) => {
                    // Date represented as an ISO8601 string
                    record.insert(
                        attr_name.into(),
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
        records.push(record);
    }

    records
}
