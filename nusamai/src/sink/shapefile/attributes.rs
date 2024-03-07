use chrono::Datelike;
use hashbrown::HashMap;
use indexmap::IndexMap;
use shapefile::dbase::{self, Date, FieldValue};

use nusamai_citygml::object::{Map, Value};
use nusamai_citygml::schema::TypeRef;
use shapefile::Shape;

pub struct FieldInfo {
    field_type: TypeRef,
    size: u8,
}

pub type FieldInfoList = HashMap<String, FieldInfo>;

pub type Features = Vec<(Shape, IndexMap<String, Value, ahash::RandomState>)>;

pub struct TableBuilder {
    pub original_features: Features,
    pub fields: FieldInfoList,
    pub builder: dbase::TableWriterBuilder,
}

impl TableBuilder {
    pub fn new(original_features: Features) -> Self {
        Self {
            original_features,
            fields: Default::default(),
            builder: dbase::TableWriterBuilder::new(),
        }
    }

    pub fn make_field_list(mut self) {
        for (_, attributes) in self.original_features {
            for (field_name, field_value) in attributes {
                match field_value {
                    Value::String(_) | Value::Code(_) | Value::Uri(_) => {
                        self.fields.insert(
                            field_name.clone(),
                            FieldInfo {
                                field_type: TypeRef::String,
                                size: 255,
                            },
                        );
                    }
                    Value::Integer(_) | Value::NonNegativeInteger(_) => {
                        self.fields.insert(
                            field_name.clone(),
                            FieldInfo {
                                field_type: TypeRef::Integer,
                                size: 4,
                            },
                        );
                    }
                    Value::Double(_) | Value::Measure(_) => {
                        self.fields.insert(
                            field_name.clone(),
                            FieldInfo {
                                field_type: TypeRef::Double,
                                size: 8,
                            },
                        );
                    }
                    Value::Boolean(_) => {
                        self.fields.insert(
                            field_name.clone(),
                            FieldInfo {
                                field_type: TypeRef::Boolean,
                                size: 1,
                            },
                        );
                    }
                    Value::Uri(_) => {
                        self.fields.insert(
                            field_name.clone(),
                            FieldInfo {
                                field_type: TypeRef::String,
                                size: 255,
                            },
                        );
                    }
                    Value::Date(_) => {
                        self.fields.insert(
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

// Returns a TableBuilder with the minimum necessary field information
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

    // todo: attributesとfieldsの属性を突き合わせて、足りないattributeはnullで埋める

    let table_builder = TableBuilder::new(fields);
    table_builder.build()
}

pub fn feature_attributes_to_field_list(
    features: &Vec<(Shape, IndexMap<String, Value, ahash::RandomState>)>,
) -> FieldInfoList {
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
                Value::Uri(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::String,
                            size: 255,
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
