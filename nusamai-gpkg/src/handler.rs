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

        // For 3D MultiPolygon features
        let mpoly3d_query = include_str!("sql/mpoly3d.sql");
        sqlx::query(mpoly3d_query).execute(&pool).await?;

        Ok(Self { pool })
    }

    ///// Connect to an existing GeoPackage database
    //pub async fn connect(path: &str) -> Result<Self, GpkgError> {
    //    let db_url = format!("sqlite://{}", path);
    //    let pool = SqlitePool::connect(&db_url).await?;
    //    Ok(Self { pool })
    //}

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
    ///
    /// Note: とりあえず地物を挿入してみるための実装です。参考にしないでください。
    pub async fn insert_feature(&mut self, bytes: &[u8]) {
        let executor = self.tx.acquire().await.unwrap();

        sqlx::query("INSERT INTO mpoly3d (geometry) VALUES (?)")
            .bind(bytes)
            .execute(&mut *executor)
            .await
            .unwrap();

        // TODO: MultiLineString
        // TODO: MultiPoint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                "mpoly3d"
            ]
        );
    }
}
