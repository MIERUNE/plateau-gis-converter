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
        let create_query = include_str!("sql/gpkg_template.sql");
        sqlx::query(create_query).execute(&pool).await?;

        Ok(Self { pool })
    }

    /// Connect to an existing GeoPackage database
    pub async fn connect(path: &str) -> Result<Self, GpkgError> {
        let db_url = format!("sqlite://{}", path);
        let pool = SqlitePool::connect(&db_url).await?;
        Ok(Self { pool })
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
    pub async fn add_object(&self, _object: &TopLevelCityObject) {
        todo!();
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
}
