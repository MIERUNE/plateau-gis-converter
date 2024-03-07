use shapefile::dbase;

fn main() {
    // Attribute fields for the features
    // FieldName byte representation cannot exceed 11 bytes
    let table_builder = dbase::TableWriterBuilder::new()
        .add_character_field("name".try_into().unwrap(), 50)
        .add_float_field("lon".try_into().unwrap(), 50, 10)
        .add_float_field("lat".try_into().unwrap(), 50, 10)
        .add_double_field("double".try_into().unwrap());

    // Create all the files needed for the shapefile to be complete (.shp, .shx, .dbf)
    let mut writer = shapefile::Writer::from_path("./tmp.shp", table_builder).unwrap();

    for lat in 20..45 {
        for lon in 120..150 {
            let shape = shapefile::Point::new(lon as f64, lat as f64);
            let mut record = dbase::Record::default();
            record.insert(
                "name".to_owned(),
                dbase::FieldValue::Character(Some("foo".to_owned())),
            );
            record.insert("lon".to_owned(), dbase::FieldValue::Float(Some(lon as f32)));
            record.insert("lat".to_owned(), dbase::FieldValue::Float(Some(lat as f32)));
            record.insert("double".to_owned(), dbase::FieldValue::Double(lat as f64));

            writer.write_shape_and_record(&shape, &record).unwrap();
        }
    }
}
