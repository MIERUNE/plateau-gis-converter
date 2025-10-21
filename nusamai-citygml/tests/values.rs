use nusamai_citygml::{
    citygml_feature, values, CityGmlElement, CityGmlReader, Color, ColorPlusOpacity, Date, Measure,
    ParseContext, ParseError, Uri, Value,
};
use url::Url;

#[test]
fn parse_date() {
    #[derive(CityGmlElement, Default)]
    struct Root {
        #[citygml(path = b"date")]
        date: Option<chrono::NaiveDate>,
    }

    let mut xml_reader = quick_xml::NsReader::from_reader(std::io::Cursor::new(
        r#"<root><date>2019-03-21</date></root>"#,
    ));
    let context = ParseContext::default();
    match CityGmlReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => {
            let mut root = Root::default();
            root.parse(&mut st).unwrap();
            assert!(root.date.is_some());
            assert_eq!(root.date, Date::from_ymd_opt(2019, 3, 21));
        }
        Err(e) => panic!("Err: {e:?}"),
    }
}

#[test]
fn parse_boolean() {
    #[derive(CityGmlElement, Default)]
    struct Root {
        #[citygml(path = b"boolean")]
        bools: Vec<bool>,
    }

    let mut xml_reader = quick_xml::NsReader::from_reader(std::io::Cursor::new(
        r#"
        <root>
            <boolean>true</boolean>
            <boolean>True</boolean>
            <boolean>TRUE</boolean>
            <boolean>1</boolean>
            <boolean>false</boolean>
            <boolean>False</boolean>
            <boolean>FALSE</boolean>
            <boolean>0</boolean>
        </root>
        "#,
    ));
    let context = ParseContext::default();
    match CityGmlReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => {
            let mut root = Root::default();
            root.parse(&mut st).unwrap();
            assert_eq!(
                root.bools,
                [true, true, true, true, false, false, false, false]
            );
        }
        Err(e) => panic!("Err: {e:?}"),
    }
}

#[test]
fn parse_basic_types() {
    #[derive(CityGmlElement, Debug, Default)]
    struct Root {
        #[citygml(path = b"int")]
        int: Option<i64>,
        #[citygml(path = b"uint")]
        uint: Option<u64>,
        #[citygml(path = b"string")]
        string: Option<String>,
        #[citygml(path = b"float")]
        float: Option<f64>,
        #[citygml(path = b"bool")]
        bool: Option<bool>,
        #[citygml(path = b"measure")]
        measure: Option<Measure>,
        #[citygml(path = b"uri")]
        uri: Option<Uri>,
        #[citygml(path = b"date")]
        date: Option<Date>,
        #[citygml(path = b"color")]
        color: Option<Color>,
        #[citygml(path = b"color-plus-opacity")]
        color_plus_opacity: Option<ColorPlusOpacity>,
        #[citygml(path = b"boxed")]
        boxed: Box<Option<Root>>,
    }

    let mut xml_reader = quick_xml::NsReader::from_reader(std::io::Cursor::new(
        r#"
        <root>
            <int>-1234567</int>
            <uint>1234567</uint>
            <float>1234.567</float>
            <string>hello</string>
            <bool>true</bool>
            <measure>3.4</measure>
            <date>2000-12-25</date>
            <uri>https://example.com/foo?bar=2000</uri>
            <color>1.0 0.5 0.0</color>
            <color-plus-opacity>1.0 0.5 0.0 0.8</color-plus-opacity>
            <boxed>
                <int>123</int>
                <float>1234.5</float>
            </boxed>
        </root>
        "#,
    ));
    let context = ParseContext::default();
    match CityGmlReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => {
            let mut root = Root::default();
            root.parse(&mut st).unwrap();
            assert_eq!(root.int.unwrap(), -1234567);
            assert_eq!(root.uint.unwrap(), 1234567);
            assert!((root.float.unwrap() - 1234.567).abs() < 1e-15);
            assert!((root.measure.as_ref().unwrap().value() - 3.4).abs() < 1e-15);
            assert_eq!(root.string.as_ref().unwrap(), "hello");
            assert_eq!(
                root.uri.as_ref().unwrap().value().to_string(),
                "https://example.com/foo?bar=2000"
            );
            assert!(root.bool.unwrap());
            assert_eq!(root.color.unwrap(), Color::new(1.0, 0.5, 0.0));
            assert_eq!(
                root.color_plus_opacity.unwrap(),
                ColorPlusOpacity::new(1.0, 0.5, 0.0, 0.8)
            );
            root.into_object();
        }
        Err(e) => panic!("Err: {e:?}"),
    }
}

