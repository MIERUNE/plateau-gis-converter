use core::panic;

use macros::citygml_feature;

#[test]
fn generics() {
    use nusamai_citygml::values;
    use nusamai_citygml::CityGMLElement;
    use nusamai_citygml::Value;

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
    match nusamai_citygml::CityGMLReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => {
            demo.parse(&mut st).unwrap();
        }
        Err(e) => panic!("Err: {:?}", e),
    };

    let obj = demo.generic_attribute.into_object().unwrap();
    if let Value::Data(data) = obj {
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
            Value::URI(values::URI::new("https://foo.com/hoge"))
        );

        let Value::Data(set1) = &data.attributes["set1"] else {
            panic!("expected data");
        };
        assert_eq!(set1.attributes["fizz"], Value::String("buzz".to_string()));
        assert_eq!(set1.attributes["d1"], Value::Double(123.45));
    } else {
        panic!("expected data");
    }
}
