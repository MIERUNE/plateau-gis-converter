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
