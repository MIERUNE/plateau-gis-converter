use chrono::Datelike;
use hashbrown::HashMap;
use indexmap::IndexMap;
use shapefile::dbase::{self, Date, FieldValue};

use nusamai_citygml::object::Value;
use nusamai_citygml::schema::TypeRef;

pub struct FieldInfo {
    field_type: TypeRef,
    size: u8,
}

pub type FieldInfoList = HashMap<String, FieldInfo>;

struct TableBuilder {
    fields: FieldInfoList,
    builder: dbase::TableWriterBuilder,
}

impl TableBuilder {
    pub fn new(fields: FieldInfoList) -> Self {
        Self {
            fields,
            builder: dbase::TableWriterBuilder::new(),
        }
    }

    fn build(mut self) -> dbase::TableWriterBuilder {
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

pub fn prepare_table_builder(
    features: &Vec<(shapefile::Shape, dbase::Record)>,
) -> dbase::TableWriterBuilder {
    let mut fields: FieldInfoList = Default::default();

    for (_, attributes) in features {
        for (field_name, field_value) in attributes.clone().into_iter() {
            match field_value {
                FieldValue::Character(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::String,
                            size: 255,
                        },
                    );
                }
                FieldValue::Numeric(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Double,
                            size: 8,
                        },
                    );
                }
                FieldValue::Logical(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Boolean,
                            size: 1,
                        },
                    );
                }
                FieldValue::Date(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Date,
                            size: 8,
                        },
                    );
                }
                FieldValue::Float(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Double,
                            size: 8,
                        },
                    );
                }
                FieldValue::Integer(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Integer,
                            size: 4,
                        },
                    );
                }
                FieldValue::Currency(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Double,
                            size: 8,
                        },
                    );
                }
                FieldValue::DateTime(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::DateTime,
                            size: 8,
                        },
                    );
                }
                FieldValue::Double(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Double,
                            size: 8,
                        },
                    );
                }
                FieldValue::Memo(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::String,
                            size: 255,
                        },
                    );
                }
            }
        }
    }

    let table_builder = TableBuilder::new(fields);
    table_builder.build()
}

pub fn prepare_shapefile_attributes(
    object: &nusamai_citygml::object::Object,
) -> IndexMap<String, FieldValue> {
    let mut attributes = IndexMap::<String, FieldValue>::new();

    for (attr_name, attr_value) in &object.attributes {
        match attr_value {
            Value::String(s) => {
                attributes.insert(attr_name.into(), FieldValue::Character(Some(s.to_owned())));
            }
            Value::Code(c) => {
                // value of the code
                attributes.insert(
                    attr_name.into(),
                    FieldValue::Character(Some(c.value().to_owned())),
                );
            }
            Value::Integer(i) => {
                attributes.insert(attr_name.into(), FieldValue::Integer(i.to_owned() as i32));
            }
            Value::NonNegativeInteger(i) => {
                attributes.insert(attr_name.into(), FieldValue::Integer(i.to_owned() as i32));
            }
            // Handle as Float
            Value::Double(d) => {
                attributes.insert(
                    attr_name.into(),
                    FieldValue::Float(Some(d.to_owned() as f32)),
                );
            }
            // Handle as Float
            Value::Measure(m) => {
                attributes.insert(
                    attr_name.into(),
                    FieldValue::Float(Some(m.value().to_owned() as f32)),
                );
            }
            Value::Boolean(b) => {
                attributes.insert(attr_name.into(), FieldValue::Logical(Some(b.to_owned())));
            }
            Value::Uri(u) => {
                attributes.insert(
                    attr_name.into(),
                    FieldValue::Character(Some(u.value().to_string())),
                );
            }
            Value::Date(d) => {
                // Date represented as an ISO8601 string
                attributes.insert(
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

    attributes
}
