use citygml::*;
use std::collections::HashMap;

#[derive(Debug)]
struct TopLevelCityObject {
    cityobj: FeatureOrData<'static>,
    geometries: citygml::Geometries,
}

#[derive(Debug)]
pub struct FeatureOrData<'a> {
    pub typename: &'a str,
    pub id: Option<&'a str>,
    pub attributes: HashMap<String, ObjectValue<'a>>,
}

#[derive(Debug)]
pub enum ObjectValue<'a> {
    String(&'a str),
    Code(&'a Code),
    Integer(i64),
    Double(f64),
    Measure(f64),
    Boolean(bool),
    URI(&'a URI),
    Date(&'a NaiveDate),
    Point(&'a Point),
    Array(Vec<ObjectValue<'a>>),
    FeatureOrData(FeatureOrData<'a>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use nusamai_geometry::*;
    use std::collections::HashMap;

    #[test]
    fn test_hello() {
        assert_eq!(2 + 2, 4);
    }

    fn test_generate_top_level_city_object() {
        let type_ = "Building";
        let id_ = Some("bldg_test");

        let mut attributes: HashMap<String, super::ObjectValue> = HashMap::new();
        let attribute_name = "building_id_attribute";
        let attribute_value = super::ObjectValue::String("bldg_test");
        attributes.insert(attribute_name.to_string(), attribute_value);

        let geometries = citygml::Geometries {
            vertices: Vec::new(),
            multipolygon: MultiPolygon::new(),
            multilinestring: MultiLineString::new(),
        };

        let cityobj = super::FeatureOrData {
            typename: type_,
            id: id_,
            attributes,
        };

        let top_level_city_object = super::TopLevelCityObject {
            cityobj,
            geometries,
        };
    }
}