fn expect_invalid<T: CityGmlElement + Default>(xml: &str) {
    let mut xml_reader = quick_xml::NsReader::from_reader(std::io::Cursor::new(xml));
    let context = ParseContext::default();
    match CityGmlReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => {
            let mut root = T::default();
            match root.parse(&mut st) {
                Err(ParseError::InvalidValue(_)) => {}
                _ => panic!("Should be invalid value"),
            }
        }
        Err(e) => panic!("Err: {e:?}"),
    }
}

#[test]
fn parse_basic_types_invalid() {
    expect_invalid::<u64>(r#"<root>123.456</root>"#); // not integer
    expect_invalid::<u64>(r#"<root>-123</root>"#); // not unsigned
    expect_invalid::<i64>(r#"<root>123.456</root>"#); // not integer
    expect_invalid::<f64>(r#"<root>foo</root>"#); // not float
    expect_invalid::<bool>(r#"<root>123</root>"#); // not boolean
    expect_invalid::<Measure>(r#"<root>foo</root>"#); // not float
    expect_invalid::<Date>(r#"<root>2022-13-00</root>"#); // not valid date
    expect_invalid::<Color>(r#"<root>0.0 0.0 0.0 0.0</root>"#); // not valid color
    expect_invalid::<ColorPlusOpacity>(r#"<root>0.0 0.0 0.0 0.0 1.0</root>"#); // not valid colorPlusOpacity
}

#[test]
fn parse_duplicate_content() {
    let mut xml_reader = quick_xml::NsReader::from_reader(std::io::Cursor::new(
        r#"
    <root>
        <only_once>123</only_once>
        <only_once>234</only_once>
    </root>
    "#,
    ));

    #[derive(CityGmlElement, Default)]
    struct Root {
        #[citygml(path = b"only_once")]
        only_once: Option<i64>,
    }
    let context = ParseContext::default();
    match CityGmlReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => {
            let mut root = Root::default();
            match root.parse(&mut st) {
                Err(ParseError::SchemaViolation(_)) => {}
                _ => panic!("Should be schema violation"),
            }
        }
        Err(e) => panic!("Err: {e:?}"),
    }
}

#[test]
fn generics() {
    #[citygml_feature(name = "foo:foo")]
    struct Demo {
        // The field `generic_attributes` will be automatically generated.
    }

    let data = r#"
        <foo xmlns:gen="http://www.opengis.net/citygml/generics/2.0">
            <gen:stringAttribute name="s1">
                <gen:value>foo</gen:value>
            </gen:stringAttribute>
            <gen:stringAttribute>
                <gen:name>s2</gen:name>
                <gen:value>bar</gen:value>
            </gen:stringAttribute>
            <gen:intAttribute name="i1">
                <gen:value>10</gen:value>
            </gen:intAttribute>
            <gen:doubleAttribute name="d1">
                <gen:value>10.2</gen:value>
            </gen:doubleAttribute>
            <gen:measureAttribute name="m1">
                <gen:value>15.1</gen:value>
            </gen:measureAttribute>
            <gen:dateAttribute name="date1">
                <gen:value>2011-11-12</gen:value>
            </gen:dateAttribute>
            <gen:uriAttribute name="u1">
                <gen:value>https://foo.com/hoge</gen:value>
            </gen:uriAttribute>
            <gen:genericAttributeSet name="set1">
                <gen:stringAttribute>
                    <gen:name>fizz</gen:name>
                    <gen:value>buzz</gen:value>
                </gen:stringAttribute>
                <gen:doubleAttribute name="d1">
                    <gen:value>123.45</gen:value>
                </gen:doubleAttribute>
            </gen:genericAttributeSet>
        </foo>
    "#;

    let reader = std::io::BufReader::new(std::io::Cursor::new(data));
    let mut xml_reader = quick_xml::NsReader::from_reader(reader);
    let context = nusamai_citygml::ParseContext::default();
    let mut demo = Demo::default();
    match nusamai_citygml::CityGmlReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => {
            demo.parse(&mut st).unwrap();
        }
        Err(e) => panic!("Err: {e:?}"),
    };

    let obj = demo.generic_attribute.into_object().unwrap();
    if let Value::Object(data) = obj {
        assert_eq!(data.attributes["s1"], Value::String("foo".to_string()));
        assert_eq!(data.attributes["s2"], Value::String("bar".to_string()));
        assert_eq!(data.attributes["i1"], Value::Integer(10));
        assert_eq!(data.attributes["d1"], Value::Double(10.2));
        assert_eq!(
            data.attributes["date1"],
            Value::Date(values::Date::from_ymd_opt(2011, 11, 12).unwrap())
        );
        assert_eq!(
            data.attributes["m1"],
            Value::Measure(values::Measure::new(15.1))
        );
        assert_eq!(
            data.attributes["u1"],
            Value::Uri(values::Uri::new(
                Url::parse("https://foo.com/hoge").unwrap()
            ))
        );

        let Value::Object(set1) = &data.attributes["set1"] else {
            panic!("expected object");
        };
        assert_eq!(set1.attributes["fizz"], Value::String("buzz".to_string()));
        assert_eq!(set1.attributes["d1"], Value::Double(123.45));
    } else {
        panic!("expected data");
    }
}

#[test]
fn feature_type_issue_with_bounded_by_structure() {
    use nusamai_citygml::geometry::GeometryRefs;

    #[derive(CityGmlElement, Default)]
    struct Root {
        #[citygml(path = b"core:cityObjectMember")]
        members: Vec<Member>,
    }

    #[derive(CityGmlElement, Default)]
    struct Member {
        #[citygml(path = b"bldg:Building")]
        building: Option<Building>,
    }

    #[derive(CityGmlElement, Default)]
    struct Building {
        #[citygml(path = b"bldg:boundedBy")]
        bounded_by: Vec<BoundedBy>,
    }

    #[derive(CityGmlElement, Default)]
    struct BoundedBy {
        #[citygml(path = b"bldg:GroundSurface")]
        ground_surface: Vec<GroundSurface>,
    }

    #[derive(CityGmlElement, Default)]
    struct GroundSurface {
        #[citygml(geom = b"bldg")]
        lod2_multi_surface: GeometryRefs,
    }

    let xml = r#"
        <core:CityModel xmlns:bldg="http://www.opengis.net/citygml/building/2.0" xmlns:core="http://www.opengis.net/citygml/2.0" xmlns:gml="http://www.opengis.net/gml">
            <core:cityObjectMember>
                <bldg:Building gml:id="test-building-id">
                    <bldg:boundedBy>
                        <bldg:GroundSurface>
                            <bldg:lod2MultiSurface>
                                <gml:MultiSurface>
                                    <gml:surfaceMember>
                                        <gml:Polygon>
                                            <gml:exterior>
                                                <gml:LinearRing>
                                                    <gml:posList>0.0 0.0 0.0 1.0 0.0 0.0 1.0 1.0 0.0 0.0 1.0 0.0 0.0 0.0 0.0</gml:posList>
                                                </gml:LinearRing>
                                            </gml:exterior>
                                        </gml:Polygon>
                                    </gml:surfaceMember>
                                </gml:MultiSurface>
                            </bldg:lod2MultiSurface>
                        </bldg:GroundSurface>
                    </bldg:boundedBy>
                </bldg:Building>
            </core:cityObjectMember>
        </core:CityModel>
    "#;

    let mut xml_reader = quick_xml::NsReader::from_reader(std::io::Cursor::new(xml));
    let context = ParseContext::default();
    match CityGmlReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => {
            let mut root = Root::default();
            root.parse(&mut st).unwrap();

            let geom_ref = &root.members[0].building.as_ref().unwrap().bounded_by[0].ground_surface
                [0]
            .lod2_multi_surface[0];
            // feature_type should be the element containing the geometry (GroundSurface),
            // not the parent feature (Building)
            assert_eq!(
                geom_ref.feature_type,
                Some("bldg:GroundSurface".to_string())
            );
            // feature_id should be from the parent with gml:id (Building)
            assert_eq!(geom_ref.feature_id, Some("test-building-id".to_string()));
        }
        Err(e) => panic!("Err: {e:?}"),
    }
}
