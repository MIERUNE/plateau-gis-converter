use nusamai_plateau::TopLevelCityObject;
use sqlx::Row;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use thiserror::Error;

pub struct GpkgHandler {
    pool: Pool<Sqlite>,
}

#[derive(Error, Debug)]
pub enum GpkgError {
    #[error("Database already exists: {0}")]
    DatabaseExists(String),
    #[error("Database does not exist: {0}")]
    DatabaseDoesNotExist(String),
    #[error("SQLx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

impl GpkgHandler {
    async fn database_exists(db_url: &str) -> Result<bool, sqlx::Error> {
        Sqlite::database_exists(db_url).await
    }

    /// Create and initialize new GeoPackage database
    pub async fn init(path: &str) -> Result<Self, GpkgError> {
        let db_url = format!("sqlite://{}", path);

        if Self::database_exists(&db_url).await? {
            return Err(GpkgError::DatabaseExists(path.to_string()));
        }

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
        if !Self::database_exists(&db_url).await? {
            return Err(GpkgError::DatabaseDoesNotExist(path.to_string()));
        }

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
    use std::fs::remove_file;
    use std::path::Path;

    #[tokio::test]
    async fn test_init_connect() {
        let test_file_path = "test.gpkg";
        if Path::new(test_file_path).exists() {
            panic!("The test file '{}' already exists", test_file_path);
        }

        let handler = GpkgHandler::init(test_file_path).await.unwrap();
        let _handler2 = GpkgHandler::connect(test_file_path).await.unwrap();

        let table_names = handler.table_names().await;
        assert_eq!(
            table_names,
            vec![
                "gpkg_contents",
                "gpkg_extensions",
                "gpkg_geometry_columns",
                "gpkg_spatial_ref_sys",
                "gpkg_tile_matrix",
                "gpkg_tile_matrix_set"
            ]
        );

        remove_file(test_file_path).unwrap();
        remove_file(format!("{}-shm", test_file_path)).unwrap();
        remove_file(format!("{}-wal", test_file_path)).unwrap();
    }
}
