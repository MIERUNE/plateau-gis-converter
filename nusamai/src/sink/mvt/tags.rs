use nusamai_citygml::object;
use nusamai_mvt::tag::TagsEncoder;

pub fn traverse_properties(
    tags: &mut Vec<u32>,
    tags_enc: &mut TagsEncoder,
    name: String,
    tree: &object::Value,
) {
    match &tree {
        nusamai_citygml::Value::String(v) => {
            tags.extend(tags_enc.add(&name, v.clone().into()));
        }
        nusamai_citygml::Value::Code(v) => {
            tags.extend(tags_enc.add(&name, v.value().into()));
        }
        nusamai_citygml::Value::Integer(v) => {
            tags.extend(tags_enc.add(&name, (*v).into()));
        }
        nusamai_citygml::Value::Double(v) => {
            tags.extend(tags_enc.add(&name, (*v).into()));
        }
        nusamai_citygml::Value::Measure(v) => {
            tags.extend(tags_enc.add(&name, v.value().into()));
        }
        nusamai_citygml::Value::Boolean(v) => {
            tags.extend(tags_enc.add(&name, (*v).into()));
        }
        nusamai_citygml::Value::URI(v) => {
            tags.extend(tags_enc.add(&name, v.value().to_string().into()));
        }
        nusamai_citygml::Value::Date(v) => {
            tags.extend(tags_enc.add(&name, v.to_string().into()));
        }
        nusamai_citygml::Value::Point(v) => {
            tags.extend(tags_enc.add(&name, format!("{:?}", v).into())); // FIXME
        }
        nusamai_citygml::Value::Array(arr) => {
            arr.iter().enumerate().for_each(|(i, v)| {
                if name.is_empty() {
                    traverse_properties(tags, tags_enc, i.to_string(), v);
                } else {
                    traverse_properties(tags, tags_enc, format!("{}.{}", name, i), v);
                }
            });
        }
        nusamai_citygml::Value::Feature(feat) => {
            tags.extend(tags_enc.add("id", feat.id.clone().into()));
            feat.attributes.iter().for_each(|(k, v)| {
                let k = match k.split_once(':') {
                    Some((_, k)) => k,
                    None => k,
                };
                if name.is_empty() {
                    traverse_properties(tags, tags_enc, k.into(), v);
                } else {
                    traverse_properties(tags, tags_enc, format!("{}.{}", name, k), v);
                }
            });
        }
        nusamai_citygml::Value::Data(data) => {
            data.attributes.iter().for_each(|(k, v)| {
                let k = match k.split_once(':') {
                    Some((_, k)) => k,
                    None => k,
                };
                if name.is_empty() {
                    traverse_properties(tags, tags_enc, k.into(), v);
                } else {
                    traverse_properties(tags, tags_enc, format!("{}.{}", name, k), v);
                }
            });
        }
    }
}
