use indexmap::IndexMap;
use nusamai_citygml::schema::{Schema, TypeDef, TypeRef};
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
            // Note: `feat_td.additional_attributes` is expected to be false (treated by the transformer already)
            feat_td.attributes.iter().for_each(|(attr_name, attr)|
                // Note: `attr.max_occurs` is expected to be 1 (treated by the transformer already)
                match &attr.type_ref {
                    TypeRef::String => {
                        columns.push(ColumnInfo {
                            name: attr_name.clone(),
                            data_type: "TEXT".into(),
                            mime_type: None,
                        });
                    }
                    TypeRef::Code => {
                        columns.push(ColumnInfo {
                            name: attr_name.clone(),
                            data_type: "TEXT".into(),
                            mime_type: None,
                        });
                    }
                    TypeRef::Integer | TypeRef::NonNegativeInteger => {
                        columns.push(ColumnInfo {
                            name: attr_name.clone(),
                            data_type: "INTEGER".into(),
                            mime_type: None,
                        });
                    }
                    TypeRef::Double => {
                        columns.push(ColumnInfo {
                            name: attr_name.clone(),
                            data_type: "REAL".into(),
                            mime_type: None,
                        });
                    }
                    TypeRef::Boolean => {
                        columns.push(ColumnInfo {
                            name: attr_name.clone(),
                            data_type: "BOOLEAN".into(),
                            mime_type: None,
                        });
                    }
                    TypeRef::JsonString => {
                        columns.push(ColumnInfo {
                            name: attr_name.clone(),
                            data_type: "TEXT".into(),
                            mime_type: None,
                        });
                    }
                    TypeRef::URI => {
                        columns.push(ColumnInfo {
                            name: attr_name.clone(),
                            data_type: "TEXT".into(),
                            mime_type: None,
                        });
                    }
                    TypeRef::Date => {
                        columns.push(ColumnInfo {
                            name: attr_name.clone(),
                            data_type: "DATE".into(),
                            mime_type: None,
                        });
                    }
                    TypeRef::DateTime => {
                        columns.push(ColumnInfo {
                            name: attr_name.clone(),
                            data_type: "TEXT".into(),
                            mime_type: None,
                        });
                    }
                    TypeRef::Measure => {
                        columns.push(ColumnInfo {
                            name: attr_name.clone(),
                            data_type: "REAL".into(),
                            mime_type: None,
                        });
                    }
                    TypeRef::Point => {
                        // TODO: implement
                        // Point struct currently does not contain any data
                        log::warn!(
                            "TypeDef::Feature - Unsupported attribute type: {:?} ('{}')",
                            attr.type_ref,
                            attr_name
                        );
                    }
                    TypeRef::Named(_name) => {
                        // TODO: Implement
                        log::warn!(
                            "TypeDef::Feature - Unsupported attribute type: {:?} ('{}')",
                            attr.type_ref,
                            attr_name
                        );
                    },
                    TypeRef::Unknown => {
                        log::warn!(
                            "TypeDef::Feature - Unsupported attribute type: {:?} ('{}')",
                            attr.type_ref,
                            attr_name
                        );
                    }
                });
        }
        TypeDef::Data(data_td) => {
            // TODO: implement
            log::warn!(
                "TypeDef::Data - Not supported yet: {:?}",
                data_td.attributes.values()
            );
        }
        TypeDef::Property(prop_td) => {
            // TODO: implement
            log::warn!(
                "TypeDef::Property - Not supported yet: {} members ({:?}, etc.)",
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
