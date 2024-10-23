use nusamai_citygml::object;
use tinymvt::tag::TagsEncoder;

pub fn convert_properties(
    tags: &mut Vec<u32>,
    tags_enc: &mut TagsEncoder,
    name: &str,
    tree: &object::Value,
) {
    match &tree {
        nusamai_citygml::Value::String(v) => {
            tags.extend(tags_enc.add(name, v.clone().into()));
        }
        nusamai_citygml::Value::Code(v) => {
            tags.extend(tags_enc.add(name, v.value().into()));
        }
        nusamai_citygml::Value::Integer(v) => {
            tags.extend(tags_enc.add(name, (*v).into()));
        }
        nusamai_citygml::Value::NonNegativeInteger(v) => {
            tags.extend(tags_enc.add(name, (*v).into()));
        }
        nusamai_citygml::Value::Double(v) => {
            tags.extend(tags_enc.add(name, (*v).into()));
        }
        nusamai_citygml::Value::Measure(v) => {
            tags.extend(tags_enc.add(name, v.value().into()));
        }
        nusamai_citygml::Value::Boolean(v) => {
            tags.extend(tags_enc.add(name, (*v).into()));
        }
        nusamai_citygml::Value::Uri(v) => {
            tags.extend(tags_enc.add(name, v.value().to_string().into()));
        }
        nusamai_citygml::Value::Date(v) => {
            tags.extend(tags_enc.add(name, v.to_string().into()));
        }
        nusamai_citygml::Value::Point(v) => {
            tags.extend(tags_enc.add(name, format!("{:?}", v).into())); // FIXME
        }
        nusamai_citygml::Value::Array(_arr) => {
            // ignore non-root attributes
        }
        nusamai_citygml::Value::Object(_obj) => {
            // ignore non-root attributes
        }
    }
}
