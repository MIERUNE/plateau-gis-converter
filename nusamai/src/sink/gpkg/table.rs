use indexmap::IndexMap;
use nusamai_citygml::schema::{Attribute, Schema, TypeDef, TypeRef};
use nusamai_gpkg::table::{ColumnInfo, TableInfo};

/// Check the schema, and prepare the information for the SQLite table
#[must_use]
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

#[must_use]
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
        TypeDef::Property(_) => {
            // Do nothing
        }
    };

    columns
}

#[must_use]
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
        TypeRef::JsonString(_) => Some(ColumnInfo {
            name: attr_name.to_string(),
            data_type: "TEXT".into(),
            mime_type: Some("application/json".into()),
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

#[cfg(test)]
mod tests {
    use nusamai_citygml::schema::{DataTypeDef, FeatureTypeDef, PropertyTypeDef};

    use super::*;

    #[test]
    fn test_schema_to_table_infos() {
        let mut types = IndexMap::with_hasher(ahash::RandomState::default());

        let mut attrs_1 = IndexMap::with_hasher(ahash::RandomState::default());
        attrs_1.insert("text".into(), Attribute::new(TypeRef::String));
        attrs_1.insert("number".into(), Attribute::new(TypeRef::Integer));
        attrs_1.insert("date".into(), Attribute::new(TypeRef::Date));
        types.insert(
            "feature".into(),
            TypeDef::Feature(FeatureTypeDef {
                attributes: attrs_1,
                additional_attributes: false,
            }),
        );

        let mut attrs_2 = IndexMap::with_hasher(ahash::RandomState::default());
        attrs_2.insert(
            "json".into(),
            Attribute::new(TypeRef::JsonString(Attribute::new(TypeRef::String).into())),
        );
        attrs_2.insert("measure".into(), Attribute::new(TypeRef::Measure));
        attrs_2.insert("bool".into(), Attribute::new(TypeRef::Boolean));
        types.insert(
            "data".into(),
            TypeDef::Data(DataTypeDef {
                attributes: attrs_2,
                additional_attributes: false,
            }),
        );
        let srs_id = 4326;
        let schema = Schema {
            types,
            epsg: Some(srs_id),
        };

        let table_infos = schema_to_table_infos(&schema);

        assert_eq!(table_infos.len(), 2);
        assert_eq!(
            table_infos.get("feature").unwrap(),
            &TableInfo {
                name: "feature".into(),
                has_geometry: true,
                columns: vec![
                    ColumnInfo {
                        name: "text".into(),
                        data_type: "TEXT".into(),
                        mime_type: None,
                    },
                    ColumnInfo {
                        name: "number".into(),
                        data_type: "INTEGER".into(),
                        mime_type: None,
                    },
                    ColumnInfo {
                        name: "date".into(),
                        data_type: "DATE".into(),
                        mime_type: None,
                    },
                ]
            }
        );
        assert_eq!(
            table_infos.get("data").unwrap(),
            &TableInfo {
                name: "data".into(),
                has_geometry: false,
                columns: vec![
                    ColumnInfo {
                        name: "json".into(),
                        data_type: "TEXT".into(),
                        mime_type: Some("application/json".into()),
                    },
                    ColumnInfo {
                        name: "measure".into(),
                        data_type: "REAL".into(),
                        mime_type: None,
                    },
                    ColumnInfo {
                        name: "bool".into(),
                        data_type: "BOOLEAN".into(),
                        mime_type: None,
                    },
                ]
            }
        );
    }

    #[test]
    fn test_typedef_to_columns() {
        let mut attrs_1 = IndexMap::with_hasher(ahash::RandomState::default());
        attrs_1.insert("text".into(), Attribute::new(TypeRef::String));
        attrs_1.insert("number".into(), Attribute::new(TypeRef::Integer));
        attrs_1.insert("date".into(), Attribute::new(TypeRef::Date));
        let typedef_feature = TypeDef::Feature(FeatureTypeDef {
            attributes: attrs_1,
            additional_attributes: false,
        });
        assert_eq!(
            typedef_to_columns(&typedef_feature),
            vec![
                ColumnInfo {
                    name: "text".into(),
                    data_type: "TEXT".into(),
                    mime_type: None,
                },
                ColumnInfo {
                    name: "number".into(),
                    data_type: "INTEGER".into(),
                    mime_type: None,
                },
                ColumnInfo {
                    name: "date".into(),
                    data_type: "DATE".into(),
                    mime_type: None,
                },
            ]
        );

        let mut attrs_2 = IndexMap::with_hasher(ahash::RandomState::default());
        attrs_2.insert(
            "json".into(),
            Attribute::new(TypeRef::JsonString(Attribute::new(TypeRef::String).into())),
        );
        attrs_2.insert("measure".into(), Attribute::new(TypeRef::Measure));
        attrs_2.insert("bool".into(), Attribute::new(TypeRef::Boolean));
        let typedef_data = TypeDef::Data(DataTypeDef {
            attributes: attrs_2,
            additional_attributes: false,
        });
        assert_eq!(
            typedef_to_columns(&typedef_data),
            vec![
                ColumnInfo {
                    name: "json".into(),
                    data_type: "TEXT".into(),
                    mime_type: Some("application/json".into()),
                },
                ColumnInfo {
                    name: "measure".into(),
                    data_type: "REAL".into(),
                    mime_type: None,
                },
                ColumnInfo {
                    name: "bool".into(),
                    data_type: "BOOLEAN".into(),
                    mime_type: None,
                },
            ]
        );

        let typedef_property = TypeDef::Property(PropertyTypeDef { members: vec![] });
        assert_eq!(typedef_to_columns(&typedef_property), vec![]);
    }

    #[test]
    fn test_attribute_to_column() {
        let result_1 = attribute_to_column("description", &Attribute::new(TypeRef::String));
        assert_eq!(
            result_1,
            Some(ColumnInfo {
                name: "description".into(),
                data_type: "TEXT".into(),
                mime_type: None,
            })
        );

        let result_2 = attribute_to_column(
            "json",
            &Attribute::new(TypeRef::JsonString(Attribute::new(TypeRef::String).into())),
        );
        assert_eq!(
            result_2,
            Some(ColumnInfo {
                name: "json".into(),
                data_type: "TEXT".into(),
                mime_type: Some("application/json".into()),
            })
        );

        let result_3 = attribute_to_column("unknown", &Attribute::new(TypeRef::Unknown));
        assert_eq!(result_3, None);
    }
}
