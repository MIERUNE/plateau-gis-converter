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

        // For 3D MultiPolygon features
        let mpoly3d_query = include_str!("sql/mpoly3d.sql");
        sqlx::query(mpoly3d_query).execute(&pool).await?;

        Ok(Self { pool })
    }

    /// Add columns to a table
    pub async fn add_columns(
        &self,
        attribute_columns: IndexMap<String, String>,
    ) -> Result<(), GpkgError> {
        for (column_name, column_type) in attribute_columns {
            let add_columns_query = format!(
                "ALTER TABLE mpoly3d ADD COLUMN {} {};",
                column_name, column_type
            );
            sqlx::query(&add_columns_query).execute(&self.pool).await?;
        }
        Ok(())
    }

    pub async fn update_bbox(
        &self,
        table_name: String,
        min_x: f64,
        min_y: f64,
        max_x: f64,
        max_y: f64,
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

    pub async fn bbox(&self, table_name: String) -> Result<(f64, f64, f64, f64), GpkgError> {
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
    pub async fn table_info(
        &self,
        table_name: String,
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

    #[tokio::test]
    async fn test_add_columns() {
        let handler = GpkgHandler::from_url(&Url::parse("sqlite::memory:").unwrap())
            .await
            .unwrap();

        let mut attribute_columns = IndexMap::new();
        attribute_columns.insert("attr1".into(), "TEXT".into());
        attribute_columns.insert("attr2".into(), "INTEGER".into());
        attribute_columns.insert("attr3".into(), "REAL".into());
        attribute_columns.insert("attr4".into(), "BOOLEAN".into());
        handler.add_columns(attribute_columns).await.unwrap();

        let columns = handler.table_info("mpoly3d".into()).await.unwrap();
        assert_eq!(
            columns,
            vec![
                ("id".into(), "INTEGER".into(), 1),
                ("geometry".into(), "BLOB".into(), 1),
                ("attr1".into(), "TEXT".into(), 0),
                ("attr2".into(), "INTEGER".into(), 0),
                ("attr3".into(), "REAL".into(), 0),
                ("attr4".into(), "BOOLEAN".into(), 0)
            ]
        );
    }

    #[tokio::test]
    async fn test_bbox() {
        let handler = GpkgHandler::from_url(&Url::parse("sqlite::memory:").unwrap())
            .await
            .unwrap();

        // initial values written in `mpoly3d.sql`
        let (min_x, min_y, max_x, max_y) = handler.bbox("mpoly3d".into()).await.unwrap();
        assert_eq!(min_x, 122.93250000);
        assert_eq!(min_y, 20.42527778);
        assert_eq!(max_x, 153.98666667);
        assert_eq!(max_y, 45.55722222);

        handler
            .update_bbox("mpoly3d".into(), -111.0, 222.0, 333.0, -444.0)
            .await
            .unwrap();
        let (min_x, min_y, max_x, max_y) = handler.bbox("mpoly3d".into()).await.unwrap();
        assert_eq!(min_x, -111.0);
        assert_eq!(min_y, 222.0);
        assert_eq!(max_x, 333.0);
        assert_eq!(max_y, -444.0);
    }
}
