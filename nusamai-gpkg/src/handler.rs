use crate::geometry::multipolygon_to_bytes;
use nusamai_plateau::TopLevelCityObject;
use sqlx::Row;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use std::path::Path;
use thiserror::Error;

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
    /// Create and initialize new GeoPackage database
    pub async fn init(path: &str) -> Result<Self, GpkgError> {
        if Path::new(path).exists() {
            return Err(GpkgError::DatabaseExists(path.to_string()));
        }

        let db_url = format!("sqlite://{}", path);

        Sqlite::create_database(&db_url).await?;
        let pool = SqlitePool::connect(&db_url).await?;

        // Initialize the database with minimum GeoPackage schema
        let create_query = include_str!("sql/init.sql");
        sqlx::query(create_query).execute(&pool).await?;

        // For 3D MultiPolygon features
        let mpoly3d_query = include_str!("sql/mpoly3d.sql");
        sqlx::query(mpoly3d_query).execute(&pool).await?;

        Ok(Self { pool })
    }

    /// Connect to an existing GeoPackage database
    pub async fn connect(path: &str) -> Result<Self, GpkgError> {
        let db_url = format!("sqlite://{}", path);
        let pool = SqlitePool::connect(&db_url).await?;
        Ok(Self { pool })
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

    /// Add a TopLevelCityObjects to the GeoPackage database
    pub async fn add_object(&self, obj: &TopLevelCityObject) {
        if !obj.geometries.multipolygon.is_empty() {
            let bytes =
                multipolygon_to_bytes(&obj.geometries.vertices, &obj.geometries.multipolygon);

            sqlx::query("INSERT INTO mpoly3d (geometry) VALUES (?)")
                .bind(bytes)
                .execute(&self.pool)
                .await
                .unwrap();
        };

        // TODO: MultiLineString
        // TODO: MultiPoint
    }

    /// Add TopLevelCityObjects to the GeoPackage database
    pub async fn add_objects(&self, _objects: &[TopLevelCityObject]) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_connect() {
        let handler = GpkgHandler::init("sqlite::memory:").await.unwrap();
        let _handler2 = GpkgHandler::connect("sqlite::memory:").await.unwrap();

        let application_id = handler.application_id().await;
        assert_eq!(application_id, 1196444487);
        let user_version = handler.user_version().await;
        assert_eq!(user_version, 0); // FIXME: should be 10200

        let table_names = handler.table_names().await;
        assert_eq!(
            table_names,
            vec![
                "gpkg_contents",
                "gpkg_geometry_columns",
                "gpkg_spatial_ref_sys",
                "mpoly3d"
            ]
        );
    }
}
