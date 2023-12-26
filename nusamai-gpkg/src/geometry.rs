/// GeoPackage SQL Geometry Binary Format
/// cf. https://www.geopackage.org/spec130/#gpb_format
fn to_binary() {
    let mut header: Vec<u8> = vec![];

    // Magic number
    header.extend_from_slice(&[0x47, 0x50]);

    // Version
    header.push(0x00);

    // Flags
    // bit layout: R=0, R=0, X=0, Y=0, E=0, E=0, E=0, B=1
    header.push(0b00000001);

    // SRS ID
    header.extend_from_slice(i32::to_le_bytes(4326));

    // Envelope
}
