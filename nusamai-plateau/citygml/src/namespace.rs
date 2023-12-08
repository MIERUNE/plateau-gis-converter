use quick_xml::name::{Namespace, ResolveResult};

/// Normalize a XML namaespace URI to a well-known prefix.
///
/// e.g. "http://www.opengis.net/citygml/2.0" -> "core:"
pub fn normalize_ns_prefix<'a>(ns: &ResolveResult<'a>) -> &'a [u8] {
    match ns {
        ResolveResult::Bound(Namespace(name)) => {
            const OPENGIS_PREFIX: &[u8] = b"http://www.opengis.net/";
            const IUR_PREFIX: &[u8] = b"https://www.geospatial.jp/iur/";
            if name.starts_with(OPENGIS_PREFIX) {
                let s = &name[OPENGIS_PREFIX.len()..];
                // GML 3.1.1
                if s == b"gml" {
                    b"gml:"
                } else if s.starts_with(b"citygml/") {
                    // CityGML 2.0
                    match &s[b"citygml/".len()..] {
                        b"2.0" => b"core:",
                        b"appearance/2.0" => b"app:",
                        b"building/2.0" => b"bldg:",
                        b"bridge/2.0" => b"brid:",
                        b"relief/2.0" => b"dem:",
                        b"cityfurniture/2.0" => b"frn:",
                        b"generics/2.0" => b"gen:",
                        b"cityobjectgroup/2.0" => b"grp:",
                        b"landuse/2.0" => b"luse:",
                        b"transportation/2.0" => b"tran:",
                        b"vegetation/2.0" => b"veg:",
                        b"waterbody/2.0" => b"wtr:",
                        b"tunnel/2.0" => b"tun:",
                        _ => b"unsupported:",
                    }
                } else {
                    b"unsupported:"
                }
            } else if name.starts_with(IUR_PREFIX) {
                let s = &name[IUR_PREFIX.len()..];
                match s {
                    // PLATEAU 3.x
                    b"uro/3.0" => b"uro:",
                    b"urf/3.0" => b"urf:",
                    // PLATEAU 2.x
                    b"uro/2.0" => b"uro:",
                    b"urf/2.0" => b"urf:",
                    _ => b"unsupported:",
                }
            } else {
                // PLATEAU 1.x
                match *name {
                    b"https://www.chisou.go.jp/tiiki/toshisaisei/itoshisaisei/iur/uro/1.5" => b"uro:",
                    b"https://www.chisou.go.jp/tiiki/toshisaisei/itoshisaisei/iur/urf/1.5" => b"urf:",
                    b"http://www.kantei.go.jp/jp/singi/tiiki/toshisaisei/itoshisaisei/iur/uro/1.4" => {
                        b"uro:"
                    }
                    b"http://www.kantei.go.jp/jp/singi/tiiki/toshisaisei/itoshisaisei/iur/urf/1.4" => {
                        b"urf:"
                    }
                    _ => b"unsupported:",
                }
            }
        }
        ResolveResult::Unbound => b"",
        ResolveResult::Unknown(_name) => b"unknown:",
    }
}
