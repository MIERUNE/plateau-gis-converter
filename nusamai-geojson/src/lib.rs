use geojson::GeoJson;

pub fn read(geojson_str: &str) -> GeoJson {
    let geojson = geojson_str.parse::<GeoJson>().unwrap();
    geojson
}

// test
#[cfg(test)]
mod tests {
    use geojson::GeoJson;

    #[test]
    fn read() {
        let geojson_str = r#"
            {
                "type": "Feature",
                "properties": {
                    "name": "北海道庁"
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [141.34694444, 43.06416667]
                }
            }
            "#;
        let geojson = super::read(geojson_str);
        let serialized = GeoJson::from(geojson).to_string();

        assert_eq!(
            serialized,
            r#"{"geometry":{"coordinates":[141.34694444,43.06416667],"type":"Point"},"properties":{"name":"北海道庁"},"type":"Feature"}"#.to_string()
        );
    }
}
