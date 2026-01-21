use quick_xml::name::{Namespace, ResolveResult};

pub const GML31_NS: Namespace = Namespace(b"http://www.opengis.net/gml");
pub const APP_2_NS: Namespace = Namespace(b"http://www.opengis.net/citygml/appearance/2.0");
pub const XLINK_NS: Namespace = Namespace(b"http://www.w3.org/1999/xlink");

/// Normalizes `quick_xml::name::ResolveResult` to the well-known prefix.
///
/// e.g. `"http://www.opengis.net/citygml/2.0"` -> `"core:"`
#[inline]
pub fn wellknown_prefix_from_nsres<'a>(ns: &ResolveResult<'a>) -> &'a [u8] {
    match ns {
        ResolveResult::Bound(Namespace(name)) => {
            if let Some(http) = name.strip_prefix(b"http") {
                if let Some(http_www) = http.strip_prefix(b"://www.") {
                    if let Some(opengis) = http_www.strip_prefix(b"opengis.net/") {
                        // GML 3.1.1
                        if opengis == b"gml" {
                            b"gml:"
                        } else if let Some(citygml) = opengis.strip_prefix(b"citygml/") {
                            // CityGML 2.0
                            match citygml {
                                b"2.0" => b"core:",
                                b"appearance/2.0" => b"app:",
                                b"building/2.0" => b"bldg:",
                                b"generics/2.0" => b"gen:",
                                b"transportation/2.0" => b"tran:",
                                b"cityfurniture/2.0" => b"frn:",
                                b"vegetation/2.0" => b"veg:",
                                b"bridge/2.0" => b"brid:",
                                b"relief/2.0" => b"dem:",
                                b"landuse/2.0" => b"luse:",
                                b"waterbody/2.0" => b"wtr:",
                                b"tunnel/2.0" => b"tun:",
                                b"cityobjectgroup/2.0" => b"grp:",
                                b"texturedsurface/2.0" => b"tex:", // deprecated
                                _ => b"unsupported:",
                            }
                        } else {
                            b"unsupported:"
                        }
                    } else if http_www == b"w3.org/1999/xlink" {
                        // xlink
                        b"xlink:"
                    } else {
                        // PLATEAU 1.x
                        match http_www {
                            b"kantei.go.jp/jp/singi/tiiki/toshisaisei/itoshisaisei/iur/uro/1.4" => {
                                b"uro:"
                            }
                            b"kantei.go.jp/jp/singi/tiiki/toshisaisei/itoshisaisei/iur/urf/1.4" => {
                                b"urf:"
                            }
                            _ => b"unsupported:",
                        }
                    }
                } else if let Some(https_www) = http.strip_prefix(b"s://www.") {
                    if let Some(iur) = https_www.strip_prefix(b"geospatial.jp/iur/ur") {
                        match iur {
                            // PLATEAU 5.x
                            b"o/3.2" => b"uro:",
                            b"f/3.2" => b"urf:",
                            // PLATEAU 4.x
                            b"o/3.1" => b"uro:",
                            b"f/3.1" => b"urf:",
                            // PLATEAU 3.x
                            b"o/3.0" => b"uro:",
                            b"f/3.0" => b"urf:",
                            // PLATEAU 2.x
                            b"o/2.0" => b"uro:",
                            b"f/2.0" => b"urf:",
                            _ => b"unsupported:",
                        }
                    } else {
                        // PLATEAU 1.x
                        match https_www {
                            b"chisou.go.jp/tiiki/toshisaisei/itoshisaisei/iur/uro/1.5" => b"uro:",
                            b"chisou.go.jp/tiiki/toshisaisei/itoshisaisei/iur/urf/1.5" => b"urf:",
                            _ => b"unsupported:",
                        }
                    }
                } else {
                    b"unsupported:"
                }
            } else if name == b"urn:oasis:names:tc:ciq:xsdschema:xAL:2.0" {
                // OASIS xAL 2.0
                b"xAL:"
            } else {
                b"unsupported:"
            }
        }
        ResolveResult::Unbound => b"",
        ResolveResult::Unknown(_name) => b"unknown:",
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn normalized_prefix() {
        use quick_xml::{events::Event, NsReader};

        use super::*;

        let data = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <core2ns:core
            xmlns:gml31ns="http://www.opengis.net/gml"
            xmlns:core2ns="http://www.opengis.net/citygml/2.0"
            xmlns:grp2ns="http://www.opengis.net/citygml/cityobjectgroup/2.0"
            xmlns:bldg2ns="http://www.opengis.net/citygml/building/2.0"
            xmlns:brid2ns="http://www.opengis.net/citygml/bridge/2.0"
            xmlns:tran2ns="http://www.opengis.net/citygml/transportation/2.0"
            xmlns:frn2ns="http://www.opengis.net/citygml/cityfurniture/2.0"
            xmlns:wtr2ns="http://www.opengis.net/citygml/waterbody/2.0"
            xmlns:veg2ns="http://www.opengis.net/citygml/vegetation/2.0"
            xmlns:tun2ns="http://www.opengis.net/citygml/tunnel/2.0"
            xmlns:tex2ns="http://www.opengis.net/citygml/texturedsurface/2.0"
            xmlns:app2ns="http://www.opengis.net/citygml/appearance/2.0"
            xmlns:gen2ns="http://www.opengis.net/citygml/generics/2.0"
            xmlns:dem2ns="http://www.opengis.net/citygml/relief/2.0"
            xmlns:luse2ns="http://www.opengis.net/citygml/landuse/2.0"
            xmlns:uro32ns="https://www.geospatial.jp/iur/uro/3.2"
            xmlns:urf32ns="https://www.geospatial.jp/iur/urf/3.2"
            xmlns:uro31ns="https://www.geospatial.jp/iur/uro/3.1"
            xmlns:urf31ns="https://www.geospatial.jp/iur/urf/3.1"
            xmlns:uro30ns="https://www.geospatial.jp/iur/uro/3.0"
            xmlns:urf30ns="https://www.geospatial.jp/iur/urf/3.0"
            xmlns:uro2ns="https://www.geospatial.jp/iur/uro/2.0"
            xmlns:urf2ns="https://www.geospatial.jp/iur/urf/3.0"
            xmlns:uro15ns="https://www.chisou.go.jp/tiiki/toshisaisei/itoshisaisei/iur/uro/1.5"
            xmlns:urf15ns="https://www.chisou.go.jp/tiiki/toshisaisei/itoshisaisei/iur/urf/1.5"
            xmlns:uro14ns="http://www.kantei.go.jp/jp/singi/tiiki/toshisaisei/itoshisaisei/iur/uro/1.4"
            xmlns:urf14ns="http://www.kantei.go.jp/jp/singi/tiiki/toshisaisei/itoshisaisei/iur/urf/1.4"
            xmlns:xAL2ns="urn:oasis:names:tc:ciq:xsdschema:xAL:2.0"
            xmlns:xlinkns="http://www.w3.org/1999/xlink"
        >
            <!-- namespace_prefix:wellknown_prefix -->
            <foobar:unknown />
            <gml31ns:gml />
            <core2ns:core />
            <grp2ns:grp />
            <bldg2ns:bldg />
            <brid2ns:brid />
            <tran2ns:tran />
            <frn2ns:frn />
            <wtr2ns:wtr />
            <veg2ns:veg />
            <tun2ns:tun />
            <tex2ns:tex />
            <app2ns:app />
            <gen2ns:gen />
            <dem2ns:dem />
            <luse2ns:luse />
            <uro32ns:uro />
            <urf32ns:urf />
            <uro31ns:uro />
            <urf31ns:urf />
            <uro30ns:uro />
            <urf30ns:urf />
            <uro2ns:uro />
            <urf2ns:urf />
            <uro15ns:uro />
            <urf15ns:urf />
            <uro14ns:uro />
            <urf14ns:urf />
            <xAL2ns:xAL />
            <xlinkns:xlink />
        </core2ns:core>
        "#;

        let mut reader = NsReader::from_str(data);
        let config = reader.config_mut();
        config.trim_text(true);
        config.expand_empty_elements = true;
        loop {
            match reader.read_resolved_event() {
                Ok((ns, Event::Start(ref e))) => {
                    let wellknown = std::str::from_utf8(wellknown_prefix_from_nsres(&ns)).unwrap();
                    let localname = e.local_name();
                    let expected = String::from_utf8_lossy(localname.as_ref()) + ":";
                    assert_eq!(expected, wellknown);
                }
                Ok((_, Event::Eof)) => break,
                Ok(_) => {}
                Err(e) => panic!("{e:?}"),
            }
        }
    }
}
