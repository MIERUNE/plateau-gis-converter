use chrono::NaiveDate;
use citygml::{CityGMLElement, CityGMLReader, ParseContext};

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
            assert_eq!(root.date, NaiveDate::from_ymd_opt(2019, 3, 21));
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
