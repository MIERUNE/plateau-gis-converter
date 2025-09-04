use nusamai_citygml::{schema, CityGmlElement};
use nusamai_plateau::models::TopLevelCityObject;

#[test]
fn render_schema() {
    let mut schema = schema::Schema::default();
    TopLevelCityObject::collect_schema(&mut schema);

    {
        let schema::TypeDef::Feature(building) = &schema.types["bldg:Building"] else {
            panic!("bldg:Building should be a feature type")
        };
        let class = &building.attributes["bldg:class"];
        assert_eq!(class.type_ref, schema::TypeRef::Code);
        assert_eq!(class.min_occurs, 0);
        assert_eq!(class.max_occurs, Some(1));

        let function = &building.attributes["bldg:function"];
        assert_eq!(function.type_ref, schema::TypeRef::Code);
        assert_eq!(function.min_occurs, 0);
        assert_eq!(function.max_occurs, None);

        assert_eq!(
            building.attributes["bldg:interiorBuildingInstallation"].type_ref,
            schema::TypeRef::Named("bldg:IntBuildingInstallation".to_string())
        );

        // property stereo type
        let schema::TypeRef::Named(name) = &building.attributes["bldg:boundedBy"].type_ref else {
            panic!("bldg:boundedBy is property type")
        };
        let schema::TypeDef::Property(_boundary) = &schema.types[name] else {
            panic!("bldg:boundedBy is property type")
        };
    }

    {
        // required attributes
        let schema::TypeDef::Data(building_id_attr) = &schema.types["uro:BuildingIDAttribute"]
        else {
            panic!("uro:BuildingIDAttribute should be a data type")
        };
        let building_id_ref = &building_id_attr.attributes["uro:buildingID"];
        assert_eq!(building_id_ref.min_occurs, 1);
        assert_eq!(building_id_ref.max_occurs, Some(1));
    }
}
