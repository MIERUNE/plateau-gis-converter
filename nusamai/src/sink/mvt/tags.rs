use nusamai_citygml::object;
use tinymvt::tag::TagsEncoder;

pub fn convert_properties(tags_enc: &mut TagsEncoder, name: &str, tree: &object::Value) {
    match &tree {
        nusamai_citygml::Value::String(v) => {
            tags_enc.add(name, v.clone());
        }
        nusamai_citygml::Value::Code(v) => {
            tags_enc.add(name, v.value());
        }
        nusamai_citygml::Value::Integer(v) => {
            tags_enc.add(name, *v);
        }
        nusamai_citygml::Value::NonNegativeInteger(v) => {
            tags_enc.add(name, *v);
        }
        nusamai_citygml::Value::Double(v) => {
            tags_enc.add(name, *v);
        }
        nusamai_citygml::Value::Measure(v) => {
            tags_enc.add(name, v.value());
        }
        nusamai_citygml::Value::Boolean(v) => {
            tags_enc.add(name, *v);
        }
        nusamai_citygml::Value::Uri(v) => {
            tags_enc.add(name, v.value().to_string());
        }
        nusamai_citygml::Value::Date(v) => {
            tags_enc.add(name, v.to_string());
        }
        nusamai_citygml::Value::Point(v) => {
            tags_enc.add(name, format!("{:?}", v)); // FIXME
        }
        nusamai_citygml::Value::Array(_arr) => {
            // ignore non-root attributes
        }
        nusamai_citygml::Value::Object(_obj) => {
            // ignore non-root attributes
        }
    }
}
