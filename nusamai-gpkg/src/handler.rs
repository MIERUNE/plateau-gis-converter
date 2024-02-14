use crate::table::TableInfo;
use indexmap::IndexMap;
use sqlx::{sqlite::*, ConnectOptions};
use sqlx::{Acquire, Row};
use sqlx::{Pool, Sqlite};
use std::path::Path;
use thiserror::Error;
use url::Url;

pub struct GpkgHandler {
    pool: Pool<Sqlite>,
}

#[derive(Error, Debug)]
pub enum GpkgError {
    #[error("Database file already exists: {0}")]
    DatabaseExists(String),
    #[error("SQLx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

impl GpkgHandler {
    /// Create and initialize new GeoPackage database at the specified path
    pub async fn from_path(path: &Path) -> Result<Self, GpkgError> {
        if path.exists() {
            return Err(GpkgError::DatabaseExists(format!("{:?}", path)));
        }
        let url = Url::parse(&format!("sqlite://{}", path.to_str().unwrap())).unwrap();
        Self::from_url(&url).await
    }

    /// Create and initialize new GeoPackage database at the specified URL
    pub async fn from_url(url: &Url) -> Result<Self, GpkgError> {
        let conn_opts = SqliteConnectOptions::from_url(url)?
            .create_if_missing(true)
            .synchronous(SqliteSynchronous::Normal)
            .journal_mode(SqliteJournalMode::Wal);
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await?;

        // Initialize the database with minimum GeoPackage schema
        let create_query = include_str!("sql/init.sql");
        sqlx::query(create_query).execute(&pool).await?;

        Ok(Self { pool })
    }

    /// Set up a table for the features / attributes
    pub async fn add_table(&self, table_info: &TableInfo) -> Result<(), GpkgError> {
        // Create the table
        let mut query = format!(
            "CREATE TABLE {} (id STRING NOT NULL PRIMARY KEY",
            table_info.name
        );
        if table_info.has_geometry {
            query.push_str(", geometry BLOB NOT NULL");
        }
        table_info.columns.iter().for_each(|column| {
            query.push_str(&format!(", {} {}", column.name, column.data_type));
        });
        query.push_str(");");
        sqlx::query(&query).execute(&self.pool).await?;

        // Add the table to `gpkg_contents`
        sqlx::query(
            "INSERT INTO gpkg_contents (table_name, data_type, identifier, srs_id) VALUES (?, ?, ?, ?);",
        )
        .bind(table_info.name.as_str())
        .bind(if table_info.has_geometry {
            "features"
        } else {
            "attributes"
        })
        .bind(table_info.name.as_str())
        .bind(4326) // Fixed for now - TODO: Change according to the data
        .execute(&self.pool)
        .await?;

        // Add the table to `gpkg_geometry_columns`
        if table_info.has_geometry {
            sqlx::query(
                "INSERT INTO gpkg_geometry_columns (table_name, column_name, geometry_type_name, srs_id, z, m) VALUES (?, ?, ?, ?, ?, ?);"
            ).bind(table_info.name.as_str())
            .bind("geometry")
            .bind("MULTIPOLYGON") // Fixed for now - TODO: Change according to the data
            .bind(4326) // Fixed for now - TODO: Change according to the data
            .bind(1)
            .bind(0)
            .execute(&self.pool)
            .await?;
        }

        // TODO: add MIME type to `gpkg_data_columns`

        Ok(())
    }

