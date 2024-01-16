use nusamai_citygml::{schema, CityGMLElement};
use nusamai_plateau::models::TopLevelCityObject;

#[test]
fn render_schema() {
    let mut schema = schema::Schema::default();
    TopLevelCityObject::collect_schema(&mut schema);

    {
        let building = &schema.types["bldg:Building"];
        assert_eq!(
            schema.types["bldg:Building"].stereotype,
            schema::TypeDef::Feature
        );
        let class = &building.attributes["bldg:class"];
        assert_eq!(class.ty, schema::Type::Code);
        assert_eq!(class.min_occurs, 0);
        assert_eq!(class.max_occurs, Some(1));

        let function = &building.attributes["bldg:function"];
        assert_eq!(function.ty, schema::Type::Code);
        assert_eq!(function.min_occurs, 0);
        assert_eq!(function.max_occurs, None);

        assert_eq!(
            building.attributes["bldg:interiorBuildingInstallation"].ty,
            schema::Type::Ref("bldg:BuildingInstallation".to_string())
        );

        // property stereo type
        let schema::Type::Property(_) = building.attributes["bldg:boundedBy"].ty else {
            panic!("bldg:boundedBy should be a property")
        };
    }

    {
        // required attributes
        let building_id = &schema.types["uro:BuildingIDAttribute"].attributes["uro:buildingID"];
        assert_eq!(building_id.min_occurs, 1);
        assert_eq!(building_id.max_occurs, Some(1));
    }
}
