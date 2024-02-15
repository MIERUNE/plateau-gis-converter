use indexmap::IndexMap;
use nusamai_citygml::object::{Object, Value};

/// Prepare the attribute values for the GeoPackage
pub fn prepare_object_attributes(obj: &Object) -> IndexMap<String, String> {
    let mut attributes = IndexMap::<String, String>::new();

    for (attr_name, attr_value) in &obj.attributes {
        match attr_value {
            Value::String(s) => {
                attributes.insert(attr_name.into(), s.into());
            }
            Value::Code(c) => {
                // value of the code
                attributes.insert(attr_name.into(), c.value().into());
            }
            Value::Integer(i) => {
                attributes.insert(attr_name.into(), i.to_string());
            }
            Value::NonNegativeInteger(i) => {
                attributes.insert(attr_name.into(), i.to_string());
            }
            Value::Double(d) => {
                attributes.insert(attr_name.into(), d.to_string());
            }
            Value::Measure(m) => {
                attributes.insert(attr_name.into(), m.value().to_string());
            }
            Value::Boolean(b) => {
                // 0 for false and 1 for true in SQLite
                attributes.insert(attr_name.into(), if *b { "1".into() } else { "0".into() });
            }
            Value::URI(u) => {
                // value of the URI
                attributes.insert(attr_name.into(), u.value().to_string());
            }
            Value::Date(d) => {
                // Date represented as an ISO8601 string
                attributes.insert(attr_name.into(), d.to_string());
            }
            Value::Point(_p) => {
                // TODO: implement
                // Point struct currently does not contain any data
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