    /// Update the bounding box of a table (min_x, min_y, max_x, max_y)
    pub async fn update_bbox(
        &self,
        table_name: &str,
        (min_x, min_y, max_x, max_y): (f64, f64, f64, f64),
    ) -> Result<(), GpkgError> {
        sqlx::query("UPDATE gpkg_contents SET min_x = ?, min_y = ?, max_x = ?, max_y = ? WHERE table_name = ?;"
)
            .bind(min_x)
            .bind(min_y)
            .bind(max_x)
            .bind(max_y)
            .bind(table_name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn bbox(&self, table_name: &str) -> Result<(f64, f64, f64, f64), GpkgError> {
        let result = sqlx::query(
            "SELECT min_x, min_y, max_x, max_y FROM gpkg_contents WHERE table_name = ?;",
        )
        .bind(table_name)
        .fetch_one(&self.pool)
        .await?;
        let min_x: f64 = result.get(0);
        let min_y: f64 = result.get(1);
        let max_x: f64 = result.get(2);
        let max_y: f64 = result.get(3);
        Ok((min_x, min_y, max_x, max_y))
    }

    pub async fn application_id(&self) -> u32 {
        let result = sqlx::query("PRAGMA application_id;")
            .fetch_one(&self.pool)
            .await
            .unwrap();
        let application_id: u32 = result.get(0);
        application_id
    }

    pub async fn user_version(&self) -> u32 {
        let result = sqlx::query("PRAGMA user_version;")
            .fetch_one(&self.pool)
            .await
            .unwrap();
        let user_version: u32 = result.get(0);
        user_version
    }

    /// Get the names of all tables in the GeoPackage database
    pub async fn table_names(&self) -> Vec<String> {
        let result = sqlx::query(
            "SELECT name
            FROM sqlite_schema
            WHERE type ='table'
            AND name NOT LIKE 'sqlite_%';",
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        let mut table_names: Vec<String> = result
            .iter()
            .map(|row| row.get::<String, &str>("name"))
            .collect();
        table_names.sort();
        table_names
    }

    /// Get the table's column information (name, type, notnull)
    pub async fn table_columns(
        &self,
        table_name: &str,
    ) -> Result<Vec<(String, String, i8)>, GpkgError> {
        let result = sqlx::query(&format!("PRAGMA table_info({});", table_name))
            .fetch_all(&self.pool)
            .await?;

        let columns = result
            .iter()
            .map(|row| {
                (
                    row.get::<String, &str>("name"),
                    row.get::<String, &str>("type"),
                    row.get::<i8, &str>("notnull"),
                )
            })
            .collect();
        Ok(columns)
    }

    pub async fn gpkg_contents(&self) -> Result<Vec<(String, String, String, i32)>, GpkgError> {
        let result =
            sqlx::query("SELECT table_name, data_type, identifier, srs_id FROM gpkg_contents;")
                .fetch_all(&self.pool)
                .await?;

        let rows = result
            .iter()
            .map(|row| {
                (
                    row.get::<String, &str>("table_name"),
                    row.get::<String, &str>("data_type"),
                    row.get::<String, &str>("identifier"),
                    row.get::<i32, _>("srs_id"),
                )
            })
            .collect();
        Ok(rows)
    }

    pub async fn gpkg_geometry_columns(
        &self,
    ) -> Result<Vec<(String, String, String, i32, i8, i8)>, GpkgError> {
        let result = sqlx::query("SELECT table_name, column_name, geometry_type_name, srs_id, z, m FROM gpkg_geometry_columns;")
            .fetch_all(&self.pool)
            .await?;

        let rows = result
            .iter()
            .map(|row| {
                (
                    row.get::<String, &str>("table_name"),
                    row.get::<String, &str>("column_name"),
                    row.get::<String, &str>("geometry_type_name"),
                    row.get::<i32, _>("srs_id"),
                    row.get::<i8, _>("z"),
                    row.get::<i8, _>("m"),
                )
            })
            .collect();
        Ok(rows)
    }

    pub async fn begin(&mut self) -> Result<GpkgTransaction, GpkgError> {
        Ok(GpkgTransaction::new(self.pool.begin().await?))
    }
}

pub struct GpkgTransaction<'c> {
    tx: sqlx::Transaction<'c, Sqlite>,
}

impl<'c> GpkgTransaction<'c> {
    pub fn new(tx: sqlx::Transaction<'c, Sqlite>) -> Self {
        Self { tx }
    }

    pub async fn commit(self) -> Result<(), GpkgError> {
        Ok(self.tx.commit().await?)
    }

    /// Add a MultiPolygonZ feature to the GeoPackage database
    // TODO: generalize method
    // TODO: handle MultiLineString, MultiPoint
    pub async fn insert_feature(
        &mut self,
        bytes: &[u8],
        attributes: &IndexMap<String, String>,
    ) -> Result<(), GpkgError> {
        let executor = self.tx.acquire().await.unwrap();

        if attributes.is_empty() {
            sqlx::query("INSERT INTO mpoly3d (geometry) VALUES (?)")
                .bind(bytes)
                .execute(&mut *executor)
                .await?;
            return Ok(());
        }

        let query_string = format!(
            "INSERT INTO mpoly3d (geometry, {}) VALUES (?, {})",
            attributes
                .keys()
                .map(|key| key.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            vec!["?"; attributes.len()].join(", ")
        );
        let mut query = sqlx::query(&query_string).bind(bytes);
        for value in attributes.values() {
            query = query.bind(value);
        }
        query.execute(&mut *executor).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::ColumnInfo;

    #[tokio::test]
    async fn test_init_connect() {
        let handler = GpkgHandler::from_url(&Url::parse("sqlite::memory:").unwrap())
            .await
            .unwrap();

        let application_id = handler.application_id().await;
        assert_eq!(application_id, 1196444487);
        let user_version = handler.user_version().await;
        assert_eq!(user_version, 0); // FIXME: should be 10301

        let table_names = handler.table_names().await;
        assert_eq!(
            table_names,
            vec![
                "gpkg_contents",
                "gpkg_geometry_columns",
                "gpkg_spatial_ref_sys",
            ]
        );
    }

    #[tokio::test]
    async fn test_add_table() {
        let handler = GpkgHandler::from_url(&Url::parse("sqlite::memory:").unwrap())
            .await
            .unwrap();

        let table_name = "mpoly3d";
        let columns = vec![
            ColumnInfo {
                name: "attr1".into(),
                data_type: "TEXT".into(),
                mime_type: None,
            },
            ColumnInfo {
                name: "attr2".into(),
                data_type: "INTEGER".into(),
                mime_type: None,
            },
            ColumnInfo {
                name: "attr3".into(),
                data_type: "REAL".into(),
                mime_type: None,
            },
            ColumnInfo {
                name: "attr4".into(),
                data_type: "BOOLEAN".into(),
                mime_type: None,
            },
        ];
        let table_info = TableInfo {
            name: table_name.into(),
            has_geometry: true,
            columns,
        };

        handler.add_table(&table_info).await.unwrap();

        let table_names = handler.table_names().await;
        assert_eq!(
            table_names,
            vec![
                "gpkg_contents",
                "gpkg_geometry_columns",
                "gpkg_spatial_ref_sys",
                table_name
            ]
        );

        let columns = handler.table_columns(table_name).await.unwrap();
        assert_eq!(
            columns,
            vec![
                ("id".into(), "STRING".into(), 1),
                ("geometry".into(), "BLOB".into(), 1),
                ("attr1".into(), "TEXT".into(), 0),
                ("attr2".into(), "INTEGER".into(), 0),
                ("attr3".into(), "REAL".into(), 0),
                ("attr4".into(), "BOOLEAN".into(), 0)
            ]
        );

        let gpkg_contents = handler.gpkg_contents().await.unwrap();
        assert_eq!(
            gpkg_contents,
            vec![(
                table_name.into(),
                "features".into(),
                table_name.into(),
                4326
            )]
        );

        let gpkg_geometry_columns = handler.gpkg_geometry_columns().await.unwrap();
        assert_eq!(
            gpkg_geometry_columns,
            vec![(
                table_name.into(),
                "geometry".into(),
                "MULTIPOLYGON".into(),
                4326,
                1,
                0
            )]
        );
    }

    #[tokio::test]
    async fn test_add_table_no_geometry() {
        let handler = GpkgHandler::from_url(&Url::parse("sqlite::memory:").unwrap())
            .await
            .unwrap();

        let table_name = "without_geometry";
        let columns = vec![ColumnInfo {
            name: "attr1".into(),
            data_type: "TEXT".into(),
            mime_type: None,
        }];
        let table_info = TableInfo {
            name: table_name.into(),
            has_geometry: false, // No geometry
            columns,
        };

        handler.add_table(&table_info).await.unwrap();

        let table_names = handler.table_names().await;
        assert_eq!(
            table_names,
            vec![
                "gpkg_contents",
                "gpkg_geometry_columns",
                "gpkg_spatial_ref_sys",
                table_name
            ]
        );

        let columns = handler.table_columns(table_name).await.unwrap();
        assert_eq!(
            columns,
            vec![
                // No geometry column
                ("id".into(), "STRING".into(), 1),
                ("attr1".into(), "TEXT".into(), 0),
            ]
        );

        let gpkg_contents = handler.gpkg_contents().await.unwrap();
        assert_eq!(
            gpkg_contents,
            vec![(
                table_name.into(),
                "attributes".into(), // "attributes", not "features"
                table_name.into(),
                4326
            )]
        );

        // No record in `gpkg_geometry_columns`
        let gpkg_geometry_columns = handler.gpkg_geometry_columns().await.unwrap();
        assert!(gpkg_geometry_columns.is_empty());
    }

    #[tokio::test]
    async fn test_bbox() {
        let handler = GpkgHandler::from_url(&Url::parse("sqlite::memory:").unwrap())
            .await
            .unwrap();

        let table_name = "mpoly3d";
        let table_info = TableInfo {
            name: table_name.into(),
            has_geometry: true,
            columns: vec![],
        };
        handler.add_table(&table_info).await.unwrap();

        // initial values written in `mpoly3d.sql`
        let (min_x, min_y, max_x, max_y) = handler.bbox(table_name).await.unwrap();
        assert_eq!(min_x, 0.0);
        assert_eq!(min_y, 0.0);
        assert_eq!(max_x, 0.0);
        assert_eq!(max_y, 0.0);

        handler
            .update_bbox(table_name, (-111.0, 222.0, 333.0, -444.0))
            .await
            .unwrap();
        let (min_x, min_y, max_x, max_y) = handler.bbox(table_name).await.unwrap();
        assert_eq!(min_x, -111.0);
        assert_eq!(min_y, 222.0);
        assert_eq!(max_x, 333.0);
        assert_eq!(max_y, -444.0);
    }
}
