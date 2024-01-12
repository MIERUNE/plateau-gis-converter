use std::io::BufRead;
use std::path::Path;

use url::Url;

use nusamai_citygml::{
    CityGMLElement, CityGMLReader, Code, GeometryStore, ParseError, SubTreeReader,Date
};
use nusamai_plateau::models::urf::{
    AreaClassification, HeightControlDistrict
};
use nusamai_plateau::models::TopLevelCityObject;

#[derive(Default, Debug)]
struct AreaClassificationData {
    area_classfications: Vec<AreaClassification>,
    geometries: Vec<GeometryStore>,
}

#[derive(Default, Debug)]
struct HeightControlDistrictData {
    height_control_districts: Vec<HeightControlDistrict>,
    geometries: Vec<GeometryStore>,
}

fn area_classfication_toplevel_dispatcher<R: BufRead>(st: &mut SubTreeReader<R>) -> Result<AreaClassificationData, ParseError> {
    let mut parsed_data = AreaClassificationData::default();

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: TopLevelCityObject = Default::default();
            cityobj.parse(st)?;
            match cityobj {
                TopLevelCityObject::AreaClassification(acf) => {
                    parsed_data.area_classfications.push(acf);
                }
                TopLevelCityObject::CityObjectGroup(_) => (),
                e => panic!("Unexpected city object type: {:?}", e),
            }
            let geometries = st.collect_geometries();
            parsed_data.geometries.push(geometries);
            Ok(())
        }
        b"gml:boundedBy" | b"app:appearanceMember" => {
            st.skip_current_element()?;
            Ok(())
        }
        other => Err(ParseError::SchemaViolation(format!(
            "Unrecognized element {}",
            String::from_utf8_lossy(other)
        ))),
    }) {
        Ok(_) => Ok(parsed_data),
        Err(e) => {
            println!("Err: {:?}", e);
            Err(e)
        }
    }
}


fn height_control_district_toplevel_dispatcher<R: BufRead>(st: &mut SubTreeReader<R>) -> Result<HeightControlDistrictData, ParseError> {
    let mut parsed_data = HeightControlDistrictData::default();

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: TopLevelCityObject = Default::default();
            cityobj.parse(st)?;
            match cityobj {
                TopLevelCityObject::HeightControlDistrict(hcd) => {
                    parsed_data.height_control_districts.push(hcd);
                }
                TopLevelCityObject::CityObjectGroup(_) => (),
                e => panic!("Unexpected city object type: {:?}", e),
            }
            let geometries = st.collect_geometries();
            parsed_data.geometries.push(geometries);
            Ok(())
        }
        b"gml:boundedBy" | b"app:appearanceMember" => {
            st.skip_current_element()?;
            Ok(())
        }
        other => Err(ParseError::SchemaViolation(format!(
            "Unrecognized element {}",
            String::from_utf8_lossy(other)
        ))),
    }) {
        Ok(_) => Ok(parsed_data),
        Err(e) => {
            println!("Err: {:?}", e);
            Err(e)
        }
    }
}

#[test]
fn test_urf_kuiki() {
    let filename = "./tests/data/kawasaki-shi/udx/urf/533915_urf_6668_kuiki_op.gml";

    let reader = std::io::BufReader::new(std::fs::File::open(filename).unwrap());
    let mut xml_reader = quick_xml::NsReader::from_reader(reader);

    let code_resolver = nusamai_plateau::codelist::Resolver::new();
    let source_url =
        Url::from_file_path(std::fs::canonicalize(Path::new(filename)).unwrap()).unwrap();
    let context = nusamai_citygml::ParseContext::new(source_url, &code_resolver);

    let parsed_data = match CityGMLReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => match area_classfication_toplevel_dispatcher(&mut st) {
            Ok(parsed_data) => parsed_data,
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };


    let area_classfication = parsed_data.area_classfications.first().unwrap();

    assert_eq!(parsed_data.area_classfications.len(), 2);
    assert_eq!(parsed_data.area_classfications.len(), parsed_data.geometries.len());

    assert_eq!(
        area_classfication.function,
        vec![Code::new("市街化区域".to_string(), "22".to_string(),)]
    );

    assert_eq!(
        area_classfication.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2009, 9, 18).unwrap())
    );

    assert_eq!(
        area_classfication.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );

    // assert_eq!(
    //     track.tran_data_quality_attribute.as_ref().unwrap().geometry_src_desc,
    //     vec![Code::new("既成図数値化".to_string(), "6".to_string())]
    // );

    // assert_eq!(
    //     track.auxiliary_traffic_area.first().unwrap().function,
    //     vec![Code::new("島".to_string(), "3000".to_string())]
    // );

    // assert_eq!(
    //     track.track_attribute.first().unwrap().admin_type,
    //     Some(Code::new("市区町村".to_string(), "3".to_string()))
    // );
}

#[test]
fn test_urf_kodo() {
    let filename = "./tests/data/kawasaki-shi/udx/urf/533915_urf_6668_kodo_op.gml";

    let reader = std::io::BufReader::new(std::fs::File::open(filename).unwrap());
    let mut xml_reader = quick_xml::NsReader::from_reader(reader);

    let code_resolver = nusamai_plateau::codelist::Resolver::new();
    let source_url =
        Url::from_file_path(std::fs::canonicalize(Path::new(filename)).unwrap()).unwrap();
    let context = nusamai_citygml::ParseContext::new(source_url, &code_resolver);

    let parsed_data = match CityGMLReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => match height_control_district_toplevel_dispatcher(&mut st) {
            Ok(parsed_data) => parsed_data,
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };


    let height_control_district = parsed_data.height_control_districts.first().unwrap();
    println!("{:?}", height_control_district);

    assert_eq!(parsed_data.height_control_districts.len(), 1);
    assert_eq!(parsed_data.height_control_districts.len(), parsed_data.geometries.len());


    assert_eq!(
        height_control_district.function,
        vec![Code::new("高度地区".to_string(), "18".to_string(),)]
    );

    assert_eq!(
        height_control_district.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2009, 11, 11).unwrap())
    );

    assert_eq!(
        height_control_district.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );
}