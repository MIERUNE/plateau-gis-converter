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
    pub async fn add_object(&self, _object: &TopLevelCityObject) {
        todo!();
    }

    /// Add TopLevelCityObjects to the GeoPackage database
    pub async fn add_objects(&self, _objects: &[TopLevelCityObject]) {
        todo!();
    }

    fn geometry_header() -> Vec<u8> {
        let mut header: Vec<u8> = vec![];
        header.extend_from_slice(&[0x47, 0x50]); // Magic number
        header.push(0x00); // Version
        header.push(0b00000001); // Flags
        header.extend_from_slice(&i32::to_le_bytes(4326)); // SRS ID
        header
    }

    /// https://www.geopackage.org/spec130/#gpb_format
    pub async fn test_insert(&self) {
        let mut bytes: Vec<u8> = Self::geometry_header();

        // Byte order: Little endian
        bytes.push(0x01);

        // Geometry type: wkbMultiPolygonZ (1006)
        bytes.extend_from_slice(&1006_u32.to_le_bytes());

        // numPolygons
        bytes.extend_from_slice(&1_u32.to_le_bytes());

        // Byte order: Little endian
        bytes.push(0x01);

        // Geometry type: wkbPolygonZ (1003)
        bytes.extend_from_slice(&1003_u32.to_le_bytes());

        // numRings
        bytes.extend_from_slice(&1_u32.to_le_bytes());

        // LinearRingZ
        let coords = [
            (140., 43., 99.),
            (140., 42., 99.),
            (141., 42., 99.),
            (141., 43., 99.),
            (140., 43., 99.),
        ];

        // numPoints
        bytes.extend_from_slice(&(coords.len() as u32).to_le_bytes());

        for c in coords {
            let x = f64::to_le_bytes(c.0);
            bytes.extend_from_slice(&x);
            let y = f64::to_le_bytes(c.1);
            bytes.extend_from_slice(&y);
            let z = f64::to_le_bytes(c.2);
            bytes.extend_from_slice(&z);
        }

        sqlx::query("INSERT INTO mpoly3d (geometry) VALUES (?)")
            .bind(bytes)
            .execute(&self.pool)
            .await
            .unwrap();
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
