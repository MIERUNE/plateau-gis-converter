/// Top-level geometry types currently emitted by the GeoPackage sink.
/// This is not an exhaustive list of geometry types defined by the GeoPackage standard.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GpkgGeometryType {
    MultiPoint,
    MultiLineString,
    MultiPolygon,
}

impl GpkgGeometryType {
    pub const fn sql_name(self) -> &'static str {
        match self {
            Self::MultiPoint => "MULTIPOINT",
            Self::MultiLineString => "MULTILINESTRING",
            Self::MultiPolygon => "MULTIPOLYGON",
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TableInfo {
    pub name: String,
    pub has_geometry: bool,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug, PartialEq)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub mime_type: Option<String>,
}
