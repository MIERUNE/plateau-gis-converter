use citygml::*;
use nusamai_geometry::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct TopLevelCityObject {
    cityobj: FeatureOrData<'static>,
    geometries: Geometries,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Default, Clone)]
pub struct Geometries {
    pub vertices: Vec<[f64; 3]>,
    pub multipolygon: MultiPolygon<'static, 1, u32>,
    pub multilinestring: MultiPolygon<'static, 1, u32>,
}

#[derive(Debug, Clone)]
pub struct FeatureOrData<'a> {
    pub typename: &'a str,
    pub id: Option<&'a str>,
    pub attributes: HashMap<String, ObjectValue<'a>>,
}

#[derive(Debug, Clone)]
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
    use std::collections::HashMap;

    fn generate_city_objects() -> Vec<TopLevelCityObject> {
        let mut top_level_city_object1 = generate_top_level_city_object();
        top_level_city_object1.cityobj.id = Some("bldg_test1");

        let mut top_level_city_object2 = generate_top_level_city_object();
        top_level_city_object2.cityobj.id = Some("bldg_test2");

        let city_objects = vec![
            top_level_city_object1.clone(),
            top_level_city_object2.clone(),
        ];

        city_objects
    }

    fn generate_top_level_city_object() -> TopLevelCityObject {
        let type_ = "Building";
        let id_ = Some("bldg_test");

        let mut attributes: HashMap<String, super::ObjectValue> = HashMap::new();
        let attribute_name = "building_id_attribute";
        let attribute_value = super::ObjectValue::String("bldg_test");
        attributes.insert(attribute_name.to_string(), attribute_value);

        let mut geometries = super::Geometries {
            vertices: Vec::new(),
            multipolygon: MultiPolygon::new(),
            multilinestring: MultiPolygon::new(),
        };
        geometries.vertices.push([0.0, 0.0, 0.0]);
        geometries.vertices.push([1.0, 0.0, 0.0]);
        geometries.vertices.push([1.0, 1.0, 0.0]);
        geometries.multipolygon.add_exterior(vec![[0], [1], [2]]);

        let cityobj = super::FeatureOrData {
            typename: type_,
            id: id_,
            attributes,
        };

        super::TopLevelCityObject {
            cityobj,
            geometries,
        }
    }

    #[test]
    fn test_generate_city_objects() {
        let city_objects = generate_city_objects();
        let top_level_city_object1 = &city_objects[0];
        let top_level_city_object2 = &city_objects[1];

        assert_eq!(top_level_city_object1.cityobj.id, Some("bldg_test1"));
        assert_eq!(top_level_city_object2.cityobj.id, Some("bldg_test2"));
    }

    #[test]
    fn test_make_buffer() {}

    #[test]
    fn test_make_gltf() {}

    #[test]
    fn test_make_glb() {}
}
