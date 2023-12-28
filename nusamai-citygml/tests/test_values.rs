use nusamai_citygml::{
    CityGMLElement, CityGMLReader, Date, Measure, ParseContext, ParseError, URI,
};

#[test]
fn parse_date() {
    #[derive(CityGMLElement, Default)]
    struct Root {
        #[citygml(path = b"date")]
        date: Option<chrono::NaiveDate>,
    }

    let mut xml_reader = quick_xml::NsReader::from_reader(std::io::Cursor::new(
        r#"<root><date>2019-03-21</date></root>"#,
    ));
    let context = ParseContext::default();
    match CityGMLReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => {
            let mut root = Root::default();
            root.parse(&mut st).unwrap();
            assert!(root.date.is_some());
            assert_eq!(root.date, Date::from_ymd_opt(2019, 3, 21));
        }
        Err(e) => panic!("Err: {:?}", e),
    }
}

#[test]
fn parse_boolean() {
    #[derive(CityGMLElement, Default)]
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
    match CityGMLReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => {
            let mut root = Root::default();
            root.parse(&mut st).unwrap();
            assert_eq!(
                root.bools,
                [true, true, true, true, false, false, false, false]
            );
        }
        Err(e) => panic!("Err: {:?}", e),
    }
}

#[test]
fn parse_basic_types() {
    #[derive(CityGMLElement, Default)]
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
        uri: Option<URI>,
        #[citygml(path = b"date")]
        date: Option<Date>,
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
        </root>
        "#,
    ));
    let context = ParseContext::default();
    match CityGMLReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => {
            let mut root = Root::default();
            root.parse(&mut st).unwrap();
            assert_eq!(root.int.unwrap(), -1234567);
            assert_eq!(root.uint.unwrap(), 1234567);
            assert!((root.float.unwrap() - 1234.567).abs() < 1e-15);
            assert!((root.measure.as_ref().unwrap().value - 3.4).abs() < 1e-15);
            assert_eq!(root.string.as_ref().unwrap(), "hello");
            assert_eq!(
                root.uri.as_ref().unwrap().0,
                "https://example.com/foo?bar=2000"
            );
            assert!(root.bool.unwrap());
            root.into_object();
        }
        Err(e) => panic!("Err: {:?}", e),
    }
}

fn expect_invalid<T: CityGMLElement + Default>(xml: &str) {
    let mut xml_reader = quick_xml::NsReader::from_reader(std::io::Cursor::new(xml));
    let context = ParseContext::default();
    match CityGMLReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => {
            let mut root = T::default();
            match root.parse(&mut st) {
                Err(ParseError::InvalidValue(_)) => {}
                _ => panic!("Should be invalid value"),
            }
        }
        Err(e) => panic!("Err: {:?}", e),
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

    #[derive(CityGMLElement, Default)]
    struct Root {
        #[citygml(path = b"only_once")]
        only_once: Option<i64>,
    }
    let context = ParseContext::default();
    match CityGMLReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => {
            let mut root = Root::default();
            match root.parse(&mut st) {
                Err(ParseError::SchemaViolation(_)) => {}
                _ => panic!("Should be schema violation"),
            }
        }
        Err(e) => panic!("Err: {:?}", e),
    }
}
