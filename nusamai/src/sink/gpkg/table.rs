use indexmap::IndexMap;
use nusamai_citygml::schema::{Attribute, Schema, TypeDef, TypeRef};
use nusamai_gpkg::table::{ColumnInfo, TableInfo};

/// Check the schema, and prepare the information for the SQLite table
pub fn schema_to_table_infos(schema: &Schema) -> IndexMap<String, TableInfo> {
    let mut table_infos = IndexMap::<String, TableInfo>::new();

    schema.types.iter().for_each(|(name, ty)| {
        table_infos.insert(
            name.clone(),
            TableInfo {
                name: name.clone(),
                has_geometry: matches!(ty, TypeDef::Feature(_)),
                columns: typedef_to_columns(ty),
            },
        );
    });

    table_infos
}

fn typedef_to_columns(ty: &TypeDef) -> Vec<ColumnInfo> {
    let mut columns: Vec<ColumnInfo> = vec![];
    match ty {
        TypeDef::Feature(feat_td) => {
            // Note: `feat_td.additional_attributes` is expected to be false (handled by the transformer in the earlier step)
            feat_td.attributes.iter().for_each(|(attr_name, attr)| {
                if let Some(column) = attribute_to_column(attr_name, attr) {
                    columns.push(column);
                }
            });
        }
        TypeDef::Data(data_td) => {
            data_td.attributes.iter().for_each(|(attr_name, attr)| {
                if let Some(column) = attribute_to_column(attr_name, attr) {
                    columns.push(column);
                }
            });
        }
        TypeDef::Property(prop_td) => {
            // Note: expected to be handled by the tranformer in the earlier step
            log::warn!(
                "TypeDef::Property - Not supported: {} members ({:?}, etc.)",
                prop_td.members.len(),
                prop_td
                    .members
                    .iter()
                    .map(|m| &m.type_ref)
                    .take(3)
                    .collect::<Vec<_>>()
            );
        }
    };

    columns
}

fn attribute_to_column(attr_name: &str, attr: &Attribute) -> Option<ColumnInfo> {
    // Note: `attr.max_occurs` is expected to be 1 (handled by the transformer in the earlier step)
    match &attr.type_ref {
        TypeRef::String => Some(ColumnInfo {
            name: attr_name.to_string(),
            data_type: "TEXT".into(),
            mime_type: None,
        }),
        TypeRef::Code => Some(ColumnInfo {
            name: attr_name.to_string(),
            data_type: "TEXT".into(),
            mime_type: None,
        }),
        TypeRef::Integer | TypeRef::NonNegativeInteger => Some(ColumnInfo {
            name: attr_name.to_string(),
            data_type: "INTEGER".into(),
            mime_type: None,
        }),
        TypeRef::Double => Some(ColumnInfo {
            name: attr_name.to_string(),
            data_type: "REAL".into(),
            mime_type: None,
        }),
        TypeRef::Boolean => Some(ColumnInfo {
            name: attr_name.to_string(),
            data_type: "BOOLEAN".into(),
            mime_type: None,
        }),
        TypeRef::JsonString => Some(ColumnInfo {
            name: attr_name.to_string(),
            data_type: "TEXT".into(),
            mime_type: None,
        }),
        TypeRef::URI => Some(ColumnInfo {
            name: attr_name.to_string(),
            data_type: "TEXT".into(),
            mime_type: None,
        }),
        TypeRef::Date => Some(ColumnInfo {
            name: attr_name.to_string(),
            data_type: "DATE".into(),
            mime_type: None,
        }),
        TypeRef::DateTime => Some(ColumnInfo {
            name: attr_name.to_string(),
            data_type: "TEXT".into(),
            mime_type: None,
        }),
        TypeRef::Measure => Some(ColumnInfo {
            name: attr_name.to_string(),
            data_type: "REAL".into(),
            mime_type: None,
        }),
        TypeRef::Point => {
            // TODO: implement
            // Point struct currently does not contain any data
            log::warn!(
                "TypeDef::Feature - Unsupported attribute type: {:?} ('{}')",
                attr.type_ref,
                attr_name
            );
            None
        }
        TypeRef::Named(_name) => {
            // Note: expected to be handled by the tranformer in the earlier step (flatten)
            log::warn!(
                "TypeDef::Feature - Unsupported attribute type: {:?} ('{}')",
                attr.type_ref,
                attr_name
            );
            None
        }
        TypeRef::Unknown => {
            log::warn!(
                "TypeDef::Feature - Unsupported attribute type: {:?} ('{}')",
                attr.type_ref,
                attr_name
            );
            None
        }
    }
}
